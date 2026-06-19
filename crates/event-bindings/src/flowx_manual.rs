//! Manual BCS decode for FlowX CLMM events.
//!
//! `move_contract!` cannot codegen the `pool` / `pool_manager` modules because
//! non-event structs (e.g. `PoolRegistry`) reference `sui::table::Table`.

use anyhow::Result;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::parsed_json;

#[derive(Debug, Deserialize)]
struct FlowxI32 {
    bits: u32,
}

#[derive(Debug, Deserialize)]
struct FlowxTypeName {
    name: String,
}

#[derive(Debug, Deserialize)]
struct FlowxSwap {
    sender: [u8; 32],
    pool_id: [u8; 32],
    x_for_y: bool,
    amount_x: u64,
    amount_y: u64,
    sqrt_price_before: u128,
    sqrt_price_after: u128,
    liquidity: u128,
    tick_index: FlowxI32,
    fee_amount: u64,
}

#[derive(Debug, Deserialize)]
struct FlowxPoolCreated {
    sender: [u8; 32],
    pool_id: [u8; 32],
    coin_type_x: FlowxTypeName,
    coin_type_y: FlowxTypeName,
    fee_rate: u64,
    tick_spacing: u32,
}

fn format_address(bytes: [u8; 32]) -> String {
    format!("0x{}", hex::encode(bytes))
}

pub fn decode_swap(bcs: &[u8]) -> Result<Value> {
    let decoded: FlowxSwap = bcs::from_bytes(bcs)?;
    Ok(parsed_json::normalize(json!({
        "sender": format_address(decoded.sender),
        "pool_id": format_address(decoded.pool_id),
        "x_for_y": decoded.x_for_y,
        "amount_x": decoded.amount_x,
        "amount_y": decoded.amount_y,
        // u128 exceeds JSON Number range — stringify before json! (normalize also stringifies)
        "sqrt_price_before": decoded.sqrt_price_before.to_string(),
        "sqrt_price_after": decoded.sqrt_price_after.to_string(),
        "liquidity": decoded.liquidity.to_string(),
        "tick_index": { "bits": decoded.tick_index.bits },
        "fee_amount": decoded.fee_amount,
    })))
}

pub fn decode_pool_created(bcs: &[u8]) -> Result<Value> {
    let decoded: FlowxPoolCreated = bcs::from_bytes(bcs)?;
    Ok(parsed_json::normalize(json!({
        "sender": format_address(decoded.sender),
        "pool_id": format_address(decoded.pool_id),
        "coin_type_x": { "name": decoded.coin_type_x.name },
        "coin_type_y": { "name": decoded.coin_type_y.name },
        "fee_rate": decoded.fee_rate,
        "tick_spacing": decoded.tick_spacing,
    })))
}
