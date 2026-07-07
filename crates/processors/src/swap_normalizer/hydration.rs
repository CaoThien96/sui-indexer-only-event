use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use lru::LruCache;
use tokio::sync::Mutex;
use tracing::warn;

use crate::metrics::ProcessorMetrics;
use crate::store::{CatalogStore, PoolRow, TokenRow};
use crate::sui_grpc::{GrpcError, GrpcFailureKind, SuiGrpcClient, classify_grpc_error};

use super::{DeferReason, HydrationConfig, SkipReason};

const WATCHLIST_SOURCE: &str = "swap_hydration";

pub enum PoolResolution {
    Found(PoolRow),
    MissingPermanent(SkipReason),
    Deferred(DeferReason),
}

pub(crate) enum DecimalsOutcome {
    Ready(u32, u32),
    Deferred(DeferReason),
}

enum TokenDecimals {
    Ready(u32),
    Deferred(DeferReason),
}

pub struct PoolHydrator {
    store: CatalogStore,
    metrics: Arc<ProcessorMetrics>,
    grpc: Arc<SuiGrpcClient>,
    config: HydrationConfig,
    pool_cache: Mutex<LruCache<String, PoolRow>>,
    pool_locks: Mutex<HashMap<String, Arc<Mutex<()>>>>,
}

impl PoolHydrator {
    pub fn new(
        store: CatalogStore,
        metrics: Arc<ProcessorMetrics>,
        grpc: Arc<SuiGrpcClient>,
        config: HydrationConfig,
        pool_cache: LruCache<String, PoolRow>,
    ) -> Self {
        Self {
            store,
            metrics,
            grpc,
            config,
            pool_cache: Mutex::new(pool_cache),
            pool_locks: Mutex::new(HashMap::new()),
        }
    }

    pub async fn resolve_pool(
        &self,
        pool_id: &str,
        protocol: &str,
        first_seen_cp: Option<i64>,
        first_seen_ms: Option<i64>,
    ) -> Result<PoolResolution> {
        if let Some(row) = self.pool_cache_get(pool_id).await {
            return Ok(PoolResolution::Found(row));
        }

        match self.store.get_pool(pool_id).await {
            Ok(Some(row)) => {
                self.pool_cache_put(pool_id, &row).await;
                return Ok(PoolResolution::Found(row));
            }
            Ok(None) => {}
            Err(e) => {
                warn!(pool_id, error = %e, "catalog get_pool failed");
                self.record_deferred(DeferReason::DbError);
                return Ok(PoolResolution::Deferred(DeferReason::DbError));
            }
        }

        if !self.config.enabled {
            return Ok(PoolResolution::MissingPermanent(SkipReason::HydrationDisabled));
        }

        self.hydrate_pool_single_flight(pool_id, protocol, first_seen_cp, first_seen_ms)
            .await
    }

    pub(crate) async fn resolve_pair_decimals(
        &self,
        coin_a: &str,
        coin_b: &str,
        first_seen_cp: Option<i64>,
    ) -> Result<DecimalsOutcome> {
        let (a, b) = tokio::join!(
            self.resolve_token_decimals(coin_a, first_seen_cp),
            self.resolve_token_decimals(coin_b, first_seen_cp),
        );

        match (a?, b?) {
            (TokenDecimals::Ready(da), TokenDecimals::Ready(db)) => {
                Ok(DecimalsOutcome::Ready(da, db))
            }
            (TokenDecimals::Deferred(r), _) | (_, TokenDecimals::Deferred(r)) => {
                Ok(DecimalsOutcome::Deferred(r))
            }
        }
    }

    async fn hydrate_pool_single_flight(
        &self,
        pool_id: &str,
        protocol: &str,
        first_seen_cp: Option<i64>,
        first_seen_ms: Option<i64>,
    ) -> Result<PoolResolution> {
        let lock = {
            let mut inflight = self.pool_locks.lock().await;
            inflight
                .entry(pool_id.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone()
        };
        let _guard = lock.lock().await;

        if let Some(row) = self.pool_cache_get(pool_id).await {
            return Ok(PoolResolution::Found(row));
        }
        if let Some(row) = self.store.get_pool(pool_id).await? {
            self.pool_cache_put(pool_id, &row).await;
            return Ok(PoolResolution::Found(row));
        }

        match self.grpc.get_pool_coin_types(pool_id).await {
            Ok((coin_a, coin_b)) => {
                if let Err(e) = self
                    .store
                    .upsert_pool_hydrated(
                        pool_id,
                        protocol,
                        &coin_a,
                        &coin_b,
                        first_seen_cp,
                        first_seen_ms,
                    )
                    .await
                {
                    warn!(pool_id, error = %e, "upsert_pool_hydrated failed");
                    self.record_deferred(DeferReason::DbError);
                    return Ok(PoolResolution::Deferred(DeferReason::DbError));
                }

                for coin in [&coin_a, &coin_b] {
                    if let Err(e) = self.store.seed_watchlist(coin, WATCHLIST_SOURCE).await {
                        warn!(coin_type = %coin, error = %e, "seed_watchlist failed");
                    }
                }

                self.metrics
                    .pool_hydrated
                    .with_label_values(&["ok"])
                    .inc();

                let row = self
                    .store
                    .get_pool(pool_id)
                    .await?
                    .context("pool missing after hydration upsert")?;
                self.pool_cache_put(pool_id, &row).await;
                Ok(PoolResolution::Found(row))
            }
            Err(e) => self.handle_pool_grpc_error(pool_id, &e),
        }
    }

    fn handle_pool_grpc_error(
        &self,
        pool_id: &str,
        err: &GrpcError,
    ) -> Result<PoolResolution> {
        match classify_grpc_error(err) {
            GrpcFailureKind::NotFound => {
                self.metrics
                    .pool_hydrated
                    .with_label_values(&["not_found"])
                    .inc();
                warn!(pool_id, error = %err, "pool hydration not found");
                Ok(PoolResolution::MissingPermanent(SkipReason::MissingPoolPermanent))
            }
            GrpcFailureKind::Transient => {
                self.metrics
                    .pool_hydrated
                    .with_label_values(&["error"])
                    .inc();
                self.record_deferred(DeferReason::PoolRpc);
                warn!(pool_id, error = %err, "pool hydration deferred");
                Ok(PoolResolution::Deferred(DeferReason::PoolRpc))
            }
            GrpcFailureKind::Permanent => {
                self.metrics
                    .pool_hydrated
                    .with_label_values(&["error"])
                    .inc();
                warn!(pool_id, error = %err, "pool hydration permanent failure");
                Ok(PoolResolution::MissingPermanent(SkipReason::MissingPoolPermanent))
            }
        }
    }

    async fn resolve_token_decimals(
        &self,
        coin_type: &str,
        first_seen_cp: Option<i64>,
    ) -> Result<TokenDecimals> {
        let normalized = crate::coin_type::normalize(coin_type);
        if normalized == crate::coin_type::SUI_COIN_TYPE {
            return Ok(TokenDecimals::Ready(9));
        }

        let existing = match self.store.get_token(&normalized).await {
            Ok(row) => row,
            Err(e) => {
                warn!(coin_type = %normalized, error = %e, "get_token failed");
                self.record_deferred(DeferReason::DbError);
                return Ok(TokenDecimals::Deferred(DeferReason::DbError));
            }
        };

        if let Some(ref token) = existing {
            if !is_token_stub(token) {
                return Ok(TokenDecimals::Ready(token.decimals as u32));
            }
        }

        match self.grpc.get_coin_metadata(&normalized).await {
            Ok(meta) => {
                if let Err(e) = self
                    .store
                    .upsert_token(
                        &normalized,
                        meta.name.as_deref(),
                        meta.symbol.as_deref(),
                        meta.decimals as i16,
                        None,
                        meta.image_url.as_deref(),
                        None,
                        None,
                        first_seen_cp,
                        "rpc_metadata",
                    )
                    .await
                {
                    warn!(coin_type = %normalized, error = %e, "upsert_token failed");
                    self.record_deferred(DeferReason::DbError);
                    return Ok(TokenDecimals::Deferred(DeferReason::DbError));
                }
                self.metrics
                    .token_metadata_hydrated
                    .with_label_values(&["ok"])
                    .inc();
                Ok(TokenDecimals::Ready(meta.decimals))
            }
            Err(e) => match classify_grpc_error(&e) {
                GrpcFailureKind::Transient => {
                    self.metrics
                        .token_metadata_hydrated
                        .with_label_values(&["error"])
                        .inc();
                    self.record_deferred(DeferReason::MetadataRpc);
                    warn!(coin_type = %normalized, error = %e, "metadata hydration deferred");
                    Ok(TokenDecimals::Deferred(DeferReason::MetadataRpc))
                }
                GrpcFailureKind::NotFound | GrpcFailureKind::Permanent => {
                    self.metrics
                        .token_metadata_hydrated
                        .with_label_values(&["error"])
                        .inc();
                    self.record_deferred(DeferReason::MetadataRpc);
                    warn!(coin_type = %normalized, error = %e, "metadata hydration failed");
                    Ok(TokenDecimals::Deferred(DeferReason::MetadataRpc))
                }
            },
        }
    }

    fn record_deferred(&self, reason: DeferReason) {
        self.metrics
            .swap_deferred
            .with_label_values(&[reason.as_str()])
            .inc();
    }

    async fn pool_cache_get(&self, pool_id: &str) -> Option<PoolRow> {
        let cache = self.pool_cache.lock().await;
        cache.peek(pool_id).cloned()
    }

    async fn pool_cache_put(&self, pool_id: &str, row: &PoolRow) {
        self.pool_cache
            .lock()
            .await
            .put(pool_id.to_string(), row.clone());
    }
}

fn is_token_stub(token: &TokenRow) -> bool {
    token.metadata_source == "stub" || token.name.is_none() || token.symbol.is_none()
}

pub fn defer_backoff(base_ms: u64, attempt: u32) -> Duration {
    let multiplier = 1u64.checked_shl(attempt.min(6)).unwrap_or(64);
    Duration::from_millis(base_ms.saturating_mul(multiplier))
}
