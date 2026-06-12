use std::ops::{Deref, DerefMut};
use std::time::Duration;

use anyhow::Context;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use diesel_migrations::EmbeddedMigrations;
use scoped_futures::{ScopedBoxFuture, ScopedFutureExt};
use sui_indexer_alt_framework_store_traits as store;
use url::Url;

use crate::model::StoredWatermark;
use crate::schema::watermarks;

#[derive(Clone)]
pub struct PostgresStore {
    pool: Pool<AsyncPgConnection>,
}

/// Pooled Postgres connection used for watermark operations.
pub struct PostgresConnection<'a>(PooledConnection<'a, AsyncPgConnection>);

impl Deref for PostgresConnection<'_> {
    type Target = AsyncPgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PostgresConnection<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct DbArgs {
    pub db_connection_pool_size: u32,
    pub db_connection_timeout_ms: u64,
}

impl Default for DbArgs {
    fn default() -> Self {
        Self {
            db_connection_pool_size: 10,
            db_connection_timeout_ms: 30_000,
        }
    }
}

impl PostgresStore {
    pub async fn for_write(database_url: Url, config: DbArgs) -> anyhow::Result<Self> {
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
            .max_size(config.db_connection_pool_size)
            .connection_timeout(Duration::from_millis(config.db_connection_timeout_ms))
            .build(manager)
            .await?;

        Ok(Self { pool })
    }

    pub async fn connect(&self) -> anyhow::Result<PostgresConnection<'_>> {
        Ok(PostgresConnection(self.pool.get().await?))
    }

    pub async fn run_migrations(&self, migrations: EmbeddedMigrations) -> anyhow::Result<()> {
        use diesel_migrations::MigrationHarness;

        let conn = self.pool.dedicated_connection().await?;
        let mut wrapper: AsyncConnectionWrapper<AsyncPgConnection> =
            AsyncConnectionWrapper::from(conn);

        tokio::task::spawn_blocking(move || {
            wrapper
                .run_pending_migrations(migrations)
                .map(|_| ())
                .map_err(|e| anyhow::anyhow!("migration failed: {e:?}"))
        })
        .await
        .map_err(|e| anyhow::anyhow!("migration task join failed: {e}"))??;

        Ok(())
    }
}

#[async_trait]
impl store::Connection for PostgresConnection<'_> {
    async fn init_watermark(
        &mut self,
        pipeline_task: &str,
        checkpoint_hi_inclusive: Option<u64>,
    ) -> anyhow::Result<Option<store::InitWatermark>> {
        let checkpoint_hi_inclusive = checkpoint_hi_inclusive.map_or(-1, |c| c as i64);
        let stored_watermark = StoredWatermark::for_init(
            pipeline_task,
            checkpoint_hi_inclusive,
            checkpoint_hi_inclusive + 1,
        );

        use diesel::pg::upsert::excluded;
        let (checkpoint_hi_inclusive, reader_lo): (i64, i64) =
            diesel::insert_into(watermarks::table)
                .values(&stored_watermark)
                .on_conflict(watermarks::pipeline)
                .do_update()
                .set(watermarks::pipeline.eq(excluded(watermarks::pipeline)))
                .returning((watermarks::checkpoint_hi_inclusive, watermarks::reader_lo))
                .get_result(self)
                .await?;

        Ok(Some(store::InitWatermark {
            checkpoint_hi_inclusive: u64::try_from(checkpoint_hi_inclusive).ok(),
            reader_lo: Some(reader_lo as u64),
        }))
    }

    async fn accepts_chain_id(
        &mut self,
        pipeline_task: &str,
        chain_id: [u8; 32],
    ) -> anyhow::Result<bool> {
        let chain_id_vec = chain_id.to_vec();
        let stored_chain_id: Option<Vec<u8>> = diesel::sql_query(
            "UPDATE watermarks SET chain_id = COALESCE(chain_id, $1) \
             WHERE pipeline = $2 RETURNING chain_id",
        )
        .bind::<diesel::sql_types::Bytea, _>(chain_id_vec)
        .bind::<diesel::sql_types::Text, _>(pipeline_task)
        .get_result::<ChainIdRow>(self)
        .await
        .optional()?
        .map(|row| row.chain_id);

        let stored_chain_id =
            stored_chain_id.context("missing chain id after update — init watermark first")?;
        let stored: [u8; 32] = stored_chain_id
            .try_into()
            .map_err(|v: Vec<u8>| anyhow::anyhow!("chain id has wrong length: {}", v.len()))?;
        Ok(stored == chain_id)
    }

    async fn committer_watermark(
        &mut self,
        pipeline_task: &str,
    ) -> anyhow::Result<Option<store::CommitterWatermark>> {
        let (
            epoch_hi_inclusive,
            checkpoint_hi_inclusive,
            tx_hi,
            timestamp_ms_hi_inclusive,
            reader_lo,
        ): (i64, i64, i64, i64, i64) = watermarks::table
            .select((
                watermarks::epoch_hi_inclusive,
                watermarks::checkpoint_hi_inclusive,
                watermarks::tx_hi,
                watermarks::timestamp_ms_hi_inclusive,
                watermarks::reader_lo,
            ))
            .filter(watermarks::pipeline.eq(pipeline_task))
            .first(self)
            .await?;

        Ok(
            (reader_lo <= checkpoint_hi_inclusive).then_some(store::CommitterWatermark {
                epoch_hi_inclusive: epoch_hi_inclusive as u64,
                checkpoint_hi_inclusive: checkpoint_hi_inclusive as u64,
                tx_hi: tx_hi as u64,
                timestamp_ms_hi_inclusive: timestamp_ms_hi_inclusive as u64,
            }),
        )
    }

    async fn set_committer_watermark(
        &mut self,
        pipeline_task: &str,
        watermark: store::CommitterWatermark,
    ) -> anyhow::Result<bool> {
        Ok(diesel::update(watermarks::table)
            .set((
                watermarks::epoch_hi_inclusive.eq(watermark.epoch_hi_inclusive as i64),
                watermarks::checkpoint_hi_inclusive.eq(watermark.checkpoint_hi_inclusive as i64),
                watermarks::tx_hi.eq(watermark.tx_hi as i64),
                watermarks::timestamp_ms_hi_inclusive
                    .eq(watermark.timestamp_ms_hi_inclusive as i64),
            ))
            .filter(watermarks::pipeline.eq(pipeline_task))
            .filter(
                watermarks::checkpoint_hi_inclusive.lt(watermark.checkpoint_hi_inclusive as i64),
            )
            .execute(self)
            .await?
            > 0)
    }
}

#[async_trait]
impl store::SequentialConnection for PostgresConnection<'_> {}

#[async_trait]
impl store::Store for PostgresStore {
    type Connection<'c> = PostgresConnection<'c>;

    async fn connect<'c>(&'c self) -> anyhow::Result<Self::Connection<'c>> {
        PostgresStore::connect(self).await
    }
}

#[async_trait]
impl store::SequentialStore for PostgresStore {
    type SequentialConnection<'c> = PostgresConnection<'c>;

    async fn transaction<'a, R, F>(&self, f: F) -> anyhow::Result<R>
    where
        R: Send + 'a,
        F: Send + 'a,
        F: for<'r> FnOnce(
            &'r mut Self::Connection<'_>,
        ) -> ScopedBoxFuture<'a, 'r, anyhow::Result<R>>,
    {
        let mut conn = self.connect().await?;
        AsyncConnection::transaction(&mut conn, |conn| async move { f(conn).await }.scope_boxed())
            .await
    }
}

#[derive(QueryableByName)]
struct ChainIdRow {
    #[diesel(sql_type = diesel::sql_types::Bytea)]
    chain_id: Vec<u8>,
}
