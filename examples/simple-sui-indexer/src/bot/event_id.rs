//! Stable on-chain event identity: `(tx_digest, event_seq)`.

pub fn format_event_id(tx_digest: &str, event_seq: impl std::fmt::Display) -> String {
    format!("{tx_digest}:{event_seq}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_event_id_is_tx_and_seq() {
        assert_eq!(
            format_event_id("0xabc", 3),
            "0xabc:3"
        );
    }
}
