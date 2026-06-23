use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clickhouse::Client;
use serde::Serialize;

pub use clickhouse::Client as ClickHouseClient;

const INIT_SQL: &str = include_str!("../../migrations_clickhouse/init.sql");

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

pub async fn run_migrations(config: &ClickHouseConfig) -> Result<()> {
    let client = Client::default().with_url(&config.url);
    for statement in INIT_SQL.split(';') {
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
    pub fee_amount: Option<String>,
    pub sender: Option<String>,
    pub checkpoint_seq: i64,
}

#[derive(clickhouse::Row, Serialize)]
pub struct ChOhlcRow {
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub bucket: DateTime<Utc>,
    pub pool_id: String,
    pub base_coin_type: String,
    pub quote_coin_type: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume_quote: String,
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

pub async fn insert_ohlc(client: &Client, rows: &[ChOhlcRow]) -> Result<()> {
    if rows.is_empty() {
        return Ok(());
    }
    let mut insert = client.insert("ohlc_1m")?;
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
pub struct ChOhlcQueryRow {
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub bucket: DateTime<Utc>,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume_quote: String,
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

pub async fn query_ohlc(
    client: &Client,
    pool_id: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    base_coin_type: Option<&str>,
) -> Result<Vec<ChOhlcQueryRow>> {
    let rows = if let Some(base) = base_coin_type {
        client
            .query(
                "SELECT bucket, open, high, low, close, volume_quote, trade_count
                 FROM ohlc_1m
                 WHERE pool_id = ? AND base_coin_type = ?
                   AND bucket >= ? AND bucket <= ?
                 ORDER BY bucket ASC",
            )
            .bind(pool_id)
            .bind(base)
            .bind(from)
            .bind(to)
            .fetch_all()
            .await?
    } else {
        client
            .query(
                "SELECT bucket, open, high, low, close, volume_quote, trade_count
                 FROM ohlc_1m
                 WHERE pool_id = ?
                   AND bucket >= ? AND bucket <= ?
                 ORDER BY bucket ASC",
            )
            .bind(pool_id)
            .bind(from)
            .bind(to)
            .fetch_all()
            .await?
    };
    Ok(rows)
}
