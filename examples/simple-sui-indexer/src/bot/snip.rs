use anyhow::Result;
use rand::Rng;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

use crate::bot::config::BotRuntime;
use crate::bot::state::{BotStateStore, Dex};
use crate::bot::token_type::normalize_coin_type;
use crate::dex::{cetus_lp, turbos_lp};
use crate::dex::agg_swap::SwapMode;
use crate::telegram_format::{
    format_add_liquidity_success, format_snip_fail_buy, format_snip_fail_lp_after_buy,
    format_snip_success,
};
use crate::telegram_notify;

#[derive(Debug, Clone, Default)]
pub struct SnipRunOptions {
    pub skip_buy: bool,
    pub skip_lp: bool,
    pub buy_amount: Option<u64>,
    pub lp_wait_ms: Option<u64>,
}

#[derive(Debug)]
pub enum SnipFailure {
    Buy(anyhow::Error),
    LpAfterBuy { buy_digest: String, err: anyhow::Error },
}

impl fmt::Display for SnipFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Buy(err) => write!(f, "{err}"),
            Self::LpAfterBuy { err, .. } => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for SnipFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Buy(err) => Some(err.as_ref()),
            Self::LpAfterBuy { err, .. } => Some(err.as_ref()),
        }
    }
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
        if let Err(failure) =
            run_snip(&runtime, dex, &token, &pool, &symbol, SnipRunOptions::default()).await
        {
            match &failure {
                SnipFailure::Buy(err) => {
                    error!(?err, symbol = %symbol, "snip buy failed");
                    telegram_notify::send_bot_message(&format_snip_fail_buy(
                        &symbol, dex, &pool, &err.to_string(),
                    ))
                    .await;
                }
                SnipFailure::LpAfterBuy { buy_digest, err } => {
                    error!(?err, symbol = %symbol, buy_digest = %buy_digest, "snip LP failed after buy");
                    telegram_notify::send_bot_message(&format_snip_fail_lp_after_buy(
                        &symbol,
                        dex,
                        &pool,
                        buy_digest,
                        &err.to_string(),
                    ))
                    .await;
                }
            }
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
) -> Result<(), SnipFailure> {
    let token = normalize_coin_type(token);
    let buy_amount = options
        .buy_amount
        .unwrap_or(runtime.config.snip_buy_amount);
    let lp_wait_ms = options
        .lp_wait_ms
        .unwrap_or(runtime.config.snip_lp_wait_ms);

    let mut buy_digest: Option<String> = None;

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
                telegram_notify::send_bot_message(&format_snip_success(symbol, dex, &digest)).await;
                buy_digest = Some(digest);
            }
            Err(err) => {
                warn!(?err, symbol = %symbol, "snip buy failed");
                return Err(SnipFailure::Buy(err));
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
    let notify_lp_fail = options.skip_buy;
    let lp_result = match dex {
        Dex::Cetus => {
            cetus_lp::open_pool_position_with_lp_fixed(runtime, pool, symbol, notify_lp_fail).await
        }
        Dex::Turbos => {
            turbos_lp::open_pool_position_with_lp_fixed(runtime, pool, symbol, notify_lp_fail).await
        }
    };

    match lp_result {
        Ok(digest) => {
            telegram_notify::send_bot_message(&format_add_liquidity_success(symbol, &digest)).await;
            Ok(())
        }
        Err(err) => {
            warn!(?err, symbol = %symbol, "snip add liquidity failed");
            if let Some(buy_digest) = buy_digest {
                Err(SnipFailure::LpAfterBuy {
                    buy_digest,
                    err,
                })
            } else {
                Err(SnipFailure::Buy(err))
            }
        }
    }
}
