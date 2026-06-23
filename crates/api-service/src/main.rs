use std::net::SocketAddr;

use anyhow::Result;
use prometheus::{Encoder, Registry, TextEncoder};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::info;

use sui_api_service::config::{self, api_metrics_address, api_port};
use sui_api_service::metrics::ApiMetrics;
use sui_api_service::routes;
use sui_api_service::state::AppState;

async fn serve_metrics(registry: Registry, addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!(%addr, "API metrics server listening");

    loop {
        let (mut stream, _) = listener.accept().await?;
        let registry = registry.clone();
        tokio::spawn(async move {
            let mut buf = [0u8; 512];
            let _ = stream.read(&mut buf).await;
            let encoder = TextEncoder::new();
            let metric_families = registry.gather();
            let mut body = Vec::new();
            if encoder.encode(&metric_families, &mut body).is_err() {
                return;
            }
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain; version=0.0.4\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(response.as_bytes()).await;
            let _ = stream.write_all(&body).await;
        });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    config::load_dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let registry = Registry::new();
    let metrics = ApiMetrics::new(&registry)?;
    let state = AppState::new(metrics.clone()).await?;

    let app = routes::router(state);
    let port = api_port();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let metrics_addr = api_metrics_address();

    info!(%addr, "sui-api-service started");

    tokio::select! {
        result = async {
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, app).await?;
            Ok::<(), anyhow::Error>(())
        } => result?,
        result = serve_metrics(registry, &metrics_addr) => result?,
    }

    Ok(())
}
