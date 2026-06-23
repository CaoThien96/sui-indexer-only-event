use std::sync::Arc;

use anyhow::Result;
use event_bindings::{
    decode_parsed_json,
    pool_id::{self, PoolCreateFields},
    protocol::{self, Protocol},
};
use indexer_store::{FactTopic, MessageEnvelope};
use prometheus::{IntCounterVec, Registry};
use serde::Serialize;
use serde_json::{Value, json};
use sui_indexer_alt_framework::{
    FieldCount,
    types::base_types::ObjectID,
    types::full_checkpoint_content::Checkpoint,
};

/// Per-checkpoint event with transaction metadata for pipeline processing.
#[derive(Debug, Clone)]
pub struct CheckpointEvent<'a> {
    pub checkpoint_sequence_number: u64,
    pub timestamp_ms: u64,
    pub tx_digest: String,
    pub sender: Option<String>,
    pub transaction_sequence_in_checkpoint: u32,
    pub event_sequence_in_transaction: u32,
    pub package_id: String,
    pub event_type: String,
    pub bcs: &'a [u8],
}

pub fn iterate_checkpoint_events<'a>(
    checkpoint: &'a Checkpoint,
) -> impl Iterator<Item = CheckpointEvent<'a>> + 'a {
    let checkpoint_sequence_number = checkpoint.summary.sequence_number;
    let timestamp_ms = checkpoint.summary.timestamp_ms;
    checkpoint.transactions.iter().enumerate().flat_map(
        move |(tx_idx, tx)| {
            let tx_digest = tx.transaction.digest().to_string();
            let events = tx.events.as_ref().map(|e| e.data.as_slice()).unwrap_or(&[]);
            events.iter().enumerate().map(move |(event_idx, event)| {
                CheckpointEvent {
                    checkpoint_sequence_number,
                    timestamp_ms,
                    tx_digest: tx_digest.clone(),
                    sender: Some(event.sender.to_string()),
                    transaction_sequence_in_checkpoint: tx_idx as u32,
                    event_sequence_in_transaction: event_idx as u32,
                    package_id: event.package_id.to_string().to_ascii_lowercase(),
                    event_type: event.type_.to_string(),
                    bcs: &event.contents,
                }
            })
        },
    )
}

#[derive(Debug, Clone, Serialize, FieldCount)]
pub struct RawSwapFact {
    pub protocol: String,
    pub package_id: String,
    pub event_type: String,
    pub checkpoint_sequence_number: u64,
    pub timestamp_ms: u64,
    pub tx_digest: String,
    pub event_sequence_in_transaction: u32,
    pub transaction_sequence_in_checkpoint: u32,
    pub sender: String,
    pub parsed_json: Value,
}

#[derive(Debug, Clone, Serialize, FieldCount)]
pub struct RawPoolFact {
    pub protocol: String,
    pub event_type: String,
    pub pool_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_type_a: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_type_b: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_spacing: Option<u32>,
    pub checkpoint_sequence_number: u64,
    pub timestamp_ms: u64,
    pub tx_digest: String,
    pub event_seq: u32,
}

#[derive(Debug, Clone, Serialize, FieldCount)]
pub struct RawTokenMetadataFact {
    pub coin_type: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub creator: String,
    pub object_id: String,
    pub created_at_ms: u64,
    pub checkpoint_sequence_number: u64,
    pub tx_digest: String,
    pub transaction_sequence_in_checkpoint: u32,
}

pub fn build_swap_envelope(fact: &RawSwapFact) -> MessageEnvelope {
    let message_id_key = format!(
        "{}:{}:{}",
        fact.tx_digest,
        fact.event_sequence_in_transaction,
        FactTopic::SwapRaw.as_str()
    );
    MessageEnvelope::new(&message_id_key, json!(fact))
}

pub fn build_pool_envelope(fact: &RawPoolFact) -> MessageEnvelope {
    let message_id_key = format!(
        "{}:{}:{}",
        fact.tx_digest,
        fact.event_seq,
        FactTopic::PoolRaw.as_str()
    );
    MessageEnvelope::new(&message_id_key, json!(fact))
}

pub fn build_token_metadata_envelope(fact: &RawTokenMetadataFact) -> MessageEnvelope {
    let message_id_key = format!(
        "{}:{}:{}",
        fact.tx_digest,
        fact.object_id,
        FactTopic::TokenMetadataRaw.as_str()
    );
    MessageEnvelope::new(&message_id_key, json!(fact))
}

pub fn decode_event(
    metrics: &AppMetrics,
    pipeline: &str,
    event: &CheckpointEvent<'_>,
    protocol: Protocol,
) -> Result<Value> {
    match decode_parsed_json(&event.event_type, event.bcs) {
        Ok(parsed) => {
            metrics
                .events_matched
                .with_label_values(&[pipeline, protocol.as_str(), &event.event_type])
                .inc();
            Ok(parsed)
        }
        Err(error) => {
            metrics
                .decode_errors
                .with_label_values(&[pipeline, protocol.as_str(), &event.event_type])
                .inc();
            Err(error)
        }
    }
}

pub fn raw_swap_fact(
    event: &CheckpointEvent<'_>,
    protocol: Protocol,
    parsed_json: Value,
) -> RawSwapFact {
    RawSwapFact {
        protocol: protocol.as_str().to_string(),
        package_id: event.package_id.clone(),
        event_type: event.event_type.clone(),
        checkpoint_sequence_number: event.checkpoint_sequence_number,
        timestamp_ms: event.timestamp_ms,
        tx_digest: event.tx_digest.clone(),
        event_sequence_in_transaction: event.event_sequence_in_transaction,
        transaction_sequence_in_checkpoint: event.transaction_sequence_in_checkpoint,
        sender: event.sender.clone().unwrap_or_default(),
        parsed_json,
    }
}

pub fn raw_pool_fact(
    event: &CheckpointEvent<'_>,
    protocol: Protocol,
    _parsed_json: Value,
    fields: PoolCreateFields,
) -> RawPoolFact {
    RawPoolFact {
        protocol: protocol.as_str().to_string(),
        event_type: event.event_type.clone(),
        pool_id: fields.pool_id,
        coin_type_a: fields.coin_type_a,
        coin_type_b: fields.coin_type_b,
        tick_spacing: fields.tick_spacing,
        checkpoint_sequence_number: event.checkpoint_sequence_number,
        timestamp_ms: event.timestamp_ms,
        tx_digest: event.tx_digest.clone(),
        event_seq: event.event_sequence_in_transaction,
    }
}

pub fn classify_swap(event_type: &str) -> Option<Protocol> {
    protocol::classify_swap_event(event_type)
}

pub fn classify_pool_create(event_type: &str) -> Option<Protocol> {
    protocol::classify_pool_create_event(event_type)
}

/// When pool-create events omit coin types (e.g. Turbos `PoolCreatedEvent`), read them from
/// the created `Pool<CoinA, CoinB, …>` object's generic type parameters in the same transaction.
pub fn enrich_pool_coin_types_from_checkpoint(
    checkpoint: &Checkpoint,
    tx_digest: &str,
    pool_id: &str,
) -> Option<(String, String)> {
    let pool_oid: ObjectID = pool_id.parse().ok()?;
    for tx in &checkpoint.transactions {
        if tx.transaction.digest().to_string() != tx_digest {
            continue;
        }
        for obj in tx.output_objects(&checkpoint.object_set) {
            if obj.id() != pool_oid {
                continue;
            }
            let move_obj = obj.data.try_as_move()?;
            let params: Vec<_> = move_obj
                .type_()
                .type_params()
                .into_iter()
                .map(|t| t.into_owned())
                .collect();
            return pool_id::coin_types_from_pool_type_params(&params);
        }
    }
    None
}

pub fn swap_partition_key(envelope: &MessageEnvelope) -> String {
    let protocol_slug = envelope
        .payload
        .get("protocol")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let parsed = envelope
        .payload
        .get("parsed_json")
        .cloned()
        .unwrap_or(Value::Null);
    let protocol = Protocol::ALL
        .into_iter()
        .find(|p| p.as_str() == protocol_slug)
        .unwrap_or(Protocol::Cetus);
    pool_id::extract_pool_id(protocol, &parsed).unwrap_or_else(|_| "unknown".to_string())
}

pub fn pool_partition_key(envelope: &MessageEnvelope) -> String {
    envelope
        .payload
        .get("pool_id")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn token_metadata_partition_key(envelope: &MessageEnvelope) -> String {
    envelope
        .payload
        .get("coin_type")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| "unknown".to_string())
}

#[derive(Clone)]
pub struct AppMetrics {
    pub decode_errors: IntCounterVec,
    pub events_matched: IntCounterVec,
    pub objects_matched: IntCounterVec,
    pub kafka_rows_published: IntCounterVec,
}

impl AppMetrics {
    pub fn new(registry: &Registry) -> Result<Arc<Self>> {
        let decode_errors = IntCounterVec::new(
            prometheus::Opts::new(
                "indexer_decode_errors_total",
                "BCS decode failures in indexer pipelines",
            ),
            &["pipeline", "protocol", "event_type"],
        )?;
        let events_matched = IntCounterVec::new(
            prometheus::Opts::new(
                "indexer_events_matched_total",
                "Events matched by pipeline filters",
            ),
            &["pipeline", "protocol", "event_type"],
        )?;
        let objects_matched = IntCounterVec::new(
            prometheus::Opts::new(
                "indexer_objects_matched_total",
                "Objects matched by pipeline filters",
            ),
            &["pipeline", "object_kind"],
        )?;
        let kafka_rows_published = IntCounterVec::new(
            prometheus::Opts::new(
                "indexer_kafka_rows_published_total",
                "Kafka fact rows published per pipeline",
            ),
            &["pipeline", "topic"],
        )?;

        registry.register(Box::new(decode_errors.clone()))?;
        registry.register(Box::new(events_matched.clone()))?;
        registry.register(Box::new(objects_matched.clone()))?;
        registry.register(Box::new(kafka_rows_published.clone()))?;

        Ok(Arc::new(Self {
            decode_errors,
            events_matched,
            objects_matched,
            kafka_rows_published,
        }))
    }
}

// Silence unused import warning for Event in public API docs.
#[allow(dead_code)]
fn _checkpoint_event_api_doc() {}
