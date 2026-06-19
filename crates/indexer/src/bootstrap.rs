use anyhow::{Context, Result};
use crate::runtime_tuning::db_args;
use indexer_store::{CompositeStore, KafkaFactWriter, PostgresStore};
use prometheus::Registry;
use sui_indexer_alt_framework::cluster::Args;
use sui_indexer_alt_framework::{Indexer, ingestion::IngestionConfig, service::Error};
use sui_indexer_alt_metrics::{MetricsService, uptime};
use tracing::info;
use url::Url;

pub struct IndexerRuntime {
    pub indexer: Indexer<CompositeStore>,
    metrics: MetricsService,
}

impl IndexerRuntime {
    pub fn metrics_registry(&self) -> &Registry {
        self.metrics.registry()
    }
    pub async fn build(
        database_url: Url,
        kafka_brokers: &str,
        kafka_client_id: &str,
        args: Args,
        ingestion_config: IngestionConfig,
        metrics_prefix: Option<String>,
    ) -> Result<Self> {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();

        let registry = Registry::new();
        registry
            .register(uptime(env!("CARGO_PKG_VERSION"))?)
            .context("failed to register uptime metric")?;

        let kafka = KafkaFactWriter::new(kafka_brokers, kafka_client_id, &registry)?;
        let pg = PostgresStore::for_write(database_url, db_args()).await?;
        let store = CompositeStore::new(pg, kafka);

        let metrics = MetricsService::new(args.metrics_args.clone(), registry);

        let indexer = Indexer::new(
            store,
            args.indexer_args,
            args.client_args,
            ingestion_config,
            metrics_prefix.as_deref(),
            metrics.registry(),
        )
        .await?;

        Ok(Self { indexer, metrics })
    }

    pub async fn run(self) -> Result<(), Error> {
        let s_indexer = self
            .indexer
            .run()
            .await
            .map_err(|e| Error::Task(e.into()))?;
        let s_metrics = self
            .metrics
            .run()
            .await
            .map_err(|e| Error::Task(e.into()))?;
        s_indexer.attach(s_metrics).main().await
    }
}

pub fn log_key_builtin_metrics(indexer: &Indexer<CompositeStore>, pipeline: &str) {
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
        watermark_checkpoint = m
            .watermark_checkpoint_in_db
            .with_label_values(&[pipeline])
            .get(),
        "Indexer metrics snapshot"
    );
}
