use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

use crate::bot::state::BotStateStore;

pub fn spawn_processed_swaps_cleanup(
    store: Arc<BotStateStore>,
    ttl_days: u32,
    interval_secs: u64,
) {
    tokio::spawn(async move {
        run_processed_swaps_cleanup(&store, ttl_days).await;

        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
        loop {
            interval.tick().await;
            run_processed_swaps_cleanup(&store, ttl_days).await;
        }
    });
}

async fn run_processed_swaps_cleanup(store: &BotStateStore, ttl_days: u32) {
    match store.delete_processed_swaps_older_than(ttl_days).await {
        Ok(0) => {
            debug!(ttl_days, "bot_processed_swaps ttl cleanup: nothing to delete");
        }
        Ok(deleted) => {
            info!(deleted, ttl_days, "bot_processed_swaps ttl cleanup");
        }
        Err(err) => {
            error!(?err, ttl_days, "bot_processed_swaps ttl cleanup failed");
        }
    }
}
