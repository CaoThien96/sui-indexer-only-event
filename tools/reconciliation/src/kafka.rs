use std::collections::HashSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::error::KafkaError;
use rdkafka::message::Message;
use rdkafka::{Offset, TopicPartitionList};
use serde_json::Value;

use crate::event_key::EventKey;

const POLL_TIMEOUT: Duration = Duration::from_secs(2);
const IDLE_ROUNDS_LIMIT: u32 = 5;

pub async fn list_event_keys_in_window(
    brokers: &str,
    topic: &str,
    event_type: &str,
    start_ms: i64,
    end_ms: i64,
) -> Result<(HashSet<EventKey>, Option<i64>)> {
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set(
            "group.id",
            format!(
                "reconciliation-{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis()
            ),
        )
        .set("enable.auto.commit", "false")
        .set("enable.partition.eof", "true")
        .create()
        .context("failed to create Kafka consumer")?;

    let metadata = consumer
        .fetch_metadata(Some(topic), Duration::from_secs(10))
        .context("failed to fetch Kafka topic metadata")?;

    let topic_meta = metadata
        .topics()
        .iter()
        .find(|t| t.name() == topic)
        .with_context(|| format!("topic not found: {topic}"))?;

    let mut assignment = TopicPartitionList::new();
    for partition in topic_meta.partitions() {
        assignment
            .add_partition_offset(topic, partition.id(), Offset::Beginning)
            .context("failed to assign partition offset")?;
    }
    consumer
        .assign(&assignment)
        .context("failed to assign Kafka partitions")?;

    let partition_count = topic_meta.partitions().len();
    let mut eof_partitions = HashSet::new();
    let mut keys = HashSet::new();
    let mut max_ts: Option<i64> = None;
    let mut idle_rounds = 0u32;

    while eof_partitions.len() < partition_count && idle_rounds < IDLE_ROUNDS_LIMIT {
        match consumer.poll(POLL_TIMEOUT) {
            Some(Ok(message)) => {
                idle_rounds = 0;

                let Some(payload) = message.payload() else {
                    continue;
                };

                let envelope: Value =
                    serde_json::from_slice(payload).context("invalid Kafka JSON envelope")?;
                let fact = envelope
                    .get("payload")
                    .context("Kafka envelope missing payload")?;

                let Some(ts) = fact
                    .get("timestamp_ms")
                    .and_then(|v| v.as_u64().or_else(|| v.as_i64().map(|n| n as u64)))
                    .map(|v| v as i64)
                else {
                    continue;
                };

                max_ts = Some(max_ts.map_or(ts, |current| current.max(ts)));

                if fact
                    .get("event_type")
                    .and_then(Value::as_str)
                    .is_none_or(|t| !t.eq_ignore_ascii_case(event_type))
                {
                    continue;
                }

                if ts < start_ms || ts > end_ms {
                    continue;
                }

                if let Some(key) = EventKey::from_kafka_fact(fact) {
                    keys.insert(key);
                }
            }
            Some(Err(KafkaError::PartitionEOF(partition))) => {
                idle_rounds = 0;
                eof_partitions.insert(partition);
            }
            Some(Err(error)) => anyhow::bail!("Kafka poll error: {error}"),
            None => {
                idle_rounds += 1;
            }
        }
    }

    Ok((keys, max_ts))
}
