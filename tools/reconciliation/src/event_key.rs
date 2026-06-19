use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

use serde_json::Value;

#[derive(Debug, Clone, Eq)]
pub struct EventKey {
    pub tx_digest: String,
    pub event_seq: i64,
}

impl PartialEq for EventKey {
    fn eq(&self, other: &Self) -> bool {
        self.tx_digest == other.tx_digest && self.event_seq == other.event_seq
    }
}

impl Hash for EventKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tx_digest.hash(state);
        self.event_seq.hash(state);
    }
}

impl PartialOrd for EventKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tx_digest
            .cmp(&other.tx_digest)
            .then(self.event_seq.cmp(&other.event_seq))
    }
}

impl fmt::Display for EventKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.tx_digest, self.event_seq)
    }
}

impl EventKey {
    pub fn from_fullnode_event(event: &Value) -> Option<Self> {
        let id = event.get("id")?;
        let tx_digest = id.get("txDigest")?.as_str()?.to_string();
        let event_seq = parse_event_seq_value(id.get("eventSeq")?)?;
        Some(Self {
            tx_digest,
            event_seq,
        })
    }

    pub fn from_kafka_fact(fact: &Value) -> Option<Self> {
        let tx_digest = fact.get("tx_digest")?.as_str()?.to_string();
        let event_seq = fact
            .get("event_sequence_in_transaction")
            .or_else(|| fact.get("event_seq"))
            .and_then(parse_event_seq_value)?;
        Some(Self {
            tx_digest,
            event_seq,
        })
    }
}

pub fn parse_event_seq_value(value: &Value) -> Option<i64> {
    value
        .as_str()
        .and_then(|s| s.parse().ok())
        .or_else(|| value.as_i64())
        .or_else(|| value.as_u64().map(|v| v as i64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_fullnode_event_id() {
        let event = json!({
            "id": { "txDigest": "abc123", "eventSeq": "7" }
        });
        let key = EventKey::from_fullnode_event(&event).unwrap();
        assert_eq!(key.tx_digest, "abc123");
        assert_eq!(key.event_seq, 7);
    }

    #[test]
    fn parses_kafka_swap_fact() {
        let fact = json!({
            "tx_digest": "abc123",
            "event_sequence_in_transaction": 3
        });
        let key = EventKey::from_kafka_fact(&fact).unwrap();
        assert_eq!(key.event_seq, 3);
    }
}
