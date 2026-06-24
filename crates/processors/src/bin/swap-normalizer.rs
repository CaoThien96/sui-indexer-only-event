use std::sync::Arc;

use anyhow::Result;
use indexer_store::{FactTopic, KafkaFactReader, KafkaFactWriter, wait_for_topics_available};
use prometheus::Registry;
use tracing::info;

use sui_processors::config::{
    self, database_url, kafka_brokers, kafka_client_id, metrics_address,
    swap_normalizer_consumer_group,
};
use sui_processors::metrics::ProcessorMetrics;
use sui_processors::runtime::{poll_kafka_envelope, serve_metrics};
use sui_processors::store::CatalogStore;
use sui_processors::swap_normalizer::{SwapNormalizer, normalized_partition_key};

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
    let group = swap_normalizer_consumer_group();

    wait_for_topics_available(&brokers, &[FactTopic::SwapRaw]).await?;

    let store = CatalogStore::connect(db_url).await?;
    store.run_migrations().await?;

    let registry = Registry::new();
    let metrics = ProcessorMetrics::new(&registry)?;
    let normalizer = Arc::new(SwapNormalizer::new(store, metrics.clone()));

    let reader = KafkaFactReader::new(
        &brokers,
        &group,
        &kafka_client_id("swap-normalizer"),
    )?;
    reader.subscribe(&[FactTopic::SwapRaw])?;

    let writer = KafkaFactWriter::new(
        &brokers,
        &kafka_client_id("swap-normalizer-producer"),
        &registry,
    )?;

    let metrics_addr = metrics_address();
    info!("swap-normalizer started");

    let topics = [FactTopic::SwapRaw];
    tokio::select! {
        _ = async {
            loop {
                let metrics = metrics.clone();
                let normalizer = Arc::clone(&normalizer);
                let writer = writer.clone();
                poll_kafka_envelope(&reader, &topics, "swap-normalizer", move |envelope| {
                    let metrics = metrics.clone();
                    let normalizer = Arc::clone(&normalizer);
                    let writer = writer.clone();
                    async move {
                        match normalizer.normalize(&envelope).await {
                            Ok(Some(normalized)) => {
                                let protocol = normalized
                                    .payload
                                    .get("protocol")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("unknown")
                                    .to_string();
                                writer
                                    .publish(
                                        FactTopic::SwapNormalized,
                                        &[normalized],
                                        normalized_partition_key,
                                    )
                                    .await?;
                                metrics
                                    .swap_normalized
                                    .with_label_values(&[&protocol])
                                    .inc();
                            }
                            Ok(None) => {}
                            Err(e) => {
                                metrics
                                    .decode_errors
                                    .with_label_values(&[
                                        "swap-normalizer",
                                        FactTopic::SwapRaw.as_str(),
                                    ])
                                    .inc();
                                return Err(e);
                            }
                        }
                        Ok(())
                    }
                })
                .await;
            }
        } => {},
        result = serve_metrics(registry, &metrics_addr) => result?,
    }

    Ok(())
}
