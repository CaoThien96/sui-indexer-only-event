use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Deserialize;

use crate::dto::{TokenListItem, TokenListResponse};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct TokenListQuery {
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

pub async fn list_tokens(
    State(state): State<AppState>,
    Query(query): Query<TokenListQuery>,
) -> Response {
    let route = "token_list";
    let limit = query.limit.unwrap_or(50).clamp(1, 100);
    let fetch_limit = limit + 1;

    let mut tokens = match state
        .list_tokens(query.q.as_deref(), fetch_limit, query.cursor.as_deref())
        .await
    {
        Ok(t) => t,
        Err(e) => return internal(route, &state, e),
    };

    let next_cursor = if tokens.len() as i64 > limit {
        tokens.pop();
        tokens
            .last()
            .map(|t| format!("{}|{}|{}", t.priority, t.first_seen_cp, t.coin_type))
    } else {
        None
    };

    let items: Vec<TokenListItem> = tokens
        .into_iter()
        .map(|t| TokenListItem {
            coin_type: t.coin_type,
            name: t.name,
            symbol: t.symbol,
            decimals: t.decimals,
            image_url: t.image_url,
        })
        .collect();

    Json(TokenListResponse {
        tokens: items,
        next_cursor,
    })
    .into_response()
}

fn internal(route: &str, state: &AppState, err: anyhow::Error) -> Response {
    state
        .metrics
        .errors
        .with_label_values(&[route, "500"])
        .inc();
    tracing::error!(error = %err, route, "API handler error");
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        Json(crate::dto::ErrorResponse {
            error: "internal error".to_string(),
        }),
    )
        .into_response()
}
