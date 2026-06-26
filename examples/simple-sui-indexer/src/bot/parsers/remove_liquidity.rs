use anyhow::Result;
use serde_json::Value;

use crate::bot::state::Dex;
use crate::provider::SuiRpcClient;

#[derive(Debug, Clone)]
pub struct RemoveLiquidityEvent {
    pub pool: String,
    pub token: String,
    pub symbol: String,
}

pub async fn parse_remove_liquidity(
    rpc: &SuiRpcClient,
    dex: Dex,
    parsed_json: &Value,
) -> Result<Option<RemoveLiquidityEvent>> {
    let pool = parsed_json
        .get("pool")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let Some(pool) = pool else {
        return Ok(None);
    };

    let token = rpc.get_pool_token_type(&pool).await?;
    let symbol = token.split("::").last().unwrap_or("TOKEN").to_string();
    let _ = dex;

    Ok(Some(RemoveLiquidityEvent {
        pool,
        token,
        symbol,
    }))
}
