use anyhow::Result;
use event_bindings::coin_metadata::{self, DecodedCoinMetadata};
use indexer_store::{CompositeStore, FactTopic, MessageEnvelope};
use std::sync::Arc;
use sui_indexer_alt_framework::{
    pipeline::Processor, pipeline::sequential::Handler, store::Store,
    types::full_checkpoint_content::Checkpoint,
};
use sui_indexer_alt_framework::types::transaction::TransactionDataAPI;
use tracing::info;

use super::common::{AppMetrics, RawTokenMetadataFact, build_token_metadata_envelope, token_metadata_partition_key};

pub const NAME: &str = "token_metadata";

#[derive(Clone)]
pub struct TokenMetadataHandler {
    metrics: Arc<AppMetrics>,
}

impl TokenMetadataHandler {
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
impl Processor for TokenMetadataHandler {
    const NAME: &'static str = NAME;

    type Value = RawTokenMetadataFact;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_seq = checkpoint.summary.sequence_number;
        let timestamp_ms = checkpoint.summary.timestamp_ms;
        let mut rows = Vec::new();
        let mut matched = 0usize;

        for (tx_idx, tx) in checkpoint.transactions.iter().enumerate() {
            let tx_digest = tx.transaction.digest().to_string();
            let creator = tx.transaction.sender().to_string();

            for obj in tx.created_objects(&checkpoint.object_set) {
                let Some(move_obj) = obj.data.try_as_move() else {
                    continue;
                };
                let type_str = move_obj.type_().to_canonical_string(true);
                if !coin_metadata::is_coin_metadata_type(&type_str) {
                    continue;
                }

                matched += 1;
                match coin_metadata::decode_coin_metadata_object(&type_str, move_obj.contents()) {
                    Ok(decoded) => {
                        self.metrics
                            .objects_matched
                            .with_label_values(&[NAME, "coin_metadata"])
                            .inc();
                        rows.push(raw_token_metadata_fact(
                            &decoded,
                            checkpoint_seq,
                            timestamp_ms,
                            &tx_digest,
                            tx_idx as u32,
                            &creator,
                        ));
                    }
                    Err(error) => {
                        self.metrics
                            .decode_errors
                            .with_label_values(&[NAME, "coin_metadata", &type_str])
                            .inc();
                        tracing::warn!(
                            pipeline = NAME,
                            type_str = %type_str,
                            error = %error,
                            "Failed to decode CoinMetadata object"
                        );
                    }
                }
            }
        }

        let log_every_n = Self::log_every_n_checkpoints();
        if checkpoint_seq % log_every_n == 0 {
            info!(
                pipeline = NAME,
                checkpoint_sequence_number = checkpoint_seq,
                matched_objects = matched,
                rows = rows.len(),
                log_every_n_checkpoints = log_every_n,
                "Token metadata pipeline checkpoint progress"
            );
        }

        Ok(rows)
    }
}

#[async_trait::async_trait]
impl Handler for TokenMetadataHandler {
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

        let topic = FactTopic::TokenMetadataRaw;
        let envelopes: Vec<MessageEnvelope> = batch
            .iter()
            .map(build_token_metadata_envelope)
            .collect();

        let published = conn
            .publish_facts(topic, &envelopes, token_metadata_partition_key)
            .await?;

        self.metrics
            .kafka_rows_published
            .with_label_values(&[NAME, topic.as_str()])
            .inc_by(published as u64);

        info!(
            pipeline = NAME,
            batch_size = batch.len(),
            published,
            "Committed token metadata batch to Kafka"
        );

        Ok(published)
    }
}

fn raw_token_metadata_fact(
    decoded: &DecodedCoinMetadata,
    checkpoint_sequence_number: u64,
    created_at_ms: u64,
    tx_digest: &str,
    transaction_sequence_in_checkpoint: u32,
    creator: &str,
) -> RawTokenMetadataFact {
    RawTokenMetadataFact {
        coin_type: decoded.coin_type.clone(),
        name: decoded.name.clone(),
        symbol: decoded.symbol.clone(),
        decimals: decoded.decimals,
        description: decoded.description.clone(),
        image_url: decoded.image_url.clone(),
        creator: creator.to_string(),
        object_id: decoded.object_id.clone(),
        created_at_ms,
        checkpoint_sequence_number,
        tx_digest: tx_digest.to_string(),
        transaction_sequence_in_checkpoint,
    }
}
