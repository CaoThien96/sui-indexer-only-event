use anyhow::{bail, Result};
use sqlx::PgPool;

use super::row::PackageEventRow;

const DEFAULT_LIMIT: u64 = 50;
const MAX_LIMIT: u64 = 50;

#[derive(Debug, Clone)]
pub struct EventIdCursor {
    pub tx_digest: String,
    pub event_seq: i64,
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
                .to_ascii_lowercase();
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

pub async fn query_events(pool: &PgPool, params: QueryEventsParams) -> Result<Vec<PackageEventRow>> {
    let fetch_limit = (params.limit + 1) as i64;
    let (sql, bind_count) = build_sql(&params);

    let mut query = sqlx::query_as::<_, PackageEventRow>(&sql);

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

    if let Some(cursor) = &params.cursor {
        query = query.bind(&cursor.tx_digest).bind(cursor.event_seq);
    }

    query = query.bind(fetch_limit);

    debug_assert_eq!(bind_count, count_binds(&params));

    query.fetch_all(pool).await.map_err(Into::into)
}

fn count_binds(params: &QueryEventsParams) -> usize {
    let filter_binds = 1 + match &params.filter {
        EventFilter::MoveModule { .. } => 1,
        _ => 0,
    };
    let cursor_binds = if params.cursor.is_some() { 2 } else { 0 };
    filter_binds + cursor_binds + 1
}

fn build_sql(params: &QueryEventsParams) -> (String, usize) {
    let filter_clause = match &params.filter {
        EventFilter::MoveEventType(_) => "event_type = $1",
        EventFilter::MoveModule { .. } => "package_id = $1 AND transaction_module = $2",
        EventFilter::MoveEventModule { .. } => "event_type LIKE $1",
        EventFilter::Sender(_) => "sender = $1",
    };

    let mut bind_idx = match &params.filter {
        EventFilter::MoveModule { .. } => 3,
        _ => 2,
    };

    let cursor_clause = if params.cursor.is_some() {
        let clause = if params.descending {
            format!(
                "AND (event_id_tx_digest, event_id_seq) < (${}, ${})",
                bind_idx,
                bind_idx + 1
            )
        } else {
            format!(
                "AND (event_id_tx_digest, event_id_seq) > (${}, ${})",
                bind_idx,
                bind_idx + 1
            )
        };
        bind_idx += 2;
        clause
    } else {
        String::new()
    };

    let order_clause = if params.descending {
        "ORDER BY event_id_tx_digest DESC, event_id_seq DESC"
    } else {
        "ORDER BY event_id_tx_digest ASC, event_id_seq ASC"
    };

    let sql = format!(
        "SELECT event_id_tx_digest, event_id_seq, checkpoint_sequence_number, \
         package_id, transaction_module, event_type, sender, timestamp_ms, bcs, json \
         FROM package_events \
         WHERE {filter_clause} {cursor_clause} \
         {order_clause} \
         LIMIT ${bind_idx}"
    );

    (sql, count_binds(params))
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
            EventFilter::MoveEventType(t) => assert_eq!(t, "0xabc::pool::swapevent"),
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
}
