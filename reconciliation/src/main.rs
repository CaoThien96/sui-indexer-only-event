mod config;
mod diff;
mod event_key;
mod fullnode;
mod indexer;
mod report;

use anyhow::Result;
use config::Config;
use diff::diff_keys;
use fullnode::FullnodeClient;
use indexer::{list_event_keys_in_window, max_indexed_timestamp_ms};
use report::{warn_if_window_beyond_indexed_data, ReconciliationReport};
use sqlx::postgres::PgPoolOptions;

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
        window_start_ms = config.start_time_ms(),
        window_end_ms = config.end_time_ms(),
        fullnode_url = %config.fullnode_url,
        "Starting reconciliation phase 2"
    );

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&config.database_url)
        .await?;

    let max_ts = max_indexed_timestamp_ms(&pool).await?;
    warn_if_window_beyond_indexed_data(config.end_time_ms(), max_ts)?;

    tracing::info!("Loading indexer event keys...");
    let indexer_keys = list_event_keys_in_window(
        &pool,
        &config.indexer_event_type(),
        config.start_time_ms(),
        config.end_time_ms(),
    )
    .await?;

    tracing::info!("Loading fullnode event keys (paginated MoveEventType)...");
    let fullnode = FullnodeClient::new(config.fullnode_url.clone());
    let fullnode_keys = fullnode
        .list_event_keys_in_window(
            &config.move_event_type,
            config.start_time_ms(),
            config.end_time_ms(),
        )
        .await?;

    let key_diff = diff_keys(indexer_keys.clone(), fullnode_keys.clone());

    let reconciliation = ReconciliationReport::new(
        config.move_event_type.clone(),
        config.start_time_ms(),
        config.end_time_ms(),
        indexer_keys.len() as i64,
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
