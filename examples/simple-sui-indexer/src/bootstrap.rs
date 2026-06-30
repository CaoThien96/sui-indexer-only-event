//! Bootstrap indexer + Prometheus metrics on a shared registry.
//!
//! Mirrors `IndexerCluster::build` but registers app metrics and uptime on the same
//! `/metrics` endpoint as framework built-ins (ingestion, pipeline, watermark, DB).

use std::sync::Arc;

use anyhow::{Context, Result};
use diesel_migrations::EmbeddedMigrations;
use prometheus::Registry;
use sui_indexer_alt_framework::{
    Indexer, cluster::Args, ingestion::IngestionConfig, postgres::Db, postgres::DbArgs,
    service::Error,
};
use sui_indexer_alt_metrics::{MetricsService, uptime};
use tracing::info;
use url::Url;

use crate::app_metrics::AppMetrics;

fn set_env_var(key: &str, value: &str) {
    // SAFETY: single-threaded process startup before tokio workers spawn.
    unsafe { std::env::set_var(key, value) };
}

fn load_env_file(path: &std::path::Path) -> Result<(), String> {
    let iter = dotenvy::from_path_iter(path).map_err(|err| err.to_string())?;
    for item in iter {
        match item {
            Ok((key, value)) => set_env_var(&key, &value),
            Err(err) => tracing::warn!(
                env_file = %path.display(),
                error = %err,
                "skipping invalid .env line"
            ),
        }
    }
    Ok(())
}

/// Load `.env` from the crate directory (works regardless of process CWD), then allow
/// a CWD `.env` to override for local experiments.
pub fn load_dotenv() -> DotenvLoadResult {
    let mut result = DotenvLoadResult::default();
    for path in dotenv_candidates() {
        if !path.is_file() {
            result.missing_paths.push(path);
            continue;
        }
        match load_env_file(&path) {
            Ok(()) => {
                result.loaded_from = Some(path);
                break;
            }
            Err(err) => result.errors.push((path, err)),
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        let cwd_env = cwd.join(".env");
        if cwd_env.is_file() {
            if let Err(err) = load_env_file(&cwd_env) {
                result.cwd_error = Some(err);
            }
        }
    }
    result
}

#[derive(Debug, Default)]
pub struct DotenvLoadResult {
    pub loaded_from: Option<std::path::PathBuf>,
    pub missing_paths: Vec<std::path::PathBuf>,
    pub errors: Vec<(std::path::PathBuf, String)>,
    pub cwd_error: Option<String>,
}

fn dotenv_candidates() -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
    paths.push(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(".env"),
    );
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            paths.push(dir.join(".env"));
            if let Some(parent) = dir.parent() {
                paths.push(parent.join(".env"));
            }
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        paths.push(cwd.join(".env"));
    }
    paths.sort();
    paths.dedup();
    paths
}

pub fn log_dotenv_load(result: &DotenvLoadResult) {
    if let Some(path) = &result.loaded_from {
        info!(env_file = %path.display(), "loaded .env");
        return;
    }
    for (path, err) in &result.errors {
        tracing::warn!(env_file = %path.display(), error = %err, ".env found but failed to parse");
    }
    if result.errors.is_empty() {
        let tried: Vec<_> = result
            .missing_paths
            .iter()
            .map(|p| p.display().to_string())
            .collect();
        tracing::warn!(
            tried_paths = ?tried,
            "no .env loaded; set env vars in the shell or place .env next to the binary"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_dotenv_sets_sell_buy_threshold() {
        let result = load_dotenv();
        let path = env!("CARGO_MANIFEST_DIR");
        let value = std::env::var("SELL_BUY_THRESHOLD").unwrap_or_default();
        assert!(
            result.loaded_from.is_some(),
            "expected .env load, result={result:?}"
        );
        assert_eq!(
            value, "100000000",
            "SELL_BUY_THRESHOLD from {path}/.env, got '{value}'"
        );
    }
}

pub fn init_tracing() {
    use tracing_subscriber::EnvFilter;

    let filter = if bot_log_only() {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(
                "simple_sui_indexer::bot=info,\
                 simple_sui_indexer::dex=info,\
                 simple_sui_indexer::handlers=error,\
                 simple_sui_indexer=warn,\
                 sui_indexer_alt_framework=off,\
                 sui_futures=warn,\
                 sui_pg_db=off,\
                 sui_indexer_alt_metrics=off",
            )
        })
    } else {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"))
    };

    tracing_subscriber::fmt().with_env_filter(filter).init();

    if bot_log_only() {
        tracing::warn!("BOT_LOG_ONLY enabled: showing bot snip logs only (set RUST_LOG to override)");
    }
}

fn bot_log_only() -> bool {
    std::env::var("BOT_LOG_ONLY")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub struct IndexerRuntime {
    pub indexer: Indexer<Db>,
    metrics: MetricsService,
    pub app_metrics: Arc<AppMetrics>,
}

impl IndexerRuntime {
    pub async fn build(
        database_url: Url,
        db_args: DbArgs,
        args: Args,
        ingestion_config: IngestionConfig,
        migrations: &'static EmbeddedMigrations,
        metrics_prefix: Option<String>,
    ) -> Result<Self> {
        let registry = Registry::new();
        let app_metrics = Arc::new(AppMetrics::register(&registry)?);
        registry
            .register(uptime(env!("CARGO_PKG_VERSION"))?)
            .context("failed to register uptime metric")?;

        let metrics = MetricsService::new(args.metrics_args.clone(), registry);

        let indexer = Indexer::new_from_pg(
            database_url,
            db_args,
            args.indexer_args,
            args.client_args,
            ingestion_config,
            Some(migrations),
            metrics_prefix.as_deref(),
            metrics.registry(),
        )
        .await?;

        Ok(Self {
            indexer,
            metrics,
            app_metrics,
        })
    }

    pub async fn run(self) -> Result<(), Error> {
        let s_indexer = self
            .indexer
            .run()
            .await
            .map_err(|_| Error::Aborted)?;
        let s_metrics = self
            .metrics
            .run()
            .await
            .map_err(|_| Error::Aborted)?;
        s_indexer.attach(s_metrics).main().await
    }
}

pub fn log_key_builtin_metrics(indexer: &Indexer<Db>, pipeline: &str) {
    let m = indexer.indexer_metrics();
    let ingestion = indexer.ingestion_metrics();

    info!(
        pipeline,
        ingested_checkpoints = ingestion.total_ingested_checkpoints.get(),
        latest_ingested_checkpoint = ingestion.latest_ingested_checkpoint.get(),
        processed_checkpoint = m
            .latest_processed_checkpoint
            .with_label_values(&[pipeline])
            .get(),
        processor_retries = m
            .total_handler_processor_retries
            .with_label_values(&[pipeline])
            .get(),
        watermark_checkpoint = m
            .watermark_checkpoint_in_db
            .with_label_values(&[pipeline])
            .get(),
        "Indexer metrics snapshot"
    );
}
