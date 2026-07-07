use std::time::Duration;

use anyhow::Result;
use indexer_store::{FactTopic, KafkaFactReader, wait_for_topics_available};
use prometheus::Registry;
use tracing::{error, info};

use sui_processors::catalog::{handle_pool_message, handle_token_metadata_message};
use sui_processors::config::{
    self, catalog_consumer_group, database_url, kafka_brokers, kafka_client_id, metrics_address,
};
use sui_processors::metrics::ProcessorMetrics;
use sui_processors::runtime::{drain_kafka_pipeline, serve_metrics, spawn_kafka_poll_task};
use sui_processors::store::CatalogStore;

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    config::load_dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let max_poll_ms = std::env::var("KAFKA_MAX_POLL_INTERVAL_MS")
        .unwrap_or_else(|_| "900000".to_string());
    info!(%max_poll_ms, "catalog-worker kafka consumer tuning");

    let db_url = database_url()?;
    let brokers = kafka_brokers()?;
    let group = catalog_consumer_group();

    wait_for_topics_available(
        &brokers,
        &[FactTopic::PoolRaw, FactTopic::TokenMetadataRaw],
    )
    .await?;

    let store = CatalogStore::connect(db_url).await?;
    store.run_migrations().await?;
    let seeded = store.seed_protocols_if_empty().await?;
    info!(seeded, "Protocol catalog seed complete");

    let registry = Registry::new();
    let metrics = ProcessorMetrics::new(&registry)?;
    let metrics_addr = metrics_address();

    let store_pools = store.clone();
    let metrics_pools = metrics.clone();
    let brokers_pools = brokers.clone();
    let group_pools = format!("{group}-pools");
    let pool_task = tokio::spawn(async move {
        let pool_reader = KafkaFactReader::new(
            &brokers_pools,
            &group_pools,
            &kafka_client_id("catalog-pools"),
        )?;
        pool_reader.subscribe(&[FactTopic::PoolRaw]).await?;

        let rx = spawn_kafka_poll_task(pool_reader.clone(), &[FactTopic::PoolRaw], "catalog-pools");
        let metrics = metrics_pools.clone();
        drain_kafka_pipeline(
            &pool_reader,
            rx,
            "catalog-pools",
            "catalog-worker",
            move |worker| {
                metrics
                    .decode_errors
                    .with_label_values(&[worker, FactTopic::PoolRaw.as_str()])
                    .inc();
            },
            |envelope| {
                let store = store_pools.clone();
                let metrics = metrics_pools.clone();
                async move { handle_pool_message(&store, &metrics, &envelope).await }
            },
        )
        .await;
        Ok::<(), anyhow::Error>(())
    });

    let store_tokens = store.clone();
    let metrics_tokens = metrics.clone();
    let brokers_tokens = brokers.clone();
    let group_tokens = format!("{group}-tokens");
    let token_task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(500)).await;

        let token_reader = KafkaFactReader::new(
            &brokers_tokens,
            &group_tokens,
            &kafka_client_id("catalog-tokens"),
        )?;
        token_reader
            .subscribe(&[FactTopic::TokenMetadataRaw])
            .await?;

        let rx = spawn_kafka_poll_task(
            token_reader.clone(),
            &[FactTopic::TokenMetadataRaw],
            "catalog-tokens",
        );
        let metrics = metrics_tokens.clone();
        drain_kafka_pipeline(
            &token_reader,
            rx,
            "catalog-tokens",
            "catalog-worker",
            move |worker| {
                metrics
                    .decode_errors
                    .with_label_values(&[worker, FactTopic::TokenMetadataRaw.as_str()])
                    .inc();
            },
            |envelope| {
                let store = store_tokens.clone();
                let metrics = metrics_tokens.clone();
                async move { handle_token_metadata_message(&store, &metrics, &envelope).await }
            },
        )
        .await;
        Ok::<(), anyhow::Error>(())
    });

    info!("catalog-worker started");
    tokio::select! {
        result = pool_task => {
            if let Ok(Err(e)) = result {
                error!(error = %e, "Pool consumer task exited with error");
            }
        },
        result = token_task => {
            if let Ok(Err(e)) = result {
                error!(error = %e, "Token consumer task exited with error");
            }
        },
        result = serve_metrics(registry, &metrics_addr) => result?,
    }

    Ok(())
}
