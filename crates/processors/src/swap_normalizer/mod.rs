mod hydration;
mod price;
mod protocol;

pub use hydration::{PoolHydrator, defer_backoff};
pub use price::{
    assign_quote_base, map_amounts_to_base_quote, price_from_trade_amounts, raw_to_decimal,
};
pub use protocol::{ExtractedSwapFields, extract_swap_fields};

use std::num::NonZeroUsize;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::TimeZone;
use indexer_store::{FactTopic, MessageEnvelope};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use tracing::warn;

use crate::catalog::validate_protocol_slug;
use crate::metrics::ProcessorMetrics;
use crate::store::CatalogStore;
use crate::sui_grpc::SuiGrpcClient;
use crate::timescale::TimescaleStore;
use crate::usd_enrichment::{UsdEnrichmentOutcome, enrich_swap_usd};

use hydration::{DecimalsOutcome, PoolResolution};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeferReason {
    PoolRpc,
    MetadataRpc,
    DbError,
    OracleMissing,
}

impl DeferReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PoolRpc => "pool_rpc",
            Self::MetadataRpc => "metadata_rpc",
            Self::DbError => "db_error",
            Self::OracleMissing => "oracle_missing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipReason {
    MissingPoolPermanent,
    BadParse,
    HydrationDisabled,
}

impl SkipReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingPoolPermanent => "missing_pool_permanent",
            Self::BadParse => "bad_parse",
            Self::HydrationDisabled => "hydration_disabled",
        }
    }
}

#[derive(Debug)]
pub enum NormalizeOutcome {
    Published(MessageEnvelope),
    Deferred { reason: DeferReason },
    SkippedPermanent { reason: SkipReason },
}

#[derive(Debug, Clone)]
pub struct HydrationConfig {
    pub enabled: bool,
    pub pool_cache_size: usize,
    pub defer_max_retries: u32,
    pub defer_backoff_ms: u64,
}

pub struct SwapNormalizer {
    hydrator: PoolHydrator,
    timescale: TimescaleStore,
    metrics: Arc<ProcessorMetrics>,
}

impl SwapNormalizer {
    pub fn new(
        catalog: CatalogStore,
        timescale: TimescaleStore,
        metrics: Arc<ProcessorMetrics>,
        grpc: Arc<SuiGrpcClient>,
        config: HydrationConfig,
    ) -> Self {
        let cache_size =
            NonZeroUsize::new(config.pool_cache_size.max(1)).unwrap_or(NonZeroUsize::MIN);
        Self {
            hydrator: PoolHydrator::new(
                catalog,
                metrics.clone(),
                grpc,
                config,
                lru::LruCache::new(cache_size),
            ),
            timescale,
            metrics,
        }
    }

    pub async fn normalize(&self, envelope: &MessageEnvelope) -> Result<NormalizeOutcome> {
        let payload = &envelope.payload;
        let protocol_slug = match payload.get("protocol").and_then(Value::as_str) {
            Some(slug) => slug,
            None => {
                return Ok(NormalizeOutcome::SkippedPermanent {
                    reason: SkipReason::BadParse,
                });
            }
        };
        let protocol = match validate_protocol_slug(protocol_slug) {
            Some(p) => p,
            None => {
                return Ok(NormalizeOutcome::SkippedPermanent {
                    reason: SkipReason::BadParse,
                });
            }
        };

        let parsed = match payload.get("parsed_json") {
            Some(v) => v,
            None => {
                return Ok(NormalizeOutcome::SkippedPermanent {
                    reason: SkipReason::BadParse,
                });
            }
        };
        let fields = match extract_swap_fields(protocol, parsed) {
            Ok(f) => f,
            Err(_) => {
                return Ok(NormalizeOutcome::SkippedPermanent {
                    reason: SkipReason::BadParse,
                });
            }
        };

        let checkpoint_seq = payload
            .get("checkpoint_sequence_number")
            .and_then(Value::as_u64)
            .map(|v| v as i64);
        let timestamp_ms = payload
            .get("timestamp_ms")
            .and_then(Value::as_u64)
            .map(|v| v as i64);

        let pool = match self
            .hydrator
            .resolve_pool(
                &fields.pool_id,
                protocol_slug,
                checkpoint_seq,
                timestamp_ms,
            )
            .await?
        {
            PoolResolution::Found(pool) => pool,
            PoolResolution::MissingPermanent(reason) => {
                return Ok(NormalizeOutcome::SkippedPermanent { reason });
            }
            PoolResolution::Deferred(reason) => {
                return Ok(NormalizeOutcome::Deferred { reason });
            }
        };

        let (base_coin_type, quote_coin_type, _quote_type) =
            assign_quote_base(&pool.coin_type_a, &pool.coin_type_b);

        let decimals_outcome = self
            .hydrator
            .resolve_pair_decimals(
                &pool.coin_type_a,
                &pool.coin_type_b,
                checkpoint_seq,
            )
            .await?;
        let (decimals_a, decimals_b) = match decimals_outcome {
            DecimalsOutcome::Ready(a, b) => (a, b),
            DecimalsOutcome::Deferred(reason) => {
                return Ok(NormalizeOutcome::Deferred { reason });
            }
        };

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

        let amount_base_decimal = raw_to_decimal(&amount_base_raw, base_decimals)
            .context("amount_base_decimal")?;
        let amount_quote_decimal = raw_to_decimal(&amount_quote_raw, quote_decimals)
            .context("amount_quote_decimal")?;
        let price_quote_per_base =
            price_from_trade_amounts(&amount_quote_decimal, &amount_base_decimal)?;

        let timestamp_ms = timestamp_ms.context("swap missing timestamp_ms")?;
        let swap_time = chrono::Utc
            .timestamp_millis_opt(timestamp_ms)
            .single()
            .context("invalid timestamp_ms")?;
        let price_dec = Decimal::from_str(&price_quote_per_base).context("price_quote_per_base")?;
        let amount_quote_dec =
            Decimal::from_str(&amount_quote_decimal).context("amount_quote_decimal")?;

        let usd_outcome = enrich_swap_usd(
            &self.timescale,
            &quote_coin_type,
            price_dec,
            amount_quote_dec,
            swap_time,
        )
        .await?;

        let usd_fields = match usd_outcome {
            UsdEnrichmentOutcome::Enriched(usd) => Some(usd),
            UsdEnrichmentOutcome::NotApplicable => None,
            UsdEnrichmentOutcome::OracleMissing => {
                self.metrics
                    .swap_deferred
                    .with_label_values(&[DeferReason::OracleMissing.as_str()])
                    .inc();
                warn!(
                    swap_time = %swap_time,
                    quote_coin_type = %quote_coin_type,
                    "deferring swap: SUI/USD oracle missing for bucket"
                );
                return Ok(NormalizeOutcome::Deferred {
                    reason: DeferReason::OracleMissing,
                });
            }
        };

        let event_seq = payload
            .get("event_sequence_in_transaction")
            .and_then(Value::as_u64)
            .unwrap_or(0) as u32;
        let tx_digest = payload
            .get("tx_digest")
            .and_then(Value::as_str)
            .context("swap missing tx_digest")?;

        let mut normalized = json!({
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
            "amount_base_decimal": amount_base_decimal,
            "amount_quote_decimal": amount_quote_decimal,
            "vault_a_raw": fields.vault_a_raw,
            "vault_b_raw": fields.vault_b_raw,
            "checkpoint_sequence_number": payload.get("checkpoint_sequence_number"),
            "timestamp_ms": payload.get("timestamp_ms"),
            "tx_digest": tx_digest,
            "event_seq": event_seq,
            "sender": payload.get("sender"),
        });

        if let Some(usd) = usd_fields {
            let obj = normalized
                .as_object_mut()
                .context("normalized payload must be object")?;
            obj.insert(
                "price_usd_per_base".to_string(),
                json!(usd.price_usd_per_base.to_string()),
            );
            obj.insert(
                "amount_usd".to_string(),
                json!(usd.amount_usd.to_string()),
            );
        }

        let message_id_key = format!(
            "{tx_digest}:{event_seq}:{protocol_slug}:{}",
            FactTopic::SwapNormalized.as_str()
        );
        Ok(NormalizeOutcome::Published(MessageEnvelope::new(
            &message_id_key,
            normalized,
        )))
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
