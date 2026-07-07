use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use prometheus::{IntCounter, Registry};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use rdkafka::{Message, Offset, TopicPartitionList};
use rdkafka::error::{KafkaError, RDKafkaErrorCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

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

/// Owned Kafka message returned by [`KafkaFactReader::recv_raw`].
#[derive(Debug, Clone)]
pub struct KafkaRawMessage {
    topic: String,
    partition: i32,
    offset: i64,
    payload: Vec<u8>,
}

impl KafkaRawMessage {
    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn partition(&self) -> i32 {
        self.partition
    }

    pub fn offset(&self) -> i64 {
        self.offset
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
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
struct ProducerConfig {
    brokers: String,
    client_id: String,
    message_timeout_ms: String,
    delivery_timeout_ms: String,
    send_timeout_secs: u64,
    max_retries: u32,
}

impl ProducerConfig {
    fn from_env(brokers: &str, client_id: &str) -> Self {
        let message_timeout_ms = std::env::var("KAFKA_PRODUCE_MESSAGE_TIMEOUT_MS")
            .unwrap_or_else(|_| "120000".to_string());
        let delivery_timeout_ms = std::env::var("KAFKA_PRODUCE_DELIVERY_TIMEOUT_MS")
            .unwrap_or_else(|_| "180000".to_string());
        let send_timeout_secs = std::env::var("KAFKA_PRODUCE_SEND_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(120);
        let max_retries = std::env::var("KAFKA_PRODUCE_MAX_RETRIES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);

        Self {
            brokers: brokers.to_string(),
            client_id: client_id.to_string(),
            message_timeout_ms,
            delivery_timeout_ms,
            send_timeout_secs,
            max_retries,
        }
    }

    fn create_producer(&self) -> Result<FutureProducer> {
        let idempotence = std::env::var("KAFKA_ENABLE_IDEMPOTENCE")
            .map(|v| v != "0" && v != "false" && v != "FALSE")
            .unwrap_or(true);

        ClientConfig::new()
            .set("bootstrap.servers", &self.brokers)
            .set("client.id", &self.client_id)
            .set("acks", "all")
            .set("enable.idempotence", idempotence.to_string())
            .set("message.timeout.ms", &self.message_timeout_ms)
            .set("delivery.timeout.ms", &self.delivery_timeout_ms)
            .set("request.timeout.ms", "60000")
            .set("metadata.max.age.ms", "30000")
            .set("topic.metadata.refresh.interval.ms", "10000")
            .set("reconnect.backoff.ms", "1000")
            .set("reconnect.backoff.max.ms", "10000")
            .set("socket.connection.setup.timeout.ms", "30000")
            .create()
            .context("failed to create Kafka producer")
    }
}

fn is_retriable_produce_error(err: &KafkaError) -> bool {
    match err {
        KafkaError::MessageProduction(code) => matches!(
            code,
            RDKafkaErrorCode::LeaderNotAvailable
                | RDKafkaErrorCode::NotLeaderForPartition
                | RDKafkaErrorCode::RequestTimedOut
                | RDKafkaErrorCode::NetworkException
                | RDKafkaErrorCode::BrokerNotAvailable
                | RDKafkaErrorCode::CoordinatorLoadInProgress
                | RDKafkaErrorCode::CoordinatorNotAvailable
        ),
        KafkaError::Global(code) => matches!(
            code,
            RDKafkaErrorCode::AllBrokersDown | RDKafkaErrorCode::RequestTimedOut
        ),
        _ => false,
    }
}

#[derive(Clone)]
pub struct KafkaFactWriter {
    producer: Arc<RwLock<FutureProducer>>,
    config: ProducerConfig,
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

        let config = ProducerConfig::from_env(brokers, client_id);
        let producer = config.create_producer()?;

        Ok(Self {
            producer: Arc::new(RwLock::new(producer)),
            config,
            produce_errors,
        })
    }

    async fn refresh_metadata(&self) -> Result<()> {
        let producer = self.producer.read().await;
        producer
            .client()
            .fetch_metadata(None, Duration::from_secs(10))
            .map(|_| ())
            .context("Kafka producer metadata refresh failed")
    }

    async fn recreate_producer(&self) -> Result<()> {
        warn!(
            client_id = %self.config.client_id,
            "Kafka producer recreating after repeated produce failures"
        );
        let new_producer = self.config.create_producer()?;
        let mut guard = self.producer.write().await;
        *guard = new_producer;
        Ok(())
    }

    async fn send_with_retry(
        &self,
        topic_name: &str,
        key: &str,
        payload: &str,
    ) -> Result<()> {
        let send_timeout = Duration::from_secs(self.config.send_timeout_secs);
        let mut attempt = 0u32;

        loop {
            let record = FutureRecord::to(topic_name).key(key).payload(payload);
            let producer = self.producer.read().await;
            match producer.send(record, send_timeout).await {
                Ok(_) => return Ok(()),
                Err((e, _)) => {
                    attempt += 1;
                    let retriable = is_retriable_produce_error(&e);

                    if !retriable || attempt > self.config.max_retries {
                        self.produce_errors.inc();
                        return Err(e).context(format!(
                            "Kafka produce failed (topic={topic_name}, key={key}, attempts={attempt})"
                        ));
                    }

                    warn!(
                        topic = topic_name,
                        attempt,
                        error = %e,
                        "Kafka produce failed, refreshing metadata and retrying"
                    );

                    drop(producer);
                    if let Err(refresh_err) = self.refresh_metadata().await {
                        warn!(error = %refresh_err, "Kafka metadata refresh failed during produce retry");
                    }
                    if attempt == self.config.max_retries {
                        if let Err(recreate_err) = self.recreate_producer().await {
                            warn!(error = %recreate_err, "Kafka producer recreate failed");
                        }
                    }

                    let backoff_ms = 500u64.saturating_mul(1u64 << attempt.min(4));
                    tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                }
            }
        }
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

            self.send_with_retry(topic_name, &key, &payload).await?;
            published += 1;
        }

        debug!(
            topic = topic_name,
            count = published,
            client_id = %self.config.client_id,
            "Published Kafka fact batch"
        );

        Ok(published)
    }
}

#[derive(Clone)]
struct ConsumerConfig {
    brokers: String,
    group_id: String,
    client_id: String,
    auto_offset_reset: String,
    session_timeout_ms: String,
    heartbeat_interval_ms: String,
    max_poll_interval_ms: String,
}

impl ConsumerConfig {
    fn from_env(brokers: &str, group_id: &str, client_id: &str) -> Self {
        Self {
            brokers: brokers.to_string(),
            group_id: group_id.to_string(),
            client_id: client_id.to_string(),
            auto_offset_reset: std::env::var("KAFKA_AUTO_OFFSET_RESET")
                .unwrap_or_else(|_| "earliest".to_string()),
            session_timeout_ms: std::env::var("KAFKA_SESSION_TIMEOUT_MS")
                .unwrap_or_else(|_| "180000".to_string()),
            heartbeat_interval_ms: std::env::var("KAFKA_HEARTBEAT_INTERVAL_MS")
                .unwrap_or_else(|_| "3000".to_string()),
            max_poll_interval_ms: std::env::var("KAFKA_MAX_POLL_INTERVAL_MS")
                .unwrap_or_else(|_| "900000".to_string()),
        }
    }

    fn create_consumer(&self) -> Result<StreamConsumer> {
        ClientConfig::new()
            .set("bootstrap.servers", &self.brokers)
            .set("group.id", &self.group_id)
            .set("client.id", &self.client_id)
            // Static membership: one logical member per worker; avoids ghost duplicates on restart/recover.
            .set("group.instance.id", &self.client_id)
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", &self.auto_offset_reset)
            .set("session.timeout.ms", &self.session_timeout_ms)
            .set("heartbeat.interval.ms", &self.heartbeat_interval_ms)
            .set("max.poll.interval.ms", &self.max_poll_interval_ms)
            .set("reconnect.backoff.ms", "1000")
            .set("reconnect.backoff.max.ms", "10000")
            .create()
            .context("failed to create Kafka consumer")
    }
}

fn leave_group_wait_ms() -> u64 {
    std::env::var("KAFKA_LEAVE_GROUP_WAIT_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(2000)
}

fn assignment_grace_secs() -> u64 {
    std::env::var("KAFKA_ASSIGNMENT_GRACE_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(90)
}

fn recovery_cooldown_secs() -> u64 {
    std::env::var("KAFKA_RECOVERY_COOLDOWN_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(60)
}

async fn drive_leave_group(consumer: &StreamConsumer) {
    let _ = consumer.unsubscribe();
    for _ in 0..5 {
        let _ = tokio::time::timeout(Duration::from_millis(200), consumer.recv()).await;
    }
}

async fn wait_for_assignment(consumer: &StreamConsumer, timeout_secs: u64) -> Result<u32> {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    loop {
        let count = consumer
            .assignment()
            .context("failed to fetch Kafka assignment")?
            .count() as u32;
        if count > 0 {
            return Ok(count);
        }
        if Instant::now() >= deadline {
            return Ok(0);
        }
        // Must poll recv() to drive heartbeats and rebalance; sleep alone triggers PollExceeded.
        let _ = tokio::time::timeout(Duration::from_millis(500), consumer.recv()).await;
    }
}

/// Kafka consumer for processor workers (Phase 2).
#[derive(Clone)]
pub struct KafkaFactReader {
    consumer: Arc<RwLock<StreamConsumer>>,
    config: ConsumerConfig,
    group_id: String,
    subscribed_topics: Arc<Mutex<Vec<String>>>,
    subscribed_at: Arc<Mutex<Option<Instant>>>,
    last_recovery_at: Arc<Mutex<Option<Instant>>>,
}

impl KafkaFactReader {
    pub fn new(brokers: &str, group_id: &str, client_id: &str) -> Result<Self> {
        let config = ConsumerConfig::from_env(brokers, group_id, client_id);
        let consumer = config.create_consumer()?;

        Ok(Self {
            consumer: Arc::new(RwLock::new(consumer)),
            config,
            group_id: group_id.to_string(),
            subscribed_topics: Arc::new(Mutex::new(Vec::new())),
            subscribed_at: Arc::new(Mutex::new(None)),
            last_recovery_at: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn subscribe(&self, topics: &[FactTopic]) -> Result<()> {
        let names: Vec<String> = topics.iter().map(|t| t.as_str().to_string()).collect();
        let name_refs: Vec<&str> = names.iter().map(String::as_str).collect();
        {
            let consumer = self.consumer.read().await;
            consumer
                .subscribe(&name_refs)
                .context("failed to subscribe to Kafka topics")?;
        }
        if let Ok(mut stored) = self.subscribed_topics.lock() {
            *stored = names;
        }
        if let Ok(mut subscribed_at) = self.subscribed_at.lock() {
            *subscribed_at = Some(Instant::now());
        }

        let timeout_secs = std::env::var("KAFKA_SUBSCRIBE_ASSIGNMENT_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);
        let assignment_count = self.wait_for_partition_assignment(timeout_secs).await?;
        if assignment_count == 0 {
            warn!(
                group_id = %self.group_id,
                client_id = %self.config.client_id,
                timeout_secs,
                "Kafka consumer subscribed but has no partition assignment yet"
            );
        } else {
            info!(
                group_id = %self.group_id,
                client_id = %self.config.client_id,
                assignment_count,
                "Kafka consumer subscribed with partition assignment"
            );
        }

        Ok(())
    }

    async fn wait_for_partition_assignment(&self, timeout_secs: u64) -> Result<u32> {
        let deadline = Instant::now() + Duration::from_secs(timeout_secs);
        loop {
            let count = {
                let consumer = self.consumer.read().await;
                consumer
                    .assignment()
                    .context("failed to fetch Kafka assignment")?
                    .count() as u32
            };
            if count > 0 {
                return Ok(count);
            }
            if Instant::now() >= deadline {
                return Ok(0);
            }
            {
                let consumer = self.consumer.read().await;
                let _ =
                    tokio::time::timeout(Duration::from_millis(500), consumer.recv()).await;
            }
        }
    }

    /// Recreate the consumer, re-subscribe, and wait until partitions are assigned.
    pub async fn recover(&self) -> Result<()> {
        let topics = self
            .subscribed_topics
            .lock()
            .map(|guard| guard.clone())
            .unwrap_or_default();
        if topics.is_empty() {
            return Ok(());
        }

        warn!(
            group_id = %self.group_id,
            client_id = %self.config.client_id,
            "Kafka consumer recreating after eviction or recv failure"
        );

        // Leave the group before joining with a new consumer; otherwise both members
        // coexist and the group stays in PreparingRebalance indefinitely.
        {
            let consumer = self.consumer.read().await;
            drive_leave_group(&consumer).await;
        }
        tokio::time::sleep(Duration::from_millis(leave_group_wait_ms())).await;

        let new_consumer = self.config.create_consumer()?;
        let topic_refs: Vec<&str> = topics.iter().map(String::as_str).collect();
        new_consumer
            .subscribe(&topic_refs)
            .context("failed to subscribe during Kafka consumer recovery")?;

        let timeout_secs = std::env::var("KAFKA_RECOVERY_ASSIGNMENT_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);
        let assignment_count = wait_for_assignment(&new_consumer, timeout_secs).await?;
        if assignment_count == 0 {
            bail!(
                "timed out after {timeout_secs}s waiting for Kafka partition assignment (group={})",
                self.group_id
            );
        }

        {
            let mut guard = self.consumer.write().await;
            *guard = new_consumer;
        }

        if let Ok(mut subscribed_at) = self.subscribed_at.lock() {
            *subscribed_at = Some(Instant::now());
        }
        if let Ok(mut last_recovery_at) = self.last_recovery_at.lock() {
            *last_recovery_at = Some(Instant::now());
        }

        info!(
            group_id = %self.group_id,
            client_id = %self.config.client_id,
            assignment_count,
            "Kafka consumer recreated and partition assignment restored"
        );

        Ok(())
    }

    async fn maybe_recover_stale_assignment(&self) -> Result<()> {
        let assignment_count = {
            let consumer = self.consumer.read().await;
            consumer
                .assignment()
                .context("failed to fetch Kafka assignment")?
                .count()
        };
        if assignment_count > 0 {
            return Ok(());
        }

        let subscribed_for = self
            .subscribed_at
            .lock()
            .ok()
            .and_then(|guard| guard.map(|started| started.elapsed()))
            .unwrap_or(Duration::ZERO);
        if subscribed_for < Duration::from_secs(assignment_grace_secs()) {
            return Ok(());
        }

        let should_recover = match self.last_recovery_at.lock() {
            Ok(mut guard) => {
                let due = guard
                    .map(|last| last.elapsed() >= Duration::from_secs(recovery_cooldown_secs()))
                    .unwrap_or(true);
                if due {
                    *guard = Some(Instant::now());
                }
                due
            }
            Err(_) => true,
        };
        if !should_recover {
            return Ok(());
        }

        self.recover().await
    }

    pub async fn recv_envelope(&self) -> Result<MessageEnvelope> {
        let message = self.recv_raw().await?;
        parse_envelope(&message)
    }

    pub async fn recv_raw(&self) -> Result<KafkaRawMessage> {
        let recv_timeout = std::env::var("KAFKA_RECV_TIMEOUT_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_secs(5));

        loop {
            if let Err(e) = self.maybe_recover_stale_assignment().await {
                warn!(
                    group_id = %self.group_id,
                    error = %e,
                    "Kafka assignment recovery failed"
                );
            }

            let consumer = self.consumer.read().await;
            match tokio::time::timeout(recv_timeout, consumer.recv()).await {
                Ok(Ok(message)) => {
                    let payload = message
                        .payload()
                        .context("Kafka message has no payload")?
                        .to_vec();
                    return Ok(KafkaRawMessage {
                        topic: message.topic().to_string(),
                        partition: message.partition(),
                        offset: message.offset(),
                        payload,
                    });
                }
                Ok(Err(e)) => {
                    return Err(e).context("Kafka consumer recv failed");
                }
                Err(_) => {
                    // Idle recv: no message within timeout. Recovery only happens via
                    // maybe_recover_stale_assignment when partition assignment is empty.
                }
            }
        }
    }

    pub async fn commit_message(&self, message: &KafkaRawMessage) -> Result<()> {
        let mut tpl = TopicPartitionList::new();
        tpl.add_partition_offset(
            message.topic(),
            message.partition(),
            Offset::Offset(message.offset() + 1),
        )
        .context("failed to build commit offset")?;
        let consumer = self.consumer.read().await;
        consumer
            .commit(&tpl, rdkafka::consumer::CommitMode::Async)
            .context("failed to commit Kafka offset")
    }

    pub fn group_id(&self) -> &str {
        &self.group_id
    }
}

/// Block until all `topics` exist on the broker (retries every 2s).
pub async fn wait_for_topics_available(brokers: &str, topics: &[FactTopic]) -> Result<()> {
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", "indexer-topic-waiter")
        .create()
        .context("failed to create Kafka metadata consumer")?;

    let required: Vec<&str> = topics.iter().map(|t| t.as_str()).collect();

    loop {
        match consumer.fetch_metadata(None, Duration::from_secs(5)) {
            Ok(meta) => {
                let present: HashSet<&str> = meta
                    .topics()
                    .iter()
                    .filter(|t| t.error().is_none())
                    .map(|t| t.name())
                    .collect();
                let missing: Vec<&str> = required
                    .iter()
                    .copied()
                    .filter(|name| !present.contains(name))
                    .collect();
                if missing.is_empty() {
                    info!(topics = ?required, "Kafka topics available");
                    return Ok(());
                }
                warn!(?missing, "Waiting for Kafka topics");
            }
            Err(e) => {
                warn!(error = %e, "Kafka metadata fetch failed, retrying")
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

/// Deserialize a Kafka message payload into a `MessageEnvelope`.
pub fn parse_envelope(message: &KafkaRawMessage) -> Result<MessageEnvelope> {
    let raw: MessageEnvelope = serde_json::from_slice(message.payload())
        .context("failed to deserialize MessageEnvelope")?;
    Ok(raw)
}
