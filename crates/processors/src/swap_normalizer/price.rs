use anyhow::{Context, Result};
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::coin_type::{self, SUI_COIN_TYPE, USDC_COIN_TYPE};

const Q64: u128 = 1u128 << 64;

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

/// Q64.64 sqrt price → price of coin_y per coin_x, adjusted for decimals.
pub fn sqrt_price_to_quote_per_base(
    sqrt_price_after: &str,
    decimals_a: u32,
    decimals_b: u32,
    a_to_b: bool,
    quote_is_a: bool,
) -> Result<String> {
    let sqrt = u128::from_str(sqrt_price_after).context("invalid sqrt_price")?;
    let sqrt_f = Decimal::from(sqrt) / Decimal::from(Q64);
    let price_y_per_x = sqrt_f * sqrt_f;

    let decimal_adj = Decimal::from(10u64.pow(decimals_a))
        / Decimal::from(10u64.pow(decimals_b));

    let price_a_per_b = price_y_per_x * decimal_adj;
    let price_b_per_a = if price_a_per_b.is_zero() {
        Decimal::ZERO
    } else {
        Decimal::ONE / price_a_per_b
    };

    let price = if quote_is_a {
        if a_to_b {
            price_b_per_a
        } else {
            price_a_per_b
        }
    } else if a_to_b {
        price_a_per_b
    } else {
        price_b_per_a
    };

    Ok(price.normalize().to_string())
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
    fn sqrt_price_produces_positive_decimal() {
        let price = sqrt_price_to_quote_per_base(
            "18446744073709551616", // 2^64
            9,
            9,
            true,
            true,
        )
        .unwrap();
        assert!(!price.is_empty());
        assert_ne!(price, "0");
    }
}
