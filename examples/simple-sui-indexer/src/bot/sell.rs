use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, error, info, warn};

use crate::bot::config::BotRuntime;
use crate::bot::state::{BotStateStore, Dex, ParsedSwap, TokenStatus};
use crate::dex::agg_swap::SwapMode;
use crate::telegram_format::{format_old_token_swap, format_sell_fail, format_sell_success};
use crate::telegram_notify;

/// First attempt sells this % of the buy's token amount; each retry drops 1%.
const SELL_RETRY_START_PERCENT: u64 = 99;
const SELL_RETRY_ATTEMPTS: usize = 5;

pub async fn process_swap_old_token(
    runtime: Arc<BotRuntime>,
    store: &BotStateStore,
    swap: ParsedSwap,
) -> Result<()> {
    if store
        .swap_exists(&swap.tx_digest, &swap.event_seq)
        .await?
    {
        debug!(
            pool = %swap.pool,
            tx_digest = %swap.tx_digest,
            event_seq = %swap.event_seq,
            "skip old token swap: already processed"
        );
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
        .insert_processed_swap(&swap.tx_digest, &swap.event_seq, &swap.pool)
        .await?;

    let vault_addr = runtime.vault.address_string();
    if token.status == TokenStatus::Done && swap.maker != vault_addr {
        telegram_notify::send_bot_message(&format_old_token_swap(
            swap.sui_amount,
            swap.token_amount,
            swap.is_buy,
            &swap.maker,
            &pool.id,
        ))
        .await;
    }

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
    let trigger_sui_mist = swap.sui_amount;
    let token_amount = swap.token_amount as u64;
    let mode = sell_mode(swap.sui_amount);

    let sell_detected_at = Instant::now();

    tokio::spawn(async move {
        let queue_ms = sell_detected_at.elapsed().as_millis() as u64;
        if let Err(err) = execute_sell_with_retry(
            &runtime,
            dex,
            &token_type,
            &pool_id,
            &symbol,
            token_amount,
            trigger_sui_mist,
            mode,
            sell_detected_at,
            queue_ms,
        )
        .await
        {
            error!(?err, symbol = %symbol, "sell old token failed after retries");
            telegram_notify::send_bot_message(&format_sell_fail(
                &symbol,
                &pool_id,
                trigger_sui_mist,
                SELL_RETRY_ATTEMPTS,
                &err.to_string(),
            ))
            .await;
        }
    });

    Ok(())
}

fn sell_mode(sui_amount: u128) -> SwapMode {
    if sui_amount >= 10_000_000_000 {
        SwapMode::Superfast
    } else if sui_amount >= 5_000_000_000 {
        SwapMode::Fast
    } else {
        SwapMode::Safe
    }
}

fn sell_amount_for_attempt(full_amount: u64, attempt: usize) -> Option<(u64, u64)> {
    let percent = SELL_RETRY_START_PERCENT.saturating_sub(attempt as u64);
    if percent == 0 {
        return None;
    }
    let amount = full_amount.saturating_mul(percent) / 100;
    if amount == 0 {
        return None;
    }
    Some((amount, percent))
}

async fn execute_sell_with_retry(
    runtime: &BotRuntime,
    dex: Dex,
    token_type: &str,
    pool_id: &str,
    symbol: &str,
    token_amount: u64,
    trigger_sui_mist: u128,
    mode: SwapMode,
    sell_detected_at: Instant,
    spawn_queue_ms: u64,
) -> Result<()> {
    let mut last_err: Option<anyhow::Error> = None;

    for attempt in 0..SELL_RETRY_ATTEMPTS {
        let Some((amount, percent)) = sell_amount_for_attempt(token_amount, attempt) else {
            warn!(
                symbol = %symbol,
                attempt = attempt + 1,
                token_amount,
                "sell retry skipped: amount rounds to zero"
            );
            continue;
        };

        info!(
            symbol = %symbol,
            attempt = attempt + 1,
            max_attempts = SELL_RETRY_ATTEMPTS,
            percent,
            amount,
            spawn_queue_ms,
            detect_elapsed_ms = sell_detected_at.elapsed().as_millis() as u64,
            "sell old token attempt"
        );

        match runtime
            .agg
            .swap_exact_amount(
                dex,
                true,
                token_type,
                pool_id,
                amount,
                true,
                mode,
                Some(sell_detected_at),
            )
            .await
        {
            Ok((digest, metrics)) => {
                let sell_total_ms = sell_detected_at.elapsed().as_millis() as u64;
                info!(
                    digest = %digest,
                    symbol = %symbol,
                    percent,
                    attempt = attempt + 1,
                    spawn_queue_ms,
                    detect_to_build_ms = ?metrics.detect_to_build_ms,
                    build_ms = metrics.build_ms,
                    submit_at = %metrics.submit_at,
                    confirm_ms = metrics.confirm_ms,
                    sell_total_ms,
                    "sell old token timing"
                );
                telegram_notify::send_bot_message(&format_sell_success(
                    symbol,
                    pool_id,
                    &digest,
                    trigger_sui_mist,
                ))
                .await;
                return Ok(());
            }
            Err(err) => {
                warn!(
                    ?err,
                    symbol = %symbol,
                    attempt = attempt + 1,
                    percent,
                    amount,
                    detect_elapsed_ms = sell_detected_at.elapsed().as_millis() as u64,
                    "sell old token attempt failed"
                );
                last_err = Some(err);
            }
        }
    }

    Err(last_err.unwrap_or_else(|| anyhow::anyhow!("sell failed with no attempts")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sell_amount_steps_down_by_one_percent() {
        let full = 1_000_000_000u64;
        assert_eq!(sell_amount_for_attempt(full, 0), Some((990_000_000, 99)));
        assert_eq!(sell_amount_for_attempt(full, 1), Some((980_000_000, 98)));
        assert_eq!(sell_amount_for_attempt(full, 4), Some((950_000_000, 95)));
        assert!(sell_amount_for_attempt(full, 5).is_none());
    }
}
