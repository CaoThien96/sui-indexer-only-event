use std::sync::Arc;

use anyhow::Result;
use indexer_store::{FactTopic, KafkaFactReader, parse_envelope, wait_for_topics_available};
use prometheus::Registry;
use rdkafka::Message;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use sui_processors::config::{
    self, kafka_brokers, kafka_client_id, ohlc_aggregator_consumer_group, ohlc_metrics_address,
    timescale_url,
};
use sui_processors::metrics::MetricsBundle;
use sui_processors::ohlc::OhlcAggregator;
use sui_processors::runtime::{kafka_backoff_resubscribe, serve_metrics};
use sui_processors::timescale::TimescaleStore;

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
    let brokers = kafka_brokers()?;
    let group = ohlc_aggregator_consumer_group();

    wait_for_topics_available(&brokers, &[FactTopic::SwapNormalized]).await?;

    let store = TimescaleStore::connect(ts_url).await?;
    store.run_migrations().await?;

    let registry = Registry::new();
    let metrics = MetricsBundle::new(&registry, "ohlc-aggregator")?;
    let aggregator = Arc::new(Mutex::new(OhlcAggregator::new(store, metrics.clone())));

    let reader = KafkaFactReader::new(
        &brokers,
        &group,
        &kafka_client_id("ohlc-aggregator"),
    )?;
    reader.subscribe(&[FactTopic::SwapNormalized])?;

    let metrics_addr = ohlc_metrics_address();
    info!("ohlc-aggregator started");

    let topics = [FactTopic::SwapNormalized];
    tokio::select! {
        _ = async {
            loop {
                let message = match reader.recv_raw().await {
                    Ok(m) => m,
                    Err(e) => {
                        error!(error = %e, "OHLC consumer recv failed");
                        kafka_backoff_resubscribe(&reader, &topics, "ohlc-aggregator").await;
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
                            "Skipping invalid Kafka envelope"
                        );
                        metrics
                            .decode_errors
                            .with_label_values(&["ohlc-aggregator", FactTopic::SwapNormalized.as_str()])
                            .inc();
                        let _ = reader.commit_message(&message);
                        continue;
                    }
                };

                let mut agg = aggregator.lock().await;
                if let Err(e) = agg.handle(&envelope).await {
                    error!(error = %e, "ohlc-aggregator handler error");
                    metrics
                        .decode_errors
                        .with_label_values(&["ohlc-aggregator", FactTopic::SwapNormalized.as_str()])
                        .inc();
                }
                if let Err(e) = reader.commit_message(&message) {
                    error!(error = %e, "Failed to commit OHLC offset");
                }
            }
        } => {},
        result = serve_metrics(registry, &metrics_addr) => result?,
    }

    Ok(())
}
