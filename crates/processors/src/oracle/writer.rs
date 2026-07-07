use std::collections::HashSet;

use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::coin_type::{SUI_COIN_TYPE, USDC_COIN_TYPE};
use crate::normalized_swap::NormalizedSwap;
use crate::oracle::sui_usdc::minute_bucket;
use crate::timescale::TimescaleStore;

const TRUSTED_POOL_QUALITY: Decimal = Decimal::ONE;

fn default_sui_usdc_quality() -> Decimal {
    Decimal::new(7, 1)
}

pub async fn record_swap_prices(
    store: &TimescaleStore,
    swap: &NormalizedSwap,
    trusted_pools: &HashSet<String>,
) -> Result<()> {
    let bucket = minute_bucket(swap.timestamp_ms as u64)?;

    let quote = swap.quote_coin_type.as_str();
    let base = swap.base_coin_type.as_str();

    if quote == USDC_COIN_TYPE {
        if let Some(price_usd) = swap.price_usd_per_base {
            store
                .upsert_token_usd_1m(
                    bucket,
                    base,
                    price_usd,
                    "onchain_usdc",
                    Some(&swap.pool_id),
                    default_sui_usdc_quality(),
                )
                .await?;
        }
        return Ok(());
    }

    if quote == SUI_COIN_TYPE {
        if base == USDC_COIN_TYPE && !swap.amount_quote.is_zero() {
            let sui_price = (swap.amount_base / swap.amount_quote).normalize();
            let trusted = trusted_pools.contains(&normalize_pool_id(&swap.pool_id));
            let quality = if trusted {
                TRUSTED_POOL_QUALITY
            } else {
                default_sui_usdc_quality()
            };
            let source = if trusted { "trusted_pool" } else { "onchain_usdc" };
            store
                .upsert_sui_usd_1m(
                    bucket,
                    sui_price,
                    source,
                    Some(&swap.pool_id),
                    quality,
                )
                .await?;
            return Ok(());
        }

        if let Some(price_usd) = swap.price_usd_per_base {
            store
                .upsert_token_usd_1m(
                    bucket,
                    base,
                    price_usd,
                    "onchain_sui",
                    Some(&swap.pool_id),
                    default_sui_usdc_quality(),
                )
                .await?;
        }
    }

    Ok(())
}

pub fn normalize_pool_id(pool_id: &str) -> String {
    pool_id.trim().to_ascii_lowercase()
}

pub fn trusted_pool_set(ids: &[String]) -> HashSet<String> {
    ids.iter().map(|id| normalize_pool_id(id)).collect()
}

pub async fn flush_sui_buckets(
    store: &TimescaleStore,
    buckets: &[(DateTime<Utc>, Decimal, String)],
) -> Result<usize> {
    let mut count = 0;
    for (bucket, price, pool_id) in buckets {
        store
            .upsert_sui_usd_1m(
                *bucket,
                *price,
                "trusted_pool",
                Some(pool_id),
                TRUSTED_POOL_QUALITY,
            )
            .await?;
        count += 1;
    }
    Ok(count)
}
