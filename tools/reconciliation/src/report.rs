use anyhow::Result;

use crate::diff::KeyDiff;
use crate::event_key::EventKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconciliationStatus {
    Match,
    KafkaLower,
    KafkaHigher,
    KeyMismatch,
}

pub struct ReconciliationReport {
    pub event_type: String,
    pub kafka_topic: String,
    pub window_start_ms: i64,
    pub window_end_ms: i64,
    pub kafka_count: i64,
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
        kafka_topic: String,
        window_start_ms: i64,
        window_end_ms: i64,
        kafka_count: i64,
        fullnode_count: u64,
        count_tolerance: i64,
        key_diff: KeyDiff,
        key_tolerance: usize,
        max_key_samples: usize,
    ) -> Self {
        let count_diff = (kafka_count - fullnode_count as i64).abs();
        let count_ok = count_diff <= count_tolerance;
        let keys_ok =
            key_diff.missing.len() <= key_tolerance && key_diff.extra.len() <= key_tolerance;

        let status = if keys_ok && count_ok {
            ReconciliationStatus::Match
        } else if !keys_ok {
            if key_diff.missing.len() > key_diff.extra.len() {
                ReconciliationStatus::KafkaLower
            } else if key_diff.extra.len() > key_diff.missing.len() {
                ReconciliationStatus::KafkaHigher
            } else {
                ReconciliationStatus::KeyMismatch
            }
        } else if kafka_count < fullnode_count as i64 {
            ReconciliationStatus::KafkaLower
        } else {
            ReconciliationStatus::KafkaHigher
        };

        Self {
            event_type,
            kafka_topic,
            window_start_ms,
            window_end_ms,
            kafka_count,
            fullnode_count,
            count_tolerance,
            key_diff,
            key_tolerance,
            max_key_samples,
            status,
        }
    }

    pub fn diff(&self) -> i64 {
        self.kafka_count - self.fullnode_count as i64
    }

    pub fn print(&self) {
        println!("=== Reconciliation (Kafka vs fullnode) ===");
        println!("event_type:       {}", self.event_type);
        println!("kafka_topic:      {}", self.kafka_topic);
        println!(
            "window:           {} .. {} (ms)",
            self.window_start_ms, self.window_end_ms
        );
        println!();
        println!("--- Phase 1: counts ---");
        println!("kafka_count:      {}", self.kafka_count);
        println!("fullnode_count:   {}", self.fullnode_count);
        println!("diff (kafka - fn): {}", self.diff());
        println!("count_tolerance:  {}", self.count_tolerance);
        println!();
        println!("--- Phase 2: key diff (txDigest#eventSeq) ---");
        println!("missing_in_kafka: {}", self.key_diff.missing.len());
        println!("extra_in_kafka:   {}", self.key_diff.extra.len());
        println!("key_tolerance:    {}", self.key_tolerance);

        print_key_samples("missing sample", &self.key_diff.missing, self.max_key_samples);
        print_key_samples("extra sample", &self.key_diff.extra, self.max_key_samples);

        println!();
        println!(
            "status:           {}",
            match self.status {
                ReconciliationStatus::Match => "MATCH",
                ReconciliationStatus::KafkaLower => "KAFKA_LOWER (events missing in Kafka)",
                ReconciliationStatus::KafkaHigher => "KAFKA_HIGHER (extra events in Kafka)",
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
    max_kafka_timestamp_ms: Option<i64>,
) -> Result<()> {
    if let Some(max_ts) = max_kafka_timestamp_ms {
        if window_end_ms > max_ts {
            tracing::warn!(
                window_end_ms,
                max_kafka_timestamp_ms = max_ts,
                "Window end is newer than max Kafka timestamp — indexer may still be catching up"
            );
        }
    }
    Ok(())
}
