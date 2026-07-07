use prometheus::{IntCounter, IntCounterVec, IntGauge, Registry};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProcessorMetrics {
    pub catalog_rows_upserted: IntCounterVec,
    pub catalog_skipped: IntCounterVec,
    pub decode_errors: IntCounterVec,
    pub swap_normalized: IntCounterVec,
    pub swap_skipped: IntCounterVec,
    pub pool_hydrated: IntCounterVec,
    pub token_metadata_hydrated: IntCounterVec,
    pub swap_deferred: IntCounterVec,
    pub swap_defer_retries: IntCounter,
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
        let catalog_skipped = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_catalog_skipped_total",
                "Catalog messages skipped during handling",
            ),
            &["reason"],
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
        let pool_hydrated = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_pool_hydrated_total",
                "Pools hydrated via gRPC in swap-normalizer",
            ),
            &["result"],
        )?;
        let token_metadata_hydrated = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_token_metadata_hydrated_total",
                "Token metadata hydrated via gRPC in swap-normalizer",
            ),
            &["result"],
        )?;
        let swap_deferred = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_swap_deferred_total",
                "Swaps deferred due to transient failures",
            ),
            &["reason"],
        )?;
        let swap_defer_retries = IntCounter::new(
            "processor_swap_defer_retries_total",
            "In-process retries for deferred swaps",
        )?;

        registry.register(Box::new(catalog_rows_upserted.clone()))?;
        registry.register(Box::new(catalog_skipped.clone()))?;
        registry.register(Box::new(decode_errors.clone()))?;
        registry.register(Box::new(swap_normalized.clone()))?;
        registry.register(Box::new(swap_skipped.clone()))?;
        registry.register(Box::new(pool_hydrated.clone()))?;
        registry.register(Box::new(token_metadata_hydrated.clone()))?;
        registry.register(Box::new(swap_deferred.clone()))?;
        registry.register(Box::new(swap_defer_retries.clone()))?;

        Ok(Arc::new(Self {
            catalog_rows_upserted,
            catalog_skipped,
            decode_errors,
            swap_normalized,
            swap_skipped,
            pool_hydrated,
            token_metadata_hydrated,
            swap_deferred,
            swap_defer_retries,
        }))
    }
}

#[derive(Clone)]
pub struct OracleBootstrapMetrics {
    pub checkpoints_scanned: IntCounter,
    pub swaps_matched: IntCounter,
    pub buckets_seeded: IntCounter,
    pub last_run_success: IntGauge,
}

impl OracleBootstrapMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Arc<Self>> {
        let checkpoints_scanned = IntCounter::new(
            "oracle_bootstrap_checkpoints_scanned_total",
            "Checkpoints scanned during oracle-bootstrap backward walk",
        )?;
        let swaps_matched = IntCounter::new(
            "oracle_bootstrap_swaps_matched_total",
            "SUI/USDC swaps matched on trusted pools during oracle-bootstrap",
        )?;
        let buckets_seeded = IntCounter::new(
            "oracle_bootstrap_buckets_seeded_total",
            "sui_usd_1m buckets upserted by oracle-bootstrap",
        )?;
        let last_run_success = IntGauge::new(
            "oracle_bootstrap_last_run_success",
            "1 if the last oracle-bootstrap run reached READY, else 0",
        )?;

        registry.register(Box::new(checkpoints_scanned.clone()))?;
        registry.register(Box::new(swaps_matched.clone()))?;
        registry.register(Box::new(buckets_seeded.clone()))?;
        registry.register(Box::new(last_run_success.clone()))?;

        Ok(Arc::new(Self {
            checkpoints_scanned,
            swaps_matched,
            buckets_seeded,
            last_run_success,
        }))
    }
}

#[derive(Clone)]
pub struct RolloffMetrics {
    pub rows: IntCounterVec,
    pub errors: IntCounterVec,
}

impl RolloffMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Arc<Self>> {
        let rows = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_rolloff_rows_total",
                "Rows rolled off from TimescaleDB to ClickHouse",
            ),
            &["table"],
        )?;
        let errors = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_rolloff_errors_total",
                "Roll-off errors by table",
            ),
            &["table"],
        )?;
        registry.register(Box::new(rows.clone()))?;
        registry.register(Box::new(errors.clone()))?;
        Ok(Arc::new(Self { rows, errors }))
    }
}

#[derive(Clone)]
pub struct MetricsBundle {
    pub swaps_fact_inserted: IntCounter,
    pub pool_liquidity_inserted: IntCounter,
    pub redis_writes: IntCounterVec,
    pub volume_skipped: IntCounterVec,
    pub token_ohlc_usd_upserts: IntCounterVec,
    pub decode_errors: IntCounterVec,
    pub token_usd_1m_upserts: IntCounter,
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
        let token_ohlc_usd_upserts = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_token_ohlc_usd_upserts_total",
                "Token USD OHLC buckets upserted by volume-engine",
            ),
            &["interval"],
        )?;
        let decode_errors = IntCounterVec::new(
            prometheus::Opts::new(
                "processor_decode_errors_total",
                "Kafka payload decode/handler errors",
            ),
            &["worker", "topic"],
        )?;
        let token_usd_1m_upserts = IntCounter::new(
            "processor_token_usd_1m_upserts_total",
            "Rows upserted into token_usd_1m or sui_usd_1m from volume-engine",
        )?;

        registry.register(Box::new(swaps_fact_inserted.clone()))?;
        registry.register(Box::new(pool_liquidity_inserted.clone()))?;
        registry.register(Box::new(redis_writes.clone()))?;
        registry.register(Box::new(volume_skipped.clone()))?;
        registry.register(Box::new(token_ohlc_usd_upserts.clone()))?;
        registry.register(Box::new(decode_errors.clone()))?;
        registry.register(Box::new(token_usd_1m_upserts.clone()))?;

        let _ = worker;

        Ok(Arc::new(Self {
            swaps_fact_inserted,
            pool_liquidity_inserted,
            redis_writes,
            volume_skipped,
            token_ohlc_usd_upserts,
            decode_errors,
            token_usd_1m_upserts,
        }))
    }
}
