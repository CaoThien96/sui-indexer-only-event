use std::env;

use anyhow::{Context, Result};
use url::Url;

pub fn load_dotenv() {
    let _ = dotenvy::dotenv();
}

pub fn database_url() -> Result<Url> {
    let raw = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    raw.parse::<Url>().context("invalid DATABASE_URL")
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
    let raw = env::var("TIMESCALE_URL").context("TIMESCALE_URL must be set")?;
    raw.parse::<Url>().context("invalid TIMESCALE_URL")
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

pub fn kafka_client_id(suffix: &str) -> String {
    env::var("KAFKA_CLIENT_ID")
        .map(|id| format!("{id}-{suffix}"))
        .unwrap_or_else(|_| format!("sui-processors-{suffix}"))
}
