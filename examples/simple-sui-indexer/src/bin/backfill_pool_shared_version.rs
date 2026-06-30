//! One-time backfill of `bot_pools.initial_shared_version` for pools sniped before deploy.
//!
//!   cargo run --release --bin backfill-pool-shared-version

use anyhow::{Context, Result};
use simple_sui_indexer::bot::config::BotRuntime;
use simple_sui_indexer::bot::state::BotStateStore;
use simple_sui_indexer::bootstrap;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    let dotenv = bootstrap::load_dotenv();
    bootstrap::init_tracing();
    bootstrap::log_dotenv_load(&dotenv);

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let store = BotStateStore::connect(&database_url).await?;
    let runtime = BotRuntime::init().await?;

    let pool_ids = store.list_pools_missing_shared_version().await?;
    if pool_ids.is_empty() {
        info!("no pools missing initial_shared_version");
        return Ok(());
    }

    info!(count = pool_ids.len(), "backfilling pool shared versions");
    let mut ok = 0u64;
    let mut failed = 0u64;

    for pool_id in pool_ids {
        match runtime.rpc.get_shared_initial_version(&pool_id).await {
            Ok(version) => {
                store
                    .set_pool_shared_initial_version(&pool_id, version)
                    .await?;
                info!(
                    pool = %pool_id,
                    initial_shared_version = version,
                    "pool shared version backfilled"
                );
                ok += 1;
            }
            Err(err) => {
                warn!(pool = %pool_id, ?err, "failed to backfill pool shared version");
                failed += 1;
            }
        }
    }

    info!(ok, failed, "backfill complete");
    Ok(())
}
