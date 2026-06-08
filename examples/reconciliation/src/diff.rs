use std::collections::{HashSet, VecDeque};

use crate::event_key::EventKey;

pub struct KeyDiff {
    pub missing: Vec<EventKey>,
    pub extra: Vec<EventKey>,
}

pub fn diff_keys(indexer: HashSet<EventKey>, fullnode: HashSet<EventKey>) -> KeyDiff {
    let mut missing: Vec<EventKey> = fullnode.difference(&indexer).cloned().collect();
    missing.sort();

    let mut extra: Vec<EventKey> = indexer.difference(&fullnode).cloned().collect();
    extra.sort();

    KeyDiff { missing, extra }
}

#[allow(dead_code)]
pub fn sample_keys(keys: &[EventKey], max_samples: usize) -> VecDeque<EventKey> {
    keys.iter().take(max_samples).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(tx: &str, seq: i64) -> EventKey {
        EventKey {
            tx_digest: tx.to_string(),
            event_seq: seq,
        }
    }

    #[test]
    fn computes_missing_and_extra() {
        let indexer = HashSet::from([key("a", 0), key("b", 1)]);
        let fullnode = HashSet::from([key("a", 0), key("c", 2)]);

        let diff = diff_keys(indexer, fullnode);
        assert_eq!(diff.missing, vec![key("c", 2)]);
        assert_eq!(diff.extra, vec![key("b", 1)]);
    }
}
