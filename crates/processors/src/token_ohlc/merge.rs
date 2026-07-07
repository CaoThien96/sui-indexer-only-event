use super::bar::TokenUsdOhlcBar;

pub fn merge_token_usd_bar(existing: &TokenUsdOhlcBar, incoming: &TokenUsdOhlcBar) -> TokenUsdOhlcBar {
    TokenUsdOhlcBar {
        bucket: existing.bucket,
        base_coin_type: existing.base_coin_type.clone(),
        open_usd: existing.open_usd,
        high_usd: existing.high_usd.max(incoming.high_usd),
        low_usd: existing.low_usd.min(incoming.low_usd),
        close_usd: incoming.close_usd,
        volume_usd: existing.volume_usd + incoming.volume_usd,
        trade_count: existing.trade_count + incoming.trade_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn bar(price: &str, vol: &str) -> TokenUsdOhlcBar {
        let p = Decimal::from_str(price).unwrap();
        let v = Decimal::from_str(vol).unwrap();
        TokenUsdOhlcBar {
            bucket: Utc.with_ymd_and_hms(2026, 7, 5, 12, 0, 0).unwrap(),
            base_coin_type: "0xtoken".to_string(),
            open_usd: p,
            high_usd: p,
            low_usd: p,
            close_usd: p,
            volume_usd: v,
            trade_count: 1,
        }
    }

    #[test]
    fn merge_updates_hlc_and_volume() {
        let b1 = bar("1.0", "10");
        let b2 = bar("2.0", "20");
        let merged = merge_token_usd_bar(&b1, &b2);
        assert_eq!(merged.open_usd, Decimal::ONE);
        assert_eq!(merged.close_usd, Decimal::from(2));
        assert_eq!(merged.high_usd, Decimal::from(2));
        assert_eq!(merged.low_usd, Decimal::ONE);
        assert_eq!(merged.volume_usd, Decimal::from(30));
        assert_eq!(merged.trade_count, 2);
    }
}
