use anyhow::Result;
use serde_json::Value;

use crate::bot::event_id::format_event_id;
use crate::bot::state::{Dex, ParsedSwap};

pub fn parse_swap(
    dex: Dex,
    parsed_json: &Value,
    tx_digest: &str,
    event_seq: &str,
    maker: &str,
) -> Result<Option<ParsedSwap>> {
    let pool = parsed_json
        .get("pool")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let Some(pool) = pool else {
        return Ok(None);
    };

    let (is_buy, sui_amount, token_amount) = match dex {
        Dex::Cetus => {
            let atob = parsed_json.get("atob").and_then(|v| v.as_bool()).unwrap_or(false);
            let amount_in = json_u128(parsed_json.get("amount_in"));
            let amount_out = json_u128(parsed_json.get("amount_out"));
            let is_buy = !atob;
            let sui_amount = if atob { amount_out } else { amount_in };
            let token_amount = if atob { amount_in } else { amount_out };
            (is_buy, sui_amount, token_amount)
        }
        Dex::Turbos => {
            let a_to_b = parsed_json
                .get("a_to_b")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let amount_a = json_u128(parsed_json.get("amount_a"));
            let amount_b = json_u128(parsed_json.get("amount_b"));
            let is_buy = !a_to_b;
            (is_buy, amount_b, amount_a)
        }
    };

    Ok(Some(ParsedSwap {
        event_id: format_event_id(tx_digest, event_seq),
        tx_digest: tx_digest.to_string(),
        event_seq: event_seq.to_string(),
        pool,
        is_buy,
        sui_amount,
        token_amount,
        maker: maker.to_string(),
        dex,
    }))
}

fn json_u128(value: Option<&Value>) -> u128 {
    value
        .and_then(|v| v.as_str().and_then(|s| s.parse().ok()))
        .or_else(|| value.and_then(|v| v.as_u64().map(|n| n as u128)))
        .unwrap_or(0)
}
