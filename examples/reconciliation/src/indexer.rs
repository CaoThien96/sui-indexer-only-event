use std::collections::HashSet;

use anyhow::{Context, Result};
use clickhouse::Client;

use crate::event_key::EventKey;

#[allow(dead_code)]
pub async fn count_events_in_window(
    client: &Client,
    event_type: &str,
    start_ms: i64,
    end_ms: i64,
) -> Result<i64> {
    let count: u64 = client
        .query(
            "SELECT count() FROM package_events FINAL \
             WHERE event_type ILIKE ? \
               AND timestamp_ms >= ? \
               AND timestamp_ms <= ?",
        )
        .bind(event_type)
        .bind(start_ms)
        .bind(end_ms)
        .fetch_one()
        .await
        .context("ClickHouse count query failed")?;

    Ok(count as i64)
}

#[derive(clickhouse::Row, serde::Deserialize)]
struct EventKeyRow {
    event_id_tx_digest: String,
    event_id_seq: i64,
}

pub async fn list_event_keys_in_window(
    client: &Client,
    event_type: &str,
    start_ms: i64,
    end_ms: i64,
) -> Result<HashSet<EventKey>> {
    let rows = client
        .query(
            "SELECT event_id_tx_digest, event_id_seq FROM package_events FINAL \
             WHERE event_type ILIKE ? \
               AND timestamp_ms >= ? \
               AND timestamp_ms <= ?",
        )
        .bind(event_type)
        .bind(start_ms)
        .bind(end_ms)
        .fetch_all::<EventKeyRow>()
        .await
        .context("ClickHouse list event keys failed")?;

    Ok(rows
        .into_iter()
        .map(|row| EventKey {
            tx_digest: row.event_id_tx_digest,
            event_seq: row.event_id_seq,
        })
        .collect())
}

pub async fn max_indexed_timestamp_ms(client: &Client) -> Result<Option<i64>> {
    #[derive(clickhouse::Row, serde::Deserialize)]
    struct MaxRow {
        max_ts: Option<i64>,
    }

    let row = client
        .query("SELECT max(timestamp_ms) AS max_ts FROM package_events FINAL")
        .fetch_one::<MaxRow>()
        .await
        .context("ClickHouse max timestamp query failed")?;

    Ok(row.max_ts)
}
