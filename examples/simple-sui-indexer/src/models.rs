use serde_json::Value;
use sui_indexer_alt_framework::FieldCount;

#[derive(Debug, Clone, FieldCount)]
pub struct StoredPackageEvent {
    pub event_id_tx_digest: String,
    pub event_id_seq: i64,
    pub checkpoint_sequence_number: i64,
    pub transaction_sequence_in_checkpoint: i32,
    pub event_sequence_in_transaction: i32,
    pub package_id: String,
    pub transaction_module: Option<String>,
    pub event_type: String,
    pub sender: Option<String>,
    pub timestamp_ms: Option<i64>,
    pub bcs: Vec<u8>,
    pub json: Value,
    pub parsed_json: Option<Value>,
}
