use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Timelike, Utc};
use event_bindings::{decode_parsed_json, protocol::Protocol};
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::coin_type::{SUI_COIN_TYPE, USDC_COIN_TYPE, normalize};
use crate::swap_normalizer::{
    assign_quote_base, extract_swap_fields, map_amounts_to_base_quote, raw_to_decimal,
};
use crate::oracle::scan::CheckpointEvent;

const SUI_DECIMALS: u32 = 9;
const USDC_DECIMALS: u32 = 6;

#[derive(Debug, Clone)]
pub struct SuiUsdcSwapObservation {
    pub bucket: DateTime<Utc>,
    pub checkpoint_seq: u64,
    pub pool_id: String,
    pub sui_amount: Decimal,
    pub usdc_amount: Decimal,
}

pub fn is_sui_usdc_pool(coin_a: &str, coin_b: &str) -> bool {
    let a = normalize(coin_a);
    let b = normalize(coin_b);
    (a == SUI_COIN_TYPE && b == USDC_COIN_TYPE) || (a == USDC_COIN_TYPE && b == SUI_COIN_TYPE)
}

pub fn extract_sui_usdc_observation(
    event: &CheckpointEvent<'_>,
    protocol: Protocol,
    coin_a: &str,
    coin_b: &str,
) -> Result<Option<SuiUsdcSwapObservation>> {
    if !is_sui_usdc_pool(coin_a, coin_b) {
        return Ok(None);
    }

    let parsed = decode_parsed_json(&event.event_type, event.bcs)
        .context("decode swap event bcs")?;
    let fields = extract_swap_fields(protocol, &parsed)?;
    let (base_coin_type, quote_coin_type, _) = assign_quote_base(coin_a, coin_b);

    if base_coin_type != USDC_COIN_TYPE || quote_coin_type != SUI_COIN_TYPE {
        return Ok(None);
    }

    let (amount_base_raw, amount_quote_raw) = map_amounts_to_base_quote(
        fields.a_to_b,
        coin_a,
        coin_b,
        &base_coin_type,
        &fields.amount_in_raw,
        &fields.amount_out_raw,
    );

    let usdc_amount =
        Decimal::from_str(&raw_to_decimal(&amount_base_raw, USDC_DECIMALS)?).context("usdc")?;
    let sui_amount =
        Decimal::from_str(&raw_to_decimal(&amount_quote_raw, SUI_DECIMALS)?).context("sui")?;

    if sui_amount.is_zero() || usdc_amount.is_zero() {
        return Ok(None);
    }

    let bucket = minute_bucket(event.timestamp_ms)?;

    Ok(Some(SuiUsdcSwapObservation {
        bucket,
        checkpoint_seq: event.checkpoint_sequence_number,
        pool_id: fields.pool_id,
        sui_amount,
        usdc_amount,
    }))
}

pub fn minute_bucket(timestamp_ms: u64) -> Result<DateTime<Utc>> {
    let secs = (timestamp_ms / 1000) as i64;
    let dt = Utc
        .timestamp_opt(secs, 0)
        .single()
        .context("invalid timestamp_ms")?;
    Ok(dt - chrono::Duration::seconds(dt.second() as i64) - chrono::Duration::nanoseconds(dt.nanosecond() as i64))
}
