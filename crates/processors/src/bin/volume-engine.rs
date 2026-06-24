use std::sync::Arc;

use anyhow::Result;
use indexer_store::{FactTopic, KafkaFactReader, wait_for_topics_available};
use prometheus::Registry;
use tracing::info;

use sui_processors::config::{
    self, kafka_brokers, kafka_client_id, redis_url, timescale_url, volume_engine_consumer_group,
    volume_metrics_address,
};
use sui_processors::metrics::MetricsBundle;
use sui_processors::redis_cache::RedisCache;
use sui_processors::runtime::{poll_kafka_envelope, serve_metrics};
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

    wait_for_topics_available(&brokers, &[FactTopic::SwapNormalized]).await?;

    let store = TimescaleStore::connect(ts_url).await?;
    store.run_migrations().await?;

    let cache = RedisCache::connect(&redis)?;
    let registry = Registry::new();
    let metrics = MetricsBundle::new(&registry, "volume-engine")?;
    let engine = Arc::new(VolumeEngine::new(store, cache, metrics.clone()));

    let reader = KafkaFactReader::new(
        &brokers,
        &group,
        &kafka_client_id("volume-engine"),
    )?;
    reader.subscribe(&[FactTopic::SwapNormalized])?;

    let metrics_addr = volume_metrics_address();
    info!("volume-engine started");

    let topics = [FactTopic::SwapNormalized];
    tokio::select! {
        _ = async {
            loop {
                let engine = Arc::clone(&engine);
                let metrics = metrics.clone();
                poll_kafka_envelope(&reader, &topics, "volume-engine", move |envelope| {
                    let engine = Arc::clone(&engine);
                    let metrics = metrics.clone();
                    async move {
                        if let Err(e) = engine.handle(&envelope).await {
                            metrics
                                .decode_errors
                                .with_label_values(&["volume-engine", FactTopic::SwapNormalized.as_str()])
                                .inc();
                            return Err(e);
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
