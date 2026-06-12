#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::kafka::{FactTopic, MessageEnvelope, compute_message_id};

    #[test]
    fn envelope_has_schema_version_and_message_id() {
        let key = "checkpoint:12345:dex.swap.raw.v1";
        let envelope = MessageEnvelope::new(key, json!({"kind": "checkpoint_heartbeat"}));

        assert_eq!(envelope.schema_version, 1);
        assert_eq!(envelope.message_id, compute_message_id(key));
        assert!(envelope.produced_at_ms > 0);
        assert_eq!(envelope.payload["kind"], "checkpoint_heartbeat");
    }

    #[test]
    fn message_id_is_deterministic_sha256_hex() {
        let key = "abc:0:dex.swap.raw.v1";
        let id1 = compute_message_id(key);
        let id2 = compute_message_id(key);
        assert_eq!(id1, id2);
        assert_eq!(id1.len(), 64);
    }

    #[test]
    fn topic_names_match_contract() {
        assert_eq!(FactTopic::SwapRaw.as_str(), "dex.swap.raw.v1");
        assert_eq!(FactTopic::PoolRaw.as_str(), "dex.pool.raw.v1");
        assert_eq!(
            FactTopic::TokenMetadataRaw.as_str(),
            "token.metadata.raw.v1"
        );
    }
}
