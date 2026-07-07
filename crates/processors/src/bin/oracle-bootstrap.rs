use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use chrono::{DateTime, Utc};
use prometheus::Registry;
use rust_decimal::Decimal;
use tracing::{info, warn};

use sui_processors::config::{
    self, bootstrap_max_lookback_checkpoints, bootstrap_max_price_age_minutes,
    bootstrap_min_buckets, bootstrap_target_first_checkpoint, bootstrap_trusted_pool_ids,
    oracle_bootstrap_metrics_address, oracle_bootstrap_metrics_hold_secs, timescale_url,
};
use sui_processors::metrics::OracleBootstrapMetrics;
use sui_processors::oracle::{
    CheckpointFetcher, FetchError, MinuteAccumulator, classify_swap, evaluate_readiness,
    extract_sui_usdc_observation, flush_sui_buckets, is_sui_usdc_pool, iterate_checkpoint_events,
    minute_bucket, normalize_pool_id, trusted_pool_set,
};
use sui_processors::runtime::serve_metrics;
use sui_processors::sui_grpc::SuiGrpcClient;
use sui_processors::timescale::TimescaleStore;

const BOOTSTRAP_RUN_ID: &str = "sui_usd_bootstrap";

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    config::load_dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let registry = Registry::new();
    let metrics = OracleBootstrapMetrics::new(&registry)?;
    let metrics_addr = oracle_bootstrap_metrics_address();
    let hold_secs = oracle_bootstrap_metrics_hold_secs();

    let metrics_registry = registry.clone();
    let metrics_bind = metrics_addr.clone();
    let metrics_server = tokio::spawn(async move { serve_metrics(metrics_registry, &metrics_bind).await });

    let result = run_bootstrap(&metrics).await;
    match &result {
        Ok(ready) => metrics.last_run_success.set(if *ready { 1 } else { 0 }),
        Err(_) => metrics.last_run_success.set(0),
    }

    if hold_secs > 0 {
        info!(
            hold_secs,
            metrics_addr = %metrics_addr,
            "holding metrics endpoint open for Prometheus scrape"
        );
        tokio::time::sleep(Duration::from_secs(hold_secs)).await;
    }

    metrics_server.abort();
    result.map(|_| ())
}

async fn run_bootstrap(metrics: &Arc<OracleBootstrapMetrics>) -> Result<bool> {
    let first_checkpoint = bootstrap_target_first_checkpoint()
        .context("FIRST_CHECKPOINT must be set for oracle-bootstrap")?;
    let trusted_ids = bootstrap_trusted_pool_ids();
    if trusted_ids.is_empty() {
        bail!("BOOTSTRAP_TRUSTED_POOL_IDS must list at least one SUI/USDC pool");
    }
    let trusted = trusted_pool_set(&trusted_ids);

    let max_lookback = bootstrap_max_lookback_checkpoints();
    let max_price_age = bootstrap_max_price_age_minutes();
    let min_buckets = bootstrap_min_buckets();

    let store = TimescaleStore::connect(timescale_url()?).await?;
    store.run_migrations().await?;

    let fetcher = CheckpointFetcher::from_env()?;
    let grpc = Arc::new(
        SuiGrpcClient::new(&config::sui_archival_grpc_url(), Duration::from_secs(30))
            .context("archival gRPC client")?,
    );

    let mut pool_coin_types: HashMap<String, (String, String)> = HashMap::new();
    for pool_id in &trusted_ids {
        let key = normalize_pool_id(pool_id);
        let (a, b) = grpc
            .get_pool_coin_types(pool_id)
            .await
            .with_context(|| format!("hydrate trusted pool {pool_id}"))?;
        if !is_sui_usdc_pool(&a, &b) {
            bail!("trusted pool {pool_id} is not SUI/USDC (got {a}, {b})");
        }
        pool_coin_types.insert(key, (a, b));
    }

    let boundary_cp = first_checkpoint.saturating_sub(1).max(0) as u64;
    let boundary_checkpoint = fetcher
        .get_checkpoint(boundary_cp)
        .await
        .map_err(|e| anyhow::anyhow!("fetch boundary checkpoint {boundary_cp}: {e}"))?;
    let boundary_time = minute_bucket(boundary_checkpoint.summary.timestamp_ms)?;

    let start_cp = boundary_cp.saturating_sub(max_lookback as u64);
    let mut buckets: HashMap<DateTime<Utc>, MinuteAccumulator> = HashMap::new();
    let mut ready = false;

    for cp in (start_cp..=boundary_cp).rev() {
        let checkpoint = match fetcher.get_checkpoint(cp).await {
            Ok(cp) => cp,
            Err(FetchError::NotFound) => {
                warn!(checkpoint = cp, "checkpoint not found, skipping");
                continue;
            }
            Err(e) => return Err(e.into()),
        };
        metrics.checkpoints_scanned.inc();

        for event in iterate_checkpoint_events(&checkpoint) {
            let Some(protocol) = classify_swap(&event.event_type) else {
                continue;
            };

            for (pool_key, (coin_a, coin_b)) in &pool_coin_types {
                let Ok(Some(obs)) =
                    extract_sui_usdc_observation(&event, protocol, coin_a, coin_b)
                else {
                    continue;
                };
                if normalize_pool_id(&obs.pool_id) != *pool_key || !trusted.contains(pool_key) {
                    continue;
                }
                metrics.swaps_matched.inc();
                let entry = buckets.entry(obs.bucket).or_default();
                entry.merge(obs.sui_amount, obs.usdc_amount, obs.checkpoint_seq, &obs.pool_id);
            }
        }

        let readiness = evaluate_readiness(&buckets, boundary_time, min_buckets, max_price_age);
        if readiness.ready {
            info!(
                checkpoint = cp,
                buckets = readiness.bucket_count,
                staleness_minutes = ?readiness.staleness_minutes,
                "bootstrap readiness satisfied"
            );
            ready = true;
            break;
        }
    }

    let flush_rows: Vec<(DateTime<Utc>, Decimal, String)> = buckets
        .iter()
        .filter_map(|(bucket, acc)| {
            acc.price_usd()
                .map(|price| (*bucket, price, acc.pool_id.clone()))
        })
        .collect();
    let buckets_seeded = flush_sui_buckets(&store, &flush_rows).await?;
    metrics.buckets_seeded.inc_by(buckets_seeded as u64);

    let readiness = evaluate_readiness(&buckets, boundary_time, min_buckets, max_price_age);
    let status = if ready && readiness.ready {
        "READY"
    } else {
        "FAILED"
    };

    let anchor_pool = buckets
        .values()
        .max_by_key(|b| b.checkpoint_seq)
        .map(|b| b.pool_id.clone());

    let checkpoints_scanned = metrics.checkpoints_scanned.get();
    let swaps_matched = metrics.swaps_matched.get();

    store
        .upsert_bootstrap_state(
            BOOTSTRAP_RUN_ID,
            first_checkpoint,
            status,
            1,
            start_cp as i64,
            boundary_cp as i64,
            serde_json::json!({
                "anchor_checkpoint": boundary_cp,
                "anchor_time": boundary_time.to_rfc3339(),
                "trusted_pool_id": anchor_pool,
                "buckets_seeded": buckets_seeded,
                "max_staleness_minutes": readiness.staleness_minutes,
                "checkpoints_scanned": checkpoints_scanned,
                "swaps_matched": swaps_matched,
            }),
        )
        .await?;

    info!(
        status,
        checkpoints_scanned,
        swaps_matched,
        buckets_seeded,
        "oracle-bootstrap finished"
    );

    if status != "READY" {
        bail!("oracle bootstrap failed readiness checks");
    }

    Ok(true)
}
