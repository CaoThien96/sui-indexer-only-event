use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use clickhouse::Client;
use serde::Serialize;

use crate::models::StoredPackageEvent;

pub struct ClickHouseEventsWriter {
    client: Client,
}

#[derive(clickhouse::Row, Serialize)]
struct PackageEventInsertRow {
    event_id_tx_digest: String,
    event_id_seq: i64,
    checkpoint_sequence_number: i64,
    transaction_sequence_in_checkpoint: i32,
    event_sequence_in_transaction: i32,
    package_id: String,
    transaction_module: Option<String>,
    event_type: String,
    sender: Option<String>,
    timestamp_ms: Option<i64>,
    bcs: String,
    json: String,
    parsed_json: Option<String>,
}

impl ClickHouseEventsWriter {
    pub fn from_env() -> Result<Self> {
        let url = std::env::var("CLICKHOUSE_URL")
            .context("CLICKHOUSE_URL must be set (e.g. http://localhost:8123)")?;
        let database =
            std::env::var("CLICKHOUSE_DATABASE").unwrap_or_else(|_| "sui_indexer".to_string());

        let mut client = Client::default().with_url(url).with_database(database);

        if let Ok(user) = std::env::var("CLICKHOUSE_USER") {
            client = client.with_user(user);
        }
        if let Ok(password) = std::env::var("CLICKHOUSE_PASSWORD") {
            client = client.with_password(password);
        }

        Ok(Self { client })
    }

    pub async fn insert_batch(&self, batch: &[StoredPackageEvent]) -> Result<usize> {
        if batch.is_empty() {
            return Ok(0);
        }

        let mut insert = self
            .client
            .insert("package_events")
            .context("failed to start ClickHouse insert")?;

        for event in batch {
            insert
                .write(&PackageEventInsertRow {
                    event_id_tx_digest: event.event_id_tx_digest.clone(),
                    event_id_seq: event.event_id_seq,
                    checkpoint_sequence_number: event.checkpoint_sequence_number,
                    transaction_sequence_in_checkpoint: event
                        .transaction_sequence_in_checkpoint,
                    event_sequence_in_transaction: event.event_sequence_in_transaction,
                    package_id: event.package_id.clone(),
                    transaction_module: event.transaction_module.clone(),
                    event_type: event.event_type.clone(),
                    sender: event.sender.clone(),
                    timestamp_ms: event.timestamp_ms,
                    bcs: STANDARD.encode(&event.bcs),
                    json: event.json.to_string(),
                    parsed_json: event
                        .parsed_json
                        .as_ref()
                        .map(|value| value.to_string()),
                })
                .await
                .context("failed to write row to ClickHouse insert buffer")?;
        }

        insert
            .end()
            .await
            .with_context(|| format!("failed to finalize ClickHouse insert (batch_size={})", batch.len()))?;
        Ok(batch.len())
    }
}
