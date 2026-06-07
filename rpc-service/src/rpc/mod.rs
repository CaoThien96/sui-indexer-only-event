use anyhow::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use tracing::error;

use crate::db::{parse_query_events_params, query_events};
use crate::mapper::build_query_events_result;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

pub async fn json_rpc(State(state): State<AppState>, Json(body): Json<Value>) -> Response {
    match dispatch(&state.pool, &body).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(rpc_error) => (StatusCode::OK, Json(rpc_error)).into_response(),
    }
}

async fn dispatch(pool: &PgPool, body: &Value) -> Result<Value, Value> {
    let id = body.get("id").cloned().unwrap_or(Value::Null);
    let method = body
        .get("method")
        .and_then(|v| v.as_str())
        .ok_or_else(|| invalid_request(id.clone(), "missing method"))?;

    if body.get("jsonrpc").and_then(|v| v.as_str()) != Some("2.0") {
        return Err(invalid_request(id, "jsonrpc must be \"2.0\""));
    }

    match method {
        "suix_queryEvents" => handle_query_events(pool, id, body.get("params")).await,
        _ => Err(method_not_found(id, method)),
    }
}

async fn handle_query_events(pool: &PgPool, id: Value, params: Option<&Value>) -> Result<Value, Value> {
    let params = params.ok_or_else(|| invalid_params(id.clone(), "missing params"))?;

    let query_params = parse_query_events_params(params).map_err(|e| {
        error!(error = %e, "invalid suix_queryEvents params");
        invalid_params(id.clone(), e.to_string())
    })?;

    let limit = query_params.limit;

    let rows = query_events(pool, query_params)
        .await
        .map_err(|e| {
            error!(error = %e, "database query failed");
            let message = e.to_string();
            if message.starts_with("cursor event not found:") {
                invalid_params(id.clone(), message)
            } else {
                internal_error(id.clone(), "database query failed")
            }
        })?;

    let result = build_query_events_result(rows, limit);

    Ok(json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    }))
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

        let err = dispatch(
            &PgPool::connect_lazy("postgres://invalid").expect("lazy pool"),
            &body,
        )
        .await
        .unwrap_err();

        assert_eq!(err["error"]["code"], -32601);
    }
}
