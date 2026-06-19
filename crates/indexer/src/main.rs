mod bootstrap;
mod config;
mod pipelines;

use anyhow::{Result, bail};
use bootstrap::{IndexerRuntime, log_key_builtin_metrics};
use config::{
    AppArgs, ingestion_config, kafka_client_id, log_indexer_args, log_metrics_endpoint,
    metrics_prefix_from_env, require_database_url, require_kafka_brokers,
};
use diesel_migrations::{EmbeddedMigrations, embed_migrations};
use indexer_store::PostgresStore;
use pipelines::{
    common::AppMetrics,
    dex_pool::{self, DexPoolHandler},
    dex_swap::{self, DexSwapHandler},
    token_metadata::{self, TokenMetadataHandler},
};
use sui_indexer_alt_framework::{
    pipeline::{CommitterConfig, sequential::SequentialConfig},
    service::Error,
};
use tracing::info;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
const COLLECT_INTERVAL_MS: u64 = 500;

fn sequential_config() -> SequentialConfig {
    SequentialConfig {
        committer: CommitterConfig {
            collect_interval_ms: COLLECT_INTERVAL_MS,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    let app_args = AppArgs::parse_with_env()?;
    let args = app_args.framework;

    log_metrics_endpoint(&args);
    log_indexer_args(&args.indexer_args);

    let database_url = require_database_url()?;
    let kafka_brokers = require_kafka_brokers()?;
    let kafka_client_id = kafka_client_id();

    let metrics_prefix = metrics_prefix_from_env();
    if let Some(ref prefix) = metrics_prefix {
        info!(metrics_prefix = %prefix, "Using Prometheus metric name prefix");
    }

    let pg =
        PostgresStore::for_write(database_url.clone(), indexer_store::DbArgs::default()).await?;
    pg.run_migrations(MIGRATIONS).await?;
    info!("Postgres migrations applied (watermarks only)");

    let mut runtime = IndexerRuntime::build(
        database_url,
        &kafka_brokers,
        &kafka_client_id,
        args,
        ingestion_config(),
        metrics_prefix,
    )
    .await?;
    info!("Indexer runtime initialized (CompositeStore + Prometheus)");

    let app_metrics = AppMetrics::new(runtime.metrics_registry())?;

    let dex_swap = DexSwapHandler::new(app_metrics.clone());
    runtime
        .indexer
        .sequential_pipeline(dex_swap.clone(), sequential_config())
        .await?;
    info!(
        pipeline = dex_swap::NAME,
        collect_interval_ms = COLLECT_INTERVAL_MS,
        "DEX swap pipeline registered"
    );

    let dex_pool = DexPoolHandler::new(app_metrics.clone());
    runtime
        .indexer
        .sequential_pipeline(dex_pool.clone(), sequential_config())
        .await?;
    info!(
        pipeline = dex_pool::NAME,
        collect_interval_ms = COLLECT_INTERVAL_MS,
        "DEX pool pipeline registered"
    );

    let token_metadata = TokenMetadataHandler::new(app_metrics);
    runtime
        .indexer
        .sequential_pipeline(token_metadata.clone(), sequential_config())
        .await?;
    info!(
        pipeline = token_metadata::NAME,
        collect_interval_ms = COLLECT_INTERVAL_MS,
        "Token metadata pipeline registered"
    );

    log_key_builtin_metrics(&runtime.indexer, dex_swap::NAME);
    log_key_builtin_metrics(&runtime.indexer, dex_pool::NAME);
    log_key_builtin_metrics(&runtime.indexer, token_metadata::NAME);

    match runtime.run().await {
        Ok(()) | Err(Error::Terminated) => {
            info!("Indexer terminated normally");
            Ok(())
        }
        Err(Error::Aborted) => bail!("Indexer aborted due to an unexpected error"),
        Err(Error::Task(e)) => bail!(e),
    }
}
