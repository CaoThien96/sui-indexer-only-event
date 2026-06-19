use std::net::SocketAddr;

use anyhow::{Context, Result, bail};
use clap::Parser;
use http::Uri;
use sui_indexer_alt_framework::{IndexerArgs, cluster::Args};
use tracing::info;
use url::Url;

fn argv_has_flag(argv: &[String], flag: &str) -> bool {
    argv.iter().any(|arg| arg == flag || arg.starts_with(&format!("{flag}=")))
}

/// Framework `remote_store_url` has no `env =` attribute; inject `.env` values before clap parse.
fn inject_env_cli_args(argv: &mut Vec<String>) -> Result<()> {
    if !argv_has_flag(argv, "--remote-store-url") {
        if let Ok(raw) = std::env::var("REMOTE_STORE_URL") {
            let trimmed = raw.trim();
            if !trimmed.is_empty() {
                argv.push("--remote-store-url".to_string());
                argv.push(trimmed.to_string());
            }
        }
    }

    if !argv_has_flag(argv, "--streaming-url") {
        if let Ok(raw) = std::env::var("STREAMING_URL") {
            let trimmed = raw.trim();
            if !trimmed.is_empty() {
                argv.push("--streaming-url".to_string());
                argv.push(trimmed.to_string());
            }
        }
    }

    if !argv_has_flag(argv, "--first-checkpoint") {
        if let Ok(raw) = std::env::var("FIRST_CHECKPOINT") {
            let trimmed = raw.trim();
            if !trimmed.is_empty() {
                argv.push("--first-checkpoint".to_string());
                argv.push(trimmed.to_string());
            }
        }
    }

    if !argv_has_flag(argv, "--metrics-address") {
        if let Ok(raw) = std::env::var("METRICS_ADDRESS") {
            let trimmed = raw.trim();
            if !trimmed.is_empty() {
                argv.push("--metrics-address".to_string());
                argv.push(trimmed.to_string());
            }
        }
    }

    Ok(())
}

/// Merge environment variables into CLI args after `dotenvy` load (non-clap fields / overrides).
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

#[derive(Parser, Debug)]
pub struct AppArgs {
    #[command(flatten)]
    pub framework: Args,
}

impl AppArgs {
    pub fn parse_with_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        let mut argv: Vec<String> = std::env::args().collect();
        inject_env_cli_args(&mut argv)?;
        let mut args = Args::parse_from(argv);
        apply_env_overrides(&mut args)?;

        if args.client_args.ingestion.remote_store_url.is_none()
            && args.client_args.ingestion.remote_store_s3.is_none()
            && args.client_args.ingestion.remote_store_gcs.is_none()
            && args.client_args.ingestion.remote_store_azure.is_none()
            && args.client_args.ingestion.local_ingestion_path.is_none()
            && args.client_args.ingestion.rpc_api_url.is_none()
        {
            bail!(
                "checkpoint source required: set REMOTE_STORE_URL in .env or pass --remote-store-url"
            );
        }

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
