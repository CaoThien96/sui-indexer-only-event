use std::ops::{Deref, DerefMut};
use std::time::Duration;

use diesel::prelude::*;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use diesel_migrations::{EmbeddedMigrations, embed_migrations, MigrationHarness};
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use event_bindings::protocol::Protocol;
use url::Url;

use crate::coin_type;
use crate::store::schema::{pools, protocols, token_watchlist, tokens};

pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Clone)]
pub struct CatalogStore {
    pool: Pool<AsyncPgConnection>,
}

pub struct CatalogConnection<'a>(PooledConnection<'a, AsyncPgConnection>);

impl Deref for CatalogConnection<'_> {
    type Target = AsyncPgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CatalogConnection<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CatalogStore {
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
                .run_pending_migrations(MIGRATIONS)
                .map(|_| ())
                .map_err(|e| anyhow::anyhow!("catalog migration failed: {e:?}"))
        })
        .await
        .map_err(|e| anyhow::anyhow!("migration task join failed: {e}"))??;

        Ok(())
    }

    async fn get_connection(&self) -> anyhow::Result<CatalogConnection<'_>> {
        Ok(CatalogConnection(self.pool.get().await?))
    }

    pub async fn seed_protocols_if_empty(&self) -> anyhow::Result<usize> {
        let mut conn = self.get_connection().await?;
        let count: i64 = protocols::table.count().get_result(&mut conn).await?;
        if count > 0 {
            return Ok(0);
        }

        for protocol in Protocol::ALL {
            diesel::insert_into(protocols::table)
                .values((
                    protocols::id.eq(protocol.as_str()),
                    protocols::package_id.eq(protocol.type_package_id()),
                    protocols::name.eq(protocol.as_str()),
                    protocols::kind.eq("clmm"),
                    protocols::is_active.eq(true),
                ))
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .await?;
        }

        Ok(Protocol::ALL.len())
    }

    pub async fn upsert_token(
        &self,
        coin_type: &str,
        name: Option<&str>,
        symbol: Option<&str>,
        decimals: i16,
        description: Option<&str>,
        image_url: Option<&str>,
        creator: Option<&str>,
        created_at_ms: Option<i64>,
        first_seen_cp: Option<i64>,
    ) -> anyhow::Result<()> {
        let coin_type = coin_type::normalize(coin_type);
        let mut conn = self.get_connection().await?;

        diesel::sql_query(
            "INSERT INTO tokens (coin_type, name, symbol, decimals, description, image_url, creator, created_at_ms, first_seen_cp, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, now())
             ON CONFLICT (coin_type) DO UPDATE SET
               name = COALESCE(EXCLUDED.name, tokens.name),
               symbol = COALESCE(EXCLUDED.symbol, tokens.symbol),
               decimals = EXCLUDED.decimals,
               description = COALESCE(EXCLUDED.description, tokens.description),
               image_url = COALESCE(EXCLUDED.image_url, tokens.image_url),
               creator = COALESCE(EXCLUDED.creator, tokens.creator),
               created_at_ms = COALESCE(tokens.created_at_ms, EXCLUDED.created_at_ms),
               first_seen_cp = LEAST(COALESCE(tokens.first_seen_cp, EXCLUDED.first_seen_cp), COALESCE(EXCLUDED.first_seen_cp, tokens.first_seen_cp)),
               updated_at = now()",
        )
        .bind::<diesel::sql_types::Text, _>(&coin_type)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(name)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(symbol)
        .bind::<diesel::sql_types::Int2, _>(decimals)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(description)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(image_url)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(creator)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::BigInt>, _>(created_at_ms)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::BigInt>, _>(first_seen_cp)
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    pub async fn ensure_token_stub(&self, coin_type: &str) -> anyhow::Result<()> {
        let coin_type = coin_type::normalize(coin_type);
        if coin_type == coin_type::SUI_COIN_TYPE {
            return Ok(());
        }
        self.upsert_token(&coin_type, None, None, 9, None, None, None, None, None)
            .await
    }

    pub async fn upsert_pool(
        &self,
        pool_id: &str,
        protocol: &str,
        coin_type_a: &str,
        coin_type_b: &str,
        tick_spacing: Option<i32>,
        created_at_ms: Option<i64>,
        created_tx: Option<&str>,
        created_cp: Option<i64>,
    ) -> anyhow::Result<bool> {
        let coin_type_a = coin_type::normalize(coin_type_a);
        let coin_type_b = coin_type::normalize(coin_type_b);

        self.ensure_token_stub(&coin_type_a).await?;
        self.ensure_token_stub(&coin_type_b).await?;

        let mut conn = self.get_connection().await?;
        let inserted = diesel::insert_into(pools::table)
            .values((
                pools::pool_id.eq(pool_id),
                pools::protocol.eq(protocol),
                pools::coin_type_a.eq(&coin_type_a),
                pools::coin_type_b.eq(&coin_type_b),
                pools::tick_spacing.eq(tick_spacing),
                pools::created_at_ms.eq(created_at_ms),
                pools::created_tx.eq(created_tx),
                pools::created_cp.eq(created_cp),
                pools::is_active.eq(true),
            ))
            .on_conflict(pools::pool_id)
            .do_nothing()
            .execute(&mut conn)
            .await?;

        Ok(inserted > 0)
    }

    pub async fn seed_watchlist(&self, coin_type: &str, source: &str) -> anyhow::Result<bool> {
        let coin_type = coin_type::normalize(coin_type);
        if coin_type == coin_type::SUI_COIN_TYPE {
            return Ok(false);
        }

        self.ensure_token_stub(&coin_type).await?;

        let mut conn = self.get_connection().await?;
        let inserted = diesel::insert_into(token_watchlist::table)
            .values((
                token_watchlist::coin_type.eq(&coin_type),
                token_watchlist::source.eq(source),
                token_watchlist::priority.eq(0),
            ))
            .on_conflict(token_watchlist::coin_type)
            .do_nothing()
            .execute(&mut conn)
            .await?;

        Ok(inserted > 0)
    }

    pub async fn get_pool(&self, pool_id: &str) -> anyhow::Result<Option<PoolRow>> {
        let mut conn = self.get_connection().await?;
        let row = pools::table
            .filter(pools::pool_id.eq(pool_id))
            .select((
                pools::pool_id,
                pools::protocol,
                pools::coin_type_a,
                pools::coin_type_b,
            ))
            .first::<(String, String, String, String)>(&mut conn)
            .await
            .optional()?;

        Ok(row.map(|(pool_id, protocol, coin_type_a, coin_type_b)| PoolRow {
            pool_id,
            protocol,
            coin_type_a,
            coin_type_b,
        }))
    }

    pub async fn get_token_decimals(&self, coin_type: &str) -> anyhow::Result<Option<i16>> {
        let coin_type = coin_type::normalize(coin_type);
        let mut conn = self.get_connection().await?;
        tokens::table
            .filter(tokens::coin_type.eq(&coin_type))
            .select(tokens::decimals)
            .first::<i16>(&mut conn)
            .await
            .optional()
            .map_err(Into::into)
    }
}

#[derive(Debug, Clone)]
pub struct PoolRow {
    pub pool_id: String,
    pub protocol: String,
    pub coin_type_a: String,
    pub coin_type_b: String,
}
