//! Checkpoint event iteration — keep in sync with `crates/indexer/src/pipelines/common.rs`.

use event_bindings::protocol;
use sui_types::full_checkpoint_content::Checkpoint;

/// Per-checkpoint event with transaction metadata for oracle scanning.
#[derive(Debug, Clone)]
pub struct CheckpointEvent<'a> {
    pub checkpoint_sequence_number: u64,
    pub timestamp_ms: u64,
    pub tx_digest: String,
    pub event_sequence_in_transaction: u32,
    pub event_type: String,
    pub bcs: &'a [u8],
}

pub fn iterate_checkpoint_events<'a>(
    checkpoint: &'a Checkpoint,
) -> impl Iterator<Item = CheckpointEvent<'a>> + 'a {
    let checkpoint_sequence_number = checkpoint.summary.sequence_number;
    let timestamp_ms = checkpoint.summary.timestamp_ms;
    checkpoint.transactions.iter().flat_map(move |tx| {
        let tx_digest = tx.transaction.digest().to_string();
        let events = tx.events.as_ref().map(|e| e.data.as_slice()).unwrap_or(&[]);
        events.iter().enumerate().map(move |(event_idx, event)| CheckpointEvent {
            checkpoint_sequence_number,
            timestamp_ms,
            tx_digest: tx_digest.clone(),
            event_sequence_in_transaction: event_idx as u32,
            event_type: event.type_.to_string(),
            bcs: &event.contents,
        })
    })
}

pub fn classify_swap(event_type: &str) -> Option<protocol::Protocol> {
    protocol::classify_swap_event(event_type)
}
