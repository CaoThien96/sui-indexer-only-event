use anyhow::Result;
use indexer_store::{FactTopic, KafkaFactReader, parse_envelope, wait_for_topics_available};
use prometheus::Registry;
use rdkafka::Message;
use tracing::{error, info, warn};

use sui_processors::catalog::{handle_pool_message, handle_token_metadata_message};
use sui_processors::config::{
    self, catalog_consumer_group, database_url, kafka_brokers, kafka_client_id, metrics_address,
};
use sui_processors::metrics::ProcessorMetrics;
use sui_processors::runtime::{kafka_backoff_resubscribe, serve_metrics};
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

    let pool_reader = KafkaFactReader::new(
        &brokers,
        &format!("{group}-pools"),
        &kafka_client_id("catalog-pools"),
    )?;
    pool_reader.subscribe(&[FactTopic::PoolRaw])?;

    let token_reader = KafkaFactReader::new(
        &brokers,
        &format!("{group}-tokens"),
        &kafka_client_id("catalog-tokens"),
    )?;
    token_reader.subscribe(&[FactTopic::TokenMetadataRaw])?;

    let store_pools = store.clone();
    let metrics_pools = metrics.clone();
    let pool_topics = [FactTopic::PoolRaw];
    let pool_task = tokio::spawn(async move {
        loop {
            let message = match pool_reader.recv_raw().await {
                Ok(m) => m,
                Err(e) => {
                    error!(error = %e, "Pool consumer recv failed");
                    kafka_backoff_resubscribe(&pool_reader, &pool_topics, "catalog-pools").await;
                    continue;
                }
            };

            let envelope = match parse_envelope(&message) {
                Ok(e) => e,
                Err(e) => {
                    warn!(
                        error = %e,
                        topic = message.topic(),
                        partition = message.partition(),
                        offset = message.offset(),
                        "Skipping invalid pool envelope"
                    );
                    metrics_pools
                        .decode_errors
                        .with_label_values(&["catalog-worker", FactTopic::PoolRaw.as_str()])
                        .inc();
                    let _ = pool_reader.commit_message(&message);
                    continue;
                }
            };

            if let Err(e) =
                handle_pool_message(&store_pools, &metrics_pools, &envelope).await
            {
                error!(error = %e, "Failed to handle pool message");
                metrics_pools
                    .decode_errors
                    .with_label_values(&["catalog-worker", FactTopic::PoolRaw.as_str()])
                    .inc();
            } else if let Err(e) = pool_reader.commit_message(&message) {
                error!(error = %e, "Failed to commit pool offset");
            }
        }
    });

    let store_tokens = store.clone();
    let metrics_tokens = metrics.clone();
    let token_topics = [FactTopic::TokenMetadataRaw];
    let token_task = tokio::spawn(async move {
        loop {
            let message = match token_reader.recv_raw().await {
                Ok(m) => m,
                Err(e) => {
                    error!(error = %e, "Token consumer recv failed");
                    kafka_backoff_resubscribe(&token_reader, &token_topics, "catalog-tokens").await;
                    continue;
                }
            };

            let envelope = match parse_envelope(&message) {
                Ok(e) => e,
                Err(e) => {
                    warn!(
                        error = %e,
                        topic = message.topic(),
                        partition = message.partition(),
                        offset = message.offset(),
                        "Skipping invalid token envelope"
                    );
                    metrics_tokens
                        .decode_errors
                        .with_label_values(&[
                            "catalog-worker",
                            FactTopic::TokenMetadataRaw.as_str(),
                        ])
                        .inc();
                    let _ = token_reader.commit_message(&message);
                    continue;
                }
            };

            if let Err(e) =
                handle_token_metadata_message(&store_tokens, &metrics_tokens, &envelope).await
            {
                error!(error = %e, "Failed to handle token metadata message");
                metrics_tokens
                    .decode_errors
                    .with_label_values(&[
                        "catalog-worker",
                        FactTopic::TokenMetadataRaw.as_str(),
                    ])
                    .inc();
            } else if let Err(e) = token_reader.commit_message(&message) {
                error!(error = %e, "Failed to commit token offset");
            }
        }
    });

    info!("catalog-worker started");
    tokio::select! {
        _ = pool_task => {},
        _ = token_task => {},
        result = serve_metrics(registry, &metrics_addr) => result?,
    }

    Ok(())
}
