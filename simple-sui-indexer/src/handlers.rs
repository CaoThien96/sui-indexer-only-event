use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use sui_indexer_alt_framework::pipeline::Processor;
use sui_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;

use crate::models::StoredPackageEvent;
use crate::prefix::matches_any_prefix;
use crate::schema::package_events::dsl::{event_id_seq, event_id_tx_digest, package_events};
use diesel_async::RunQueryDsl;
use sui_indexer_alt_framework::{
    pipeline::sequential::Handler,
    postgres::{Connection, Db},
};
use tracing::{debug, info};

pub struct EventTypeHandler {
    event_type_prefixes: Vec<String>,
}

impl EventTypeHandler {
    pub fn new(event_type_prefixes: Vec<String>) -> Self {
        Self {
            event_type_prefixes,
        }
    }

    fn log_every_n_checkpoints() -> i64 {
        std::env::var("LOG_EVERY_N_CHECKPOINTS")
            .ok()
            .and_then(|value| value.parse::<i64>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(100)
    }
}

#[async_trait::async_trait]
impl Processor for EventTypeHandler {
    const NAME: &'static str = "event_type_handler";

    type Value = StoredPackageEvent;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_seq = checkpoint.summary.sequence_number as i64;
        let mut matched_events = 0usize;
        let mut rows = Vec::new();

        for (tx_idx, tx) in checkpoint.transactions.iter().enumerate() {
            let tx_digest = tx.transaction.digest().to_string();
            let checkpoint_timestamp_ms = Some(checkpoint.summary.timestamp_ms as i64);

            if let Some(events) = &tx.events {
                for (event_idx, event) in events.data.iter().enumerate() {
                    let event_type_str = event.type_.to_string().to_ascii_lowercase();

                    if !matches_any_prefix(&event_type_str, &self.event_type_prefixes) {
                        continue;
                    }

                    let package_id_str = event.package_id.to_string().to_ascii_lowercase();
                    let transaction_module_name = Some(event.transaction_module.to_string());

                    matched_events += 1;
                    rows.push(StoredPackageEvent {
                        event_id_tx_digest: tx_digest.clone(),
                        event_id_seq: event_idx as i64,
                        checkpoint_sequence_number: checkpoint_seq,
                        transaction_sequence_in_checkpoint: tx_idx as i32,
                        event_sequence_in_transaction: event_idx as i32,
                        package_id: package_id_str,
                        transaction_module: transaction_module_name,
                        event_type: event_type_str,
                        sender: Some(event.sender.to_string()),
                        timestamp_ms: checkpoint_timestamp_ms,
                        bcs: event.contents.clone(),
                        json: serde_json::to_value(event).unwrap_or(Value::Null),
                    });
                }
            }
        }

        let log_every_n = Self::log_every_n_checkpoints();
        if checkpoint_seq % log_every_n == 0 {
            info!(
                checkpoint_sequence_number = checkpoint_seq,
                matched_events = matched_events,
                log_every_n_checkpoints = log_every_n,
                "Event prefix indexing progress"
            );
        }

        Ok(rows)
    }
}

#[async_trait::async_trait]
impl Handler for EventTypeHandler {
    type Store = Db;
    type Batch = Vec<Self::Value>;

    fn batch(&self, batch: &mut Self::Batch, values: std::vec::IntoIter<Self::Value>) {
        batch.extend(values);
    }

    async fn commit<'a>(&self, batch: &Self::Batch, conn: &mut Connection<'a>) -> Result<usize> {
        let inserted = diesel::insert_into(package_events)
            .values(batch)
            .on_conflict((event_id_tx_digest, event_id_seq))
            .do_nothing()
            .execute(conn)
            .await?;

        debug!(
            batch_size = batch.len(),
            inserted_rows = inserted,
            "Committed event prefix batch to PostgreSQL"
        );

        Ok(inserted)
    }
}
