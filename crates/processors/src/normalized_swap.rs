use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use indexer_store::MessageEnvelope;
use rust_decimal::Decimal;
use serde_json::Value;
use std::str::FromStr;

use crate::coin_type;

#[derive(Debug, Clone)]
pub struct NormalizedSwap {
    pub protocol: String,
    pub pool_id: String,
    pub base_coin_type: String,
    pub quote_coin_type: String,
    pub coin_type_a: String,
    pub coin_type_b: String,
    pub amount_base: Decimal,
    pub amount_quote: Decimal,
    pub price_quote_per_base: Decimal,
    pub fee_amount: Option<Decimal>,
    pub vault_a_raw: Option<String>,
    pub vault_b_raw: Option<String>,
    pub time: DateTime<Utc>,
    pub timestamp_ms: i64,
    pub tx_digest: String,
    pub event_seq: i32,
    pub sender: Option<String>,
    pub checkpoint_seq: i64,
    pub swap_key: String,
}

pub fn parse_normalized_swap(envelope: &MessageEnvelope) -> Result<NormalizedSwap> {
    let p = &envelope.payload;
    let protocol = str_field(p, "protocol")?;
    let pool_id = str_field(p, "pool_id")?;
    let base_coin_type = coin_type::normalize(&str_field(p, "base_coin_type")?);
    let quote_coin_type = coin_type::normalize(&str_field(p, "quote_coin_type")?);
    let coin_type_a = coin_type::normalize(&str_field(p, "coin_type_a")?);
    let coin_type_b = coin_type::normalize(&str_field(p, "coin_type_b")?);
    let amount_base = decimal_field(p, "amount_base_decimal")?;
    let amount_quote = decimal_field(p, "amount_quote_decimal")?;
    let price_quote_per_base = decimal_field(p, "price_quote_per_base")?;
    let timestamp_ms = p
        .get("timestamp_ms")
        .and_then(Value::as_i64)
        .context("missing timestamp_ms")?;
    let time = Utc
        .timestamp_millis_opt(timestamp_ms)
        .single()
        .context("invalid timestamp_ms")?;
    let tx_digest = str_field(p, "tx_digest")?;
    let event_seq = p
        .get("event_seq")
        .or_else(|| p.get("event_sequence_in_transaction"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let checkpoint_seq = p
        .get("checkpoint_sequence_number")
        .and_then(Value::as_i64)
        .context("missing checkpoint_sequence_number")?;
    let fee_amount = optional_decimal_field(p, "fee_amount_raw").ok();
    let vault_a_raw = optional_str_field(p, "vault_a_raw");
    let vault_b_raw = optional_str_field(p, "vault_b_raw");
    let sender = p.get("sender").and_then(Value::as_str).map(str::to_string);
    let swap_key = format!("{tx_digest}:{event_seq}:{protocol}");

    Ok(NormalizedSwap {
        protocol,
        pool_id,
        base_coin_type,
        quote_coin_type,
        coin_type_a,
        coin_type_b,
        amount_base,
        amount_quote,
        price_quote_per_base,
        fee_amount,
        vault_a_raw,
        vault_b_raw,
        time,
        timestamp_ms,
        tx_digest,
        event_seq,
        sender,
        checkpoint_seq,
        swap_key,
    })
}

fn str_field(payload: &Value, key: &str) -> Result<String> {
    payload
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .with_context(|| format!("missing field `{key}`"))
}

fn optional_str_field(payload: &Value, key: &str) -> Option<String> {
    payload
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn decimal_field(payload: &Value, key: &str) -> Result<Decimal> {
    let raw = payload
        .get(key)
        .and_then(|v| v.as_str().map(str::to_string).or_else(|| v.as_i64().map(|n| n.to_string())))
        .with_context(|| format!("missing field `{key}`"))?;
    Decimal::from_str(&raw).with_context(|| format!("invalid decimal `{key}`"))
}

fn optional_decimal_field(payload: &Value, key: &str) -> Result<Decimal> {
    let Some(raw) = payload.get(key) else {
        return Err(anyhow::anyhow!("missing {key}"));
    };
    let s = match raw {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => return Err(anyhow::anyhow!("invalid {key}")),
    };
    Decimal::from_str(&s).context("invalid decimal")
}

pub fn minute_bucket(timestamp_ms: i64) -> DateTime<Utc> {
    let floored = (timestamp_ms / 60_000) * 60_000;
    Utc.timestamp_millis_opt(floored)
        .single()
        .unwrap_or_else(Utc::now)
}
