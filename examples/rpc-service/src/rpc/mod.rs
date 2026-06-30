use anyhow::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use clickhouse::Client;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info, warn};

use crate::db::{event_filter_label, parse_query_events_params, query_events};
use crate::mapper::build_query_events_result;

#[derive(Clone)]
pub struct AppState {
    pub clickhouse: Arc<Client>,
}

pub async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

pub async fn json_rpc(State(state): State<AppState>, Json(body): Json<Value>) -> Response {
    match dispatch(&state.clickhouse, &body).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(rpc_error) => (StatusCode::OK, Json(rpc_error)).into_response(),
    }
}

async fn dispatch(client: &Client, body: &Value) -> Result<Value, Value> {
    let started = Instant::now();
    let id = body.get("id").cloned().unwrap_or(Value::Null);
    let method = body.get("method").and_then(|v| v.as_str()).ok_or_else(|| {
        let err = invalid_request(id.clone(), "missing method");
        log_rpc_request("<missing>", started.elapsed_ms(), &Err(err.clone()));
        err
    })?;

    if body.get("jsonrpc").and_then(|v| v.as_str()) != Some("2.0") {
        let err = invalid_request(id, "jsonrpc must be \"2.0\"");
        log_rpc_request(method, started.elapsed_ms(), &Err(err.clone()));
        return Err(err);
    }

    let result = match method {
        "suix_queryEvents" => handle_query_events(client, id, body.get("params")).await,
        _ => Err(method_not_found(id, method)),
    };

    if method != "suix_queryEvents" {
        log_rpc_request(method, started.elapsed_ms(), &result);
    }

    result
}

async fn handle_query_events(
    client: &Client,
    id: Value,
    params: Option<&Value>,
) -> Result<Value, Value> {
    let started = Instant::now();

    let params = params.ok_or_else(|| {
        let err = invalid_params(id.clone(), "missing params");
        log_query_events(
            started.elapsed_ms(),
            0.0,
            0,
            0,
            false,
            false,
            "?",
            false,
            Some(-32602),
        );
        err
    })?;

    let query_params = parse_query_events_params(params).map_err(|e| {
        error!(error = %e, "invalid suix_queryEvents params");
        let err = invalid_params(id.clone(), e.to_string());
        log_query_events(
            started.elapsed_ms(),
            0.0,
            0,
            0,
            false,
            false,
            "?",
            false,
            Some(-32602),
        );
        err
    })?;

    let filter = event_filter_label(&query_params.filter);
    let has_cursor = query_params.cursor.is_some();
    let descending = query_params.descending;
    let limit = query_params.limit;

    let ch_started = Instant::now();
    let rows = query_events(client, query_params).await.map_err(|e| {
        error!(error = %e, "ClickHouse query failed");
        let ch_query_ms = ch_started.elapsed_ms();
        let message = e.to_string();
        let err = if message.starts_with("cursor event not found:") {
            invalid_params(id.clone(), message)
        } else {
            internal_error(id.clone(), "database query failed")
        };
        let error_code = err
            .get("error")
            .and_then(|v| v.get("code"))
            .and_then(|c| c.as_i64());
        log_query_events(
            started.elapsed_ms(),
            ch_query_ms,
            0,
            limit,
            descending,
            has_cursor,
            &filter,
            false,
            error_code,
        );
        err
    })?;
    let ch_query_ms = ch_started.elapsed_ms();

    let row_count = rows.len();
    let result = build_query_events_result(rows, limit);

    log_query_events(
        started.elapsed_ms(),
        ch_query_ms,
        row_count,
        limit,
        descending,
        has_cursor,
        &filter,
        true,
        None,
    );

    Ok(json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    }))
}

trait ElapsedMs {
    fn elapsed_ms(self) -> f64;
}

impl ElapsedMs for Instant {
    fn elapsed_ms(self) -> f64 {
        self.elapsed().as_secs_f64() * 1000.0
    }
}

fn slow_warn_threshold_ms() -> f64 {
    std::env::var("RPC_SLOW_WARN_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(500.0)
}

fn maybe_warn_slow(method: &str, elapsed_ms: f64) {
    let threshold_ms = slow_warn_threshold_ms();
    if elapsed_ms >= threshold_ms {
        warn!(method, elapsed_ms, threshold_ms, "slow rpc request");
    }
}

fn log_rpc_request(method: &str, elapsed_ms: f64, result: &Result<Value, Value>) {
    let ok = result.is_ok();
    let error_code = result
        .as_ref()
        .err()
        .and_then(|e| e.get("error"))
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_i64());

    info!(method, elapsed_ms, ok, error_code, "json-rpc request");
    maybe_warn_slow(method, elapsed_ms);
}

fn log_query_events(
    elapsed_ms: f64,
    ch_query_ms: f64,
    rows: usize,
    limit: u64,
    descending: bool,
    has_cursor: bool,
    filter: &str,
    ok: bool,
    error_code: Option<i64>,
) {
    info!(
        method = "suix_queryEvents",
        elapsed_ms,
        ch_query_ms,
        rows,
        limit,
        descending,
        has_cursor,
        filter,
        ok,
        error_code,
        "suix_queryEvents"
    );
    maybe_warn_slow("suix_queryEvents", elapsed_ms);
}

fn method_not_found(id: Value, method: &str) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": -32601,
            "message": format!("Method not found: {method}"),
        }
    })
}

fn invalid_request(id: Value, message: impl Into<String>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": -32600,
            "message": message.into(),
        }
    })
}

fn invalid_params(id: Value, message: impl Into<String>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": -32602,
            "message": message.into(),
        }
    })
}

fn internal_error(id: Value, message: impl Into<String>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": -32603,
            "message": message.into(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unknown_method_returns_32601() {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sui_getObject",
            "params": []
        });

        let client = Client::default().with_url("http://127.0.0.1:1");
        let err = dispatch(&client, &body).await.unwrap_err();

        assert_eq!(err["error"]["code"], -32601);
    }
}
