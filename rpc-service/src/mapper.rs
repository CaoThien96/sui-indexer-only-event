use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::{json, Value};

use crate::db::PackageEventRow;

pub fn row_to_sui_event(row: &PackageEventRow) -> Value {
    json!({
        "id": {
            "txDigest": row.event_id_tx_digest,
            "eventSeq": row.event_id_seq.to_string(),
        },
        "packageId": row.package_id,
        "transactionModule": row.transaction_module,
        "sender": row.sender,
        "type": row.event_type,
        "parsedJson": row.parsed_json.clone().unwrap_or(Value::Null),
        "bcs": STANDARD.encode(&row.bcs),
        "timestampMs": row.timestamp_ms.map(|ms| ms.to_string()),
    })
}

pub fn build_query_events_result(rows: Vec<PackageEventRow>, limit: u64) -> Value {
    let has_next_page = rows.len() as u64 > limit;
    let page_rows: Vec<_> = rows.into_iter().take(limit as usize).collect();

    let next_cursor = page_rows.last().map(|row| {
        json!({
            "txDigest": row.event_id_tx_digest,
            "eventSeq": row.event_id_seq.to_string(),
        })
    });

    let data: Vec<Value> = page_rows.iter().map(row_to_sui_event).collect();

    json!({
        "data": data,
        "nextCursor": next_cursor,
        "hasNextPage": has_next_page,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn maps_row_to_sui_event_shape() {
        let row = PackageEventRow {
            event_id_tx_digest: "digest1".to_string(),
            event_id_seq: 2,
            checkpoint_sequence_number: 100,
            package_id: "0xpkg".to_string(),
            transaction_module: Some("pool".to_string()),
            event_type: "0xpkg::pool::swapevent".to_string(),
            sender: Some("0xsender".to_string()),
            timestamp_ms: Some(1_234_567_890),
            bcs: vec![1, 2, 3],
            json: json!({ "sender": "0xsender" }),
            parsed_json: Some(json!({ "pool": "0xpool", "amount_in": "100" })),
        };

        let event = row_to_sui_event(&row);
        assert_eq!(event["id"]["txDigest"], "digest1");
        assert_eq!(event["id"]["eventSeq"], "2");
        assert_eq!(event["bcs"], "AQID");
        assert_eq!(event["timestampMs"], "1234567890");
        assert_eq!(event["parsedJson"]["pool"], "0xpool");
    }

    #[test]
    fn paginates_with_has_next_page() {
        let rows = vec![
            PackageEventRow {
                event_id_tx_digest: "a".into(),
                event_id_seq: 0,
                checkpoint_sequence_number: 1,
                package_id: "0x1".into(),
                transaction_module: None,
                event_type: "0x1::m::e".into(),
                sender: None,
                timestamp_ms: None,
                bcs: vec![],
                json: json!(null),
                parsed_json: None,
            },
            PackageEventRow {
                event_id_tx_digest: "b".into(),
                event_id_seq: 1,
                checkpoint_sequence_number: 1,
                package_id: "0x1".into(),
                transaction_module: None,
                event_type: "0x1::m::e".into(),
                sender: None,
                timestamp_ms: None,
                bcs: vec![],
                json: json!(null),
                parsed_json: None,
            },
        ];

        let result = build_query_events_result(rows, 1);
        assert_eq!(result["data"].as_array().unwrap().len(), 1);
        assert!(result["hasNextPage"].as_bool().unwrap());
        assert_eq!(result["nextCursor"]["txDigest"], "a");
    }
}
