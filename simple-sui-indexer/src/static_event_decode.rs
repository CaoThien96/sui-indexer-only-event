use anyhow::Result;
use serde_json::Value;

/// Sync static BCS decode — dispatches on canonical `event_type` (ASCII case-insensitive).
pub fn decode_parsed_json(event_type: &str, bcs: &[u8]) -> Result<Value> {
    event_bindings::decode_parsed_json(event_type, bcs)
}
