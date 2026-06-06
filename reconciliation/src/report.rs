use anyhow::Result;

use crate::diff::KeyDiff;
use crate::event_key::EventKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconciliationStatus {
    Match,
    IndexerLower,
    IndexerHigher,
    KeyMismatch,
}

pub struct ReconciliationReport {
    pub event_type: String,
    pub window_start_ms: i64,
    pub window_end_ms: i64,
    pub indexer_count: i64,
    pub fullnode_count: u64,
    pub count_tolerance: i64,
    pub key_diff: KeyDiff,
    pub key_tolerance: usize,
    pub max_key_samples: usize,
    pub status: ReconciliationStatus,
}

impl ReconciliationReport {
    pub fn new(
        event_type: String,
        window_start_ms: i64,
        window_end_ms: i64,
        indexer_count: i64,
        fullnode_count: u64,
        count_tolerance: i64,
        key_diff: KeyDiff,
        key_tolerance: usize,
        max_key_samples: usize,
    ) -> Self {
        let count_diff = (indexer_count - fullnode_count as i64).abs();
        let count_ok = count_diff <= count_tolerance;
        let keys_ok =
            key_diff.missing.len() <= key_tolerance && key_diff.extra.len() <= key_tolerance;

        let status = if keys_ok && count_ok {
            ReconciliationStatus::Match
        } else if !keys_ok {
            if key_diff.missing.len() > key_diff.extra.len() {
                ReconciliationStatus::IndexerLower
            } else if key_diff.extra.len() > key_diff.missing.len() {
                ReconciliationStatus::IndexerHigher
            } else {
                ReconciliationStatus::KeyMismatch
            }
        } else if indexer_count < fullnode_count as i64 {
            ReconciliationStatus::IndexerLower
        } else {
            ReconciliationStatus::IndexerHigher
        };

        Self {
            event_type,
            window_start_ms,
            window_end_ms,
            indexer_count,
            fullnode_count,
            count_tolerance,
            key_diff,
            key_tolerance,
            max_key_samples,
            status,
        }
    }

    pub fn diff(&self) -> i64 {
        self.indexer_count - self.fullnode_count as i64
    }

    pub fn print(&self) {
        println!("=== Reconciliation Phase 2 (count + key diff) ===");
        println!("event_type:       {}", self.event_type);
        println!(
            "window:           {} .. {} (ms)",
            self.window_start_ms, self.window_end_ms
        );
        println!();
        println!("--- Phase 1: counts ---");
        println!("indexer_count:    {}", self.indexer_count);
        println!("fullnode_count:   {}", self.fullnode_count);
        println!("diff (idx - fn):  {}", self.diff());
        println!("count_tolerance:  {}", self.count_tolerance);
        println!();
        println!("--- Phase 2: key diff (txDigest#eventSeq) ---");
        println!("missing_in_indexer: {}", self.key_diff.missing.len());
        println!("extra_in_indexer:   {}", self.key_diff.extra.len());
        println!("key_tolerance:      {}", self.key_tolerance);

        print_key_samples("missing sample", &self.key_diff.missing, self.max_key_samples);
        print_key_samples("extra sample", &self.key_diff.extra, self.max_key_samples);

        println!();
        println!(
            "status:           {}",
            match self.status {
                ReconciliationStatus::Match => "MATCH",
                ReconciliationStatus::IndexerLower => "INDEXER_LOWER (events missing in DB)",
                ReconciliationStatus::IndexerHigher => "INDEXER_HIGHER (extra events in DB)",
                ReconciliationStatus::KeyMismatch => "KEY_MISMATCH",
            }
        );
    }

    pub fn is_ok(&self) -> bool {
        self.status == ReconciliationStatus::Match
    }
}

fn print_key_samples(label: &str, keys: &[EventKey], max_samples: usize) {
    if keys.is_empty() {
        return;
    }

    println!("{label}:");
    for key in keys.iter().take(max_samples) {
        println!("  - {key}");
    }
    if keys.len() > max_samples {
        println!("  ... and {} more", keys.len() - max_samples);
    }
}

pub fn warn_if_window_beyond_indexed_data(
    window_end_ms: i64,
    max_indexed_timestamp_ms: Option<i64>,
) -> Result<()> {
    if let Some(max_ts) = max_indexed_timestamp_ms {
        if window_end_ms > max_ts {
            tracing::warn!(
                window_end_ms,
                max_indexed_timestamp_ms = max_ts,
                "Window end is newer than max indexed timestamp — indexer may still be catching up"
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::KeyDiff;
    use crate::event_key::EventKey;

    fn key(tx: &str, seq: i64) -> EventKey {
        EventKey {
            tx_digest: tx.to_string(),
            event_seq: seq,
        }
    }

    #[test]
    fn match_when_no_key_diff() {
        let report = ReconciliationReport::new(
            "t".into(),
            0,
            1,
            10,
            10,
            0,
            KeyDiff {
                missing: vec![],
                extra: vec![],
            },
            0,
            20,
        );
        assert!(report.is_ok());
    }

    #[test]
    fn fails_on_missing_keys() {
        let report = ReconciliationReport::new(
            "t".into(),
            0,
            1,
            9,
            10,
            100,
            KeyDiff {
                missing: vec![key("a", 0)],
                extra: vec![],
            },
            0,
            20,
        );
        assert_eq!(report.status, ReconciliationStatus::IndexerLower);
        assert!(!report.is_ok());
    }
}
