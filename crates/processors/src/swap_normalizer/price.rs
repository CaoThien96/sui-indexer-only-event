use anyhow::{Context, Result};
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::coin_type::{self, SUI_COIN_TYPE, USDC_COIN_TYPE};

/// Assign quote/base per frozen product rules.
pub fn assign_quote_base(coin_type_a: &str, coin_type_b: &str) -> (String, String, String) {
    let a = coin_type::normalize(coin_type_a);
    let b = coin_type::normalize(coin_type_b);

    if a == SUI_COIN_TYPE || b == SUI_COIN_TYPE {
        if a == SUI_COIN_TYPE {
            return (b.clone(), a.clone(), a);
        }
        return (a.clone(), b.clone(), b);
    }

    if a == USDC_COIN_TYPE || b == USDC_COIN_TYPE {
        if a == USDC_COIN_TYPE {
            return (b.clone(), a.clone(), a);
        }
        return (a.clone(), b.clone(), b);
    }

    (b.clone(), a.clone(), a)
}

/// Map pool swap amounts to base/quote raw strings.
pub fn map_amounts_to_base_quote(
    a_to_b: bool,
    coin_type_a: &str,
    _coin_type_b: &str,
    base_coin_type: &str,
    amount_in_raw: &str,
    amount_out_raw: &str,
) -> (String, String) {
    let a = coin_type::normalize(coin_type_a);
    let base = coin_type::normalize(base_coin_type);

    let (amount_a, amount_b) = if a_to_b {
        (amount_in_raw.to_string(), amount_out_raw.to_string())
    } else {
        (amount_out_raw.to_string(), amount_in_raw.to_string())
    };

    if base == a {
        (amount_a, amount_b)
    } else {
        (amount_b, amount_a)
    }
}

pub fn raw_to_decimal(raw: &str, decimals: u32) -> Result<String> {
    let value = Decimal::from_str(raw).context("invalid raw amount")?;
    let scale = Decimal::from(10u64.pow(decimals));
    Ok((value / scale).normalize().to_string())
}

/// Execution price quote/base from decimal trade amounts (canonical for charts).
pub fn price_from_trade_amounts(amount_quote_decimal: &str, amount_base_decimal: &str) -> Result<String> {
    let quote = Decimal::from_str(amount_quote_decimal).context("invalid quote decimal")?;
    let base = Decimal::from_str(amount_base_decimal).context("invalid base decimal")?;
    if base.is_zero() {
        anyhow::bail!("amount_base_decimal is zero");
    }
    Ok((quote / base).normalize().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sui_pool_uses_sui_as_quote() {
        let (base, quote, quote_type) =
            assign_quote_base(SUI_COIN_TYPE, "0xabc::token::TKN");
        assert_eq!(quote, SUI_COIN_TYPE);
        assert_eq!(quote_type, SUI_COIN_TYPE);
        assert_ne!(base, SUI_COIN_TYPE);
    }

    #[test]
    fn raw_to_decimal_scales() {
        assert_eq!(raw_to_decimal("1000000000", 9).unwrap(), "1");
    }

    #[test]
    fn usdc_pool_uses_usdc_as_quote_when_no_sui() {
        let token = "0xabc::token::TKN";
        let (base, quote, quote_type) = assign_quote_base(token, USDC_COIN_TYPE);
        assert_eq!(quote, USDC_COIN_TYPE);
        assert_eq!(quote_type, USDC_COIN_TYPE);
        assert_ne!(base, USDC_COIN_TYPE);
    }

    #[test]
    fn exotic_pair_uses_coin_a_as_quote() {
        let a = "0xaaa::a::A";
        let b = "0xbbb::b::B";
        let (base, quote, quote_type) = assign_quote_base(a, b);
        assert_eq!(quote, a);
        assert_eq!(quote_type, a);
        assert_eq!(base, b);
    }

    #[test]
    fn map_amounts_to_base_quote_respects_direction() {
        let (base_raw, quote_raw) = map_amounts_to_base_quote(
            true,
            SUI_COIN_TYPE,
            "0xabc::token::TKN",
            "0xabc::token::TKN",
            "100",
            "200",
        );
        assert_eq!(base_raw, "200");
        assert_eq!(quote_raw, "100");
    }

    #[test]
    fn price_from_trade_amounts_matches_execution_price() {
        // STARBASE/SUI: ~25 SUI for ~1.5M tokens ≈ 0.000017 SUI per token
        let price = price_from_trade_amounts("25.453951421", "1500529.141321119").unwrap();
        assert!(price.starts_with("0.00001"));
        let p = Decimal::from_str(&price).unwrap();
        assert!(p < Decimal::ONE);
    }
}
