use anyhow::Result;
use rand::Rng;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

use crate::bot::config::BotRuntime;
use crate::bot::state::{BotStateStore, Dex};
use crate::bot::token_type::normalize_coin_type;
use crate::dex::{cetus_lp, turbos_lp};
use crate::dex::agg_swap::SwapMode;
use crate::telegram_notify;

#[derive(Debug, Clone, Default)]
pub struct SnipRunOptions {
    pub skip_buy: bool,
    pub skip_lp: bool,
    pub buy_amount: Option<u64>,
    pub lp_wait_ms: Option<u64>,
}

pub fn schedule_snip(
    runtime: Arc<BotRuntime>,
    _store: Arc<BotStateStore>,
    dex: Dex,
    token: String,
    pool: String,
    symbol: String,
) {
    let delay_ms = {
        let mut rng = rand::thread_rng();
        rng.gen_range(runtime.config.snip_delay_ms_min..=runtime.config.snip_delay_ms_max)
    };

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
        if let Err(err) = run_snip(&runtime, dex, &token, &pool, &symbol, SnipRunOptions::default()).await
        {
            let msg = err.to_string();
            if msg.contains("add liquidity") || msg.contains("lp fixed") || msg.contains("TypeMismatch") {
                error!(?err, symbol = %symbol, "snip failed after buy (check LP step)");
            } else {
                error!(?err, symbol = %symbol, "snip failed");
            }
            telegram_notify::send_message(&format!("⭕️ Snip {symbol} failed: {err}"))
                .await;
        }
    });
}

pub async fn run_snip(
    runtime: &BotRuntime,
    dex: Dex,
    token: &str,
    pool: &str,
    symbol: &str,
    options: SnipRunOptions,
) -> Result<()> {
    let token = normalize_coin_type(token);
    let buy_amount = options
        .buy_amount
        .unwrap_or(runtime.config.snip_buy_amount);
    let lp_wait_ms = options
        .lp_wait_ms
        .unwrap_or(runtime.config.snip_lp_wait_ms);

    if !options.skip_buy {
        match runtime
            .agg
            .swap_exact_amount(
                dex,
                false,
                &token,
                pool,
                buy_amount,
                false,
                SwapMode::Safe,
                None,
            )
            .await
        {
            Ok((digest, _)) => {
                info!(digest = %digest, symbol = %symbol, "snip buy submitted");
                telegram_notify::send_message(&format!("⚡️ Snip {symbol} success {digest}")).await;
            }
            Err(err) => {
                warn!(?err, symbol = %symbol, "snip buy failed");
                return Err(err);
            }
        }
    } else {
        info!(symbol = %symbol, "skip snip buy");
    }

    if options.skip_lp {
        info!(symbol = %symbol, "skip add liquidity");
        return Ok(());
    }

    if lp_wait_ms > 0 {
        tokio::time::sleep(Duration::from_millis(lp_wait_ms)).await;
    }

    info!(symbol = %symbol, pool = %pool, dex = ?dex, "snip add liquidity starting");
    match dex {
        Dex::Cetus => cetus_lp::open_pool_position_with_lp_fixed(runtime, pool).await,
        Dex::Turbos => turbos_lp::open_pool_position_with_lp_fixed(runtime, pool).await,
    }
    .map_err(|err| {
        warn!(?err, symbol = %symbol, "snip add liquidity failed (buy may have succeeded)");
        err
    })
}
