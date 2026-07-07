use prometheus::{Encoder, Registry, TextEncoder};
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use indexer_store::{FactTopic, KafkaFactReader, KafkaRawMessage, MessageEnvelope, parse_envelope};

pub async fn kafka_recover(reader: &KafkaFactReader, _topics: &[FactTopic], label: &str) {
    tokio::time::sleep(Duration::from_secs(5)).await;
    if let Err(e) = reader.recover().await {
        error!(label, error = %e, "Kafka consumer recovery failed");
    }
}

/// Dedicated recv loop → unbounded channel so slow DB handlers never block Kafka polling.
pub fn spawn_kafka_poll_task(
    reader: KafkaFactReader,
    topics: &'static [FactTopic],
    label: &'static str,
) -> mpsc::UnboundedReceiver<Result<KafkaRawMessage, anyhow::Error>> {
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        loop {
            match reader.recv_raw().await {
                Ok(message) => {
                    if tx.send(Ok(message)).is_err() {
                        break;
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(anyhow::anyhow!(e)));
                    kafka_recover(&reader, topics, label).await;
                }
            }
        }
    });
    rx
}

/// Process messages from `spawn_kafka_poll_task`; commit after handler succeeds.
pub async fn drain_kafka_pipeline<F, Fut>(
    reader: &KafkaFactReader,
    mut rx: mpsc::UnboundedReceiver<Result<KafkaRawMessage, anyhow::Error>>,
    label: &'static str,
    worker: &'static str,
    decode_metric: impl Fn(&str),
    mut handler: F,
) where
    F: FnMut(MessageEnvelope) -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<()>>,
{
    while let Some(result) = rx.recv().await {
        let message = match result {
            Ok(m) => m,
            Err(e) => {
                error!(label, error = %e, "Kafka recv failed in poll task");
                continue;
            }
        };

        let envelope = match parse_envelope(&message) {
            Ok(e) => e,
            Err(e) => {
                warn!(
                    label,
                    error = %e,
                    topic = message.topic(),
                    partition = message.partition(),
                    offset = message.offset(),
                    "Skipping invalid Kafka envelope"
                );
                decode_metric(worker);
                let _ = reader.commit_message(&message).await;
                continue;
            }
        };

        let started = Instant::now();
        if let Err(e) = handler(envelope).await {
            error!(label, error = %e, "Kafka handler failed");
        } else if let Err(e) = reader.commit_message(&message).await {
            error!(label, error = %e, "Kafka commit failed");
        }

        let elapsed = started.elapsed();
        if elapsed >= Duration::from_secs(30) {
            warn!(
                label,
                elapsed_ms = elapsed.as_millis() as u64,
                "Slow catalog Kafka handler (poll task stays independent)"
            );
        }
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
            kafka_recover(reader, topics, label).await;
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
            let _ = reader.commit_message(&message).await;
            return;
        }
    };

    if let Err(e) = handler(envelope).await {
        error!(label, error = %e, "Kafka handler failed");
    }
    if let Err(e) = reader.commit_message(&message).await {
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
