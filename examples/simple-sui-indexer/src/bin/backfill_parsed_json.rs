//! DEPRECATED: package_events moved to ClickHouse. Re-ingest with new bindings
//! via ReplacingMergeTree inserts instead of PostgreSQL UPDATE.
use anyhow::{bail, Context, Result};
use clap::Parser;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use std::collections::HashMap;
use simple_sui_indexer::bootstrap;
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "backfill-parsed-json")]
struct Args {
    /// Rows per fetch/update batch.
    #[arg(long, default_value_t = 5000)]
    batch_size: i64,

    /// Stop after processing this many rows (0 = no limit).
    #[arg(long, default_value_t = 0)]
    limit: i64,
}

#[derive(QueryableByName)]
struct Row {
    #[diesel(sql_type = diesel::sql_types::Text)]
    event_id_tx_digest: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    event_id_seq: i64,
    #[diesel(sql_type = diesel::sql_types::Text)]
    event_type: String,
    #[diesel(sql_type = diesel::sql_types::Bytea)]
    bcs: Vec<u8>,
}

struct PendingUpdate {
    event_id_tx_digest: String,
    event_id_seq: i64,
    parsed_json: serde_json::Value,
}

#[tokio::main]
async fn main() -> Result<()> {
    let dotenv = bootstrap::load_dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    bootstrap::log_dotenv_load(&dotenv);

    let args = Args::parse();
    let _ = args;
    bail!(
        "backfill-parsed-json is deprecated: package_events lives in ClickHouse. \
         Decode at ingest; for new bindings re-insert rows with ReplacingMergeTree."
    );

    #[allow(unreachable_code)]
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    let mut conn = AsyncPgConnection::establish(&database_url).await?;
    info!(
        batch_size = args.batch_size,
        limit = args.limit,
        "Starting parsed_json backfill"
    );

    let mut total_updated = 0usize;
    let mut total_failed = 0usize;
    let mut failures_by_type: HashMap<String, usize> = HashMap::new();
    let mut last_tx: Option<String> = None;
    let mut last_seq: i64 = -1;

    loop {
        if args.limit > 0 && total_updated + total_failed >= args.limit as usize {
            break;
        }

        let batch_limit = if args.limit > 0 {
            args.batch_size.min(args.limit - (total_updated + total_failed) as i64)
        } else {
            args.batch_size
        };

        if batch_limit <= 0 {
            break;
        }

        let rows: Vec<Row> = if let Some(ref tx_digest) = last_tx {
            diesel::sql_query(
                "SELECT event_id_tx_digest, event_id_seq, event_type, bcs \
                 FROM package_events \
                 WHERE parsed_json IS NULL \
                   AND (event_id_tx_digest, event_id_seq) > ($1, $2) \
                 ORDER BY event_id_tx_digest, event_id_seq \
                 LIMIT $3",
            )
            .bind::<diesel::sql_types::Text, _>(tx_digest)
            .bind::<diesel::sql_types::BigInt, _>(last_seq)
            .bind::<diesel::sql_types::BigInt, _>(batch_limit)
            .load(&mut conn)
            .await?
        } else {
            diesel::sql_query(
                "SELECT event_id_tx_digest, event_id_seq, event_type, bcs \
                 FROM package_events \
                 WHERE parsed_json IS NULL \
                 ORDER BY event_id_tx_digest, event_id_seq \
                 LIMIT $1",
            )
            .bind::<diesel::sql_types::BigInt, _>(batch_limit)
            .load(&mut conn)
            .await?
        };

        if rows.is_empty() {
            break;
        }

        let mut pending = Vec::new();
        for row in &rows {
            match event_bindings::decode_parsed_json(&row.event_type, &row.bcs) {
                Ok(parsed) => pending.push(PendingUpdate {
                    event_id_tx_digest: row.event_id_tx_digest.clone(),
                    event_id_seq: row.event_id_seq,
                    parsed_json: parsed,
                }),
                Err(error) => {
                    total_failed += 1;
                    *failures_by_type.entry(row.event_type.clone()).or_default() += 1;
                    warn!(
                        event_type = %row.event_type,
                        event_id_tx_digest = %row.event_id_tx_digest,
                        event_id_seq = row.event_id_seq,
                        error = %error,
                        "Failed to decode row; skipping"
                    );
                }
            }
        }

        if !pending.is_empty() {
            apply_batch_update(&mut conn, &pending).await?;
            total_updated += pending.len();
        }

        if let Some(last) = rows.last() {
            last_tx = Some(last.event_id_tx_digest.clone());
            last_seq = last.event_id_seq;
        }

        info!(
            total_updated,
            total_failed,
            last_event_id_tx_digest = ?last_tx,
            last_event_id_seq = last_seq,
            "Backfill progress"
        );
    }

    info!(
        total_updated,
        total_failed,
        failures_by_type = ?failures_by_type,
        "Backfill complete"
    );

    if total_failed > 0 {
        warn!(
            total_failed,
            "Some rows failed to decode; review failures_by_type above"
        );
    }

    Ok(())
}

async fn apply_batch_update(
    conn: &mut AsyncPgConnection,
    batch: &[PendingUpdate],
) -> Result<()> {
    let tx_digests: Vec<&str> = batch.iter().map(|row| row.event_id_tx_digest.as_str()).collect();
    let seqs: Vec<i64> = batch.iter().map(|row| row.event_id_seq).collect();
    let parsed: Vec<serde_json::Value> = batch.iter().map(|row| row.parsed_json.clone()).collect();

    diesel::sql_query(
        "UPDATE package_events AS pe \
         SET parsed_json = batch.parsed_json \
         FROM ( \
           SELECT \
             unnest($1::text[]) AS event_id_tx_digest, \
             unnest($2::bigint[]) AS event_id_seq, \
             unnest($3::jsonb[]) AS parsed_json \
         ) AS batch \
         WHERE pe.event_id_tx_digest = batch.event_id_tx_digest \
           AND pe.event_id_seq = batch.event_id_seq",
    )
    .bind::<diesel::sql_types::Array<diesel::sql_types::Text>, _>(&tx_digests)
    .bind::<diesel::sql_types::Array<diesel::sql_types::BigInt>, _>(&seqs)
    .bind::<diesel::sql_types::Array<diesel::sql_types::Jsonb>, _>(&parsed)
    .execute(conn)
    .await?;

    Ok(())
}
