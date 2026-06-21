use anyhow::Result;
use indexer_store::{FactTopic, KafkaFactReader, KafkaFactWriter};
use prometheus::Registry;
use tracing::{error, info};

use sui_processors::config::{
    self, database_url, kafka_brokers, kafka_client_id, metrics_address,
    swap_normalizer_consumer_group,
};
use sui_processors::metrics::ProcessorMetrics;
use sui_processors::runtime::serve_metrics;
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

    let store = CatalogStore::connect(db_url).await?;
    store.run_migrations().await?;

    let registry = Registry::new();
    let metrics = ProcessorMetrics::new(&registry)?;
    let normalizer = SwapNormalizer::new(store, metrics.clone());

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

    tokio::select! {
        _ = async {
            loop {
                match reader.recv_envelope().await {
                    Ok((envelope, message)) => {
                        match normalizer.normalize(&envelope).await {
                            Ok(Some(normalized)) => {
                                let protocol = normalized
                                    .payload
                                    .get("protocol")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("unknown")
                                    .to_string();
                                match writer
                                    .publish(
                                        FactTopic::SwapNormalized,
                                        &[normalized],
                                        normalized_partition_key,
                                    )
                                    .await
                                {
                                    Ok(_) => {
                                        metrics
                                            .swap_normalized
                                            .with_label_values(&[&protocol])
                                            .inc();
                                    }
                                    Err(e) => {
                                        error!(error = %e, "Failed to publish normalized swap");
                                    }
                                }
                            }
                            Ok(None) => {}
                            Err(e) => {
                                error!(error = %e, "Failed to normalize swap");
                                metrics
                                    .decode_errors
                                    .with_label_values(&["swap-normalizer", FactTopic::SwapRaw.as_str()])
                                    .inc();
                            }
                        }
                        if let Err(e) = reader.commit_message(&message) {
                            error!(error = %e, "Failed to commit swap offset");
                        }
                    }
                    Err(e) => {
                        error!(error = %e, "Swap consumer error");
                        metrics
                            .decode_errors
                            .with_label_values(&["swap-normalizer", FactTopic::SwapRaw.as_str()])
                            .inc();
                    }
                }
            }
        } => {},
        result = serve_metrics(registry, &metrics_addr) => result?,
    }

    Ok(())
}
