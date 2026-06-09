//! Bootstrap indexer + Prometheus metrics on a shared registry.
//!
//! Mirrors `IndexerCluster::build` but registers app metrics and uptime on the same
//! `/metrics` endpoint as framework built-ins (ingestion, pipeline, watermark, DB).

use std::sync::Arc;

use anyhow::{Context, Result};
use diesel_migrations::EmbeddedMigrations;
use prometheus::Registry;
use sui_indexer_alt_framework::{
    Indexer, cluster::Args, ingestion::IngestionConfig, postgres::Db, postgres::DbArgs,
    service::Error,
};
use sui_indexer_alt_metrics::{MetricsService, uptime};
use tracing::info;
use url::Url;

use crate::app_metrics::AppMetrics;

pub struct IndexerRuntime {
    pub indexer: Indexer<Db>,
    metrics: MetricsService,
    pub app_metrics: Arc<AppMetrics>,
}

impl IndexerRuntime {
    pub async fn build(
        database_url: Url,
        db_args: DbArgs,
        args: Args,
        ingestion_config: IngestionConfig,
        migrations: &'static EmbeddedMigrations,
        metrics_prefix: Option<String>,
    ) -> Result<Self> {
        tracing_subscriber::fmt::init();

        let registry = Registry::new();
        let app_metrics = Arc::new(AppMetrics::register(&registry)?);
        registry
            .register(uptime(env!("CARGO_PKG_VERSION"))?)
            .context("failed to register uptime metric")?;

        let metrics = MetricsService::new(args.metrics_args.clone(), registry);

        let indexer = Indexer::new_from_pg(
            database_url,
            db_args,
            args.indexer_args,
            args.client_args,
            ingestion_config,
            Some(migrations),
            metrics_prefix.as_deref(),
            metrics.registry(),
        )
        .await?;

        Ok(Self {
            indexer,
            metrics,
            app_metrics,
        })
    }

    pub async fn run(self) -> Result<(), Error> {
        let s_indexer = self.indexer.run().await?;
        let s_metrics = self.metrics.run().await?;
        s_indexer.attach(s_metrics).main().await
    }
}

pub fn log_key_builtin_metrics(indexer: &Indexer<Db>, pipeline: &str) {
    let m = indexer.indexer_metrics();
    let ingestion = indexer.ingestion_metrics();

    info!(
        pipeline,
        ingested_checkpoints = ingestion.total_ingested_checkpoints.get(),
        latest_ingested_checkpoint = ingestion.latest_ingested_checkpoint.get(),
        processed_checkpoint = m
            .latest_processed_checkpoint
            .with_label_values(&[pipeline])
            .get(),
        processor_retries = m
            .total_handler_processor_retries
            .with_label_values(&[pipeline])
            .get(),
        watermark_checkpoint = m
            .watermark_checkpoint_in_db
            .with_label_values(&[pipeline])
            .get(),
        "Indexer metrics snapshot"
    );
}
