use anyhow::{bail, Context, Result};
use clickhouse::Client;

use super::row::{PackageEventChRow, PackageEventRow};

const DEFAULT_LIMIT: u64 = 50;
const MAX_LIMIT: u64 = 50;

const STREAM_ORDER_COLS: &str =
    "checkpoint_sequence_number, transaction_sequence_in_checkpoint, event_sequence_in_transaction";

const SELECT_COLS: &str = "event_id_tx_digest, event_id_seq, checkpoint_sequence_number, \
    package_id, transaction_module, event_type, sender, timestamp_ms, bcs, json, parsed_json";

#[derive(Debug, Clone)]
pub struct EventIdCursor {
    pub tx_digest: String,
    pub event_seq: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventStreamPosition {
    pub checkpoint_sequence_number: i64,
    pub transaction_sequence_in_checkpoint: i32,
    pub event_sequence_in_transaction: i32,
}

#[derive(Debug, Clone)]
pub enum EventFilter {
    MoveEventType(String),
    MoveModule { package: String, module: String },
    MoveEventModule { package: String, module: String },
    Sender(String),
}

#[derive(Debug, Clone)]
pub struct QueryEventsParams {
    pub filter: EventFilter,
    pub cursor: Option<EventIdCursor>,
    pub limit: u64,
    pub descending: bool,
}

pub fn parse_query_events_params(params: &serde_json::Value) -> Result<QueryEventsParams> {
    let arr = params
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("params must be a JSON array"))?;

    if arr.is_empty() {
        bail!("params must include at least a filter");
    }

    let filter = parse_event_filter(&arr[0])?;

    let cursor = match arr.get(1) {
        None | Some(serde_json::Value::Null) => None,
        Some(value) => Some(parse_cursor(value)?),
    };

    let limit = match arr.get(2) {
        None | Some(serde_json::Value::Null) => DEFAULT_LIMIT,
        Some(value) => {
            let n = value
                .as_u64()
                .ok_or_else(|| anyhow::anyhow!("limit must be a positive integer"))?;
            if n == 0 {
                bail!("limit must be greater than 0");
            }
            n.min(MAX_LIMIT)
        }
    };

    let descending = match arr.get(3) {
        None | Some(serde_json::Value::Null) => false,
        Some(value) => value
            .as_bool()
            .ok_or_else(|| anyhow::anyhow!("descending must be a boolean"))?,
    };

    Ok(QueryEventsParams {
        filter,
        cursor,
        limit,
        descending,
    })
}

fn parse_cursor(value: &serde_json::Value) -> Result<EventIdCursor> {
    let tx_digest = value
        .get("txDigest")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("cursor.txDigest is required"))?
        .to_string();

    let event_seq = value
        .get("eventSeq")
        .map(parse_event_seq)
        .transpose()?
        .ok_or_else(|| anyhow::anyhow!("cursor.eventSeq is required"))?;

    Ok(EventIdCursor {
        tx_digest,
        event_seq,
    })
}

fn parse_event_seq(value: &serde_json::Value) -> Result<i64> {
    if let Some(s) = value.as_str() {
        return s.parse().map_err(|_| anyhow::anyhow!("invalid eventSeq"));
    }
    value
        .as_i64()
        .ok_or_else(|| anyhow::anyhow!("eventSeq must be a string or integer"))
}

fn parse_event_filter(value: &serde_json::Value) -> Result<EventFilter> {
    let obj = value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("filter must be a JSON object"))?;

    if obj.len() != 1 {
        bail!("filter must contain exactly one variant key");
    }

    let (key, inner) = obj.iter().next().expect("checked len");

    match key.as_str() {
        "MoveEventType" => {
            let event_type = inner
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("MoveEventType must be a string"))?
                .to_string();
            Ok(EventFilter::MoveEventType(event_type))
        }
        "MoveModule" => {
            let package = field_str(inner, "package")?.to_ascii_lowercase();
            let module = field_str(inner, "module")?.to_string();
            Ok(EventFilter::MoveModule { package, module })
        }
        "MoveEventModule" => {
            let package = field_str(inner, "package")?.to_ascii_lowercase();
            let module = field_str(inner, "module")?.to_string();
            Ok(EventFilter::MoveEventModule { package, module })
        }
        "Sender" => {
            let sender = inner
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Sender must be a string"))?
                .to_string();
            Ok(EventFilter::Sender(sender))
        }
        "All" | "Any" | "Transaction" | "TimeRange" => {
            bail!("filter variant '{key}' is not supported in this MVP")
        }
        other => bail!("unknown filter variant '{other}'"),
    }
}

fn field_str(value: &serde_json::Value, key: &str) -> Result<String> {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| anyhow::anyhow!("missing or invalid '{key}'"))
}

async fn resolve_cursor_position(
    client: &Client,
    cursor: &EventIdCursor,
) -> Result<EventStreamPosition> {
    #[derive(clickhouse::Row, serde::Deserialize)]
    struct CursorRow {
        checkpoint_sequence_number: i64,
        transaction_sequence_in_checkpoint: i32,
        event_sequence_in_transaction: i32,
    }

    let row = client
        .query(
            "SELECT checkpoint_sequence_number, transaction_sequence_in_checkpoint, \
             event_sequence_in_transaction \
             FROM package_events \
             WHERE event_id_tx_digest = ? AND event_id_seq = ? \
             LIMIT 1",
        )
        .bind(&cursor.tx_digest)
        .bind(cursor.event_seq)
        .fetch_optional::<CursorRow>()
        .await
        .context("failed to resolve cursor position")?;

    row.map(
        |CursorRow {
             checkpoint_sequence_number,
             transaction_sequence_in_checkpoint,
             event_sequence_in_transaction,
         }| EventStreamPosition {
            checkpoint_sequence_number,
            transaction_sequence_in_checkpoint,
            event_sequence_in_transaction,
        },
    )
    .ok_or_else(|| {
        anyhow::anyhow!(
            "cursor event not found: txDigest={} eventSeq={}",
            cursor.tx_digest,
            cursor.event_seq
        )
    })
}

pub async fn query_events(
    client: &Client,
    params: QueryEventsParams,
) -> Result<Vec<PackageEventRow>> {
    let cursor_position = if let Some(cursor) = &params.cursor {
        Some(resolve_cursor_position(client, cursor).await?)
    } else {
        None
    };

    let fetch_limit = (params.limit + 1) as i64;
    let (sql, bind_count) = build_sql(&params.filter, cursor_position.as_ref(), params.descending);

    let mut query = client.query(&sql);

    match &params.filter {
        EventFilter::MoveEventType(event_type) => {
            query = query.bind(event_type);
        }
        EventFilter::MoveModule { package, module } => {
            query = query.bind(package).bind(module);
        }
        EventFilter::MoveEventModule { package, module } => {
            let prefix = format!("{package}::{module}::%");
            query = query.bind(prefix);
        }
        EventFilter::Sender(sender) => {
            query = query.bind(sender);
        }
    }

    if let Some(position) = &cursor_position {
        query = query
            .bind(position.checkpoint_sequence_number)
            .bind(position.transaction_sequence_in_checkpoint)
            .bind(position.event_sequence_in_transaction);
    }

    query = query.bind(fetch_limit);

    debug_assert_eq!(bind_count, count_binds(&params.filter, cursor_position.is_some()));

    let rows = query
        .fetch_all::<PackageEventChRow>()
        .await
        .context("ClickHouse query failed")?;

    Ok(rows.into_iter().map(PackageEventRow::from).collect())
}

fn count_binds(filter: &EventFilter, has_cursor: bool) -> usize {
    let filter_binds = 1 + match filter {
        EventFilter::MoveModule { .. } => 1,
        _ => 0,
    };
    let cursor_binds = if has_cursor { 3 } else { 0 };
    filter_binds + cursor_binds + 1
}

fn build_sql(
    filter: &EventFilter,
    cursor_position: Option<&EventStreamPosition>,
    descending: bool,
) -> (String, usize) {
    let filter_clause = match filter {
        EventFilter::MoveEventType(_) => "event_type ILIKE ?",
        EventFilter::MoveModule { .. } => "package_id = ? AND transaction_module = ?",
        EventFilter::MoveEventModule { .. } => "event_type LIKE ?",
        EventFilter::Sender(_) => "sender = ?",
    };

    let cursor_clause = if cursor_position.is_some() {
        let op = if descending { "<" } else { ">" };
        format!("AND ({STREAM_ORDER_COLS}) {op} (?, ?, ?)")
    } else {
        String::new()
    };

    let order_clause = if descending {
        format!("ORDER BY {STREAM_ORDER_COLS} DESC")
    } else {
        format!("ORDER BY {STREAM_ORDER_COLS} ASC")
    };

    let sql = format!(
        "SELECT {SELECT_COLS} \
         FROM package_events FINAL \
         WHERE {filter_clause} {cursor_clause} \
         {order_clause} \
         LIMIT ?"
    );

    (sql, count_binds(filter, cursor_position.is_some()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_move_event_type_filter() {
        let params = parse_query_events_params(&json!([
            { "MoveEventType": "0xabc::pool::SwapEvent" },
            null,
            10,
            false
        ]))
        .unwrap();

        match params.filter {
            EventFilter::MoveEventType(t) => assert_eq!(t, "0xabc::pool::SwapEvent"),
            _ => panic!("wrong filter"),
        }
        assert_eq!(params.limit, 10);
        assert!(!params.descending);
    }

    #[test]
    fn parse_cursor_and_descending() {
        let params = parse_query_events_params(&json!([
            { "Sender": "0x123" },
            { "txDigest": "abc", "eventSeq": "5" },
            3,
            true
        ]))
        .unwrap();

        let cursor = params.cursor.unwrap();
        assert_eq!(cursor.tx_digest, "abc");
        assert_eq!(cursor.event_seq, 5);
        assert!(params.descending);
    }

    #[test]
    fn rejects_unsupported_filter() {
        let err = parse_query_events_params(&json!([{ "All": [] }, null, 10, false])).unwrap_err();
        assert!(err.to_string().contains("not supported"));
    }

    #[test]
    fn descending_sql_orders_by_stream_position_and_cursor() {
        let filter = EventFilter::MoveEventType("0x1::pool::SwapEvent".into());
        let cursor = EventStreamPosition {
            checkpoint_sequence_number: 100,
            transaction_sequence_in_checkpoint: 2,
            event_sequence_in_transaction: 1,
        };
        let (sql, binds) = build_sql(&filter, Some(&cursor), true);

        assert!(sql.contains("FROM package_events FINAL"));
        assert!(sql.contains(
            "AND (checkpoint_sequence_number, transaction_sequence_in_checkpoint, event_sequence_in_transaction) < (?, ?, ?)"
        ));
        assert!(sql.contains(
            "ORDER BY checkpoint_sequence_number, transaction_sequence_in_checkpoint, event_sequence_in_transaction DESC"
        ));
        assert_eq!(binds, 5);
    }

    #[test]
    fn ascending_sql_orders_forward_from_cursor() {
        let filter = EventFilter::Sender("0xsender".into());
        let cursor = EventStreamPosition {
            checkpoint_sequence_number: 50,
            transaction_sequence_in_checkpoint: 0,
            event_sequence_in_transaction: 0,
        };
        let (sql, binds) = build_sql(&filter, Some(&cursor), false);

        assert!(sql.contains(
            "AND (checkpoint_sequence_number, transaction_sequence_in_checkpoint, event_sequence_in_transaction) > (?, ?, ?)"
        ));
        assert!(sql.contains(
            "ORDER BY checkpoint_sequence_number, transaction_sequence_in_checkpoint, event_sequence_in_transaction ASC"
        ));
        assert_eq!(binds, 5);
    }

    #[test]
    fn first_page_has_no_cursor_clause() {
        let filter = EventFilter::MoveEventType("0x1::pool::SwapEvent".into());
        let (sql, binds) = build_sql(&filter, None, true);

        assert!(sql.contains("FROM package_events FINAL"));
        assert!(!sql.contains(") < (?, ?, ?)"));
        assert!(!sql.contains(") > (?, ?, ?)"));
        assert!(sql.contains("ORDER BY checkpoint_sequence_number"));
        assert_eq!(binds, 2);
    }
}
