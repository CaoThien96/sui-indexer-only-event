use std::env;

use anyhow::{Context, Result};
use url::Url;

pub fn load_dotenv() {
    let _ = dotenvy::dotenv();
}

pub fn database_url() -> Result<Url> {
    indexer_store::postgres_url::resolve_postgres_url("POSTGRES", "DATABASE_URL")
}

pub fn kafka_brokers() -> Result<String> {
    env::var("KAFKA_BROKERS").context("KAFKA_BROKERS must be set")
}

pub fn metrics_address() -> String {
    env::var("PROCESSOR_METRICS_ADDRESS").unwrap_or_else(|_| "0.0.0.0:9185".to_string())
}

pub fn catalog_consumer_group() -> String {
    env::var("CATALOG_CONSUMER_GROUP").unwrap_or_else(|_| "catalog-worker".to_string())
}

pub fn swap_normalizer_consumer_group() -> String {
    env::var("SWAP_NORMALIZER_CONSUMER_GROUP").unwrap_or_else(|_| "swap-normalizer".to_string())
}

pub fn timescale_url() -> Result<Url> {
    indexer_store::postgres_url::resolve_postgres_url("TIMESCALE", "TIMESCALE_URL")
}

pub fn redis_url() -> Result<String> {
    env::var("REDIS_URL").context("REDIS_URL must be set")
}

pub fn volume_engine_consumer_group() -> String {
    env::var("VOLUME_ENGINE_CONSUMER_GROUP").unwrap_or_else(|_| "volume-engine".to_string())
}

pub fn volume_metrics_address() -> String {
    env::var("VOLUME_METRICS_ADDRESS").unwrap_or_else(|_| "0.0.0.0:9186".to_string())
}

pub fn volume_rollup_refresh_secs() -> u64 {
    env::var("VOLUME_ROLLUP_REFRESH_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(60)
}

pub fn volume_engine_workers() -> usize {
    env::var("VOLUME_ENGINE_WORKERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|v| *v > 0)
        .unwrap_or(6)
}

pub fn swap_normalizer_workers() -> usize {
    env::var("SWAP_NORMALIZER_WORKERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|v| *v > 0)
        .unwrap_or(4)
}

pub fn clickhouse_url() -> String {
    env::var("CLICKHOUSE_URL").unwrap_or_else(|_| "http://localhost:8123".to_string())
}

pub fn clickhouse_database() -> String {
    env::var("CLICKHOUSE_DATABASE").unwrap_or_else(|_| "sui_metrics".to_string())
}

pub fn rolloff_metrics_address() -> String {
    env::var("ROLLOFF_METRICS_ADDRESS").unwrap_or_else(|_| "0.0.0.0:9189".to_string())
}

pub fn rolloff_interval_secs() -> u64 {
    env::var("ROLLOFF_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3600)
}

pub fn kafka_client_id(suffix: &str) -> String {
    env::var("KAFKA_CLIENT_ID")
        .map(|id| format!("{id}-{suffix}"))
        .unwrap_or_else(|_| format!("sui-processors-{suffix}"))
}

pub fn bootstrap_max_lookback_checkpoints() -> i64 {
    env::var("BOOTSTRAP_MAX_LOOKBACK_CHECKPOINTS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(200_000)
}

pub fn bootstrap_target_first_checkpoint() -> Option<i64> {
    env::var("FIRST_CHECKPOINT").ok().and_then(|v| v.parse().ok())
}

pub fn sui_archival_grpc_url() -> String {
    env::var("SUI_ARCHIVAL_GRPC_URL")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "https://archive.mainnet.sui.io:443".to_string())
}

pub fn remote_store_url() -> Option<String> {
    env::var("REMOTE_STORE_URL")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

pub fn bootstrap_trusted_pool_ids() -> Vec<String> {
    env::var("BOOTSTRAP_TRUSTED_POOL_IDS")
        .ok()
        .map(|raw| {
            raw.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

pub fn bootstrap_max_price_age_minutes() -> i64 {
    env::var("BOOTSTRAP_MAX_PRICE_AGE_MINUTES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(60)
}

pub fn bootstrap_min_buckets() -> i64 {
    env::var("BOOTSTRAP_MIN_BUCKETS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
}

pub fn oracle_bootstrap_gate_enabled() -> bool {
    env::var("ORACLE_BOOTSTRAP_GATE")
        .ok()
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes"))
        .unwrap_or(true)
}

pub fn oracle_bootstrap_gate_timeout_secs() -> u64 {
    env::var("ORACLE_BOOTSTRAP_GATE_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(300)
}

pub fn oracle_bootstrap_metrics_address() -> String {
    env::var("ORACLE_BOOTSTRAP_METRICS_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:9190".to_string())
}

pub fn oracle_bootstrap_metrics_hold_secs() -> u64 {
    env::var("ORACLE_BOOTSTRAP_METRICS_HOLD_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(30)
}

/// gRPC fullnode URL for swap-normalizer lazy hydration.
pub fn sui_grpc_url() -> Result<String> {
    if let Ok(url) = env::var("SUI_GRPC_URL") {
        let trimmed = url.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }
    env::var("STREAMING_URL")
        .context("SUI_GRPC_URL (or STREAMING_URL fallback) must be set for swap hydration")
}

pub fn swap_hydration_enabled() -> bool {
    env::var("SWAP_HYDRATION_ENABLED")
        .ok()
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes"))
        .unwrap_or(true)
}

pub fn swap_hydration_rpc_timeout_ms() -> u64 {
    env::var("SWAP_HYDRATION_RPC_TIMEOUT_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3000)
}

pub fn swap_hydration_pool_cache_size() -> usize {
    env::var("SWAP_HYDRATION_POOL_CACHE_SIZE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10_000)
}

pub fn swap_hydration_defer_max_retries() -> u32 {
    env::var("SWAP_HYDRATION_DEFER_MAX_RETRIES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5)
}

pub fn swap_hydration_defer_backoff_ms() -> u64 {
    env::var("SWAP_HYDRATION_DEFER_BACKOFF_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(500)
}
