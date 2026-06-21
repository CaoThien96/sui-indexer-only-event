mod liquidity;
mod rollup;

pub use liquidity::estimate_tvl_quote;
pub use rollup::sum_quote_volume;

use std::sync::Arc;

use anyhow::Result;
use indexer_store::MessageEnvelope;
use tracing::warn;

use crate::metrics::MetricsBundle;
use crate::normalized_swap::{NormalizedSwap, parse_normalized_swap};
use crate::redis_cache::RedisCache;
use crate::timescale::TimescaleStore;

pub struct VolumeEngine {
    store: TimescaleStore,
    redis: RedisCache,
    metrics: Arc<MetricsBundle>,
}

impl VolumeEngine {
    pub fn new(store: TimescaleStore, redis: RedisCache, metrics: Arc<MetricsBundle>) -> Self {
        Self {
            store,
            redis,
            metrics,
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

        let inserted = self.store.insert_swap_fact(&swap).await?;
        if inserted {
            self.metrics.swaps_fact_inserted.inc();
            self.update_redis(&swap).await?;
        }

        if let (Some(va), Some(vb)) = (&swap.vault_a_raw, &swap.vault_b_raw) {
            let tvl = estimate_tvl_quote(&swap);
            if self
                .store
                .insert_pool_liquidity(&swap, va, vb, tvl)
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

    async fn update_redis(&self, swap: &NormalizedSwap) -> Result<()> {
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

        let (volume, tx_count) = self.store.sum_token_volume_24h(&swap.base_coin_type).await?;
        self.redis
            .set_token_vol_24h(&swap.base_coin_type, &volume.to_string(), tx_count)
            .await?;
        self.metrics
            .redis_writes
            .with_label_values(&["token_vol_24h"])
            .inc();

        Ok(())
    }
}
