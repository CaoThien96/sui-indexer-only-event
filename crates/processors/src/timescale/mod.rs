use std::ops::{Deref, DerefMut};
use std::time::Duration;

use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use diesel::OptionalExtension;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use rust_decimal::Decimal;
use url::Url;

use crate::normalized_swap::NormalizedSwap;
use crate::token_ohlc::{
    OHLC_INTERVALS, TokenUsdOhlcBar, swap_to_token_usd_bar, token_ohlc_usd_table,
};

pub const TIMESCALE_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations_timescale");

#[derive(Clone)]
pub struct TimescaleStore {
    pool: Pool<AsyncPgConnection>,
}

struct TimescaleConnection<'a>(PooledConnection<'a, AsyncPgConnection>);

impl Deref for TimescaleConnection<'_> {
    type Target = AsyncPgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TimescaleConnection<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TimescaleStore {
    pub async fn connect(database_url: Url) -> anyhow::Result<Self> {
        let mut manager_config = ManagerConfig::default();
        manager_config.custom_setup = Box::new(|url| {
            Box::pin(async move {
                let conn = AsyncPgConnection::establish(url).await?;
                Ok(conn)
            })
        });

        let manager =
            AsyncDieselConnectionManager::new_with_config(database_url.as_str(), manager_config);
        let pool = Pool::builder()
            .max_size(10)
            .connection_timeout(Duration::from_millis(30_000))
            .build(manager)
            .await?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> anyhow::Result<()> {
        let conn = self.pool.dedicated_connection().await?;
        let mut wrapper: AsyncConnectionWrapper<AsyncPgConnection> =
            AsyncConnectionWrapper::from(conn);

        tokio::task::spawn_blocking(move || {
            wrapper
                .run_pending_migrations(TIMESCALE_MIGRATIONS)
                .map(|_| ())
                .map_err(|e| anyhow::anyhow!("timescale migration failed: {e:?}"))
        })
        .await
        .map_err(|e| anyhow::anyhow!("migration task join failed: {e}"))??;
        Ok(())
    }

    async fn get_connection(&self) -> anyhow::Result<TimescaleConnection<'_>> {
        Ok(TimescaleConnection(self.pool.get().await?))
    }

    /// Returns true if a new row was inserted.
    pub async fn insert_swap_fact(&self, swap: &NormalizedSwap) -> anyhow::Result<bool> {
        let mut conn = self.get_connection().await?;
        let fee = swap.fee_amount.map(|d| d.to_string());
        let amount_usd = swap.amount_usd.map(|d| d.to_string());
        let price_usd = swap.price_usd_per_base.map(|d| d.to_string());
        let inserted = diesel::sql_query(
            "INSERT INTO swaps_fact (
                time, tx_digest, event_seq, protocol, pool_id,
                base_coin_type, quote_coin_type, amount_base, amount_quote,
                price_quote_per_base, amount_usd, price_usd_per_base, fee_amount, sender, checkpoint_seq
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8::numeric, $9::numeric, $10::numeric, $11::numeric, $12::numeric, $13::numeric, $14, $15)
            ON CONFLICT DO NOTHING",
        )
        .bind::<diesel::sql_types::Timestamptz, _>(swap.time)
        .bind::<diesel::sql_types::Text, _>(&swap.tx_digest)
        .bind::<diesel::sql_types::Int4, _>(swap.event_seq)
        .bind::<diesel::sql_types::Text, _>(&swap.protocol)
        .bind::<diesel::sql_types::Text, _>(&swap.pool_id)
        .bind::<diesel::sql_types::Text, _>(&swap.base_coin_type)
        .bind::<diesel::sql_types::Text, _>(&swap.quote_coin_type)
        .bind::<diesel::sql_types::Text, _>(&swap.amount_base.to_string())
        .bind::<diesel::sql_types::Text, _>(&swap.amount_quote.to_string())
        .bind::<diesel::sql_types::Text, _>(&swap.price_quote_per_base.to_string())
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(amount_usd)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(price_usd)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(fee)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(swap.sender.as_deref())
        .bind::<diesel::sql_types::Int8, _>(swap.checkpoint_seq)
        .execute(&mut conn)
        .await?;

        Ok(inserted > 0)
    }

    pub async fn insert_pool_liquidity(
        &self,
        swap: &NormalizedSwap,
        vault_a: &str,
        vault_b: &str,
        tvl_quote: Option<Decimal>,
        tvl_usd: Option<Decimal>,
    ) -> anyhow::Result<bool> {
        let mut conn = self.get_connection().await?;
        let tvl = tvl_quote.map(|d| d.to_string());
        let tvl_usd = tvl_usd.map(|d| d.to_string());
        let inserted = diesel::sql_query(
            "INSERT INTO pool_liquidity (time, pool_id, vault_a_raw, vault_b_raw, tvl_quote, tvl_usd, source)
             VALUES ($1, $2, $3::numeric, $4::numeric, $5::numeric, $6::numeric, 'swap_event')
             ON CONFLICT DO NOTHING",
        )
        .bind::<diesel::sql_types::Timestamptz, _>(swap.time)
        .bind::<diesel::sql_types::Text, _>(&swap.pool_id)
        .bind::<diesel::sql_types::Text, _>(vault_a)
        .bind::<diesel::sql_types::Text, _>(vault_b)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(tvl)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(tvl_usd)
        .execute(&mut conn)
        .await?;

        Ok(inserted > 0)
    }

    pub async fn upsert_token_ohlc_usd(
        &self,
        interval: &str,
        bar: &TokenUsdOhlcBar,
    ) -> anyhow::Result<()> {
        let table = token_ohlc_usd_table(interval)
            .ok_or_else(|| anyhow::anyhow!("invalid interval: {interval}"))?;
        let sql = format!(
            "INSERT INTO {table} (
                bucket, base_coin_type, open_usd, high_usd, low_usd, close_usd, volume_usd, trade_count
            ) VALUES ($1, $2, $3::numeric, $4::numeric, $5::numeric, $6::numeric, $7::numeric, $8)
            ON CONFLICT (bucket, base_coin_type) DO UPDATE SET
                high_usd = GREATEST({table}.high_usd, EXCLUDED.high_usd),
                low_usd = LEAST({table}.low_usd, EXCLUDED.low_usd),
                close_usd = EXCLUDED.close_usd,
                volume_usd = {table}.volume_usd + EXCLUDED.volume_usd,
                trade_count = {table}.trade_count + EXCLUDED.trade_count"
        );

        let mut conn = self.get_connection().await?;
        diesel::sql_query(&sql)
            .bind::<diesel::sql_types::Timestamptz, _>(bar.bucket)
            .bind::<diesel::sql_types::Text, _>(&bar.base_coin_type)
            .bind::<diesel::sql_types::Text, _>(&bar.open_usd.to_string())
            .bind::<diesel::sql_types::Text, _>(&bar.high_usd.to_string())
            .bind::<diesel::sql_types::Text, _>(&bar.low_usd.to_string())
            .bind::<diesel::sql_types::Text, _>(&bar.close_usd.to_string())
            .bind::<diesel::sql_types::Text, _>(&bar.volume_usd.to_string())
            .bind::<diesel::sql_types::Int4, _>(bar.trade_count)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn upsert_token_ohlc_usd_all_intervals(
        &self,
        swap: &NormalizedSwap,
    ) -> anyhow::Result<()> {
        if swap.price_usd_per_base.is_none() || swap.amount_usd.is_none() {
            return Ok(());
        }
        for interval in OHLC_INTERVALS {
            if let Some(bar) = swap_to_token_usd_bar(swap, interval) {
                self.upsert_token_ohlc_usd(interval, &bar).await?;
            }
        }
        Ok(())
    }

    pub async fn sum_token_volume_24h(
        &self,
        base_coin_type: &str,
    ) -> anyhow::Result<(Decimal, i64)> {
        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Text)]
            volume: String,
            #[diesel(sql_type = diesel::sql_types::Int8)]
            tx_count: i64,
        }

        let mut conn = self.get_connection().await?;
        let row: Row = diesel::sql_query(
            "SELECT COALESCE(SUM(volume_quote), 0)::text AS volume, COALESCE(SUM(tx_count), 0)::bigint AS tx_count
             FROM token_volume_24h
             WHERE base_coin_type = $1 AND bucket > now() - INTERVAL '24 hours'",
        )
        .bind::<diesel::sql_types::Text, _>(base_coin_type)
        .get_result(&mut conn)
        .await?;

        let volume = row.volume.parse().unwrap_or(Decimal::ZERO);
        Ok((volume, row.tx_count))
    }

    pub async fn sum_token_volume_24h_usd(
        &self,
        base_coin_type: &str,
    ) -> anyhow::Result<(Decimal, i64)> {
        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Text)]
            volume: String,
            #[diesel(sql_type = diesel::sql_types::Int8)]
            tx_count: i64,
        }

        let mut conn = self.get_connection().await?;
        let row: Row = diesel::sql_query(
            "SELECT COALESCE(SUM(volume_usd), 0)::text AS volume,
                    COALESCE(SUM(trade_count), 0)::bigint AS tx_count
             FROM token_ohlc_usd_1m
             WHERE base_coin_type = $1
               AND bucket > now() - INTERVAL '24 hours'",
        )
        .bind::<diesel::sql_types::Text, _>(base_coin_type)
        .get_result(&mut conn)
        .await?;

        let volume = row.volume.parse().unwrap_or(Decimal::ZERO);
        Ok((volume, row.tx_count))
    }

    pub async fn latest_price_for_token(
        &self,
        base_coin_type: &str,
    ) -> anyhow::Result<Option<(String, String)>> {
        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Text)]
            price: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            quote_coin_type: String,
        }

        let mut conn = self.get_connection().await?;
        let row: Option<Row> = diesel::sql_query(
            "SELECT price_quote_per_base::text AS price, quote_coin_type
             FROM swaps_fact
             WHERE base_coin_type = $1
             ORDER BY time DESC
             LIMIT 1",
        )
        .bind::<diesel::sql_types::Text, _>(base_coin_type)
        .get_result(&mut conn)
        .await
        .optional()?;

        Ok(row.map(|r| (r.price, r.quote_coin_type)))
    }

    pub async fn latest_price_usd_for_token(
        &self,
        base_coin_type: &str,
    ) -> anyhow::Result<Option<String>> {
        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Text)]
            price_usd: String,
        }

        let mut conn = self.get_connection().await?;
        let row: Option<Row> = diesel::sql_query(
            "SELECT price_usd_per_base::text AS price_usd
             FROM swaps_fact
             WHERE base_coin_type = $1 AND price_usd_per_base IS NOT NULL
             ORDER BY time DESC
             LIMIT 1",
        )
        .bind::<diesel::sql_types::Text, _>(base_coin_type)
        .get_result(&mut conn)
        .await
        .optional()?;

        Ok(row.map(|r| r.price_usd))
    }

    pub async fn latest_sui_usd_at_or_before(
        &self,
        ts: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<Option<Decimal>> {
        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Text)]
            price_usd: String,
        }

        let mut conn = self.get_connection().await?;
        let row: Option<Row> = diesel::sql_query(
            "SELECT price_usd::text AS price_usd
             FROM sui_usd_1m
             WHERE bucket <= $1
             ORDER BY bucket DESC
             LIMIT 1",
        )
        .bind::<diesel::sql_types::Timestamptz, _>(ts)
        .get_result(&mut conn)
        .await
        .optional()?;

        Ok(row.and_then(|r| r.price_usd.parse().ok()))
    }

    pub async fn upsert_sui_usd_1m(
        &self,
        bucket: chrono::DateTime<chrono::Utc>,
        price_usd: Decimal,
        source_type: &str,
        source_pool_id: Option<&str>,
        quality_score: Decimal,
    ) -> anyhow::Result<()> {
        let mut conn = self.get_connection().await?;
        diesel::sql_query(
            "INSERT INTO sui_usd_1m (bucket, price_usd, source_type, source_pool_id, quality_score, updated_at)
             VALUES ($1, $2::numeric, $3, $4, $5::numeric, now())
             ON CONFLICT (bucket) DO UPDATE SET
                 price_usd = EXCLUDED.price_usd,
                 source_type = EXCLUDED.source_type,
                 source_pool_id = EXCLUDED.source_pool_id,
                 quality_score = EXCLUDED.quality_score,
                 updated_at = now()",
        )
        .bind::<diesel::sql_types::Timestamptz, _>(bucket)
        .bind::<diesel::sql_types::Text, _>(price_usd.to_string())
        .bind::<diesel::sql_types::Text, _>(source_type)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(source_pool_id)
        .bind::<diesel::sql_types::Text, _>(quality_score.to_string())
        .execute(&mut conn)
        .await?;
        Ok(())
    }

    pub async fn upsert_token_usd_1m(
        &self,
        bucket: chrono::DateTime<chrono::Utc>,
        base_coin_type: &str,
        price_usd: Decimal,
        source_type: &str,
        source_pool_id: Option<&str>,
        quality_score: Decimal,
    ) -> anyhow::Result<()> {
        let mut conn = self.get_connection().await?;
        diesel::sql_query(
            "INSERT INTO token_usd_1m (
                bucket, base_coin_type, price_usd, source_type, source_pool_id, quality_score, updated_at
            ) VALUES ($1, $2, $3::numeric, $4, $5, $6::numeric, now())
            ON CONFLICT (bucket, base_coin_type) DO UPDATE SET
                price_usd = EXCLUDED.price_usd,
                source_type = EXCLUDED.source_type,
                source_pool_id = EXCLUDED.source_pool_id,
                quality_score = EXCLUDED.quality_score,
                updated_at = now()",
        )
        .bind::<diesel::sql_types::Timestamptz, _>(bucket)
        .bind::<diesel::sql_types::Text, _>(base_coin_type)
        .bind::<diesel::sql_types::Text, _>(price_usd.to_string())
        .bind::<diesel::sql_types::Text, _>(source_type)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(source_pool_id)
        .bind::<diesel::sql_types::Text, _>(quality_score.to_string())
        .execute(&mut conn)
        .await?;
        Ok(())
    }

    pub async fn latest_token_usd_at_or_before(
        &self,
        base_coin_type: &str,
        ts: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<Option<Decimal>> {
        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Text)]
            price_usd: String,
        }

        let mut conn = self.get_connection().await?;
        let row: Option<Row> = diesel::sql_query(
            "SELECT price_usd::text AS price_usd
             FROM token_usd_1m
             WHERE base_coin_type = $1 AND bucket <= $2
             ORDER BY bucket DESC
             LIMIT 1",
        )
        .bind::<diesel::sql_types::Text, _>(base_coin_type)
        .bind::<diesel::sql_types::Timestamptz, _>(ts)
        .get_result(&mut conn)
        .await
        .optional()?;

        Ok(row.and_then(|r| r.price_usd.parse().ok()))
    }

    pub async fn get_bootstrap_status(&self, run_id: &str) -> anyhow::Result<Option<String>> {
        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Text)]
            status: String,
        }

        let mut conn = self.get_connection().await?;
        let row: Option<Row> = diesel::sql_query(
            "SELECT status FROM bootstrap_state WHERE run_id = $1",
        )
        .bind::<diesel::sql_types::Text, _>(run_id)
        .get_result(&mut conn)
        .await
        .optional()?;

        Ok(row.map(|r| r.status))
    }

    pub async fn upsert_bootstrap_state(
        &self,
        run_id: &str,
        target_first_checkpoint: i64,
        status: &str,
        iteration: i32,
        window_start_checkpoint: i64,
        window_end_checkpoint: i64,
        metrics_json: serde_json::Value,
    ) -> anyhow::Result<()> {
        let mut conn = self.get_connection().await?;
        diesel::sql_query(
            "INSERT INTO bootstrap_state (
                run_id, target_first_checkpoint, status, iteration,
                window_start_checkpoint, window_end_checkpoint, metrics_json, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7::jsonb, now())
            ON CONFLICT (run_id) DO UPDATE SET
                status = EXCLUDED.status,
                iteration = EXCLUDED.iteration,
                window_start_checkpoint = EXCLUDED.window_start_checkpoint,
                window_end_checkpoint = EXCLUDED.window_end_checkpoint,
                metrics_json = EXCLUDED.metrics_json,
                updated_at = now()",
        )
        .bind::<diesel::sql_types::Text, _>(run_id)
        .bind::<diesel::sql_types::Int8, _>(target_first_checkpoint)
        .bind::<diesel::sql_types::Text, _>(status)
        .bind::<diesel::sql_types::Int4, _>(iteration)
        .bind::<diesel::sql_types::Int8, _>(window_start_checkpoint)
        .bind::<diesel::sql_types::Int8, _>(window_end_checkpoint)
        .bind::<diesel::sql_types::Text, _>(metrics_json.to_string())
        .execute(&mut conn)
        .await?;
        Ok(())
    }


    pub async fn list_swaps(
        &self,
        base_coin_type: &str,
        pool_id: Option<&str>,
        limit: i64,
        before_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> anyhow::Result<Vec<SwapRow>> {
        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Timestamptz)]
            time: chrono::DateTime<chrono::Utc>,
            #[diesel(sql_type = diesel::sql_types::Text)]
            tx_digest: String,
            #[diesel(sql_type = diesel::sql_types::Int4)]
            event_seq: i32,
            #[diesel(sql_type = diesel::sql_types::Text)]
            protocol: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            pool_id: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            amount_base: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            amount_quote: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            price_quote_per_base: String,
        }

        let mut conn = self.get_connection().await?;
        let rows: Vec<Row> = if let Some(pool_id) = pool_id {
            if let Some(t) = before_time {
                diesel::sql_query(
                    "SELECT time, tx_digest, event_seq, protocol, pool_id,
                            amount_base::text, amount_quote::text, price_quote_per_base::text
                     FROM swaps_fact
                     WHERE base_coin_type = $1 AND pool_id = $2 AND time < $3
                     ORDER BY time DESC
                     LIMIT $4",
                )
                .bind::<diesel::sql_types::Text, _>(base_coin_type)
                .bind::<diesel::sql_types::Text, _>(pool_id)
                .bind::<diesel::sql_types::Timestamptz, _>(t)
                .bind::<diesel::sql_types::Int8, _>(limit)
                .load(&mut conn)
                .await?
            } else {
                diesel::sql_query(
                    "SELECT time, tx_digest, event_seq, protocol, pool_id,
                            amount_base::text, amount_quote::text, price_quote_per_base::text
                     FROM swaps_fact
                     WHERE base_coin_type = $1 AND pool_id = $2
                     ORDER BY time DESC
                     LIMIT $3",
                )
                .bind::<diesel::sql_types::Text, _>(base_coin_type)
                .bind::<diesel::sql_types::Text, _>(pool_id)
                .bind::<diesel::sql_types::Int8, _>(limit)
                .load(&mut conn)
                .await?
            }
        } else if let Some(t) = before_time {
            diesel::sql_query(
                "SELECT time, tx_digest, event_seq, protocol, pool_id,
                        amount_base::text, amount_quote::text, price_quote_per_base::text
                 FROM swaps_fact
                 WHERE base_coin_type = $1 AND time < $2
                 ORDER BY time DESC
                 LIMIT $3",
            )
            .bind::<diesel::sql_types::Text, _>(base_coin_type)
            .bind::<diesel::sql_types::Timestamptz, _>(t)
            .bind::<diesel::sql_types::Int8, _>(limit)
            .load(&mut conn)
            .await?
        } else {
            diesel::sql_query(
                "SELECT time, tx_digest, event_seq, protocol, pool_id,
                        amount_base::text, amount_quote::text, price_quote_per_base::text
                 FROM swaps_fact
                 WHERE base_coin_type = $1
                 ORDER BY time DESC
                 LIMIT $2",
            )
            .bind::<diesel::sql_types::Text, _>(base_coin_type)
            .bind::<diesel::sql_types::Int8, _>(limit)
            .load(&mut conn)
            .await?
        };

        Ok(rows
            .into_iter()
            .map(|r| SwapRow {
                time: r.time,
                tx_digest: r.tx_digest,
                event_seq: r.event_seq,
                protocol: r.protocol,
                pool_id: r.pool_id,
                amount_base: r.amount_base,
                amount_quote: r.amount_quote,
                price_quote_per_base: r.price_quote_per_base,
            })
            .collect())
    }

    pub async fn query_token_ohlc_usd(
        &self,
        coin_type: &str,
        interval: &str,
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<Vec<TokenOhlcUsdRow>> {
        let table = match interval {
            "1m" => "token_ohlc_usd_1m",
            "5m" => "token_ohlc_usd_5m",
            "15m" => "token_ohlc_usd_15m",
            "30m" => "token_ohlc_usd_30m",
            "1h" => "token_ohlc_usd_1h",
            "4h" => "token_ohlc_usd_4h",
            "24h" => "token_ohlc_usd_24h",
            _ => return Err(anyhow::anyhow!("invalid interval: {interval}")),
        };

        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Timestamptz)]
            bucket: chrono::DateTime<chrono::Utc>,
            #[diesel(sql_type = diesel::sql_types::Text)]
            open: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            high: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            low: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            close: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            volume_usd: String,
            #[diesel(sql_type = diesel::sql_types::Int4)]
            trade_count: i32,
        }

        let sql = format!(
            "SELECT bucket,
                    open_usd::text AS open,
                    high_usd::text AS high,
                    low_usd::text AS low,
                    close_usd::text AS close,
                    volume_usd::text AS volume_usd,
                    trade_count::int AS trade_count
             FROM {table}
             WHERE base_coin_type = $1
               AND bucket >= $2 AND bucket <= $3
             ORDER BY bucket ASC"
        );

        let mut conn = self.get_connection().await?;
        let rows: Vec<Row> = diesel::sql_query(&sql)
            .bind::<diesel::sql_types::Text, _>(coin_type)
            .bind::<diesel::sql_types::Timestamptz, _>(from)
            .bind::<diesel::sql_types::Timestamptz, _>(to)
            .load(&mut conn)
            .await?;

        Ok(rows
            .into_iter()
            .map(|r| TokenOhlcUsdRow {
                bucket: r.bucket,
                open: r.open,
                high: r.high,
                low: r.low,
                close: r.close,
                volume_usd: r.volume_usd,
                trade_count: r.trade_count,
            })
            .collect())
    }

    pub async fn fetch_swaps_for_rolloff(
        &self,
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
        limit: i64,
    ) -> anyhow::Result<Vec<RolloffSwapRow>> {
        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Timestamptz)]
            time: chrono::DateTime<chrono::Utc>,
            #[diesel(sql_type = diesel::sql_types::Text)]
            tx_digest: String,
            #[diesel(sql_type = diesel::sql_types::Int4)]
            event_seq: i32,
            #[diesel(sql_type = diesel::sql_types::Text)]
            protocol: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            pool_id: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            base_coin_type: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            quote_coin_type: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            amount_base: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            amount_quote: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            price_quote_per_base: String,
            #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
            amount_usd: Option<String>,
            #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
            price_usd_per_base: Option<String>,
            #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
            fee_amount: Option<String>,
            #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
            sender: Option<String>,
            #[diesel(sql_type = diesel::sql_types::Int8)]
            checkpoint_seq: i64,
        }

        let mut conn = self.get_connection().await?;
        let rows: Vec<Row> = diesel::sql_query(
            "SELECT time, tx_digest, event_seq, protocol, pool_id,
                    base_coin_type, quote_coin_type,
                    amount_base::text, amount_quote::text, price_quote_per_base::text,
                    amount_usd::text, price_usd_per_base::text,
                    fee_amount::text, sender, checkpoint_seq
             FROM swaps_fact
             WHERE time >= $1 AND time < $2
             ORDER BY time ASC
             LIMIT $3",
        )
        .bind::<diesel::sql_types::Timestamptz, _>(from)
        .bind::<diesel::sql_types::Timestamptz, _>(to)
        .bind::<diesel::sql_types::Int8, _>(limit)
        .load(&mut conn)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| RolloffSwapRow {
                time: r.time,
                tx_digest: r.tx_digest,
                event_seq: r.event_seq,
                protocol: r.protocol,
                pool_id: r.pool_id,
                base_coin_type: r.base_coin_type,
                quote_coin_type: r.quote_coin_type,
                amount_base: r.amount_base,
                amount_quote: r.amount_quote,
                price_quote_per_base: r.price_quote_per_base,
                amount_usd: r.amount_usd,
                price_usd_per_base: r.price_usd_per_base,
                fee_amount: r.fee_amount,
                sender: r.sender,
                checkpoint_seq: r.checkpoint_seq,
            })
            .collect())
    }

    pub async fn fetch_token_ohlc_usd_for_rolloff(
        &self,
        table: &str,
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
        limit: i64,
    ) -> anyhow::Result<Vec<RolloffTokenOhlcUsdRow>> {
        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Timestamptz)]
            bucket: chrono::DateTime<chrono::Utc>,
            #[diesel(sql_type = diesel::sql_types::Text)]
            base_coin_type: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            open_usd: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            high_usd: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            low_usd: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            close_usd: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            volume_usd: String,
            #[diesel(sql_type = diesel::sql_types::Int4)]
            trade_count: i32,
        }

        let sql = format!(
            "SELECT bucket, base_coin_type,
                    open_usd::text, high_usd::text, low_usd::text, close_usd::text,
                    volume_usd::text, trade_count
             FROM {table}
             WHERE bucket >= $1 AND bucket < $2
             ORDER BY bucket ASC
             LIMIT $3"
        );

        let mut conn = self.get_connection().await?;
        let rows: Vec<Row> = diesel::sql_query(&sql)
            .bind::<diesel::sql_types::Timestamptz, _>(from)
            .bind::<diesel::sql_types::Timestamptz, _>(to)
            .bind::<diesel::sql_types::Int8, _>(limit)
            .load(&mut conn)
            .await?;

        Ok(rows
            .into_iter()
            .map(|r| RolloffTokenOhlcUsdRow {
                bucket: r.bucket,
                base_coin_type: r.base_coin_type,
                open_usd: r.open_usd,
                high_usd: r.high_usd,
                low_usd: r.low_usd,
                close_usd: r.close_usd,
                volume_usd: r.volume_usd,
                trade_count: r.trade_count,
            })
            .collect())
    }

    pub async fn get_rolloff_watermark(
        &self,
        table_name: &str,
    ) -> anyhow::Result<chrono::DateTime<chrono::Utc>> {
        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Timestamptz)]
            last_rolled_time: chrono::DateTime<chrono::Utc>,
        }

        let mut conn = self.get_connection().await?;
        let row: Option<Row> = diesel::sql_query(
            "SELECT last_rolled_time FROM rolloff_watermarks WHERE table_name = $1",
        )
        .bind::<diesel::sql_types::Text, _>(table_name)
        .get_result(&mut conn)
        .await
        .optional()?;

        Ok(row.map(|r| r.last_rolled_time).unwrap_or_else(|| {
            chrono::DateTime::from_timestamp(0, 0).unwrap_or_default()
        }))
    }

    pub async fn set_rolloff_watermark(
        &self,
        table_name: &str,
        last_rolled_time: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<()> {
        let mut conn = self.get_connection().await?;
        diesel::sql_query(
            "INSERT INTO rolloff_watermarks (table_name, last_rolled_time)
             VALUES ($1, $2)
             ON CONFLICT (table_name) DO UPDATE SET last_rolled_time = EXCLUDED.last_rolled_time",
        )
        .bind::<diesel::sql_types::Text, _>(table_name)
        .bind::<diesel::sql_types::Timestamptz, _>(last_rolled_time)
        .execute(&mut conn)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SwapRow {
    pub time: chrono::DateTime<chrono::Utc>,
    pub tx_digest: String,
    pub event_seq: i32,
    pub protocol: String,
    pub pool_id: String,
    pub amount_base: String,
    pub amount_quote: String,
    pub price_quote_per_base: String,
}

#[derive(Debug, Clone)]
pub struct TokenOhlcUsdRow {
    pub bucket: chrono::DateTime<chrono::Utc>,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume_usd: String,
    pub trade_count: i32,
}


#[derive(Debug, Clone)]
pub struct RolloffSwapRow {
    pub time: chrono::DateTime<chrono::Utc>,
    pub tx_digest: String,
    pub event_seq: i32,
    pub protocol: String,
    pub pool_id: String,
    pub base_coin_type: String,
    pub quote_coin_type: String,
    pub amount_base: String,
    pub amount_quote: String,
    pub price_quote_per_base: String,
    pub amount_usd: Option<String>,
    pub price_usd_per_base: Option<String>,
    pub fee_amount: Option<String>,
    pub sender: Option<String>,
    pub checkpoint_seq: i64,
}

#[derive(Debug, Clone)]
pub struct RolloffTokenOhlcUsdRow {
    pub bucket: chrono::DateTime<chrono::Utc>,
    pub base_coin_type: String,
    pub open_usd: String,
    pub high_usd: String,
    pub low_usd: String,
    pub close_usd: String,
    pub volume_usd: String,
    pub trade_count: i32,
}
