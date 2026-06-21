use prometheus::{IntCounter, IntCounterVec, Registry};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProcessorMetrics {
    pub catalog_rows_upserted: IntCounterVec,
    pub decode_errors: IntCounterVec,
    pub swap_normalized: IntCounterVec,
    pub swap_skipped: IntCounterVec,
    pub swap_missing_pool: IntCounter,
    pub swap_missing_decimals: IntCounterVec,
}

impl ProcessorMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Arc<Self>> {
        let catalog_rows_upserted = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_catalog_rows_upserted_total",
                "Catalog rows upserted by entity type",
            ),
            &["entity"],
        )?;
        let decode_errors = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_decode_errors_total",
                "Kafka payload decode/handler errors",
            ),
            &["worker", "topic"],
        )?;
        let swap_normalized = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_swap_normalized_total",
                "Swaps published to dex.swap.normalized.v1",
            ),
            &["protocol"],
        )?;
        let swap_skipped = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_swap_skipped_total",
                "Swaps skipped during normalization",
            ),
            &["reason"],
        )?;
        let swap_missing_pool = IntCounter::new(
            "processor_swap_missing_pool_total",
            "Swaps skipped because pool not in catalog",
        )?;
        let swap_missing_decimals = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_swap_missing_decimals_total",
                "Swaps normalized with default decimals (metadata missing)",
            ),
            &["coin_type"],
        )?;

        registry.register(Box::new(catalog_rows_upserted.clone()))?;
        registry.register(Box::new(decode_errors.clone()))?;
        registry.register(Box::new(swap_normalized.clone()))?;
        registry.register(Box::new(swap_skipped.clone()))?;
        registry.register(Box::new(swap_missing_pool.clone()))?;
        registry.register(Box::new(swap_missing_decimals.clone()))?;

        Ok(Arc::new(Self {
            catalog_rows_upserted,
            decode_errors,
            swap_normalized,
            swap_skipped,
            swap_missing_pool,
            swap_missing_decimals,
        }))
    }
}
