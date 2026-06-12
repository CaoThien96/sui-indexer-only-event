use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use prometheus::{IntCounter, Registry};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use tracing::debug;

/// Kafka topics for raw fact streams (Phase 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactTopic {
    SwapRaw,
    PoolRaw,
    TokenMetadataRaw,
}

impl FactTopic {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SwapRaw => "dex.swap.raw.v1",
            Self::PoolRaw => "dex.pool.raw.v1",
            Self::TokenMetadataRaw => "token.metadata.raw.v1",
        }
    }
}

/// Message envelope per docs/04-data-contracts.md §1.
#[derive(Debug, Clone, Serialize)]
pub struct MessageEnvelope {
    pub schema_version: u32,
    pub message_id: String,
    pub produced_at_ms: u64,
    pub payload: Value,
}

impl MessageEnvelope {
    pub fn new(message_id_key: &str, payload: Value) -> Self {
        Self {
            schema_version: 1,
            message_id: compute_message_id(message_id_key),
            produced_at_ms: now_ms(),
            payload,
        }
    }
}

/// SHA-256 hex digest used as `message_id`.
pub fn compute_message_id(key: &str) -> String {
    let digest = Sha256::digest(key.as_bytes());
    hex::encode(digest)
}

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX epoch")
        .as_millis() as u64
}

#[derive(Clone)]
pub struct KafkaFactWriter {
    producer: Arc<FutureProducer>,
    client_id: String,
    pub produce_errors: IntCounter,
}

impl KafkaFactWriter {
    pub fn new(brokers: &str, client_id: &str, registry: &Registry) -> Result<Self> {
        let produce_errors = IntCounter::new(
            "indexer_kafka_produce_errors_total",
            "Kafka produce failures in BYOS commit path",
        )
        .context("failed to create kafka produce errors counter")?;
        registry
            .register(Box::new(produce_errors.clone()))
            .context("failed to register kafka produce errors counter")?;

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("client.id", client_id)
            .set("acks", "all")
            .set("enable.idempotence", "true")
            .set("message.timeout.ms", "30000")
            .create()
            .context("failed to create Kafka producer")?;

        Ok(Self {
            producer: Arc::new(producer),
            client_id: client_id.to_string(),
            produce_errors,
        })
    }

    /// Publish a batch of envelopes to the given topic.
    pub async fn publish(
        &self,
        topic: FactTopic,
        records: &[MessageEnvelope],
        partition_key_fn: impl Fn(&MessageEnvelope) -> String,
    ) -> Result<usize> {
        if records.is_empty() {
            return Ok(0);
        }

        let topic_name = topic.as_str();
        let mut published = 0usize;

        for envelope in records {
            let payload =
                serde_json::to_string(envelope).context("failed to serialize message envelope")?;
            let key = partition_key_fn(envelope);

            let record = FutureRecord::to(topic_name).key(&key).payload(&payload);

            match self
                .producer
                .send(record, std::time::Duration::from_secs(30))
                .await
            {
                Ok(_) => published += 1,
                Err((e, _)) => {
                    self.produce_errors.inc();
                    return Err(e).context("Kafka produce failed");
                }
            }
        }

        debug!(
            topic = topic_name,
            count = published,
            client_id = %self.client_id,
            "Published Kafka fact batch"
        );

        Ok(published)
    }
}
