use std::env;

use anyhow::{Context, Result};
use url::Url;

pub fn load_dotenv() {
    let _ = dotenvy::dotenv();
}

pub fn database_url() -> Result<Url> {
    indexer_store::postgres_url::resolve_postgres_url("POSTGRES", "DATABASE_URL")
}

pub fn timescale_url() -> Result<Url> {
    indexer_store::postgres_url::resolve_postgres_url("TIMESCALE", "TIMESCALE_URL")
}

pub fn redis_url() -> Result<String> {
    env::var("REDIS_URL").context("REDIS_URL must be set")
}

pub fn api_port() -> u16 {
    env::var("API_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8081)
}

pub fn api_metrics_address() -> String {
    env::var("API_METRICS_ADDRESS").unwrap_or_else(|_| "0.0.0.0:9188".to_string())
}

pub fn hot_storage_days() -> i64 {
    env::var("HOT_STORAGE_DAYS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(30)
}

pub fn api_cors_origins() -> Vec<String> {
    env::var("API_CORS_ORIGINS")
        .ok()
        .map(|v| {
            v.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_else(|| {
            vec![
                "http://localhost:5173".to_string(),
                "http://127.0.0.1:5173".to_string(),
                "http://localhost:5174".to_string(),
                "http://127.0.0.1:5174".to_string(),
            ]
        })
}
