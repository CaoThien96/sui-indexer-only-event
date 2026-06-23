use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sui_processors::clickhouse;

use crate::dto::{OhlcBarDto, OhlcResponse};
use crate::query_router::{route_query, StorageTarget};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct OhlcQuery {
    pub interval: String,
    pub from: String,
    pub to: String,
    pub base_coin_type: Option<String>,
}

pub async fn pool_ohlc(
    State(state): State<AppState>,
    axum::extract::Path(pool_id): axum::extract::Path<String>,
    Query(query): Query<OhlcQuery>,
) -> Response {
    let from = match DateTime::parse_from_rfc3339(&query.from) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "invalid from timestamp"})),
            )
                .into_response();
        }
    };
    let to = match DateTime::parse_from_rfc3339(&query.to) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "invalid to timestamp"})),
            )
                .into_response();
        }
    };

    let target = route_query(from, to, state.hot_storage_days);
    let mut bars = Vec::new();

    if matches!(target, StorageTarget::Hot | StorageTarget::Both) {
        if let Ok(rows) = state
            .timescale
            .query_ohlc(
                &pool_id,
                &query.interval,
                from,
                to,
                query.base_coin_type.as_deref(),
            )
            .await
        {
            for r in rows {
                bars.push(OhlcBarDto {
                    bucket: r.bucket.to_rfc3339(),
                    open: r.open,
                    high: r.high,
                    low: r.low,
                    close: r.close,
                    volume_quote: r.volume_quote,
                    trade_count: r.trade_count,
                });
            }
        }
    }

    if matches!(target, StorageTarget::Cold | StorageTarget::Both) {
        if let Ok(rows) = clickhouse::query_ohlc(
            &state.clickhouse,
            &pool_id,
            from,
            to,
            query.base_coin_type.as_deref(),
        )
        .await
        {
            for r in rows {
                bars.push(OhlcBarDto {
                    bucket: r.bucket.to_rfc3339(),
                    open: r.open,
                    high: r.high,
                    low: r.low,
                    close: r.close,
                    volume_quote: r.volume_quote,
                    trade_count: r.trade_count,
                });
            }
        }
    }

    bars.sort_by(|a, b| a.bucket.cmp(&b.bucket));
    bars.dedup_by(|a, b| a.bucket == b.bucket);

    Json(OhlcResponse {
        pool_id,
        interval: query.interval,
        bars,
    })
    .into_response()
}
