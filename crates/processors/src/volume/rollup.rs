use rust_decimal::Decimal;
use std::str::FromStr;

/// Sums quote-side volume from per-swap amounts (used for in-process rollups / tests).
pub fn sum_quote_volume(amounts: &[&str]) -> Decimal {
    amounts
        .iter()
        .filter_map(|raw| Decimal::from_str(raw).ok())
        .fold(Decimal::ZERO, |acc, v| acc + v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_quote_volume() {
        let total = sum_quote_volume(&["10.5", "20", "invalid", "0.5"]);
        assert_eq!(total, Decimal::from(31));
    }
}
