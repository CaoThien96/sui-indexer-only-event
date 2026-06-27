use anyhow::{Context, Result};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{debug, info};

use crate::bot::cleanup::spawn_processed_swaps_cleanup;
use crate::bot::config::{BotConfig, BotRuntime};
use crate::bot::event_types;
use crate::bot::parsers::{parse_create_pool, parse_swap};
use crate::bot::sell::process_swap_old_token;
use crate::bot::snip::schedule_snip;
use crate::bot::state::{BotStateStore, Dex};
use crate::telegram_notify;

#[derive(Clone, Debug)]
pub struct BotEventContext {
    pub event_type: String,
    pub tx_digest: String,
    pub event_seq: usize,
    pub sender: String,
    pub parsed_json: Value,
}

impl BotEventContext {
    pub fn event_seq_str(&self) -> String {
        self.event_seq.to_string()
    }
}

pub struct BotReactor {
    runtime: Arc<BotRuntime>,
    store: Arc<BotStateStore>,
    concurrency: Arc<Semaphore>,
}

impl BotReactor {
    pub fn new(runtime: Arc<BotRuntime>, store: Arc<BotStateStore>) -> Arc<Self> {
        let max_concurrent = std::env::var("BOT_MAX_CONCURRENT_EVENTS")
            .ok()
            .and_then(|v| v.parse().ok())
            .filter(|v| *v > 0)
            .unwrap_or(8);
        Arc::new(Self {
            runtime,
            store,
            concurrency: Arc::new(Semaphore::new(max_concurrent)),
        })
    }

    pub async fn handle(self: Arc<Self>, ctx: BotEventContext) -> Result<()> {
        let _permit = self
            .concurrency
            .acquire()
            .await
            .context("bot concurrency permit")?;
        if self
            .store
            .event_exists(&ctx.tx_digest, &ctx.event_seq_str())
            .await?
        {
            return Ok(());
        }

        let dex = match BotRuntime::dex_from_event_type(&ctx.event_type) {
            Some(d) => d,
            None => return Ok(()),
        };

        let result = match ctx.event_type.as_str() {
            t if t == event_types::cetus_swap() || t == event_types::turbos_swap() => {
                self.handle_swap(dex, &ctx).await
            }
            t if t == event_types::cetus_create_pool() || t == event_types::turbos_create_pool() => {
                self.handle_create_pool(dex, &ctx).await
            }
            t if t == event_types::cetus_remove_liquidity()
                || t == event_types::turbos_remove_liquidity() =>
            {
                self.handle_remove_liquidity(dex, &ctx).await
            }
            _ => Ok(()),
        };

        if result.is_ok() {
            self.store
                .insert_processed_event(
                    &ctx.tx_digest,
                    &ctx.event_seq_str(),
                    &ctx.event_type,
                )
                .await?;
        }

        result
    }

    async fn handle_swap(&self, dex: Dex, ctx: &BotEventContext) -> Result<()> {
        let Some(swap) = parse_swap(
            dex,
            &ctx.parsed_json,
            &ctx.tx_digest,
            &ctx.event_seq_str(),
            &ctx.sender,
        )? else {
            return Ok(());
        };
        process_swap_old_token(Arc::clone(&self.runtime), &self.store, swap).await
    }

    async fn handle_create_pool(&self, dex: Dex, ctx: &BotEventContext) -> Result<()> {
        let config = &self.runtime.config;
        let Some(event) = parse_create_pool(&self.runtime.rpc, dex, &ctx.parsed_json).await?
        else {
            return Ok(());
        };

        if event.reserve <= config.min_pool_reserve_sui || config.is_blacklisted(&event.token) {
            debug!(
                token = %event.token,
                reserve = event.reserve,
                "skip create pool"
            );
            return Ok(());
        }

        if self.store.token_exists(&event.token).await? {
            return Ok(());
        }

        self.store
            .upsert_token_listing(
                &event.token,
                &event.symbol,
                &self.runtime.vault.address_string(),
                &event.pool,
                dex,
                &ctx.tx_digest,
            )
            .await?;

        info!(token = %event.token, pool = %event.pool, dex = ?dex, "detected new pool");
        telegram_notify::send_message(&format!(
            "🚀 Detect {} on {:?} added pool {}",
            event.symbol, dex, event.pool
        ))
        .await;

        schedule_snip(
            Arc::clone(&self.runtime),
            Arc::clone(&self.store),
            dex,
            event.token,
            event.pool,
            event.symbol,
        );

        Ok(())
    }

    async fn handle_remove_liquidity(&self, dex: Dex, ctx: &BotEventContext) -> Result<()> {
        let config = &self.runtime.config;
        let Some(pool_id) = ctx.parsed_json.get("pool").and_then(|v| v.as_str()) else {
            return Ok(());
        };

        // Only pools we track (same as bot-snip getTokenByPool).
        let Some((_, token)) = self.store.get_pool_with_token(pool_id).await? else {
            return Ok(());
        };

        if config.is_blacklisted(&token.id) {
            return Ok(());
        }

        let reserve = self.runtime.rpc.get_pool_coin_b(pool_id).await?;
        if reserve <= config.remove_reserve_threshold {
            self.store.mark_token_done(pool_id).await?;
            info!(symbol = %token.symbol, pool = %pool_id, "token marked done");
            telegram_notify::send_message(&format!(
                "🗑 Detect {} on {:?} removed pool {}",
                token.symbol, dex, pool_id
            ))
            .await;
        } else {
            debug!(
                symbol = %token.symbol,
                reserve = reserve,
                "remove liquidity but reserve still high"
            );
        }

        Ok(())
    }
}

pub async fn try_init_reactor(database_url: &str) -> Result<Option<Arc<BotReactor>>> {
    let config = BotConfig::from_env()?;
    if !config.enabled {
        return Ok(None);
    }
    info!(
        sell_buy_threshold = config.sell_buy_threshold,
        sell_buy_threshold_env = std::env::var("SELL_BUY_THRESHOLD")
            .ok()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| "<unset>".into()),
        "bot SELL_BUY_THRESHOLD loaded"
    );
    let runtime = BotRuntime::try_from_env()
        .await?
        .context("BOT_ENABLED but runtime init failed")?;
    let store = Arc::new(BotStateStore::connect(database_url).await?);
    spawn_processed_swaps_cleanup(
        Arc::clone(&store),
        runtime.config.processed_swaps_ttl_days,
        runtime.config.cleanup_interval_secs,
    );
    Ok(Some(BotReactor::new(runtime, store)))
}
