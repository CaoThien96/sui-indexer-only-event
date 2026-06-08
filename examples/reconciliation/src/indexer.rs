use std::collections::HashSet;

use anyhow::Result;
use sqlx::PgPool;

use crate::event_key::EventKey;

#[allow(dead_code)]
pub async fn count_events_in_window(
    pool: &PgPool,
    event_type: &str,
    start_ms: i64,
    end_ms: i64,
) -> Result<i64> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM package_events \
         WHERE event_type ILIKE $1 \
           AND timestamp_ms >= $2 \
           AND timestamp_ms <= $3",
    )
    .bind(event_type)
    .bind(start_ms)
    .bind(end_ms)
    .fetch_one(pool)
    .await?;

    Ok(count)
}

pub async fn list_event_keys_in_window(
    pool: &PgPool,
    event_type: &str,
    start_ms: i64,
    end_ms: i64,
) -> Result<HashSet<EventKey>> {
    let rows: Vec<(String, i64)> = sqlx::query_as(
        "SELECT event_id_tx_digest, event_id_seq FROM package_events \
         WHERE event_type ILIKE $1 \
           AND timestamp_ms >= $2 \
           AND timestamp_ms <= $3",
    )
    .bind(event_type)
    .bind(start_ms)
    .bind(end_ms)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(tx_digest, event_seq)| EventKey {
            tx_digest,
            event_seq,
        })
        .collect())
}

pub async fn max_indexed_timestamp_ms(pool: &PgPool) -> Result<Option<i64>> {
    let value: Option<i64> = sqlx::query_scalar("SELECT MAX(timestamp_ms) FROM package_events")
        .fetch_one(pool)
        .await?;

    Ok(value)
}
