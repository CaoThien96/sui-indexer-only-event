use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct BootstrapReadiness {
    pub ready: bool,
    pub bucket_count: i64,
    pub anchor_time: Option<DateTime<Utc>>,
    pub staleness_minutes: Option<i64>,
    pub anchor_checkpoint: Option<u64>,
    pub trusted_pool_id: Option<String>,
}

pub fn evaluate_readiness(
    buckets: &HashMap<DateTime<Utc>, MinuteAccumulator>,
    boundary_time: DateTime<Utc>,
    min_buckets: i64,
    max_price_age_minutes: i64,
) -> BootstrapReadiness {
    let bucket_count = buckets.len() as i64;
    let anchor_time = buckets.keys().max().copied();
    let staleness_minutes = anchor_time.map(|t| {
        let delta = boundary_time.signed_duration_since(t);
        delta.num_minutes().max(0)
    });

    let ready = bucket_count >= min_buckets
        && staleness_minutes.is_some_and(|age| age <= max_price_age_minutes);

    BootstrapReadiness {
        ready,
        bucket_count,
        anchor_time,
        staleness_minutes,
        anchor_checkpoint: None,
        trusted_pool_id: None,
    }
}

#[derive(Debug, Default, Clone)]
pub struct MinuteAccumulator {
    pub sui_raw: Decimal,
    pub usdc_raw: Decimal,
    pub checkpoint_seq: u64,
    pub pool_id: String,
}

impl MinuteAccumulator {
    pub fn merge(&mut self, sui: Decimal, usdc: Decimal, checkpoint_seq: u64, pool_id: &str) {
        self.sui_raw += sui;
        self.usdc_raw += usdc;
        if checkpoint_seq >= self.checkpoint_seq {
            self.checkpoint_seq = checkpoint_seq;
            self.pool_id = pool_id.to_string();
        }
    }

    pub fn price_usd(&self) -> Option<Decimal> {
        if self.sui_raw.is_zero() {
            return None;
        }
        Some((self.usdc_raw / self.sui_raw).normalize())
    }
}
