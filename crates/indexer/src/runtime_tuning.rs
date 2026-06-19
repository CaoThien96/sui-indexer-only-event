//! Runtime knobs per [docs/07-indexer-optimization-checklist.md].
//!
//! Override via environment variables; see `.env.example` for the full list.

use indexer_store::DbArgs;
use sui_indexer_alt_framework::{
    config::ConcurrencyConfig as IngestConcurrencyConfig,
    ingestion::IngestionConfig,
    pipeline::{CommitterConfig, sequential::SequentialConfig},
};
use tracing::info;

/// `steady` (default) — gRPC streaming at tip; `backfill` — HTTPS catch-up before/at tip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeMode {
    Steady,
    Backfill,
}

fn parse_runtime_mode() -> RuntimeMode {
    match std::env::var("INDEXER_RUNTIME_MODE")
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .as_deref()
    {
        Some("backfill") => RuntimeMode::Backfill,
        _ => RuntimeMode::Steady,
    }
}

fn env_usize(name: &str) -> Option<usize> {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse().ok())
}

fn env_u64(name: &str) -> Option<u64> {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse().ok())
}

fn env_u32(name: &str) -> Option<u32> {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse().ok())
}

/// Ingestion service config (broadcaster + remote store client).
pub fn ingestion_config() -> IngestionConfig {
    let mode = parse_runtime_mode();

    let ingest_concurrency = match env_usize("INGEST_CONCURRENCY") {
        Some(value) => IngestConcurrencyConfig::Fixed { value },
        None if mode == RuntimeMode::Backfill => IngestConcurrencyConfig::Fixed { value: 200 },
        None => IngestConcurrencyConfig::Adaptive {
            initial: 1,
            min: 1,
            max: env_usize("INGEST_CONCURRENCY_MAX").unwrap_or(500),
            dead_band: None,
        },
    };

    let config = IngestionConfig {
        ingest_concurrency,
        ..IngestionConfig::default()
    };

    info!(
        runtime_mode = ?mode,
        ingest_concurrency = ?config.ingest_concurrency,
        "Ingestion runtime tuning"
    );

    config
}

/// Sequential pipeline config shared by dex_swap, dex_pool, token_metadata.
pub fn sequential_config() -> SequentialConfig {
    let mode = parse_runtime_mode();

    let default_collect_ms = match mode {
        RuntimeMode::Backfill => 750,
        RuntimeMode::Steady => 300,
    };

    let collect_interval_ms = env_u64("COLLECT_INTERVAL_MS").unwrap_or(default_collect_ms);

    let subscriber_channel_size = env_usize("SUBSCRIBER_CHANNEL_SIZE");
    let pipeline_depth = env_usize("PIPELINE_DEPTH").or(Some(2));
    let processor_channel_size = env_usize("PROCESSOR_CHANNEL_SIZE");

    let fanout = env_usize("PROCESSOR_FANOUT_MAX").map(|max| IngestConcurrencyConfig::Adaptive {
        initial: 2,
        min: 1,
        max,
        dead_band: None,
    });

    let config = SequentialConfig {
        committer: CommitterConfig {
            collect_interval_ms,
            ..Default::default()
        },
        ingestion: sui_indexer_alt_framework::pipeline::IngestionConfig {
            subscriber_channel_size,
        },
        fanout,
        pipeline_depth,
        processor_channel_size,
        ..Default::default()
    };

    info!(
        runtime_mode = ?mode,
        collect_interval_ms,
        subscriber_channel_size = ?subscriber_channel_size,
        pipeline_depth = ?pipeline_depth,
        processor_fanout_max = ?env_usize("PROCESSOR_FANOUT_MAX"),
        "Sequential pipeline runtime tuning"
    );

    config
}

/// Postgres pool for watermark operations (one connection per concurrent committer write).
pub fn db_args() -> DbArgs {
    let pipeline_count = 3u32;
    let default_pool = pipeline_count.saturating_mul(5).max(10);

    let db_connection_pool_size =
        env_u32("DB_CONNECTION_POOL_SIZE").unwrap_or(default_pool);

    let db_connection_timeout_ms =
        env_u64("DB_CONNECTION_TIMEOUT_MS").unwrap_or(30_000);

    let args = DbArgs {
        db_connection_pool_size,
        db_connection_timeout_ms,
    };

    info!(
        db_connection_pool_size = args.db_connection_pool_size,
        db_connection_timeout_ms = args.db_connection_timeout_ms,
        "Postgres watermark pool tuning"
    );

    args
}
