use anyhow::Result;
use indexer_store::{CompositeStore, FactTopic, MessageEnvelope};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use sui_indexer_alt_framework::{
    FieldCount, pipeline::Processor, pipeline::sequential::Handler, store::Store,
    types::full_checkpoint_content::Checkpoint,
};
use tracing::info;

#[derive(Debug, Clone, Serialize, FieldCount)]
pub struct StubHeartbeat {
    pub kind: String,
    pub checkpoint_sequence_number: u64,
    pub event_count: u64,
    pub timestamp_ms: u64,
}

#[derive(Clone, Default)]
pub struct StubEventHandler;

impl StubEventHandler {
    fn log_every_n_checkpoints() -> u64 {
        std::env::var("LOG_EVERY_N_CHECKPOINTS")
            .ok()
            .and_then(|value| value.parse().ok())
            .filter(|value| *value > 0)
            .unwrap_or(100)
    }
}

#[async_trait::async_trait]
impl Processor for StubEventHandler {
    const NAME: &'static str = "stub_events";

    type Value = StubHeartbeat;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_seq = checkpoint.summary.sequence_number;
        let event_count = checkpoint
            .transactions
            .iter()
            .filter_map(|tx| tx.events.as_ref().map(|e| e.data.len()))
            .sum::<usize>() as u64;

        let log_every_n = Self::log_every_n_checkpoints();
        if checkpoint_seq % log_every_n == 0 {
            info!(
                pipeline = Self::NAME,
                checkpoint_sequence_number = checkpoint_seq,
                event_count,
                log_every_n_checkpoints = log_every_n,
                "Stub pipeline checkpoint progress"
            );
        }

        Ok(vec![StubHeartbeat {
            kind: "checkpoint_heartbeat".to_string(),
            checkpoint_sequence_number: checkpoint_seq,
            event_count,
            timestamp_ms: checkpoint.summary.timestamp_ms,
        }])
    }
}

#[async_trait::async_trait]
impl Handler for StubEventHandler {
    type Store = CompositeStore;
    type Batch = Vec<Self::Value>;

    fn batch(&self, batch: &mut Self::Batch, values: std::vec::IntoIter<Self::Value>) {
        batch.extend(values);
    }

    async fn commit<'a>(
        &self,
        batch: &Self::Batch,
        conn: &mut <Self::Store as Store>::Connection<'a>,
    ) -> Result<usize> {
        if batch.is_empty() {
            return Ok(0);
        }

        let topic = FactTopic::SwapRaw;
        let envelopes: Vec<MessageEnvelope> = batch
            .iter()
            .map(|row| {
                let message_id_key = format!(
                    "checkpoint:{}:{}",
                    row.checkpoint_sequence_number,
                    topic.as_str()
                );
                MessageEnvelope::new(&message_id_key, json!(row))
            })
            .collect();

        let published = conn
            .publish_facts(topic, &envelopes, |envelope| {
                envelope
                    .payload
                    .get("checkpoint_sequence_number")
                    .and_then(|v| v.as_u64())
                    .map(|seq| seq.to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            })
            .await?;

        info!(
            pipeline = Self::NAME,
            batch_size = batch.len(),
            published,
            "Committed stub heartbeat batch to Kafka"
        );

        Ok(published)
    }
}
