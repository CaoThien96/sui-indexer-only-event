use prometheus::{Encoder, Registry, TextEncoder};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::info;

pub async fn serve_metrics(registry: Registry, addr: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!(%addr, "Processor metrics server listening");

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
