use anyhow::Result;
use serde_json::Value;

use crate::bot::state::Dex;
use crate::bot::token_type::{normalize_coin_type, symbol_from_coin_type};
use crate::provider::SuiRpcClient;

#[derive(Debug, Clone)]
pub struct AddLiquiditySnipEvent {
    pub pool: String,
    pub token: String,
    pub symbol: String,
    pub reserve_sui: u128,
}

pub async fn parse_add_liquidity_for_snip(
    rpc: &SuiRpcClient,
    dex: Dex,
    parsed_json: &Value,
) -> Result<Option<AddLiquiditySnipEvent>> {
    match dex {
        Dex::Cetus => {
            let Some(pool) = parsed_json.get("pool").and_then(|v| v.as_str()) else {
                return Ok(None);
            };
            let reserve_sui = json_u128(parsed_json.get("amount_b"));
            if reserve_sui == 0 {
                return Ok(None);
            }
            let Some(token) = rpc.get_pool_token_type(pool).await.ok() else {
                return Ok(None);
            };
            if crate::bot::token_type::is_sui_coin_type(&token) {
                return Ok(None);
            }
            let symbol = symbol_from_coin_type(&token);
            let token = normalize_coin_type(&token);
            Ok(Some(AddLiquiditySnipEvent {
                pool: pool.to_string(),
                token,
                symbol,
                reserve_sui,
            }))
        }
        Dex::Turbos => Ok(None),
    }
}

fn json_u128(value: Option<&Value>) -> u128 {
    value
        .and_then(|v| v.as_str().and_then(|s| s.parse().ok()))
        .or_else(|| value.and_then(|v| v.as_u64().map(|n| n as u128)))
        .unwrap_or(0)
}
