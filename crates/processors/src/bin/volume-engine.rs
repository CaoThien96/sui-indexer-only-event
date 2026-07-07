use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use indexer_store::{FactTopic, KafkaFactReader, wait_for_topics_available};
use prometheus::Registry;
use tracing::info;

use sui_processors::config::{
    self, bootstrap_trusted_pool_ids, kafka_brokers, kafka_client_id, redis_url, timescale_url,
    volume_engine_consumer_group, volume_engine_workers, volume_metrics_address,
    volume_rollup_refresh_secs,
};
use sui_processors::metrics::MetricsBundle;
use sui_processors::oracle::trusted_pool_set;
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
    let trusted_pools = trusted_pool_set(&bootstrap_trusted_pool_ids());
    let rollup_refresh = Duration::from_secs(volume_rollup_refresh_secs());
    let workers = volume_engine_workers();
    let engine = Arc::new(VolumeEngine::new(
        store,
        cache,
        metrics.clone(),
        trusted_pools,
        rollup_refresh,
    ));

    let metrics_addr = volume_metrics_address();
    info!(workers, "volume-engine started");

    let topics = [FactTopic::SwapNormalized];
    let mut worker_tasks = Vec::with_capacity(workers);
    for idx in 0..workers {
        let brokers = brokers.clone();
        let group = group.clone();
        let engine = Arc::clone(&engine);
        let metrics = metrics.clone();
        let task = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500 * idx as u64)).await;
            let reader = KafkaFactReader::new(
                &brokers,
                &group,
                &kafka_client_id(&format!("volume-engine-{idx}")),
            )?;
            reader.subscribe(&[FactTopic::SwapNormalized]).await?;
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
            #[allow(unreachable_code)]
            Ok::<(), anyhow::Error>(())
        });
        worker_tasks.push(task);
    }

    tokio::select! {
        _ = async {
            for task in worker_tasks {
                let _ = task.await;
            }
        } => {},
        result = serve_metrics(registry, &metrics_addr) => result?,
    }

    Ok(())
}
