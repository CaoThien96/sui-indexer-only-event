use anyhow::{Context, Result, bail};
use sui_types::transaction::ObjectArg;
use tracing::info;

use crate::bot::state::BotStateStore;
use crate::provider::rpc::{
    SuiRpcClient, initial_shared_version_from_arg, shared_pool_arg_mutable,
};

pub async fn persist_pool_shared_version(
    store: Option<&BotStateStore>,
    pool_id: &str,
    pool_arg: &ObjectArg,
) -> Result<()> {
    let Some(store) = store else {
        return Ok(());
    };
    let version = initial_shared_version_from_arg(pool_arg)?;
    store
        .set_pool_shared_initial_version(pool_id, version)
        .await?;
    info!(
        pool = pool_id,
        initial_shared_version = version,
        "pool shared version saved"
    );
    Ok(())
}

/// Production sell hot path: DB only, no pool RPC.
pub async fn pool_arg_for_sell(store: &BotStateStore, pool_id: &str) -> Result<ObjectArg> {
    let version = store
        .get_pool_shared_initial_version(pool_id)
        .await?
        .with_context(|| {
            format!(
                "pool initial_shared_version missing for {pool_id}; run snip or backfill-pool-shared-version"
            )
        })?;
    info!(
        pool = pool_id,
        initial_shared_version = version,
        pool_shared_version_source = "db",
        "pool shared version for sell"
    );
    shared_pool_arg_mutable(pool_id, version)
}

/// Manual CLI: DB first, optional one-time RPC fallback (not used on production sell hot path).
pub async fn pool_arg_for_sell_with_fallback(
    store: Option<&BotStateStore>,
    rpc: &SuiRpcClient,
    pool_id: &str,
    allow_rpc_fallback: bool,
) -> Result<ObjectArg> {
    if let Some(store) = store {
        if let Some(version) = store.get_pool_shared_initial_version(pool_id).await? {
            info!(
                pool = pool_id,
                initial_shared_version = version,
                pool_shared_version_source = "db",
                "pool shared version for sell"
            );
            return shared_pool_arg_mutable(pool_id, version);
        }
    }

    if !allow_rpc_fallback {
        bail!(
            "pool initial_shared_version missing for {pool_id}; run snip or backfill-pool-shared-version"
        );
    }

    let arg = rpc.object_arg(pool_id, true).await?;
    info!(
        pool = pool_id,
        pool_shared_version_source = "rpc_fallback",
        "pool shared version fetched via RPC (manual sell)"
    );
    persist_pool_shared_version(store, pool_id, &arg).await?;
    Ok(arg)
}
