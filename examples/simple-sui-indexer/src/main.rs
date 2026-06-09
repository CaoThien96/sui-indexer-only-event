mod app_metrics;
mod bootstrap;
mod handlers;
mod metrics_config;
mod models;
mod prefix;
mod static_event_decode;
mod telegram_notify;

use handlers::EventTypeHandler;

pub mod schema;

use std::sync::Arc;

use anyhow::{Result, bail};
use clap::Parser;
use diesel_migrations::{EmbeddedMigrations, embed_migrations};
use sui_indexer_alt_framework::{
    cluster::Args,
    pipeline::{CommitterConfig, sequential::SequentialConfig},
    postgres::DbArgs,
    service::Error,
};
use tracing::info;
use url::Url;

use crate::bootstrap::{IndexerRuntime, log_key_builtin_metrics};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
const COLLECT_INTERVAL_MS: u64 = 100;
fn sequential_config() -> SequentialConfig {
    SequentialConfig {
        committer: CommitterConfig {
            collect_interval_ms: COLLECT_INTERVAL_MS,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn parse_event_type_prefixes() -> Result<Vec<String>> {
    let raw = std::env::var("EVENT_TYPE_PREFIXES").map_err(|_| {
        anyhow::anyhow!(
            "EVENT_TYPE_PREFIXES must be set (comma-separated Move event type prefixes)"
        )
    })?;

    let prefixes: Vec<String> = raw
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect();

    if prefixes.is_empty() {
        bail!("EVENT_TYPE_PREFIXES is empty after parsing; provide at least one prefix");
    }

    for prefix in &prefixes {
        if !prefix.contains("::") {
            bail!(
                "Invalid prefix '{prefix}': each prefix must contain '::' (e.g. 0xabc:: or 0xabc::pool)"
            );
        }
    }

    Ok(prefixes)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment")
        .parse::<Url>()
        .expect("Invalid database URL");

    let mut args = Args::parse();
    metrics_config::apply_metrics_env(&mut args);
    metrics_config::log_metrics_endpoint(&args);

    info!("Parsed CLI arguments and initialized runtime");
    let event_type_prefixes = parse_event_type_prefixes()?;
    info!(
        event_type_prefix_count = event_type_prefixes.len(),
        event_type_prefixes = ?event_type_prefixes,
        "Loaded EVENT_TYPE_PREFIXES configuration"
    );

    let metrics_prefix = metrics_config::metrics_prefix_from_env();
    if let Some(ref prefix) = metrics_prefix {
        info!(metrics_prefix = %prefix, "Using Prometheus metric name prefix");
    }

    let pipeline_config = sequential_config();
    info!(
        collect_interval_ms = COLLECT_INTERVAL_MS,
        "Using sequential pipeline config"
    );

    let mut runtime = IndexerRuntime::build(
        database_url,
        DbArgs::default(),
        args,
        sui_indexer_alt_framework::ingestion::IngestionConfig::default(),
        &MIGRATIONS,
        metrics_prefix,
    )
    .await?;
    info!("Indexer runtime initialized (Postgres + Prometheus)");

    let app_metrics = Arc::clone(&runtime.app_metrics);
    runtime
        .indexer
        .sequential_pipeline(
            EventTypeHandler::new(event_type_prefixes, app_metrics),
            pipeline_config,
        )
        .await?;
    info!("Event prefix pipeline registered, indexer is starting");

    log_key_builtin_metrics(&runtime.indexer, EventTypeHandler::NAME);

    match runtime.run().await {
        Ok(()) | Err(Error::Terminated) => {
            info!("Indexer terminated normally");
            Ok(())
        }
        Err(Error::Aborted) => bail!("Indexer aborted due to an unexpected error"),
        Err(Error::Task(e)) => bail!(e),
    }
}
