mod price;
mod protocol;

pub use price::{assign_quote_base, map_amounts_to_base_quote, raw_to_decimal, sqrt_price_to_quote_per_base};
pub use protocol::{ExtractedSwapFields, extract_swap_fields};

use anyhow::{Context, Result};
use indexer_store::{FactTopic, MessageEnvelope};
use lru::LruCache;
use serde_json::{Value, json};
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::catalog::validate_protocol_slug;
use crate::metrics::ProcessorMetrics;
use crate::store::{CatalogStore, PoolRow};

pub struct SwapNormalizer {
    store: CatalogStore,
    metrics: Arc<ProcessorMetrics>,
    pool_cache: Mutex<LruCache<String, PoolRow>>,
}

impl SwapNormalizer {
    pub fn new(store: CatalogStore, metrics: Arc<ProcessorMetrics>) -> Self {
        Self {
            store,
            metrics,
            pool_cache: Mutex::new(LruCache::new(NonZeroUsize::new(10_000).unwrap())),
        }
    }

    pub async fn normalize(&self, envelope: &MessageEnvelope) -> Result<Option<MessageEnvelope>> {
        let payload = &envelope.payload;
        let protocol_slug = payload
            .get("protocol")
            .and_then(Value::as_str)
            .context("swap missing protocol")?;
        let protocol = validate_protocol_slug(protocol_slug)
            .with_context(|| format!("unknown protocol `{protocol_slug}`"))?;

        let parsed = payload
            .get("parsed_json")
            .context("swap missing parsed_json")?;
        let fields = extract_swap_fields(protocol, parsed)?;

        let pool = self.get_pool(&fields.pool_id).await?;
        let Some(pool) = pool else {
            self.metrics.swap_missing_pool.inc();
            return Ok(None);
        };

        let (base_coin_type, quote_coin_type, quote_type) =
            assign_quote_base(&pool.coin_type_a, &pool.coin_type_b);

        let decimals_a = self.decimals_for(&pool.coin_type_a).await;
        let decimals_b = self.decimals_for(&pool.coin_type_b).await;

        let quote_is_a = quote_type == crate::coin_type::normalize(&pool.coin_type_a);
        let price_quote_per_base = sqrt_price_to_quote_per_base(
            &fields.sqrt_price_after,
            decimals_a,
            decimals_b,
            fields.a_to_b,
            quote_is_a,
        )?;

        let (amount_base_raw, amount_quote_raw) = map_amounts_to_base_quote(
            fields.a_to_b,
            &pool.coin_type_a,
            &pool.coin_type_b,
            &base_coin_type,
            &fields.amount_in_raw,
            &fields.amount_out_raw,
        );

        let base_decimals = if base_coin_type == crate::coin_type::normalize(&pool.coin_type_a) {
            decimals_a
        } else {
            decimals_b
        };
        let quote_decimals = if quote_coin_type == crate::coin_type::normalize(&pool.coin_type_a) {
            decimals_a
        } else {
            decimals_b
        };

        let event_seq = payload
            .get("event_sequence_in_transaction")
            .and_then(Value::as_u64)
            .unwrap_or(0) as u32;
        let tx_digest = payload
            .get("tx_digest")
            .and_then(Value::as_str)
            .context("swap missing tx_digest")?;

        let normalized = json!({
            "protocol": protocol_slug,
            "pool_id": fields.pool_id,
            "coin_type_a": pool.coin_type_a,
            "coin_type_b": pool.coin_type_b,
            "amount_in_raw": fields.amount_in_raw,
            "amount_out_raw": fields.amount_out_raw,
            "amount_in_decimal": raw_to_decimal(&fields.amount_in_raw, if fields.a_to_b { decimals_a } else { decimals_b })?,
            "amount_out_decimal": raw_to_decimal(&fields.amount_out_raw, if fields.a_to_b { decimals_b } else { decimals_a })?,
            "fee_amount_raw": fields.fee_amount_raw,
            "a_to_b": fields.a_to_b,
            "sqrt_price_before": fields.sqrt_price_before,
            "sqrt_price_after": fields.sqrt_price_after,
            "price_quote_per_base": price_quote_per_base,
            "quote_coin_type": quote_coin_type,
            "base_coin_type": base_coin_type,
            "amount_base_raw": amount_base_raw,
            "amount_quote_raw": amount_quote_raw,
            "amount_base_decimal": raw_to_decimal(&amount_base_raw, base_decimals)?,
            "amount_quote_decimal": raw_to_decimal(&amount_quote_raw, quote_decimals)?,
            "vault_a_raw": fields.vault_a_raw,
            "vault_b_raw": fields.vault_b_raw,
            "checkpoint_sequence_number": payload.get("checkpoint_sequence_number"),
            "timestamp_ms": payload.get("timestamp_ms"),
            "tx_digest": tx_digest,
            "event_seq": event_seq,
            "sender": payload.get("sender"),
        });

        let message_id_key = format!("{tx_digest}:{event_seq}:{protocol_slug}:{}", FactTopic::SwapNormalized.as_str());
        Ok(Some(MessageEnvelope::new(&message_id_key, normalized)))
    }

    async fn get_pool(&self, pool_id: &str) -> Result<Option<PoolRow>> {
        {
            let cache = self.pool_cache.lock().await;
            if let Some(row) = cache.peek(pool_id) {
                return Ok(Some(row.clone()));
            }
        }

        let row = self.store.get_pool(pool_id).await?;
        if let Some(ref pool) = row {
            self.pool_cache.lock().await.put(pool_id.to_string(), pool.clone());
        }
        Ok(row)
    }

    async fn decimals_for(&self, coin_type: &str) -> u32 {
        match self.store.get_token_decimals(coin_type).await {
            Ok(Some(d)) => d as u32,
            _ => {
                self.metrics
                    .swap_missing_decimals
                    .with_label_values(&[coin_type])
                    .inc();
                9
            }
        }
    }
}

pub fn normalized_partition_key(envelope: &MessageEnvelope) -> String {
    envelope
        .payload
        .get("pool_id")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| "unknown".to_string())
}
