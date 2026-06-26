use anyhow::Result;
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::bot::config::BotRuntime;
use crate::bot::state::{BotStateStore, Dex, ParsedSwap, TokenStatus};
use crate::dex::agg_swap::SwapMode;
use crate::telegram_notify;

pub async fn process_swap_old_token(
    runtime: Arc<BotRuntime>,
    store: &BotStateStore,
    swap: ParsedSwap,
) -> Result<()> {
    if store.swap_exists(&swap.event_id).await? {
        debug!(pool = %swap.pool, "skip old token swap: already processed");
        return Ok(());
    }

    let Some((pool, token)) = store.get_pool_with_token(&swap.pool).await? else {
        return Ok(());
    };

    if token.status == TokenStatus::Listing {
        debug!(
            pool = %swap.pool,
            symbol = %token.symbol,
            "skip old token swap: token listing"
        );
        return Ok(());
    }

    if runtime.config.is_blacklisted(&token.id) {
        debug!(
            pool = %swap.pool,
            symbol = %token.symbol,
            "skip old token swap: blacklisted"
        );
        return Ok(());
    }

    info!(
        pool = %swap.pool,
        symbol = %token.symbol,
        is_buy = swap.is_buy,
        sui_amount = swap.sui_amount,
        "old token swap detected"
    );

    store
        .insert_processed_swap(
            &swap.event_id,
            &swap.pool,
            &swap.tx_digest,
            &swap.event_seq,
        )
        .await?;

    if !swap.is_buy {
        debug!(
            pool = %swap.pool,
            symbol = %token.symbol,
            sui_amount = swap.sui_amount,
            "skip sell: swap is not a buy"
        );
        return Ok(());
    }

    if swap.sui_amount < runtime.config.sell_buy_threshold {
        info!(
            pool = %swap.pool,
            symbol = %token.symbol,
            sui_amount = swap.sui_amount,
            threshold = runtime.config.sell_buy_threshold,
            "skip sell: buy below SELL_BUY_THRESHOLD"
        );
        return Ok(());
    }

    let symbol = token.symbol.clone();
    let token_type = crate::bot::token_type::normalize_coin_type(&token.id);
    let pool_id = pool.id.clone();
    let dex = pool.dex;
    let sui_f = swap.sui_amount as f64 / 1e9;
    let token_amount = swap.token_amount as u64;
    let mode = sell_mode(swap.sui_amount);

    telegram_notify::send_message(&format!(
        "🚀 Try sell old token {} {:.1} SUI ~ {}",
        symbol, sui_f, swap.token_amount
    ))
    .await;

    // Sell can take several seconds on RPC; don't block the reactor semaphore.
    tokio::spawn(async move {
        if let Err(err) =
            execute_sell(&runtime, dex, &token_type, &pool_id, &symbol, token_amount, mode).await
        {
            error!(?err, symbol = %symbol, "sell old token failed");
            telegram_notify::send_message(&format!(
                "⭕️ Sell old token {} failed: {err}",
                symbol
            ))
            .await;
        }
    });

    Ok(())
}

fn sell_mode(sui_amount: u128) -> SwapMode {
    if sui_amount >= 20_000_000_000 {
        SwapMode::Superfast
    } else if sui_amount >= 10_000_000_000 {
        SwapMode::Fast
    } else {
        SwapMode::Safe
    }
}

async fn execute_sell(
    runtime: &BotRuntime,
    dex: Dex,
    token_type: &str,
    pool_id: &str,
    symbol: &str,
    token_amount: u64,
    mode: SwapMode,
) -> Result<()> {
    match runtime
        .agg
        .swap_exact_amount(dex, true, token_type, pool_id, token_amount, true, mode)
        .await
    {
        Ok(digest) => {
            info!(digest = %digest, symbol = %symbol, "sell old token submitted");
            telegram_notify::send_message(&format!(
                "✅ Sold old token {} tx {}",
                symbol, digest
            ))
            .await;
            Ok(())
        }
        Err(err) => Err(err),
    }
}
