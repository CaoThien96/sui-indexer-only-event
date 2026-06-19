mod config;
mod diff;
mod event_key;
mod fullnode;
mod kafka;
mod report;

use anyhow::Result;
use config::Config;
use diff::diff_keys;
use fullnode::FullnodeClient;
use kafka::list_event_keys_in_window;
use report::{ReconciliationReport, warn_if_window_beyond_indexed_data};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env()?;

    tracing::info!(
        event_type = %config.move_event_type,
        kafka_topic = %config.kafka_topic,
        window_start_ms = config.start_time_ms(),
        window_end_ms = config.end_time_ms(),
        fullnode_url = %config.fullnode_url,
        kafka_brokers = %config.kafka_brokers,
        "Starting reconciliation (Kafka vs fullnode)"
    );

    tracing::info!("Loading Kafka event keys...");
    let (kafka_keys, max_ts) = list_event_keys_in_window(
        &config.kafka_brokers,
        &config.kafka_topic,
        &config.move_event_type,
        config.start_time_ms(),
        config.end_time_ms(),
    )
    .await?;

    warn_if_window_beyond_indexed_data(config.end_time_ms(), max_ts)?;

    tracing::info!("Loading fullnode event keys (paginated MoveEventType)...");
    let fullnode = FullnodeClient::new(config.fullnode_url.clone());
    let fullnode_keys = fullnode
        .list_event_keys_in_window(
            &config.move_event_type,
            config.start_time_ms(),
            config.end_time_ms(),
        )
        .await?;

    let key_diff = diff_keys(kafka_keys.clone(), fullnode_keys.clone());

    let reconciliation = ReconciliationReport::new(
        config.move_event_type.clone(),
        config.kafka_topic.clone(),
        config.start_time_ms(),
        config.end_time_ms(),
        kafka_keys.len() as i64,
        fullnode_keys.len() as u64,
        config.count_tolerance,
        key_diff,
        config.key_tolerance,
        config.max_key_samples,
    );

    reconciliation.print();

    if !reconciliation.is_ok() {
        std::process::exit(1);
    }

    Ok(())
}
