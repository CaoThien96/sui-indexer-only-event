use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use prometheus::{IntCounter, Registry};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::{Message, Offset, TopicPartitionList};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use tracing::debug;

/// Kafka topics for raw fact streams (Phase 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactTopic {
    SwapRaw,
    PoolRaw,
    TokenMetadataRaw,
    SwapNormalized,
}

impl FactTopic {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SwapRaw => "dex.swap.raw.v1",
            Self::PoolRaw => "dex.pool.raw.v1",
            Self::TokenMetadataRaw => "token.metadata.raw.v1",
            Self::SwapNormalized => "dex.swap.normalized.v1",
        }
    }
}

/// Message envelope per docs/04-data-contracts.md §1.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

        let idempotence = std::env::var("KAFKA_ENABLE_IDEMPOTENCE")
            .map(|v| v != "0" && v != "false" && v != "FALSE")
            .unwrap_or(true);

        let mut config = ClientConfig::new();
        config
            .set("bootstrap.servers", brokers)
            .set("client.id", client_id)
            .set("acks", "all")
            .set("enable.idempotence", idempotence.to_string())
            .set("message.timeout.ms", "30000");

        let producer: FutureProducer = config
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

/// Kafka consumer for processor workers (Phase 2).
#[derive(Clone)]
pub struct KafkaFactReader {
    consumer: Arc<StreamConsumer>,
    group_id: String,
}

impl KafkaFactReader {
    pub fn new(brokers: &str, group_id: &str, client_id: &str) -> Result<Self> {
        let auto_offset_reset = std::env::var("KAFKA_AUTO_OFFSET_RESET")
            .unwrap_or_else(|_| "earliest".to_string());

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", group_id)
            .set("client.id", client_id)
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", &auto_offset_reset)
            .set("session.timeout.ms", "30000")
            .create()
            .context("failed to create Kafka consumer")?;

        Ok(Self {
            consumer: Arc::new(consumer),
            group_id: group_id.to_string(),
        })
    }

    pub fn subscribe(&self, topics: &[FactTopic]) -> Result<()> {
        let names: Vec<&str> = topics.iter().map(|t| t.as_str()).collect();
        self.consumer
            .subscribe(&names)
            .context("failed to subscribe to Kafka topics")
    }

    pub async fn recv_envelope(&self) -> Result<(MessageEnvelope, BorrowedMessage<'_>)> {
        let message = self
            .consumer
            .recv()
            .await
            .context("Kafka consumer recv failed")?;
        let envelope = parse_envelope(&message)?;
        Ok((envelope, message))
    }

    pub fn commit_message(&self, message: &BorrowedMessage<'_>) -> Result<()> {
        let mut tpl = TopicPartitionList::new();
        tpl.add_partition_offset(
            message.topic(),
            message.partition(),
            Offset::Offset(message.offset() + 1),
        )
        .context("failed to build commit offset")?;
        self.consumer
            .commit(&tpl, rdkafka::consumer::CommitMode::Async)
            .context("failed to commit Kafka offset")
    }

    pub fn group_id(&self) -> &str {
        &self.group_id
    }
}

/// Deserialize a Kafka message payload into a `MessageEnvelope`.
pub fn parse_envelope(message: &BorrowedMessage<'_>) -> Result<MessageEnvelope> {
    let payload = message
        .payload()
        .context("Kafka message has no payload")?;
    let raw: MessageEnvelope =
        serde_json::from_slice(payload).context("failed to deserialize MessageEnvelope")?;
    Ok(raw)
}
