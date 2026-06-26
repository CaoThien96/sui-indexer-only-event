//! Application-specific Prometheus metrics (same registry as framework built-ins).

use anyhow::Result;
use prometheus::{
    Histogram, IntCounter, IntCounterVec, Registry, register_histogram_with_registry,
    register_int_counter_vec_with_registry, register_int_counter_with_registry,
};

#[derive(Clone)]
pub struct AppMetrics {
    /// Events matching `EVENT_TYPE_PREFIXES` seen during `process()`.
    pub events_matched: IntCounter,
    /// Static BCS decode failures, labeled by Move event type.
    pub decode_errors: IntCounterVec,
    /// Rows inserted into `package_events` (excludes conflict skips).
    pub rows_inserted: IntCounter,
    /// Bot reactor handler errors.
    pub bot_errors: IntCounter,
    /// Bot event handling latency in milliseconds.
    pub bot_event_latency_ms: Histogram,
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
            bot_errors: register_int_counter_with_registry!(
                "simple_sui_indexer_bot_errors_total",
                "Bot reactor handler errors",
                registry,
            )?,
            bot_event_latency_ms: register_histogram_with_registry!(
                "simple_sui_indexer_bot_event_latency_ms",
                "Bot reactor event handling latency in milliseconds",
                registry,
            )?,
        })
    }
}
