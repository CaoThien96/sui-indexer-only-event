mod health;
mod pools;
mod tokens;

use axum::routing::get;
use axum::Router;

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/v1/tokens/{*rest}", get(tokens::token_dispatch))
        .route("/v1/pools/{pool_id}/ohlc", get(pools::pool_ohlc))
        .with_state(state)
}
