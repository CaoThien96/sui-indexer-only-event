use anyhow::{Context, Result};
use serde_json::Value;

use crate::bot::debug_log::agent_log;
use crate::bot::state::Dex;
use crate::bot::token_type::{normalize_coin_type, pick_non_sui_token, symbol_from_coin_type};
use crate::provider::SuiRpcClient;

#[derive(Debug, Clone)]
pub struct CreatePoolEvent {
    pub pool: String,
    pub token: String,
    pub symbol: String,
    pub reserve: u128,
}

pub async fn parse_create_pool(
    rpc: &SuiRpcClient,
    dex: Dex,
    parsed_json: &Value,
) -> Result<Option<CreatePoolEvent>> {
    let pool = match dex {
        Dex::Cetus => parsed_json
            .get("pool_id")
            .or_else(|| parsed_json.get("pool"))
            .and_then(|v| v.as_str())
            .map(str::to_string),
        Dex::Turbos => parsed_json
            .get("pool")
            .and_then(|v| v.as_str())
            .map(str::to_string),
    };

    let Some(pool) = pool else {
        return Ok(None);
    };

    let token = if dex == Dex::Cetus {
        let coin_a = parsed_json
            .get("coin_type_a")
            .and_then(|v| v.as_str());
        let coin_b = parsed_json
            .get("coin_type_b")
            .and_then(|v| v.as_str());
        pick_non_sui_token(coin_a, coin_b)
    } else {
        rpc.get_pool_token_type(&pool).await.ok()
    };

    let Some(token) = token else {
        return Ok(None);
    };

    if crate::bot::token_type::is_sui_coin_type(&token) {
        return Ok(None);
    }

    let reserve = match rpc.get_pool_coin_b(&pool).await {
        Ok(v) => v,
        Err(err) => {
            // #region agent log
            agent_log(
                "H1",
                "create_pool.rs:reserve_rpc_err",
                "get_pool_coin_b failed",
                serde_json::json!({
                    "pool": pool,
                    "error": err.to_string(),
                }),
            );
            // #endregion
            0
        }
    };
    // #region agent log
    agent_log(
        "H1",
        "create_pool.rs:reserve",
        "create pool reserve from rpc",
        serde_json::json!({
            "pool": pool,
            "token": normalize_coin_type(&token),
            "reserve": reserve.to_string(),
        }),
    );
    // #endregion
    let symbol = symbol_from_coin_type(&token);

    Ok(Some(CreatePoolEvent {
        pool,
        token: normalize_coin_type(&token),
        symbol,
        reserve,
    }))
}
