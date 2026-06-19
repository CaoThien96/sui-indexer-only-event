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
        Protocol::Cetus => (
            parsed_json.get("coin_type_a").and_then(Value::as_str),
            parsed_json.get("coin_type_b").and_then(Value::as_str),
        ),
        Protocol::Turbos => (
            parsed_json.get("coin_a").and_then(Value::as_str),
            parsed_json.get("coin_b").and_then(Value::as_str),
        ),
        Protocol::Bluefin => (
            parsed_json.get("coin_a").and_then(Value::as_str),
            parsed_json.get("coin_b").and_then(Value::as_str),
        ),
        Protocol::Mmt => (
            parsed_json
                .get("type_x")
                .and_then(|v| v.get("name"))
                .and_then(Value::as_str),
            parsed_json
                .get("type_y")
                .and_then(|v| v.get("name"))
                .and_then(Value::as_str),
        ),
        Protocol::Flowx => (
            parsed_json
                .get("coin_type_x")
                .and_then(|v| v.get("name"))
                .and_then(Value::as_str),
            parsed_json
                .get("coin_type_y")
                .and_then(|v| v.get("name"))
                .and_then(Value::as_str),
        ),
        Protocol::Magma => (
            parsed_json.get("coin_type_a").and_then(Value::as_str),
            parsed_json.get("coin_type_b").and_then(Value::as_str),
        ),
    };

    let tick_spacing = parsed_json
        .get("tick_spacing")
        .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
        .map(|v| v as u32);

    Ok(PoolCreateFields {
        pool_id,
        coin_type_a: coin_type_a.map(str::to_string),
        coin_type_b: coin_type_b.map(str::to_string),
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

    #[test]
    fn rejects_missing_pool_id() {
        let err = extract_pool_id(Protocol::Mmt, &json!({})).unwrap_err();
        assert!(err.to_string().contains("pool_id"));
    }
}
