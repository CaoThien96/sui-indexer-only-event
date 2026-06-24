mod health;
mod pools;
mod tokens;
mod tokens_list;

use axum::routing::get;
use axum::Router;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

use crate::config;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    let mut cors = CorsLayer::new().allow_methods(Any).allow_headers(Any);
    let origins: Vec<_> = config::api_cors_origins()
        .into_iter()
        .filter_map(|o| o.parse().ok())
        .collect();
    if origins.is_empty() {
        cors = cors.allow_origin(Any);
    } else {
        cors = cors.allow_origin(AllowOrigin::list(origins));
    }

    Router::new()
        .route("/health", get(health::health))
        .route("/v1/tokens", get(tokens_list::list_tokens))
        .route("/v1/tokens/{*rest}", get(tokens::token_dispatch))
        .route("/v1/pools/{pool_id}/ohlc", get(pools::pool_ohlc))
        .layer(cors)
        .with_state(state)
}
