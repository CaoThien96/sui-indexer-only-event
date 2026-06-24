use prometheus::{Encoder, Registry, TextEncoder};
use rdkafka::Message;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::{error, info};

use indexer_store::{FactTopic, KafkaFactReader, MessageEnvelope, parse_envelope};

pub async fn kafka_backoff_resubscribe(
    reader: &KafkaFactReader,
    topics: &[FactTopic],
    label: &str,
) {
    tokio::time::sleep(Duration::from_secs(5)).await;
    if let Err(e) = reader.subscribe(topics) {
        error!(label, error = %e, "Kafka re-subscribe failed");
    }
}

/// Poll one Kafka message, skip invalid envelopes, invoke `handler` on success.
pub async fn poll_kafka_envelope<F, Fut>(
    reader: &KafkaFactReader,
    topics: &[FactTopic],
    label: &str,
    mut handler: F,
) where
    F: FnMut(MessageEnvelope) -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<()>>,
{
    let message = match reader.recv_raw().await {
        Ok(m) => m,
        Err(e) => {
            error!(label, error = %e, "Kafka recv failed");
            kafka_backoff_resubscribe(reader, topics, label).await;
            return;
        }
    };

    let envelope = match parse_envelope(&message) {
        Ok(e) => e,
        Err(e) => {
            tracing::warn!(
                label,
                error = %e,
                topic = message.topic(),
                partition = message.partition(),
                offset = message.offset(),
                "Skipping invalid Kafka envelope"
            );
            let _ = reader.commit_message(&message);
            return;
        }
    };

    if let Err(e) = handler(envelope).await {
        error!(label, error = %e, "Kafka handler failed");
    }
    if let Err(e) = reader.commit_message(&message) {
        error!(label, error = %e, "Kafka commit failed");
    }
}

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
