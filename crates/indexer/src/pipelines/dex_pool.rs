use anyhow::Result;
use event_bindings::pool_id::extract_pool_create_fields;
use indexer_store::{CompositeStore, FactTopic, MessageEnvelope};
use std::sync::Arc;
use sui_indexer_alt_framework::{
    pipeline::Processor, pipeline::sequential::Handler, store::Store,
    types::full_checkpoint_content::Checkpoint,
};
use tracing::info;

use super::common::{
    AppMetrics, RawPoolFact, build_pool_envelope, classify_pool_create, decode_event,
    iterate_checkpoint_events, pool_partition_key, raw_pool_fact,
};

pub const NAME: &str = "dex_pool";

#[derive(Clone)]
pub struct DexPoolHandler {
    metrics: Arc<AppMetrics>,
}

impl DexPoolHandler {
    pub fn new(metrics: Arc<AppMetrics>) -> Self {
        Self { metrics }
    }

    fn log_every_n_checkpoints() -> u64 {
        std::env::var("LOG_EVERY_N_CHECKPOINTS")
            .ok()
            .and_then(|value| value.parse().ok())
            .filter(|value| *value > 0)
            .unwrap_or(100)
    }
}

#[async_trait::async_trait]
impl Processor for DexPoolHandler {
    const NAME: &'static str = NAME;

    type Value = RawPoolFact;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_seq = checkpoint.summary.sequence_number;
        let mut rows = Vec::new();
        let mut matched = 0usize;

        for event in iterate_checkpoint_events(checkpoint) {
            let Some(protocol) = classify_pool_create(&event.event_type) else {
                continue;
            };
            matched += 1;
            let parsed_json = decode_event(&self.metrics, NAME, &event, protocol)?;
            let fields = extract_pool_create_fields(protocol, &parsed_json)?;
            rows.push(raw_pool_fact(&event, protocol, parsed_json, fields));
        }

        let log_every_n = Self::log_every_n_checkpoints();
        if checkpoint_seq % log_every_n == 0 {
            info!(
                pipeline = NAME,
                checkpoint_sequence_number = checkpoint_seq,
                matched_events = matched,
                rows = rows.len(),
                log_every_n_checkpoints = log_every_n,
                "DEX pool pipeline checkpoint progress"
            );
        }

        Ok(rows)
    }
}

#[async_trait::async_trait]
impl Handler for DexPoolHandler {
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

        let topic = FactTopic::PoolRaw;
        let envelopes: Vec<MessageEnvelope> =
            batch.iter().map(build_pool_envelope).collect();

        let published = conn
            .publish_facts(topic, &envelopes, pool_partition_key)
            .await?;

        self.metrics
            .kafka_rows_published
            .with_label_values(&[NAME, topic.as_str()])
            .inc_by(published as u64);

        info!(
            pipeline = NAME,
            batch_size = batch.len(),
            published,
            "Committed pool fact batch to Kafka"
        );

        Ok(published)
    }
}
