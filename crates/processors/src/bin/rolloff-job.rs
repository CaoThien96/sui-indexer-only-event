use anyhow::Result;
use prometheus::Registry;
use std::time::Duration;
use tracing::info;

use sui_processors::config::{self, rolloff_interval_secs, rolloff_metrics_address};
use sui_processors::metrics::RolloffMetrics;
use sui_processors::rolloff::RolloffJob;
use sui_processors::runtime::serve_metrics;

#[tokio::main]
async fn main() -> Result<()> {
    config::load_dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let registry = Registry::new();
    let metrics = RolloffMetrics::new(&registry)?;
    let job = RolloffJob::new(metrics).await?;

    let metrics_addr = rolloff_metrics_address();
    let interval = Duration::from_secs(rolloff_interval_secs());

    info!(?interval, "rolloff-job started");

    tokio::select! {
        result = job.run_loop(interval) => result?,
        result = serve_metrics(registry, &metrics_addr) => result?,
    }

    Ok(())
}
