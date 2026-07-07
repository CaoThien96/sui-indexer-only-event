use anyhow::Result;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::normalized_swap::NormalizedSwap;

const DEFAULT_DECIMALS: u32 = 9;

pub fn estimate_tvl_quote(swap: &NormalizedSwap) -> Option<Decimal> {
    let va_raw = swap.vault_a_raw.as_ref()?;
    let vb_raw = swap.vault_b_raw.as_ref()?;
    let vault_a = scale_raw(va_raw, DEFAULT_DECIMALS).ok()?;
    let vault_b = scale_raw(vb_raw, DEFAULT_DECIMALS).ok()?;
    let price = swap.price_quote_per_base;
    if price.is_zero() {
        return None;
    }

    if swap.quote_coin_type == swap.coin_type_a {
        Some(vault_a + vault_b * price)
    } else {
        Some(vault_b + vault_a / price)
    }
}

fn scale_raw(raw: &str, decimals: u32) -> Result<Decimal> {
    let value = Decimal::from_str(raw)?;
    let scale = Decimal::from(10u64.pow(decimals));
    Ok(value / scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::normalized_swap::NormalizedSwap;

    #[test]
    fn estimates_tvl_when_vaults_present() {
        let swap = NormalizedSwap {
            protocol: "cetus".into(),
            pool_id: "0xpool".into(),
            base_coin_type: "0xtoken".into(),
            quote_coin_type: "0x2::sui::SUI".into(),
            coin_type_a: "0x2::sui::SUI".into(),
            coin_type_b: "0xtoken".into(),
            amount_base: Decimal::ONE,
            amount_quote: Decimal::ONE,
            price_quote_per_base: Decimal::from(2),
            price_usd_per_base: None,
            amount_usd: None,
            fee_amount: None,
            vault_a_raw: Some("1000000000".into()),
            vault_b_raw: Some("2000000000".into()),
            time: Utc::now(),
            timestamp_ms: 0,
            tx_digest: "tx".into(),
            event_seq: 0,
            sender: None,
            checkpoint_seq: 1,
            swap_key: "tx:0:cetus".into(),
        };
        let tvl = estimate_tvl_quote(&swap).unwrap();
        assert!(tvl > Decimal::ZERO);
    }
}
