mod liquidity;
mod rollup;

pub use liquidity::estimate_tvl_quote;
pub use rollup::sum_quote_volume;

use std::collections::HashSet;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::sync::Mutex;

use anyhow::Result;
use indexer_store::MessageEnvelope;
use rust_decimal::Decimal;
use tracing::warn;

use crate::metrics::MetricsBundle;
use crate::normalized_swap::{NormalizedSwap, parse_normalized_swap};
use crate::oracle::record_swap_prices;
use crate::redis_cache::RedisCache;
use crate::timescale::TimescaleStore;

pub struct VolumeEngine {
    store: TimescaleStore,
    redis: RedisCache,
    metrics: Arc<MetricsBundle>,
    trusted_pools: HashSet<String>,
    rollup_refresh_interval: Duration,
    last_rollup_refresh: Mutex<HashMap<String, Instant>>,
}

impl VolumeEngine {
    pub fn new(
        store: TimescaleStore,
        redis: RedisCache,
        metrics: Arc<MetricsBundle>,
        trusted_pools: HashSet<String>,
        rollup_refresh_interval: Duration,
    ) -> Self {
        Self {
            store,
            redis,
            metrics,
            trusted_pools,
            rollup_refresh_interval,
            last_rollup_refresh: Mutex::new(HashMap::new()),
        }
    }

    pub async fn handle(&self, envelope: &MessageEnvelope) -> Result<()> {
        let swap = match parse_normalized_swap(envelope) {
            Ok(s) => s,
            Err(e) => {
                self.metrics
                    .volume_skipped
                    .with_label_values(&["parse_error"])
                    .inc();
                warn!(error = %e, "Failed to parse normalized swap for volume");
                return Ok(());
            }
        };

        if let Err(e) = record_swap_prices(&self.store, &swap, &self.trusted_pools).await {
            warn!(error = %e, swap_key = %swap.swap_key, "failed to record oracle prices");
        } else {
            self.metrics.token_usd_1m_upserts.inc();
        }

        let inserted = self.store.insert_swap_fact(&swap).await?;
        if inserted {
            self.metrics.swaps_fact_inserted.inc();
            self.update_redis_fast(&swap).await?;
            if self.should_refresh_rollup(&swap.base_coin_type) {
                self.update_redis_rollups(&swap).await?;
            }
            if swap.price_usd_per_base.is_some() && swap.amount_usd.is_some() {
                if let Err(e) = self.store.upsert_token_ohlc_usd_all_intervals(&swap).await {
                    warn!(error = %e, swap_key = %swap.swap_key, "failed to upsert token OHLC USD");
                } else {
                    for interval in crate::token_ohlc::OHLC_INTERVALS {
                        self.metrics
                            .token_ohlc_usd_upserts
                            .with_label_values(&[interval])
                            .inc();
                    }
                }
            }
        }

        if let (Some(va), Some(vb)) = (&swap.vault_a_raw, &swap.vault_b_raw) {
            let tvl = estimate_tvl_quote(&swap);
            let tvl_usd = tvl.and_then(|v| self.quote_usd_rate(&swap).map(|rate| (v * rate).normalize()));
            if self
                .store
                .insert_pool_liquidity(&swap, va, vb, tvl, tvl_usd)
                .await?
            {
                self.metrics.pool_liquidity_inserted.inc();
                if let Some(tvl) = tvl {
                    self.redis
                        .set_pool_tvl(&swap.pool_id, &tvl.to_string())
                        .await?;
                    self.metrics
                        .redis_writes
                        .with_label_values(&["pool_tvl"])
                        .inc();
                }
            }
        }
        Ok(())
    }

    async fn update_redis_fast(&self, swap: &NormalizedSwap) -> Result<()> {
        self.redis
            .set_token_price(
                &swap.base_coin_type,
                &swap.price_quote_per_base.to_string(),
                &swap.pool_id,
                &swap.quote_coin_type,
            )
            .await?;
        self.metrics
            .redis_writes
            .with_label_values(&["token_price"])
            .inc();

        if let Some(price_usd) = swap.price_usd_per_base {
            self.redis
                .set_token_price_usd(&swap.base_coin_type, &price_usd.to_string(), Some(&swap.pool_id))
                .await?;
            self.metrics
                .redis_writes
                .with_label_values(&["token_price_usd"])
                .inc();
        }

        Ok(())
    }

    async fn update_redis_rollups(&self, swap: &NormalizedSwap) -> Result<()> {

        let (volume, tx_count) = self.store.sum_token_volume_24h(&swap.base_coin_type).await?;
        self.redis
            .set_token_vol_24h(&swap.base_coin_type, &volume.to_string(), tx_count)
            .await?;
        self.metrics
            .redis_writes
            .with_label_values(&["token_vol_24h"])
            .inc();

        let (volume_usd, tx_count) = self.store.sum_token_volume_24h_usd(&swap.base_coin_type).await?;
        self.redis
            .set_token_vol_24h_usd(&swap.base_coin_type, &volume_usd.to_string(), tx_count)
            .await?;
        self.metrics
            .redis_writes
            .with_label_values(&["token_vol_24h_usd"])
            .inc();

        Ok(())
    }

    fn should_refresh_rollup(&self, coin_type: &str) -> bool {
        let mut guard = match self.last_rollup_refresh.lock() {
            Ok(g) => g,
            Err(_) => return true,
        };
        let now = Instant::now();
        if let Some(last) = guard.get(coin_type)
            && now.duration_since(*last) < self.rollup_refresh_interval
        {
            return false;
        }
        guard.insert(coin_type.to_string(), now);
        true
    }

    fn quote_usd_rate(&self, swap: &NormalizedSwap) -> Option<Decimal> {
        match (swap.amount_usd, swap.amount_quote) {
            (Some(usd), quote) if !quote.is_zero() => Some((usd / quote).normalize()),
            _ => None,
        }
    }
}
