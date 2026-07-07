use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clickhouse::Client;
use serde::Serialize;

pub use clickhouse::Client as ClickHouseClient;

const INIT_SQL: &str = include_str!("../../migrations_clickhouse/init.sql");
const MIGRATION_TOKEN_OHLC_USD: &str =
    include_str!("../../migrations_clickhouse/2026-07-05-token-ohlc-usd.sql");

#[derive(Debug, Clone)]
pub struct ClickHouseConfig {
    pub url: String,
    pub database: String,
}

impl ClickHouseConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            url: std::env::var("CLICKHOUSE_URL")
                .unwrap_or_else(|_| "http://localhost:8123".to_string()),
            database: std::env::var("CLICKHOUSE_DATABASE")
                .unwrap_or_else(|_| "sui_metrics".to_string()),
        })
    }

    pub fn client(&self) -> Client {
        Client::default()
            .with_url(&self.url)
            .with_database(&self.database)
    }
}

pub fn create_client(config: &ClickHouseConfig) -> Client {
    config.client()
}

async fn run_sql_script(client: &Client, script: &str) -> Result<()> {
    for statement in script.split(';') {
        let stmt = statement.trim();
        if stmt.is_empty() {
            continue;
        }
        client
            .query(stmt)
            .execute()
            .await
            .with_context(|| format!("clickhouse migration failed: {stmt}"))?;
    }
    Ok(())
}

pub async fn run_migrations(config: &ClickHouseConfig) -> Result<()> {
    let client = Client::default().with_url(&config.url);
    run_sql_script(&client, INIT_SQL).await?;
    run_sql_script(&client, MIGRATION_TOKEN_OHLC_USD).await?;
    Ok(())
}

#[derive(clickhouse::Row, Serialize)]
pub struct ChSwapRow {
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub time: DateTime<Utc>,
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

#[derive(clickhouse::Row, Serialize)]
pub struct ChTokenOhlcUsdRow {
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub bucket: DateTime<Utc>,
    pub base_coin_type: String,
    pub open_usd: String,
    pub high_usd: String,
    pub low_usd: String,
    pub close_usd: String,
    pub volume_usd: String,
    pub trade_count: i32,
}

pub async fn insert_swaps(client: &Client, rows: &[ChSwapRow]) -> Result<()> {
    if rows.is_empty() {
        return Ok(());
    }
    let mut insert = client.insert("swaps_fact")?;
    for row in rows {
        insert.write(row).await?;
    }
    insert.end().await?;
    Ok(())
}

pub async fn insert_token_ohlc_usd(
    client: &Client,
    table: &str,
    rows: &[ChTokenOhlcUsdRow],
) -> Result<()> {
    if rows.is_empty() {
        return Ok(());
    }
    let mut insert = client.insert(table)?;
    for row in rows {
        insert.write(row).await?;
    }
    insert.end().await?;
    Ok(())
}

#[derive(Debug, Clone, clickhouse::Row, serde::Deserialize)]
pub struct ChSwapQueryRow {
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub time: DateTime<Utc>,
    pub tx_digest: String,
    pub event_seq: i32,
    pub protocol: String,
    pub pool_id: String,
    pub amount_base: String,
    pub amount_quote: String,
    pub price_quote_per_base: String,
}

#[derive(Debug, Clone, clickhouse::Row, serde::Deserialize)]
pub struct ChTokenOhlcUsdQueryRow {
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub bucket: DateTime<Utc>,
    pub open_usd: String,
    pub high_usd: String,
    pub low_usd: String,
    pub close_usd: String,
    pub volume_usd: String,
    pub trade_count: i32,
}

pub async fn query_swaps(
    client: &Client,
    base_coin_type: &str,
    pool_id: Option<&str>,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    limit: u64,
) -> Result<Vec<ChSwapQueryRow>> {
    let rows = if let Some(pool_id) = pool_id {
        client
            .query(
                "SELECT time, tx_digest, event_seq, protocol, pool_id,
                        amount_base, amount_quote, price_quote_per_base
                 FROM swaps_fact
                 WHERE base_coin_type = ? AND pool_id = ?
                   AND time >= ? AND time <= ?
                 ORDER BY time DESC
                 LIMIT ?",
            )
            .bind(base_coin_type)
            .bind(pool_id)
            .bind(from)
            .bind(to)
            .bind(limit)
            .fetch_all()
            .await?
    } else {
        client
            .query(
                "SELECT time, tx_digest, event_seq, protocol, pool_id,
                        amount_base, amount_quote, price_quote_per_base
                 FROM swaps_fact
                 WHERE base_coin_type = ?
                   AND time >= ? AND time <= ?
                 ORDER BY time DESC
                 LIMIT ?",
            )
            .bind(base_coin_type)
            .bind(from)
            .bind(to)
            .bind(limit)
            .fetch_all()
            .await?
    };
    Ok(rows)
}

pub async fn query_token_ohlc_usd(
    client: &Client,
    table: &str,
    base_coin_type: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Result<Vec<ChTokenOhlcUsdQueryRow>> {
    let sql = format!(
        "SELECT bucket, open_usd, high_usd, low_usd, close_usd, volume_usd, trade_count
         FROM {table}
         WHERE base_coin_type = ?
           AND bucket >= ? AND bucket <= ?
         ORDER BY bucket ASC"
    );
    let rows = client
        .query(&sql)
        .bind(base_coin_type)
        .bind(from)
        .bind(to)
        .fetch_all()
        .await?;
    Ok(rows)
}
