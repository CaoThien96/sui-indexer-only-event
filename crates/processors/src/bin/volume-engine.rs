use anyhow::Result;
use indexer_store::{FactTopic, KafkaFactReader};
use prometheus::Registry;
use tracing::{error, info};

use sui_processors::config::{
    self, kafka_brokers, kafka_client_id, redis_url, timescale_url, volume_engine_consumer_group,
    volume_metrics_address,
};
use sui_processors::metrics::MetricsBundle;
use sui_processors::redis_cache::RedisCache;
use sui_processors::runtime::serve_metrics;
use sui_processors::timescale::TimescaleStore;
use sui_processors::volume::VolumeEngine;

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    config::load_dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let ts_url = timescale_url()?;
    let redis = redis_url()?;
    let brokers = kafka_brokers()?;
    let group = volume_engine_consumer_group();

    let store = TimescaleStore::connect(ts_url).await?;
    store.run_migrations().await?;

    let cache = RedisCache::connect(&redis)?;
    let registry = Registry::new();
    let metrics = MetricsBundle::new(&registry, "volume-engine")?;
    let engine = VolumeEngine::new(store, cache, metrics.clone());

    let reader = KafkaFactReader::new(
        &brokers,
        &group,
        &kafka_client_id("volume-engine"),
    )?;
    reader.subscribe(&[FactTopic::SwapNormalized])?;

    let metrics_addr = volume_metrics_address();
    info!("volume-engine started");

    tokio::select! {
        _ = async {
            loop {
                match reader.recv_envelope().await {
                    Ok((envelope, message)) => {
                        if let Err(e) = engine.handle(&envelope).await {
                            error!(error = %e, "volume-engine handler error");
                            metrics
                                .decode_errors
                                .with_label_values(&["volume-engine", FactTopic::SwapNormalized.as_str()])
                                .inc();
                        }
                        if let Err(e) = reader.commit_message(&message) {
                            error!(error = %e, "Failed to commit volume offset");
                        }
                    }
                    Err(e) => {
                        error!(error = %e, "Volume consumer error");
                        metrics
                            .decode_errors
                            .with_label_values(&["volume-engine", FactTopic::SwapNormalized.as_str()])
                            .inc();
                    }
                }
            }
        } => {},
        result = serve_metrics(registry, &metrics_addr) => result?,
    }

    Ok(())
}
