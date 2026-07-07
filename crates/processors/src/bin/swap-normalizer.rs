use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use indexer_store::{FactTopic, KafkaFactReader, KafkaFactWriter, MessageEnvelope, parse_envelope, wait_for_topics_available};
use prometheus::Registry;
use tracing::{error, info, warn};

use sui_processors::config::{
    self, database_url, kafka_brokers, kafka_client_id, metrics_address,
    swap_hydration_defer_backoff_ms, swap_hydration_defer_max_retries,
    swap_hydration_enabled, swap_hydration_pool_cache_size, swap_hydration_rpc_timeout_ms,
    swap_normalizer_consumer_group, swap_normalizer_workers, sui_grpc_url, timescale_url,
};
use sui_processors::metrics::ProcessorMetrics;
use sui_processors::runtime::{kafka_recover, serve_metrics};
use sui_processors::store::CatalogStore;
use sui_processors::sui_grpc::SuiGrpcClient;
use sui_processors::timescale::TimescaleStore;
use sui_processors::swap_normalizer::{
    HydrationConfig, NormalizeOutcome, SwapNormalizer, defer_backoff,
    normalized_partition_key,
};

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
    let ts_url = timescale_url()?;
    let brokers = kafka_brokers()?;
    let group = swap_normalizer_consumer_group();
    let grpc_url = sui_grpc_url()?;
    let rpc_timeout = Duration::from_millis(swap_hydration_rpc_timeout_ms());

    wait_for_topics_available(&brokers, &[FactTopic::SwapRaw]).await?;

    let store = CatalogStore::connect(db_url).await?;
    store.run_migrations().await?;

    let timescale = TimescaleStore::connect(ts_url).await?;
    timescale.run_migrations().await?;

    let grpc = Arc::new(SuiGrpcClient::new(&grpc_url, rpc_timeout)?);
    info!(%grpc_url, "swap-normalizer gRPC client configured");

    let registry = Registry::new();
    let metrics = ProcessorMetrics::new(&registry)?;
    let hydration_config = HydrationConfig {
        enabled: swap_hydration_enabled(),
        pool_cache_size: swap_hydration_pool_cache_size(),
        defer_max_retries: swap_hydration_defer_max_retries(),
        defer_backoff_ms: swap_hydration_defer_backoff_ms(),
    };
    let normalizer = Arc::new(SwapNormalizer::new(
        store,
        timescale,
        metrics.clone(),
        grpc,
        hydration_config.clone(),
    ));

    let writer = Arc::new(KafkaFactWriter::new(
        &brokers,
        &kafka_client_id("swap-normalizer-producer"),
        &registry,
    )?);

    let workers = swap_normalizer_workers();
    let metrics_addr = metrics_address();
    info!(workers, "swap-normalizer started");

    let topics = [FactTopic::SwapRaw];
    let mut worker_tasks = Vec::with_capacity(workers);
    for idx in 0..workers {
        let brokers = brokers.clone();
        let group = group.clone();
        let normalizer = Arc::clone(&normalizer);
        let writer = Arc::clone(&writer);
        let metrics = Arc::clone(&metrics);
        let hydration_config = hydration_config.clone();
        let task = tokio::spawn(async move {
            // Stagger joins to avoid rebalance storms when multiple workers start together.
            tokio::time::sleep(Duration::from_millis(500 * idx as u64)).await;
            let reader = KafkaFactReader::new(
                &brokers,
                &group,
                &kafka_client_id(&format!("swap-normalizer-{idx}")),
            )?;
            reader.subscribe(&[FactTopic::SwapRaw]).await?;
            loop {
                if let Err(e) = process_one_message(
                    &reader,
                    &topics,
                    &writer,
                    &normalizer,
                    &metrics,
                    &hydration_config,
                )
                .await
                {
                    error!(worker = idx, error = %e, "swap-normalizer message loop failed");
                    kafka_recover(&reader, &topics, "swap-normalizer").await;
                }
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

async fn process_one_message(
    reader: &KafkaFactReader,
    topics: &[FactTopic],
    writer: &KafkaFactWriter,
    normalizer: &Arc<SwapNormalizer>,
    metrics: &Arc<ProcessorMetrics>,
    hydration_config: &HydrationConfig,
) -> Result<()> {
    let message = match reader.recv_raw().await {
        Ok(m) => m,
        Err(e) => {
            error!(error = %e, "Kafka recv failed");
            kafka_recover(reader, topics, "swap-normalizer").await;
            return Ok(());
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
                "Skipping invalid Kafka envelope"
            );
            reader.commit_message(&message).await?;
            return Ok(());
        }
    };

    let mut retries = 0u32;
    loop {
        match normalizer.normalize(&envelope).await {
            Ok(NormalizeOutcome::Published(normalized)) => {
                publish_normalized(writer, metrics, &normalized).await?;
                break;
            }
            Ok(NormalizeOutcome::SkippedPermanent { reason }) => {
                metrics
                    .swap_skipped
                    .with_label_values(&[reason.as_str()])
                    .inc();
                break;
            }
            Ok(NormalizeOutcome::Deferred { reason }) => {
                retries += 1;
                metrics.swap_defer_retries.inc();
                if retries >= hydration_config.defer_max_retries {
                    warn!(
                        reason = reason.as_str(),
                        retries,
                        message_id = %envelope.message_id,
                        "swap deferred max retries exceeded; leaving offset uncommitted"
                    );
                    return Ok(());
                }
                let delay = defer_backoff(hydration_config.defer_backoff_ms, retries);
                tokio::time::sleep(delay).await;
                continue;
            }
            Err(e) => {
                metrics
                    .decode_errors
                    .with_label_values(&["swap-normalizer", FactTopic::SwapRaw.as_str()])
                    .inc();
                warn!(
                    error = %e,
                    message_id = %envelope.message_id,
                    "normalize failed with unexpected error; committing to avoid stall"
                );
                break;
            }
        }
    }

    reader.commit_message(&message).await?;
    Ok(())
}

async fn publish_normalized(
    writer: &KafkaFactWriter,
    metrics: &Arc<ProcessorMetrics>,
    normalized: &MessageEnvelope,
) -> Result<()> {
    let protocol = normalized
        .payload
        .get("protocol")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    writer
        .publish(
            FactTopic::SwapNormalized,
            &[normalized.clone()],
            normalized_partition_key,
        )
        .await?;
    metrics
        .swap_normalized
        .with_label_values(&[&protocol])
        .inc();
    Ok(())
}
