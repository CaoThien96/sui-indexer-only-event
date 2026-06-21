use anyhow::{Context, Result};
use serde_json::Value;

use crate::protocol::Protocol;

/// Extract pool id from decoded `parsed_json` for Kafka partition key.
pub fn extract_pool_id(protocol: Protocol, parsed_json: &Value) -> Result<String> {
    let field = protocol.pool_id_field();

    parsed_json
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_string)
        .with_context(|| format!("missing pool id field `{field}` for {}", protocol.as_str()))
}

fn read_type_name(parsed: &Value, key: &str) -> Option<String> {
    match parsed.get(key)? {
        Value::String(s) => Some(s.clone()),
        Value::Object(map) => map
            .get("name")
            .and_then(Value::as_str)
            .map(str::to_string),
        _ => None,
    }
}

/// Extract pool id fields for pool-create facts.
pub fn extract_pool_create_fields(
    protocol: Protocol,
    parsed_json: &Value,
) -> Result<PoolCreateFields> {
    let pool_id = match protocol {
        Protocol::Cetus => parsed_json
            .get("pool_id")
            .or_else(|| parsed_json.get("id"))
            .and_then(Value::as_str),
        Protocol::Turbos => parsed_json
            .get("pool")
            .or_else(|| parsed_json.get("pool_id"))
            .and_then(Value::as_str),
        Protocol::Bluefin => parsed_json.get("id").and_then(Value::as_str),
        Protocol::Mmt => parsed_json.get("pool_id").and_then(Value::as_str),
        Protocol::Flowx => parsed_json.get("pool_id").and_then(Value::as_str),
        Protocol::Magma => parsed_json.get("pool_id").and_then(Value::as_str),
    }
    .map(str::to_string)
    .with_context(|| format!("missing pool id in pool-create event for {}", protocol.as_str()))?;

    let (coin_type_a, coin_type_b) = match protocol {
        Protocol::Cetus | Protocol::Magma => (
            read_type_name(parsed_json, "coin_type_a"),
            read_type_name(parsed_json, "coin_type_b"),
        ),
        Protocol::Turbos | Protocol::Bluefin => (
            read_type_name(parsed_json, "coin_a"),
            read_type_name(parsed_json, "coin_b"),
        ),
        Protocol::Mmt => (
            read_type_name(parsed_json, "type_x"),
            read_type_name(parsed_json, "type_y"),
        ),
        Protocol::Flowx => (
            read_type_name(parsed_json, "coin_type_x"),
            read_type_name(parsed_json, "coin_type_y"),
        ),
    };

    let tick_spacing = parsed_json
        .get("tick_spacing")
        .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
        .map(|v| v as u32);

    Ok(PoolCreateFields {
        pool_id,
        coin_type_a,
        coin_type_b,
        tick_spacing,
    })
}

#[derive(Debug, Clone)]
pub struct PoolCreateFields {
    pub pool_id: String,
    pub coin_type_a: Option<String>,
    pub coin_type_b: Option<String>,
    pub tick_spacing: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_cetus_swap_pool_id() {
        let parsed = json!({ "pool": "0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9" });
        let id = extract_pool_id(Protocol::Cetus, &parsed).unwrap();
        assert_eq!(id, "0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9");
    }

    #[test]
    fn extracts_bluefin_swap_pool_id() {
        let parsed = json!({ "pool_id": "0xb62e60fa7dc5b32f069532ff8182a6aee264dd7ae658c863db186e68b4e06229" });
        let id = extract_pool_id(Protocol::Bluefin, &parsed).unwrap();
        assert!(id.starts_with("0x"));
    }

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    }

    const MMT_POOL_CREATE_BCS: &str = "506ecadb1d93eb2f9e7e1d32e5146b60d734f6d02bd763e8ec705ba00eaded3057e2a855ab75bffe6095e49b27617666a9840ff6ef09e27e29fe3566c315e8814a303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030323a3a7375693a3a5355494c373237343538323132636130626530353664396363633064343239383162646634313130393737396662643963376661643536633434353665306333643063363a3a626565673a3a42454547c40900000000000032000000";

    const MAGMA_POOL_CREATE_BCS: &str = "470898ac48eead3db575d66efac344cc652844e8ebdbf3f7c9cc6a9423486e8d4e643162373239383265343033343864303639626231666637303165363334633131376262356637343166343464666639316534373264336230313436316535353a3a73747375693a3a53545355494a303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030323a3a7375693a3a5355493c000000";

    const FLOWX_POOL_CREATE_BCS: &str = "ca0aab96d2b0f7a1d3a26c48e8d3c61c9037585a1e20782cc1a722e7966db041e41cc018b503930d83a912107119f5ce85c377e770ad4e5660be92e998d01c974a346339383166336666373836636462396535313464613839376162386139353336343764616532616365393637396538333538656563316533653838373161633a3a646d633a3a444d434c623435666366636332636330376365303730326363326432323936323165303436633930366566313464396232356538653464323566366538373633666566373a3a73656e643a3a53454e441027000000000000c8000000";

    #[test]
    fn extracts_coin_types_from_flowx_pool_create() {
        use crate::{config::flowx, decode_parsed_json};
        let parsed = decode_parsed_json(
            flowx::POOL_CREATE_EVENT,
            &hex_to_bytes(FLOWX_POOL_CREATE_BCS),
        )
        .unwrap();
        let fields =
            extract_pool_create_fields(Protocol::Flowx, &parsed).expect("flowx pool create fields");
        assert!(
            fields.coin_type_a.is_some() && fields.coin_type_b.is_some(),
            "parsed={parsed}"
        );
    }

    #[test]
    fn extracts_coin_types_from_mmt_pool_create() {
        use crate::{config::mmt, decode_parsed_json};
        let parsed =
            decode_parsed_json(mmt::POOL_CREATE_EVENT, &hex_to_bytes(MMT_POOL_CREATE_BCS)).unwrap();
        let fields =
            extract_pool_create_fields(Protocol::Mmt, &parsed).expect("mmt pool create fields");
        assert!(
            fields.coin_type_a.is_some() && fields.coin_type_b.is_some(),
            "parsed={parsed}"
        );
    }

    #[test]
    fn extracts_coin_types_from_magma_pool_create() {
        use crate::{config::magma, decode_parsed_json};
        let parsed = decode_parsed_json(
            magma::POOL_CREATE_EVENT,
            &hex_to_bytes(MAGMA_POOL_CREATE_BCS),
        )
        .unwrap();
        let fields =
            extract_pool_create_fields(Protocol::Magma, &parsed).expect("magma pool create fields");
        assert!(
            fields.coin_type_a.is_some() && fields.coin_type_b.is_some(),
            "parsed={parsed}"
        );
    }

    #[test]
    fn rejects_missing_pool_id() {
        let err = extract_pool_id(Protocol::Mmt, &json!({})).unwrap_err();
        assert!(err.to_string().contains("pool_id"));
    }
}
