use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::normalized_swap::{NormalizedSwap, minute_bucket};

#[derive(Debug, Clone)]
pub struct OhlcBar {
    pub bucket: DateTime<Utc>,
    pub pool_id: String,
    pub base_coin_type: String,
    pub quote_coin_type: String,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume_quote: Decimal,
    pub trade_count: i32,
}

pub fn swap_to_bar(swap: &NormalizedSwap) -> OhlcBar {
    let price = swap.price_quote_per_base;
    OhlcBar {
        bucket: minute_bucket(swap.timestamp_ms),
        pool_id: swap.pool_id.clone(),
        base_coin_type: swap.base_coin_type.clone(),
        quote_coin_type: swap.quote_coin_type.clone(),
        open: price,
        high: price,
        low: price,
        close: price,
        volume_quote: swap.amount_quote,
        trade_count: 1,
    }
}

pub fn merge_bar(existing: &OhlcBar, incoming: &OhlcBar) -> OhlcBar {
    OhlcBar {
        bucket: existing.bucket,
        pool_id: existing.pool_id.clone(),
        base_coin_type: existing.base_coin_type.clone(),
        quote_coin_type: existing.quote_coin_type.clone(),
        open: existing.open,
        high: existing.high.max(incoming.high),
        low: existing.low.min(incoming.low),
        close: incoming.close,
        volume_quote: existing.volume_quote + incoming.volume_quote,
        trade_count: existing.trade_count + incoming.trade_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn sample_swap(price: &str, volume: &str, ts: i64) -> NormalizedSwap {
        NormalizedSwap {
            protocol: "cetus".to_string(),
            pool_id: "0xpool".to_string(),
            base_coin_type: "0xtoken".to_string(),
            quote_coin_type: "0x2::sui::SUI".to_string(),
            coin_type_a: "0x2::sui::SUI".to_string(),
            coin_type_b: "0xtoken".to_string(),
            amount_base: Decimal::ONE,
            amount_quote: Decimal::from_str(volume).unwrap(),
            price_quote_per_base: Decimal::from_str(price).unwrap(),
            fee_amount: None,
            vault_a_raw: None,
            vault_b_raw: None,
            time: Utc.timestamp_millis_opt(ts).unwrap(),
            timestamp_ms: ts,
            tx_digest: "tx1".to_string(),
            event_seq: 0,
            sender: None,
            checkpoint_seq: 1,
            swap_key: "tx1:0:cetus".to_string(),
        }
    }

    #[test]
    fn single_swap_bar() {
        let swap = sample_swap("1.5", "100", 1_710_000_000_000);
        let bar = swap_to_bar(&swap);
        assert_eq!(bar.open, bar.close);
        assert_eq!(bar.volume_quote, Decimal::from(100));
        assert_eq!(bar.trade_count, 1);
    }

    #[test]
    fn merge_updates_hlc_and_volume() {
        let b1 = swap_to_bar(&sample_swap("1.0", "10", 1_710_000_000_000));
        let b2 = swap_to_bar(&sample_swap("2.0", "20", 1_710_000_030_000));
        let merged = merge_bar(&b1, &b2);
        assert_eq!(merged.open, Decimal::ONE);
        assert_eq!(merged.close, Decimal::from(2));
        assert_eq!(merged.high, Decimal::from(2));
        assert_eq!(merged.low, Decimal::ONE);
        assert_eq!(merged.volume_quote, Decimal::from(30));
        assert_eq!(merged.trade_count, 2);
    }
}
