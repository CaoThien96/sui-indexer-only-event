mod config;
mod db;
mod mapper;
mod rpc;

use std::net::SocketAddr;

use anyhow::Result;
use axum::routing::{get, post};
use axum::Router;
use rpc::{health, json_rpc, AppState};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = config::Config::from_env()?;
    let pool = db::create_pool(&config.database_url).await?;

    let state = AppState { pool };

    let app = Router::new()
        .route("/health", get(health))
        .route("/", post(json_rpc))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.rpc_port));
    info!(%addr, "Starting rpc-service (suix_queryEvents MVP)");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
