use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::normalized_swap::NormalizedSwap;

use super::bucket::bucket_for_interval;

#[derive(Debug, Clone)]
pub struct TokenUsdOhlcBar {
    pub bucket: DateTime<Utc>,
    pub base_coin_type: String,
    pub open_usd: Decimal,
    pub high_usd: Decimal,
    pub low_usd: Decimal,
    pub close_usd: Decimal,
    pub volume_usd: Decimal,
    pub trade_count: i32,
}

pub fn swap_to_token_usd_bar(swap: &NormalizedSwap, interval: &str) -> Option<TokenUsdOhlcBar> {
    let price = swap.price_usd_per_base?;
    let volume = swap.amount_usd?;
    Some(TokenUsdOhlcBar {
        bucket: bucket_for_interval(swap.time, interval),
        base_coin_type: swap.base_coin_type.clone(),
        open_usd: price,
        high_usd: price,
        low_usd: price,
        close_usd: price,
        volume_usd: volume,
        trade_count: 1,
    })
}
