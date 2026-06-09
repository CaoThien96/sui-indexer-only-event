//! Application-specific Prometheus metrics (same registry as framework built-ins).

use anyhow::Result;
use prometheus::{
    IntCounter, IntCounterVec, Registry, register_int_counter_vec_with_registry,
    register_int_counter_with_registry,
};

#[derive(Clone)]
pub struct AppMetrics {
    /// Events matching `EVENT_TYPE_PREFIXES` seen during `process()`.
    pub events_matched: IntCounter,
    /// Static BCS decode failures, labeled by Move event type.
    pub decode_errors: IntCounterVec,
    /// Rows inserted into `package_events` (excludes conflict skips).
    pub rows_inserted: IntCounter,
}

impl AppMetrics {
    pub fn register(registry: &Registry) -> Result<Self> {
        Ok(Self {
            events_matched: register_int_counter_with_registry!(
                "simple_sui_indexer_events_matched_total",
                "Move events matching configured prefixes across all checkpoints",
                registry,
            )?,
            decode_errors: register_int_counter_vec_with_registry!(
                "simple_sui_indexer_decode_errors_total",
                "Static BCS decode failures by Move event type",
                &["event_type"],
                registry,
            )?,
            rows_inserted: register_int_counter_with_registry!(
                "simple_sui_indexer_package_events_inserted_total",
                "Rows inserted into package_events (on_conflict do_nothing excluded)",
                registry,
            )?,
        })
    }
}
