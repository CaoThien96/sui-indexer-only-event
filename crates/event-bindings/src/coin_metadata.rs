//! Decode `0x2::coin::CoinMetadata<T>` objects from checkpoint object BCS.
//!
//! Regulated coins also create `0x2::coin::RegulatedCoinMetadata<T>` (links to CoinMetadata id);
//! display fields live on `CoinMetadata` — index that type per Mysten GP indexer / GraphQL model.

use anyhow::{Context, Result};
use move_core_types::{
    account_address::AccountAddress,
    language_storage::StructTag,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::str::FromStr;

const COIN_MODULE: &str = "coin";
const COIN_METADATA_STRUCT: &str = "CoinMetadata";

fn sui_framework_address() -> AccountAddress {
    AccountAddress::from_hex_literal("0x2").expect("valid Sui framework address")
}

#[derive(Debug, Deserialize)]
struct MoveUrl {
    url: String,
}

#[derive(Debug, Clone)]
pub struct DecodedCoinMetadata {
    pub coin_type: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub description: String,
    pub image_url: Option<String>,
    pub object_id: String,
}

/// Check struct tag matches `0x2::coin::CoinMetadata<T>`.
pub fn is_coin_metadata_struct(tag: &StructTag) -> bool {
    tag.address == sui_framework_address()
        && tag.module.as_str() == COIN_MODULE
        && tag.name.as_str() == COIN_METADATA_STRUCT
        && tag.type_params.len() == 1
}

/// Check struct tag matches `0x2::coin::CoinMetadata<T>` from a canonical or short type string.
pub fn is_coin_metadata_type(type_str: &str) -> bool {
    StructTag::from_str(type_str)
        .map(|tag| is_coin_metadata_struct(&tag))
        .unwrap_or(false)
}

/// Extract coin type param from `0x2::coin::CoinMetadata<CoinType>`.
pub fn extract_coin_type(type_str: &str) -> Option<String> {
    let tag = StructTag::from_str(type_str).ok()?;
    if !is_coin_metadata_struct(&tag) {
        return None;
    }
    tag.type_params.first().map(|param| param.to_string())
}

fn format_id(bytes: [u8; 32]) -> String {
    format!("0x{}", hex::encode(bytes))
}

/// Decode object BCS contents for a `CoinMetadata` object.
pub fn decode_coin_metadata_object(coin_type: &str, bcs: &[u8]) -> Result<DecodedCoinMetadata> {

    #[derive(Debug, Deserialize)]
    struct CoinMetadataBody {
        id: [u8; 32],
        decimals: u8,
        name: String,
        symbol: String,
        description: String,
        icon_url: Option<MoveUrl>,
    }

    let decoded: CoinMetadataBody = bcs::from_bytes(bcs)
        .with_context(|| format!("failed to BCS decode CoinMetadata for {coin_type}"))?;

    Ok(DecodedCoinMetadata {
        coin_type: coin_type.to_string(),
        name: decoded.name,
        symbol: decoded.symbol,
        decimals: decoded.decimals,
        description: decoded.description,
        image_url: decoded.icon_url.map(|u| u.url),
        object_id: format_id(decoded.id),
    })
}

pub fn decoded_to_json(meta: &DecodedCoinMetadata) -> Value {
    json!({
        "coin_type": meta.coin_type,
        "name": meta.name,
        "symbol": meta.symbol,
        "decimals": meta.decimals,
        "description": meta.description,
        "image_url": meta.image_url,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLX_COIN_METADATA_TYPE: &str = "0x2::coin::CoinMetadata<0x6dae8ca14311574fdfe555524ea48558e3d1360d1607d1c7f98af867e3b7976c::flx::FLX>";
    const FLX_COIN_METADATA_BCS: &str = "0bfe34db38444ff4a38e44b86128c7e02d551b8fdca9709fde140292d1ef6e950805466c6f775803464c580000";

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn recognizes_coin_metadata_type() {
        assert!(is_coin_metadata_type(FLX_COIN_METADATA_TYPE));
        assert!(!is_coin_metadata_type("0x2::coin::Coin<0x2::sui::SUI>"));
    }

    #[test]
    fn recognizes_coin_metadata_canonical_type_string() {
        let canonical = "0x0000000000000000000000000000000000000000000000000000000000000002::coin::CoinMetadata<0x6dae8ca14311574fdfe555524ea48558e3d1360d1607d1c7f98af867e3b7976c::flx::FLX>";
        assert!(is_coin_metadata_type(canonical));
    }

    #[test]
    fn decodes_flx_coin_metadata_from_mainnet_bcs() {
        let coin_type = "0x6dae8ca14311574fdfe555524ea48558e3d1360d1607d1c7f98af867e3b7976c::flx::FLX";
        let decoded = decode_coin_metadata_object(coin_type, &hex_to_bytes(FLX_COIN_METADATA_BCS))
        .unwrap();
        assert_eq!(decoded.name, "FlowX");
        assert_eq!(decoded.symbol, "FLX");
        assert_eq!(decoded.decimals, 8);
        assert_eq!(
            decoded.coin_type,
            "0x6dae8ca14311574fdfe555524ea48558e3d1360d1607d1c7f98af867e3b7976c::flx::FLX"
        );
        assert_eq!(
            decoded.object_id,
            "0x0bfe34db38444ff4a38e44b86128c7e02d551b8fdca9709fde140292d1ef6e95"
        );
        assert!(decoded.image_url.is_none());
    }
}
