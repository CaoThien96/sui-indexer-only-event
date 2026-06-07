use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PackageEventRow {
    pub event_id_tx_digest: String,
    pub event_id_seq: i64,
    #[allow(dead_code)]
    pub checkpoint_sequence_number: i64,
    pub package_id: String,
    pub transaction_module: Option<String>,
    pub event_type: String,
    pub sender: Option<String>,
    pub timestamp_ms: Option<i64>,
    pub bcs: Vec<u8>,
    pub json: Value,
    pub parsed_json: Option<Value>,
}
