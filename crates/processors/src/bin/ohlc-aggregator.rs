use anyhow::Result;
use indexer_store::{FactTopic, KafkaFactReader};
use prometheus::Registry;
use tracing::{error, info};

use sui_processors::config::{
    self, kafka_brokers, kafka_client_id, ohlc_aggregator_consumer_group, ohlc_metrics_address,
    timescale_url,
};
use sui_processors::metrics::MetricsBundle;
use sui_processors::ohlc::OhlcAggregator;
use sui_processors::runtime::serve_metrics;
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

    let store = TimescaleStore::connect(ts_url).await?;
    store.run_migrations().await?;

    let registry = Registry::new();
    let metrics = MetricsBundle::new(&registry, "ohlc-aggregator")?;
    let mut aggregator = OhlcAggregator::new(store, metrics.clone());

    let reader = KafkaFactReader::new(
        &brokers,
        &group,
        &kafka_client_id("ohlc-aggregator"),
    )?;
    reader.subscribe(&[FactTopic::SwapNormalized])?;

    let metrics_addr = ohlc_metrics_address();
    info!("ohlc-aggregator started");

    tokio::select! {
        _ = async {
            loop {
                match reader.recv_envelope().await {
                    Ok((envelope, message)) => {
                        if let Err(e) = aggregator.handle(&envelope).await {
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
                    Err(e) => {
                        error!(error = %e, "OHLC consumer error");
                        metrics
                            .decode_errors
                            .with_label_values(&["ohlc-aggregator", FactTopic::SwapNormalized.as_str()])
                            .inc();
                    }
                }
            }
        } => {},
        result = serve_metrics(registry, &metrics_addr) => result?,
    }

    Ok(())
}
