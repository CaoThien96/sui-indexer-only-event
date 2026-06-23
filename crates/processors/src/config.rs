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

pub fn ohlc_aggregator_consumer_group() -> String {
    env::var("OHLC_AGGREGATOR_CONSUMER_GROUP").unwrap_or_else(|_| "ohlc-aggregator".to_string())
}

pub fn volume_metrics_address() -> String {
    env::var("VOLUME_METRICS_ADDRESS").unwrap_or_else(|_| "0.0.0.0:9186".to_string())
}

pub fn ohlc_metrics_address() -> String {
    env::var("OHLC_METRICS_ADDRESS").unwrap_or_else(|_| "0.0.0.0:9187".to_string())
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
