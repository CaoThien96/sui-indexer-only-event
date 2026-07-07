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

/// Serializes catalog migrations across concurrent container restarts.
const CATALOG_MIGRATION_LOCK_ID: i64 = 748_301_923;

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
        let mut conn = self.pool.dedicated_connection().await?;
        diesel::sql_query("SELECT pg_advisory_lock($1)")
            .bind::<diesel::sql_types::BigInt, _>(CATALOG_MIGRATION_LOCK_ID)
            .execute(&mut conn)
            .await
            .map_err(|e| anyhow::anyhow!("catalog migration lock failed: {e:?}"))?;

        let mut wrapper: AsyncConnectionWrapper<AsyncPgConnection> =
            AsyncConnectionWrapper::from(conn);

        tokio::task::spawn_blocking(move || {
            match wrapper.run_pending_migrations(MIGRATIONS) {
                Ok(_) => Ok(()),
                Err(e) => {
                    let msg = format!("{e:?}");
                    let display = e.to_string();
                    if (msg.contains("UniqueViolation") || display.contains("UniqueViolation"))
                        && (msg.contains("__diesel_schema_migrations")
                            || display.contains("__diesel_schema_migrations"))
                    {
                        tracing::info!(
                            "catalog migration already recorded by concurrent instance; continuing"
                        );
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("catalog migration failed: {e:?}"))
                    }
                }
            }
            // dedicated connection dropped with wrapper → advisory lock released
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
        metadata_source: &str,
    ) -> anyhow::Result<()> {
        let coin_type = coin_type::normalize(coin_type);
        let mut conn = self.get_connection().await?;

        diesel::sql_query(
            "INSERT INTO tokens (coin_type, name, symbol, decimals, description, image_url, creator, created_at_ms, first_seen_cp, metadata_source, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, now())
             ON CONFLICT (coin_type) DO UPDATE SET
               name = COALESCE(EXCLUDED.name, tokens.name),
               symbol = COALESCE(EXCLUDED.symbol, tokens.symbol),
               decimals = EXCLUDED.decimals,
               description = COALESCE(EXCLUDED.description, tokens.description),
               image_url = COALESCE(EXCLUDED.image_url, tokens.image_url),
               creator = COALESCE(EXCLUDED.creator, tokens.creator),
               created_at_ms = COALESCE(tokens.created_at_ms, EXCLUDED.created_at_ms),
               first_seen_cp = LEAST(COALESCE(tokens.first_seen_cp, EXCLUDED.first_seen_cp), COALESCE(EXCLUDED.first_seen_cp, tokens.first_seen_cp)),
               metadata_source = CASE
                 WHEN tokens.metadata_source = 'indexer_metadata' THEN 'indexer_metadata'
                 ELSE COALESCE(EXCLUDED.metadata_source, tokens.metadata_source)
               END,
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
        .bind::<diesel::sql_types::Text, _>(metadata_source)
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    pub async fn ensure_token_stub(&self, coin_type: &str) -> anyhow::Result<()> {
        let coin_type = coin_type::normalize(coin_type);
        if coin_type == coin_type::SUI_COIN_TYPE {
            return Ok(());
        }
        let mut conn = self.get_connection().await?;
        diesel::sql_query(
            "INSERT INTO tokens (coin_type, decimals, metadata_source, updated_at)
             VALUES ($1, 9, 'stub', now())
             ON CONFLICT (coin_type) DO NOTHING",
        )
        .bind::<diesel::sql_types::Text, _>(&coin_type)
        .execute(&mut conn)
        .await?;
        Ok(())
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
        let (coin_type_a, coin_type_b) = self.prepare_pool_coins(coin_type_a, coin_type_b).await?;

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
                pools::discovery_source.eq("pool_create"),
            ))
            .on_conflict(pools::pool_id)
            .do_nothing()
            .execute(&mut conn)
            .await?;

        Ok(inserted > 0)
    }

    pub async fn upsert_pool_hydrated(
        &self,
        pool_id: &str,
        protocol: &str,
        coin_type_a: &str,
        coin_type_b: &str,
        first_seen_cp: Option<i64>,
        first_seen_ms: Option<i64>,
    ) -> anyhow::Result<()> {
        let (coin_type_a, coin_type_b) = self.prepare_pool_coins(coin_type_a, coin_type_b).await?;

        let mut conn = self.get_connection().await?;
        diesel::sql_query(
            "INSERT INTO pools (pool_id, protocol, coin_type_a, coin_type_b, created_at_ms, created_cp, is_active, discovery_source)
             VALUES ($1, $2, $3, $4, $5, $6, true, 'swap_hydration')
             ON CONFLICT (pool_id) DO NOTHING",
        )
        .bind::<diesel::sql_types::Text, _>(pool_id)
        .bind::<diesel::sql_types::Text, _>(protocol)
        .bind::<diesel::sql_types::Text, _>(&coin_type_a)
        .bind::<diesel::sql_types::Text, _>(&coin_type_b)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::BigInt>, _>(first_seen_ms)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::BigInt>, _>(first_seen_cp)
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    async fn prepare_pool_coins(
        &self,
        coin_type_a: &str,
        coin_type_b: &str,
    ) -> anyhow::Result<(String, String)> {
        let coin_type_a = coin_type::normalize(coin_type_a);
        let coin_type_b = coin_type::normalize(coin_type_b);
        self.ensure_token_stub(&coin_type_a).await?;
        self.ensure_token_stub(&coin_type_b).await?;
        Ok((coin_type_a, coin_type_b))
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

    pub async fn get_token(&self, coin_type: &str) -> anyhow::Result<Option<TokenRow>> {
        let coin_type = coin_type::normalize(coin_type);
        let mut conn = self.get_connection().await?;
        let row = tokens::table
            .filter(tokens::coin_type.eq(&coin_type))
            .select((
                tokens::coin_type,
                tokens::name,
                tokens::symbol,
                tokens::decimals,
                tokens::image_url,
                tokens::metadata_source,
            ))
            .first::<(
                String,
                Option<String>,
                Option<String>,
                i16,
                Option<String>,
                String,
            )>(&mut conn)
            .await
            .optional()?;

        Ok(row.map(
            |(coin_type, name, symbol, decimals, image_url, metadata_source)| TokenRow {
                coin_type,
                name,
                symbol,
                decimals,
                image_url,
                metadata_source,
            },
        ))
    }

    pub async fn count_pools_for_token(&self, coin_type: &str) -> anyhow::Result<i64> {
        let variants = coin_type::pool_lookup_variants(coin_type);
        let mut conn = self.get_connection().await?;
        let count = pools::table
            .filter(
                pools::coin_type_a
                    .eq_any(&variants)
                    .or(pools::coin_type_b.eq_any(&variants)),
            )
            .filter(pools::is_active.eq(true))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;
        Ok(count)
    }

    pub async fn list_pools_for_token(
        &self,
        coin_type: &str,
        limit: i64,
    ) -> anyhow::Result<Vec<PoolRow>> {
        let variants = coin_type::pool_lookup_variants(coin_type);
        let mut conn = self.get_connection().await?;
        let rows = pools::table
            .filter(
                pools::coin_type_a
                    .eq_any(&variants)
                    .or(pools::coin_type_b.eq_any(&variants)),
            )
            .filter(pools::is_active.eq(true))
            .select((
                pools::pool_id,
                pools::protocol,
                pools::coin_type_a,
                pools::coin_type_b,
            ))
            .limit(limit)
            .load::<(String, String, String, String)>(&mut conn)
            .await?;

        Ok(rows
            .into_iter()
            .map(|(pool_id, protocol, coin_type_a, coin_type_b)| PoolRow {
                pool_id,
                protocol,
                coin_type_a,
                coin_type_b,
            })
            .collect())
    }

    pub async fn list_tokens(
        &self,
        q: Option<&str>,
        limit: i64,
        cursor: Option<&str>,
    ) -> anyhow::Result<Vec<TokenListRow>> {
        let mut conn = self.get_connection().await?;
        let pattern = q.map(|s| format!("%{s}%"));
        let (cursor_priority, cursor_cp, cursor_coin_type) = parse_token_list_cursor(cursor);

        #[derive(diesel::QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Text)]
            coin_type: String,
            #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
            name: Option<String>,
            #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
            symbol: Option<String>,
            #[diesel(sql_type = diesel::sql_types::Int2)]
            decimals: i16,
            #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
            image_url: Option<String>,
            #[diesel(sql_type = diesel::sql_types::Int4)]
            priority: i32,
            #[diesel(sql_type = diesel::sql_types::Int8)]
            first_seen_cp: i64,
        }

        let rows: Vec<Row> = diesel::sql_query(
            "SELECT t.coin_type, t.name, t.symbol, t.decimals, t.image_url,
                    COALESCE(w.priority, 0) AS priority,
                    COALESCE(t.first_seen_cp, 0) AS first_seen_cp
             FROM tokens t
             LEFT JOIN token_watchlist w ON w.coin_type = t.coin_type
             WHERE ($1::text IS NULL OR t.coin_type ILIKE $1 OR t.symbol ILIKE $1 OR t.name ILIKE $1)
               AND (
                 $2::int IS NULL
                 OR COALESCE(w.priority, 0) < $2
                 OR (COALESCE(w.priority, 0) = $2 AND COALESCE(t.first_seen_cp, 0) < $3)
                 OR (
                   COALESCE(w.priority, 0) = $2
                   AND COALESCE(t.first_seen_cp, 0) = $3
                   AND t.coin_type > $4
                 )
               )
             ORDER BY COALESCE(w.priority, 0) DESC, COALESCE(t.first_seen_cp, 0) DESC, t.coin_type ASC
             LIMIT $5",
        )
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(pattern)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Int4>, _>(cursor_priority)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Int8>, _>(cursor_cp)
        .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(cursor_coin_type)
        .bind::<diesel::sql_types::Int8, _>(limit)
        .load(&mut conn)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| TokenListRow {
                coin_type: r.coin_type,
                name: r.name,
                symbol: r.symbol,
                decimals: r.decimals,
                image_url: r.image_url,
                priority: r.priority,
                first_seen_cp: r.first_seen_cp,
            })
            .collect())
    }
}

fn parse_token_list_cursor(cursor: Option<&str>) -> (Option<i32>, Option<i64>, Option<String>) {
    let Some(cursor) = cursor else {
        return (None, None, None);
    };
    let mut parts = cursor.splitn(3, '|');
    let priority = parts.next().and_then(|p| p.parse().ok());
    let cp = parts.next().and_then(|p| p.parse().ok());
    let coin_type = parts.next().map(str::to_string);
    if priority.is_none() || cp.is_none() || coin_type.is_none() {
        return (None, None, None);
    }
    (priority, cp, coin_type)
}

#[derive(Debug, Clone)]
pub struct TokenListRow {
    pub coin_type: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: i16,
    pub image_url: Option<String>,
    pub priority: i32,
    pub first_seen_cp: i64,
}

#[derive(Debug, Clone)]
pub struct TokenRow {
    pub coin_type: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: i16,
    pub image_url: Option<String>,
    pub metadata_source: String,
}

#[derive(Debug, Clone)]
pub struct PoolRow {
    pub pool_id: String,
    pub protocol: String,
    pub coin_type_a: String,
    pub coin_type_b: String,
}
