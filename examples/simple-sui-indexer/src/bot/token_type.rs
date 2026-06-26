use anyhow::{Context, Result};
use move_core_types::language_storage::StructTag;
use sui_types::TypeTag;

const SUI_TYPE: &str = "0x2::sui::SUI";

/// Canonical `0x<addr>::module::Struct` (Cetus events often omit the `0x` prefix).
pub fn normalize_coin_type(token: &str) -> String {
    let token = token.trim();
    if let Ok(tag) = token.parse::<StructTag>() {
        return tag.to_string();
    }
    if let Some(fixed) = add_0x_prefix_if_needed(token) {
        if let Ok(tag) = fixed.parse::<StructTag>() {
            return tag.to_string();
        }
    }
    token.to_string()
}

fn add_0x_prefix_if_needed(token: &str) -> Option<String> {
    let (addr, rest) = token.split_once("::")?;
    if addr.starts_with("0x") {
        return None;
    }
    if !addr.is_empty() && addr.chars().all(|c| c.is_ascii_hexdigit()) {
        return Some(format!("0x{addr}::{rest}"));
    }
    None
}

pub fn is_sui_coin_type(token: &str) -> bool {
    normalize_coin_type(token) == SUI_TYPE
}

/// Pick the non-SUI leg of a pool pair (memecoin side).
pub fn pick_non_sui_token(coin_a: Option<&str>, coin_b: Option<&str>) -> Option<String> {
    for coin in [coin_a, coin_b] {
        let coin = coin?;
        if !is_sui_coin_type(coin) {
            return Some(normalize_coin_type(coin));
        }
    }
    None
}

/// Parse `Pool<CoinA, CoinB, Fee>` and return the non-SUI coin type.
pub fn meme_token_from_pool_type(pool_type: &str) -> Option<String> {
    let inner = pool_type.split('<').nth(1)?.trim_end_matches('>');
    let mut parts = inner.split(',').map(str::trim);
    let coin_a = parts.next()?;
    let coin_b = parts.next()?;
    pick_non_sui_token(Some(coin_a), Some(coin_b))
}

/// Parse `Package::module::Pool<CoinA, CoinB, Fee>` generics from the on-chain pool type.
pub fn parse_pool_generics(pool_type: &str) -> Option<(String, String, String, String)> {
    let package = pool_type.split("::").next()?.to_string();
    let inner = pool_type.split('<').nth(1)?.trim_end_matches('>');
    let mut parts = inner.split(',').map(str::trim);
    let coin_a = normalize_coin_type(parts.next()?);
    let coin_b = normalize_coin_type(parts.next()?);
    let fee = normalize_coin_type(parts.next()?);
    Some((package, coin_a, coin_b, fee))
}

pub fn symbol_from_coin_type(token: &str) -> String {
    normalize_coin_type(token)
        .split("::")
        .last()
        .unwrap_or("TOKEN")
        .to_string()
}

pub fn parse_type_tag(s: &str) -> Result<TypeTag> {
    let canonical = normalize_coin_type(s);
    Ok(TypeTag::Struct(Box::new(
        canonical
            .parse::<StructTag>()
            .with_context(|| format!("Invalid struct type: {canonical}"))?,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_adds_0x_prefix_from_cetus_event_format() {
        let raw = "9ba2573e31978148d69aeab42eeb0cf241b84539030dd1dd0fc82216088b4b68::valora::VALORA";
        let normalized = normalize_coin_type(raw);
        assert!(normalized.starts_with("0x"));
        assert!(normalized.contains("::valora::VALORA"));
        parse_type_tag(raw).expect("should parse after normalization");
    }
}
