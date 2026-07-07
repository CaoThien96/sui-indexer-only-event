use std::sync::Arc;

use anyhow::{Context, Result};
use redis::AsyncCommands;
use serde::Deserialize;
use sui_processors::store::{CatalogStore, PoolRow, TokenRow};
use sui_processors::timescale::TimescaleStore;

use crate::config;
use crate::metrics::ApiMetrics;

#[derive(Debug, Deserialize)]
struct PriceCache {
    price: String,
    quote_coin_type: String,
}

#[derive(Debug, Deserialize)]
struct VolCache {
    volume: String,
    tx_count: i64,
}

#[derive(Debug, Deserialize)]
struct PriceUsdCache {
    price_usd: String,
    source_type: Option<String>,
    confidence_score: Option<String>,
    is_stale: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct VolUsdCache {
    volume_usd: String,
    tx_count: i64,
}

#[derive(Clone)]
pub struct AppState {
    pub catalog: CatalogStore,
    pub timescale: TimescaleStore,
    pub clickhouse: clickhouse::Client,
    pub redis: redis::Client,
    pub metrics: Arc<ApiMetrics>,
    pub hot_storage_days: i64,
}

impl AppState {
    pub async fn new(metrics: Arc<ApiMetrics>) -> Result<Self> {
        let catalog = CatalogStore::connect(config::database_url()?).await?;
        catalog.run_migrations().await?;
        let timescale = TimescaleStore::connect(config::timescale_url()?).await?;
        timescale.run_migrations().await?;
        let clickhouse = crate::query_router::init_clickhouse().await?;
        let redis =
            redis::Client::open(config::redis_url()?).context("invalid REDIS_URL")?;

        Ok(Self {
            catalog,
            timescale,
            clickhouse,
            redis,
            metrics,
            hot_storage_days: config::hot_storage_days(),
        })
    }

    async fn redis_conn(&self) -> Result<redis::aio::MultiplexedConnection> {
        self.redis
            .get_multiplexed_async_connection()
            .await
            .context("redis connection failed")
    }

    pub async fn get_token(&self, coin_type: &str) -> Result<Option<TokenRow>> {
        self.catalog.get_token(coin_type).await
    }

    pub async fn count_pools(&self, coin_type: &str) -> Result<i64> {
        self.catalog.count_pools_for_token(coin_type).await
    }

    pub async fn list_pools(&self, coin_type: &str, limit: i64) -> Result<Vec<PoolRow>> {
        self.catalog.list_pools_for_token(coin_type, limit).await
    }

    pub async fn list_tokens(
        &self,
        q: Option<&str>,
        limit: i64,
        cursor: Option<&str>,
    ) -> Result<Vec<sui_processors::store::TokenListRow>> {
        self.catalog.list_tokens(q, limit, cursor).await
    }

    pub async fn redis_price(
        &self,
        coin_type: &str,
    ) -> Result<Option<(String, String)>> {
        let key = format!("token:{coin_type}:price");
        let mut conn = self.redis_conn().await?;
        let raw: Option<String> = conn.get(&key).await?;
        let Some(raw) = raw else {
            return Ok(None);
        };
        let parsed: PriceCache = serde_json::from_str(&raw)?;
        self.metrics.cache_hits.with_label_values(&["redis"]).inc();
        Ok(Some((parsed.price, parsed.quote_coin_type)))
    }

    pub async fn redis_vol(&self, coin_type: &str) -> Result<Option<(String, i64)>> {
        let key = format!("token:{coin_type}:vol:24h");
        let mut conn = self.redis_conn().await?;
        let raw: Option<String> = conn.get(&key).await?;
        let Some(raw) = raw else {
            return Ok(None);
        };
        let parsed: VolCache = serde_json::from_str(&raw)?;
        self.metrics.cache_hits.with_label_values(&["redis"]).inc();
        Ok(Some((parsed.volume, parsed.tx_count)))
    }

    pub async fn redis_pool_tvl(&self, pool_id: &str) -> Result<Option<String>> {
        let key = format!("pool:{pool_id}:tvl");
        let mut conn = self.redis_conn().await?;
        let raw: Option<String> = conn.get(&key).await?;
        let Some(raw) = raw else {
            return Ok(None);
        };
        #[derive(Deserialize)]
        struct TvlCache {
            tvl_quote: String,
        }
        let parsed: TvlCache = serde_json::from_str(&raw)?;
        Ok(Some(parsed.tvl_quote))
    }

    pub async fn redis_price_usd(
        &self,
        coin_type: &str,
    ) -> Result<Option<(String, Option<String>, Option<String>, bool)>> {
        let key = format!("token:{coin_type}:price:usd");
        let mut conn = self.redis_conn().await?;
        let raw: Option<String> = conn.get(&key).await?;
        let Some(raw) = raw else {
            return Ok(None);
        };
        let parsed: PriceUsdCache = serde_json::from_str(&raw)?;
        self.metrics.cache_hits.with_label_values(&["redis"]).inc();
        Ok(Some((
            parsed.price_usd,
            parsed.source_type,
            parsed.confidence_score,
            parsed.is_stale.unwrap_or(false),
        )))
    }

    pub async fn redis_vol_usd(&self, coin_type: &str) -> Result<Option<(String, i64)>> {
        let key = format!("token:{coin_type}:vol:24h:usd");
        let mut conn = self.redis_conn().await?;
        let raw: Option<String> = conn.get(&key).await?;
        let Some(raw) = raw else {
            return Ok(None);
        };
        let parsed: VolUsdCache = serde_json::from_str(&raw)?;
        self.metrics.cache_hits.with_label_values(&["redis"]).inc();
        Ok(Some((parsed.volume_usd, parsed.tx_count)))
    }
}
