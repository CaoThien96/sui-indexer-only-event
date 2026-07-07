use anyhow::{Context, Result};
use redis::AsyncCommands;
use serde_json::json;

#[derive(Clone)]
pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    pub fn connect(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url).context("invalid REDIS_URL")?;
        Ok(Self { client })
    }

    async fn connection(&self) -> Result<redis::aio::MultiplexedConnection> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .context("redis connection failed")
    }

    pub async fn set_token_price(
        &self,
        coin_type: &str,
        price: &str,
        pool_id: &str,
        quote_coin_type: &str,
    ) -> Result<()> {
        let key = format!("token:{coin_type}:price");
        let payload = json!({
            "price": price,
            "pool_id": pool_id,
            "quote_coin_type": quote_coin_type,
        });
        let mut conn = self.connection().await?;
        conn.set_ex::<_, _, ()>(&key, payload.to_string(), 60)
            .await?;
        Ok(())
    }

    pub async fn set_token_price_usd(
        &self,
        coin_type: &str,
        price_usd: &str,
        source_pool_id: Option<&str>,
    ) -> Result<()> {
        let key = format!("token:{coin_type}:price:usd");
        let payload = json!({
            "price_usd": price_usd,
            "source_type": "processors",
            "source_pool_id": source_pool_id,
            "confidence_score": "1",
            "is_stale": false,
        });
        let mut conn = self.connection().await?;
        conn.set_ex::<_, _, ()>(&key, payload.to_string(), 60)
            .await?;
        Ok(())
    }

    pub async fn set_token_vol_24h(
        &self,
        coin_type: &str,
        volume: &str,
        tx_count: i64,
    ) -> Result<()> {
        let key = format!("token:{coin_type}:vol:24h");
        let payload = json!({
            "volume": volume,
            "tx_count": tx_count,
        });
        let mut conn = self.connection().await?;
        conn.set_ex::<_, _, ()>(&key, payload.to_string(), 120)
            .await?;
        Ok(())
    }

    pub async fn set_token_vol_24h_usd(
        &self,
        coin_type: &str,
        volume_usd: &str,
        tx_count: i64,
    ) -> Result<()> {
        let key = format!("token:{coin_type}:vol:24h:usd");
        let payload = json!({
            "volume_usd": volume_usd,
            "tx_count": tx_count,
        });
        let mut conn = self.connection().await?;
        conn.set_ex::<_, _, ()>(&key, payload.to_string(), 120)
            .await?;
        Ok(())
    }

    pub async fn set_pool_tvl(&self, pool_id: &str, tvl_quote: &str) -> Result<()> {
        let key = format!("pool:{pool_id}:tvl");
        let payload = json!({ "tvl_quote": tvl_quote });
        let mut conn = self.connection().await?;
        conn.set_ex::<_, _, ()>(&key, payload.to_string(), 300)
            .await?;
        Ok(())
    }
}
