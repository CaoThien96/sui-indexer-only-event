use std::time::{Duration, Instant};

use anyhow::{Result, bail};
use tracing::info;

use sui_processors::config::{
    bootstrap_target_first_checkpoint, oracle_bootstrap_gate_enabled,
    oracle_bootstrap_gate_timeout_secs, timescale_url,
};
use sui_processors::timescale::TimescaleStore;

const BOOTSTRAP_RUN_ID: &str = "sui_usd_bootstrap";

pub async fn wait_for_oracle_bootstrap() -> Result<()> {
    if !oracle_bootstrap_gate_enabled() {
        info!("oracle bootstrap gate disabled");
        return Ok(());
    }
    if bootstrap_target_first_checkpoint().is_none() {
        info!("FIRST_CHECKPOINT unset; skipping oracle bootstrap gate");
        return Ok(());
    }

    let store = TimescaleStore::connect(timescale_url()?).await?;
    let deadline = Instant::now() + Duration::from_secs(oracle_bootstrap_gate_timeout_secs());

    loop {
        match store.get_bootstrap_status(BOOTSTRAP_RUN_ID).await? {
            Some(status) if status == "READY" => {
                info!("oracle bootstrap READY; starting indexer");
                return Ok(());
            }
            Some(status) => {
                info!(status, "waiting for oracle bootstrap");
            }
            None => {
                info!("bootstrap_state missing; waiting for oracle-bootstrap");
            }
        }

        if Instant::now() >= deadline {
            bail!("oracle bootstrap not READY before timeout");
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
