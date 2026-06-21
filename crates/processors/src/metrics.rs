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

#[derive(Clone)]
pub struct MetricsBundle {
    pub swaps_fact_inserted: IntCounter,
    pub pool_liquidity_inserted: IntCounter,
    pub redis_writes: IntCounterVec,
    pub volume_skipped: IntCounterVec,
    pub ohlc_skipped: IntCounterVec,
    pub ohlc_buckets_updated: IntCounterVec,
    pub decode_errors: IntCounterVec,
}

impl MetricsBundle {
    pub fn new(registry: &Registry, worker: &str) -> anyhow::Result<Arc<Self>> {
        let swaps_fact_inserted = IntCounter::new(
            "processor_swaps_fact_inserted_total",
            "Rows inserted into swaps_fact",
        )?;
        let pool_liquidity_inserted = IntCounter::new(
            "processor_pool_liquidity_inserted_total",
            "Rows inserted into pool_liquidity",
        )?;
        let redis_writes = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_redis_writes_total",
                "Redis cache key writes",
            ),
            &["key_type"],
        )?;
        let volume_skipped = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_volume_skipped_total",
                "Swaps skipped by volume-engine",
            ),
            &["reason"],
        )?;
        let ohlc_skipped = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_ohlc_skipped_total",
                "Swaps skipped by ohlc-aggregator",
            ),
            &["reason"],
        )?;
        let ohlc_buckets_updated = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_ohlc_buckets_updated_total",
                "OHLC 1m buckets upserted",
            ),
            &["protocol"],
        )?;
        let decode_errors = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_decode_errors_total",
                "Kafka payload decode/handler errors",
            ),
            &["worker", "topic"],
        )?;

        registry.register(Box::new(swaps_fact_inserted.clone()))?;
        registry.register(Box::new(pool_liquidity_inserted.clone()))?;
        registry.register(Box::new(redis_writes.clone()))?;
        registry.register(Box::new(volume_skipped.clone()))?;
        registry.register(Box::new(ohlc_skipped.clone()))?;
        registry.register(Box::new(ohlc_buckets_updated.clone()))?;
        registry.register(Box::new(decode_errors.clone()))?;

        let _ = worker;

        Ok(Arc::new(Self {
            swaps_fact_inserted,
            pool_liquidity_inserted,
            redis_writes,
            volume_skipped,
            ohlc_skipped,
            ohlc_buckets_updated,
            decode_errors,
        }))
    }
}
