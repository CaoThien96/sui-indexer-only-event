use anyhow::{Context, Result};
use event_bindings::protocol::Protocol;
use indexer_store::MessageEnvelope;
use serde_json::Value;
use tracing::warn;

use crate::coin_type;
use crate::metrics::ProcessorMetrics;
use crate::store::CatalogStore;

const WATCHLIST_SOURCE: &str = "pool_discovery";

pub async fn handle_pool_message(
    store: &CatalogStore,
    metrics: &ProcessorMetrics,
    envelope: &MessageEnvelope,
) -> Result<()> {
    let payload = &envelope.payload;
    let pool_id = payload
        .get("pool_id")
        .and_then(Value::as_str)
        .context("pool fact missing pool_id")?;
    let protocol = payload
        .get("protocol")
        .and_then(Value::as_str)
        .context("pool fact missing protocol")?;
    let coin_type_a = payload.get("coin_type_a").and_then(Value::as_str);
    let coin_type_b = payload.get("coin_type_b").and_then(Value::as_str);

    let (Some(coin_type_a), Some(coin_type_b)) = (coin_type_a, coin_type_b) else {
        log_skip(
            metrics,
            "missing_coin_types",
            &format!("pool_id={pool_id} protocol={protocol}"),
        );
        return Ok(());
    };
    let tick_spacing = payload
        .get("tick_spacing")
        .and_then(|v| v.as_u64().or_else(|| v.as_i64().map(|n| n as u64)))
        .map(|v| v as i32);
    let created_at_ms = payload
        .get("timestamp_ms")
        .and_then(Value::as_u64)
        .map(|v| v as i64);
    let created_cp = payload
        .get("checkpoint_sequence_number")
        .and_then(Value::as_u64)
        .map(|v| v as i64);
    let created_tx = payload.get("tx_digest").and_then(Value::as_str);

    let inserted = store
        .upsert_pool(
            pool_id,
            protocol,
            coin_type_a,
            coin_type_b,
            tick_spacing,
            created_at_ms,
            created_tx,
            created_cp,
        )
        .await?;

    if inserted {
        metrics.catalog_rows_upserted.with_label_values(&["pools"]).inc();
    }

    for coin in [coin_type_a, coin_type_b] {
        if store.seed_watchlist(coin, WATCHLIST_SOURCE).await? {
            metrics
                .catalog_rows_upserted
                .with_label_values(&["watchlist"])
                .inc();
        }
    }

    Ok(())
}

pub async fn handle_token_metadata_message(
    store: &CatalogStore,
    metrics: &ProcessorMetrics,
    envelope: &MessageEnvelope,
) -> Result<()> {
    let payload = &envelope.payload;
    let coin_type = payload
        .get("coin_type")
        .and_then(Value::as_str)
        .context("token metadata missing coin_type")?;
    let name = payload.get("name").and_then(Value::as_str);
    let symbol = payload.get("symbol").and_then(Value::as_str);
    let decimals = payload
        .get("decimals")
        .and_then(|v| v.as_u64().or_else(|| v.as_i64().map(|n| n as u64)))
        .context("token metadata missing decimals")? as i16;
    let description = payload.get("description").and_then(Value::as_str);
    let image_url = payload.get("image_url").and_then(Value::as_str);
    let creator = payload.get("creator").and_then(Value::as_str);
    let created_at_ms = payload
        .get("created_at_ms")
        .and_then(Value::as_u64)
        .map(|v| v as i64);
    let first_seen_cp = payload
        .get("checkpoint_sequence_number")
        .and_then(Value::as_u64)
        .map(|v| v as i64);

    store
        .upsert_token(
            coin_type,
            name,
            symbol,
            decimals,
            description,
            image_url,
            creator,
            created_at_ms,
            first_seen_cp,
        )
        .await?;

    metrics
        .catalog_rows_upserted
        .with_label_values(&["tokens"])
        .inc();

    Ok(())
}

pub fn validate_protocol_slug(slug: &str) -> Option<Protocol> {
    Protocol::ALL.iter().copied().find(|p| p.as_str() == slug)
}

pub fn log_skip(metrics: &ProcessorMetrics, reason: &str, detail: &str) {
    metrics.swap_skipped.with_label_values(&[reason]).inc();
    warn!(reason, detail, "Skipped catalog/normalizer message");
}

pub fn normalized_coin_types(coin_a: &str, coin_b: &str) -> (String, String) {
    (
        coin_type::normalize(coin_a),
        coin_type::normalize(coin_b),
    )
}
