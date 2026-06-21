use std::ops::{Deref, DerefMut};
use std::time::Duration;

use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use rust_decimal::Decimal;
use url::Url;

use crate::normalized_swap::NormalizedSwap;
use crate::ohlc::OhlcBar;

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
        let inserted = diesel::sql_query(
            "INSERT INTO swaps_fact (
                time, tx_digest, event_seq, protocol, pool_id,
                base_coin_type, quote_coin_type, amount_base, amount_quote,
                price_quote_per_base, fee_amount, sender, checkpoint_seq
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8::numeric, $9::numeric, $10::numeric, $11::numeric, $12, $13)
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
    ) -> anyhow::Result<bool> {
        let mut conn = self.get_connection().await?;
        let tvl = tvl_quote.map(|d| d.to_string());
        let inserted = diesel::sql_query(
            "INSERT INTO pool_liquidity (time, pool_id, vault_a_raw, vault_b_raw, tvl_quote, source)
             VALUES ($1, $2, $3::numeric, $4::numeric, $5::numeric, 'swap_event')
             ON CONFLICT DO NOTHING",
        )
        .bind::<diesel::sql_types::Timestamptz, _>(swap.time)
        .bind::<diesel::sql_types::Text, _>(&swap.pool_id)
        .bind::<diesel::sql_types::Text, _>(vault_a)
        .bind::<diesel::sql_types::Text, _>(vault_b)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(tvl)
        .execute(&mut conn)
        .await?;

        Ok(inserted > 0)
    }

    pub async fn upsert_ohlc_1m(&self, bar: &OhlcBar) -> anyhow::Result<()> {
        let mut conn = self.get_connection().await?;
        diesel::sql_query(
            "INSERT INTO ohlc_1m (
                bucket, pool_id, base_coin_type, quote_coin_type,
                open, high, low, close, volume_quote, trade_count
            ) VALUES ($1, $2, $3, $4, $5::numeric, $6::numeric, $7::numeric, $8::numeric, $9::numeric, $10)
            ON CONFLICT (bucket, pool_id, base_coin_type, quote_coin_type) DO UPDATE SET
                high = GREATEST(ohlc_1m.high, EXCLUDED.high),
                low = LEAST(ohlc_1m.low, EXCLUDED.low),
                close = EXCLUDED.close,
                volume_quote = ohlc_1m.volume_quote + EXCLUDED.volume_quote,
                trade_count = ohlc_1m.trade_count + EXCLUDED.trade_count",
        )
        .bind::<diesel::sql_types::Timestamptz, _>(bar.bucket)
        .bind::<diesel::sql_types::Text, _>(&bar.pool_id)
        .bind::<diesel::sql_types::Text, _>(&bar.base_coin_type)
        .bind::<diesel::sql_types::Text, _>(&bar.quote_coin_type)
        .bind::<diesel::sql_types::Text, _>(&bar.open.to_string())
        .bind::<diesel::sql_types::Text, _>(&bar.high.to_string())
        .bind::<diesel::sql_types::Text, _>(&bar.low.to_string())
        .bind::<diesel::sql_types::Text, _>(&bar.close.to_string())
        .bind::<diesel::sql_types::Text, _>(&bar.volume_quote.to_string())
        .bind::<diesel::sql_types::Int4, _>(bar.trade_count)
        .execute(&mut conn)
        .await?;

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
            "SELECT COALESCE(SUM(amount_quote), 0)::text AS volume, COUNT(*)::bigint AS tx_count
             FROM swaps_fact
             WHERE base_coin_type = $1 AND time > now() - INTERVAL '24 hours'",
        )
        .bind::<diesel::sql_types::Text, _>(base_coin_type)
        .get_result(&mut conn)
        .await?;

        let volume = row.volume.parse().unwrap_or(Decimal::ZERO);
        Ok((volume, row.tx_count))
    }
}
