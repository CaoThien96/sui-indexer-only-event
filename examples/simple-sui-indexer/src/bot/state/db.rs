use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use super::models::{BotPool, BotToken, Dex, TokenStatus};
use crate::bot::event_id::format_event_id;

pub struct BotStateStore {
    client: Arc<Mutex<Client>>,
}

impl BotStateStore {
    pub async fn connect(database_url: &str) -> Result<Self> {
        let (client, connection) =
            tokio_postgres::connect(database_url, tokio_postgres::NoTls)
                .await
                .context("connect bot state db")?;
        tokio::spawn(async move {
            if let Err(err) = connection.await {
                tracing::error!(?err, "bot state postgres connection error");
            }
        });
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }

    async fn client(&self) -> tokio::sync::MutexGuard<'_, Client> {
        self.client.lock().await
    }

    pub async fn event_exists(&self, tx_digest: &str, event_seq: &str) -> Result<bool> {
        let client = self.client().await;
        let row = client
            .query_opt(
                "SELECT 1 FROM bot_processed_events WHERE tx_digest = $1 AND event_seq = $2",
                &[&tx_digest, &event_seq],
            )
            .await?;
        Ok(row.is_some())
    }

    pub async fn insert_processed_event(
        &self,
        tx_digest: &str,
        event_seq: &str,
        event_type_str: &str,
    ) -> Result<()> {
        let id = format_event_id(tx_digest, event_seq);
        let client = self.client().await;
        client
            .execute(
                "INSERT INTO bot_processed_events (id, event_type, tx_digest, event_seq)
                 VALUES ($1, $2, $3, $4)
                 ON CONFLICT (tx_digest, event_seq) DO NOTHING",
                &[&id, &event_type_str, &tx_digest, &event_seq],
            )
            .await?;
        Ok(())
    }

    pub async fn swap_exists(&self, tx_digest: &str, event_seq: &str) -> Result<bool> {
        let client = self.client().await;
        let row = client
            .query_opt(
                "SELECT 1 FROM bot_processed_swaps WHERE tx_digest = $1 AND event_seq = $2",
                &[&tx_digest, &event_seq],
            )
            .await?;
        Ok(row.is_some())
    }

    pub async fn delete_processed_swaps_older_than(&self, ttl_days: u32) -> Result<u64> {
        let client = self.client().await;
        let deleted = client
            .execute(
                "DELETE FROM bot_processed_swaps WHERE created_at < NOW() - ($1::integer * INTERVAL '1 day')",
                &[&(ttl_days as i32)],
            )
            .await?;
        Ok(deleted)
    }

    pub async fn insert_processed_swap(
        &self,
        tx_digest: &str,
        event_seq: &str,
        pool_id_str: &str,
    ) -> Result<()> {
        let id = format_event_id(tx_digest, event_seq);
        let client = self.client().await;
        client
            .execute(
                "INSERT INTO bot_processed_swaps (id, pool_id, tx_digest, event_seq)
                 VALUES ($1, $2, $3, $4)
                 ON CONFLICT (tx_digest, event_seq) DO NOTHING",
                &[&id, &pool_id_str, &tx_digest, &event_seq],
            )
            .await?;
        Ok(())
    }

    pub async fn get_pool_with_token(&self, pool_id_str: &str) -> Result<Option<(BotPool, BotToken)>> {
        let client = self.client().await;
        let row = client
            .query_opt(
                r#"
                SELECT p.id, p.token_id, p.dex::text,
                       t.id, t.symbol, t.status::text, t.pool_id
                FROM bot_pools p
                INNER JOIN bot_tokens t ON p.token_id = t.id
                WHERE p.id = $1
                "#,
                &[&pool_id_str],
            )
            .await?;

        Ok(row.map(|r| {
            (
                BotPool {
                    id: r.get(0),
                    token_id: r.get(1),
                    dex: Dex::from_db(r.get::<_, String>(2).as_str()).unwrap_or(Dex::Cetus),
                },
                BotToken {
                    id: r.get(3),
                    symbol: r.get(4),
                    status: TokenStatus::from_db(r.get::<_, String>(5).as_str()),
                    pool_id: r.get(6),
                },
            )
        }))
    }

    pub async fn token_exists(&self, token_id: &str) -> Result<bool> {
        let client = self.client().await;
        let row = client
            .query_opt("SELECT 1 FROM bot_tokens WHERE id = $1", &[&token_id])
            .await?;
        Ok(row.is_some())
    }

    pub async fn upsert_token_listing(
        &self,
        token_id: &str,
        symbol: &str,
        owner: &str,
        pool_id_str: &str,
        dex: Dex,
        tx_digest_str: &str,
    ) -> Result<()> {
        let client = self.client().await;
        client
            .execute(
                r#"
                INSERT INTO bot_tokens (
                    id, name, symbol, decimals, total_supply, owner, deny_cap_id,
                    status, pool_id
                ) VALUES ($1, $2, $3, 9, $4, $5, '', 'listing', $6)
                ON CONFLICT (id) DO UPDATE SET
                    status = 'listing',
                    pool_id = EXCLUDED.pool_id,
                    updated_at = NOW()
                "#,
                &[
                    &token_id,
                    &symbol,
                    &symbol,
                    &(1_000_000_000_i64 * 1_000_000_000),
                    &owner,
                    &pool_id_str,
                ],
            )
            .await?;
        client
            .execute(
                r#"
                INSERT INTO bot_pools (id, token_id, dex, tx_digest)
                VALUES ($1, $2, $3::text::bot_dex, $4)
                ON CONFLICT (id) DO NOTHING
                "#,
                &[&pool_id_str, &token_id, &dex.as_str(), &tx_digest_str],
            )
            .await?;
        Ok(())
    }

    pub async fn mark_token_done(&self, pool_id_str: &str) -> Result<()> {
        let client = self.client().await;
        client
            .execute(
                "UPDATE bot_tokens SET status = 'done', updated_at = NOW() WHERE pool_id = $1",
                &[&pool_id_str],
            )
            .await?;
        Ok(())
    }

    pub async fn get_pool_shared_initial_version(&self, pool_id: &str) -> Result<Option<u64>> {
        let client = self.client().await;
        let row = client
            .query_opt(
                "SELECT initial_shared_version FROM bot_pools WHERE id = $1",
                &[&pool_id],
            )
            .await?;
        Ok(row.and_then(|r| {
            r.get::<_, Option<i64>>(0)
                .and_then(|v| u64::try_from(v).ok())
        }))
    }

    pub async fn set_pool_shared_initial_version(
        &self,
        pool_id: &str,
        initial_shared_version: u64,
    ) -> Result<()> {
        let version_i64 = i64::try_from(initial_shared_version).context("version exceeds i64")?;
        let client = self.client().await;
        client
            .execute(
                "UPDATE bot_pools SET initial_shared_version = $2, updated_at = NOW() WHERE id = $1",
                &[&pool_id, &version_i64],
            )
            .await?;
        Ok(())
    }

    pub async fn list_pools_missing_shared_version(&self) -> Result<Vec<String>> {
        let client = self.client().await;
        let rows = client
            .query(
                "SELECT id FROM bot_pools WHERE initial_shared_version IS NULL ORDER BY created_at",
                &[],
            )
            .await?;
        Ok(rows.into_iter().map(|r| r.get(0)).collect())
    }
}
