use anyhow::Result;
use serde_json::Value;

/// Sync static BCS decode — dispatches on full lowercase `event_type`.
pub fn decode_parsed_json(event_type: &str, bcs: &[u8]) -> Result<Value> {
    event_bindings::decode_parsed_json(event_type, bcs)
}
