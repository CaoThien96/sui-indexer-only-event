//! Normalize coin type strings to short canonical form (GraphQL style).

use move_core_types::{
    account_address::AccountAddress,
    language_storage::{StructTag, TypeTag},
};
use std::str::FromStr;

pub const SUI_COIN_TYPE: &str = "0x2::sui::SUI";
pub const USDC_COIN_TYPE: &str =
    "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC";

/// Normalize a coin type to short-address canonical form.
pub fn normalize(coin_type: &str) -> String {
    let trimmed = coin_type.trim();
    if let Some(normalized) = parse_struct_tag(trimmed) {
        return normalized;
    }
    trimmed.to_string()
}

/// Coin-type strings to match when querying `pools` (legacy rows may omit the `0x` prefix).
pub fn pool_lookup_variants(coin_type: &str) -> Vec<String> {
    let normalized = normalize(coin_type);
    let mut variants = vec![normalized.clone()];
    if let Some(stripped) = normalized.strip_prefix("0x") {
        variants.push(stripped.to_string());
    }
    variants.sort();
    variants.dedup();
    variants
}

fn parse_struct_tag(coin_type: &str) -> Option<String> {
    if let Ok(tag) = StructTag::from_str(coin_type) {
        return Some(struct_tag_to_short(&tag));
    }

    // MMT TypeName strings omit the `0x` prefix on the address segment.
    if let Some((addr, rest)) = coin_type.split_once("::") {
        if !addr.starts_with("0x") && addr.chars().all(|c| c.is_ascii_hexdigit()) {
            if let Ok(tag) = StructTag::from_str(&format!("0x{addr}::{rest}")) {
                return Some(struct_tag_to_short(&tag));
            }
        }
    }

    None
}

fn struct_tag_to_short(tag: &StructTag) -> String {
    let address = short_address(&tag.address);
    let mut out = format!("{}::{}::{}", address, tag.module, tag.name);
    if !tag.type_params.is_empty() {
        let params: Vec<String> = tag.type_params.iter().map(type_tag_to_short).collect();
        out.push('<');
        out.push_str(&params.join(", "));
        out.push('>');
    }
    out
}

fn type_tag_to_short(tag: &TypeTag) -> String {
    match tag {
        TypeTag::Bool => "bool".to_string(),
        TypeTag::U8 => "u8".to_string(),
        TypeTag::U64 => "u64".to_string(),
        TypeTag::U128 => "u128".to_string(),
        TypeTag::Address => "address".to_string(),
        TypeTag::Signer => "signer".to_string(),
        TypeTag::Vector(inner) => format!("vector<{}>", type_tag_to_short(inner)),
        TypeTag::Struct(s) => struct_tag_to_short(s),
        TypeTag::U16 => "u16".to_string(),
        TypeTag::U32 => "u32".to_string(),
        TypeTag::U256 => "u256".to_string(),
    }
}

fn short_address(addr: &AccountAddress) -> String {
    let hex = hex::encode(addr.as_ref());
    let trimmed = hex.trim_start_matches('0');
    let trimmed = if trimmed.is_empty() { "0" } else { trimmed };
    format!("0x{trimmed}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_sui_from_long_address() {
        let long = "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI";
        assert_eq!(normalize(long), SUI_COIN_TYPE);
    }

    #[test]
    fn preserves_short_form() {
        assert_eq!(normalize(SUI_COIN_TYPE), SUI_COIN_TYPE);
    }

    #[test]
    fn normalizes_mmt_type_name_without_0x_prefix() {
        let mmt_sui =
            "0000000000000000000000000000000000000000000000000000000000000002::sui::SUI";
        assert_eq!(normalize(mmt_sui), SUI_COIN_TYPE);
    }

    #[test]
    fn pool_lookup_variants_includes_legacy_without_0x() {
        let coin = "0x657b848cf176a16ae9f8eb3638e9d5f759c2c54fb92fd7e05255404dee455e73::sntl::SNTL";
        let variants = pool_lookup_variants(coin);
        assert!(variants.contains(&coin.to_string()));
        assert!(variants.contains(
            &"657b848cf176a16ae9f8eb3638e9d5f759c2c54fb92fd7e05255404dee455e73::sntl::SNTL"
                .to_string()
        ));
    }
}
