use std::net::SocketAddr;

use anyhow::{Context, Result};
use clap::Parser;
use http::Uri;
use sui_indexer_alt_framework::{IndexerArgs, cluster::Args, ingestion::IngestionConfig};
use tracing::info;
use url::Url;

/// Merge environment variables into CLI args after `dotenvy` load.
pub fn apply_env_overrides(args: &mut Args) -> Result<()> {
    if let Ok(raw) = std::env::var("METRICS_ADDRESS") {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            args.metrics_args.metrics_address = trimmed
                .parse::<SocketAddr>()
                .with_context(|| format!("invalid METRICS_ADDRESS: {trimmed}"))?;
        }
    }

    if let Ok(raw) = std::env::var("REMOTE_STORE_URL") {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            args.client_args.ingestion.remote_store_url = Some(
                trimmed
                    .parse::<Url>()
                    .with_context(|| format!("invalid REMOTE_STORE_URL: {trimmed}"))?,
            );
        }
    }

    if let Ok(raw) = std::env::var("STREAMING_URL") {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            args.client_args.streaming.streaming_url = Some(
                trimmed
                    .parse::<Uri>()
                    .with_context(|| format!("invalid STREAMING_URL: {trimmed}"))?,
            );
        }
    }

    if let Ok(raw) = std::env::var("FIRST_CHECKPOINT") {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            args.indexer_args.first_checkpoint = Some(
                trimmed
                    .parse::<u64>()
                    .with_context(|| format!("invalid FIRST_CHECKPOINT: {trimmed}"))?,
            );
        }
    }

    Ok(())
}

pub fn metrics_prefix_from_env() -> Option<String> {
    std::env::var("METRICS_PREFIX")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn log_metrics_endpoint(args: &Args) {
    let addr = args.metrics_args.metrics_address;
    info!(
        metrics_address = %addr,
        metrics_url = format!("http://{addr}/metrics"),
        "Prometheus metrics endpoint"
    );
}

/// Default ingestion config; tune per docs/07-indexer-optimization-checklist.md (Week 7+).
pub fn ingestion_config() -> IngestionConfig {
    IngestionConfig::default()
}

#[derive(Parser, Debug)]
pub struct AppArgs {
    #[command(flatten)]
    pub framework: Args,
}

impl AppArgs {
    pub fn parse_with_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        let mut args = Args::parse();
        apply_env_overrides(&mut args)?;
        Ok(Self { framework: args })
    }
}

pub fn require_database_url() -> Result<Url> {
    std::env::var("DATABASE_URL")
        .context("DATABASE_URL must be set")
        .and_then(|raw| raw.parse::<Url>().context("invalid DATABASE_URL"))
}

pub fn require_kafka_brokers() -> Result<String> {
    std::env::var("KAFKA_BROKERS").context("KAFKA_BROKERS must be set")
}

pub fn kafka_client_id() -> String {
    std::env::var("KAFKA_CLIENT_ID").unwrap_or_else(|_| "sui-token-indexer".to_string())
}

pub fn log_indexer_args(indexer_args: &IndexerArgs) {
    info!(
        first_checkpoint = ?indexer_args.first_checkpoint,
        last_checkpoint = ?indexer_args.last_checkpoint,
        pipelines = ?indexer_args.pipeline,
        "Indexer arguments"
    );
}
