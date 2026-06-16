use clickhouse::Row;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Row, Deserialize)]
pub struct PackageEventChRow {
    pub event_id_tx_digest: String,
    pub event_id_seq: i64,
    pub checkpoint_sequence_number: i64,
    pub package_id: String,
    pub transaction_module: Option<String>,
    pub event_type: String,
    pub sender: Option<String>,
    pub timestamp_ms: Option<i64>,
    pub bcs: String,
    pub parsed_json: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PackageEventRow {
    pub event_id_tx_digest: String,
    pub event_id_seq: i64,
    pub checkpoint_sequence_number: i64,
    pub package_id: String,
    pub transaction_module: Option<String>,
    pub event_type: String,
    pub sender: Option<String>,
    pub timestamp_ms: Option<i64>,
    pub bcs: String,
    pub parsed_json: Option<Value>,
}

impl From<PackageEventChRow> for PackageEventRow {
    fn from(row: PackageEventChRow) -> Self {
        Self {
            event_id_tx_digest: row.event_id_tx_digest,
            event_id_seq: row.event_id_seq,
            checkpoint_sequence_number: row.checkpoint_sequence_number,
            package_id: row.package_id,
            transaction_module: row.transaction_module,
            event_type: row.event_type,
            sender: row.sender,
            timestamp_ms: row.timestamp_ms,
            bcs: row.bcs,
            parsed_json: row
                .parsed_json
                .as_deref()
                .map(|value| serde_json::from_str(value).unwrap_or(Value::Null)),
        }
    }
}
