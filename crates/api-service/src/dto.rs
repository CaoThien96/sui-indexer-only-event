use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct AmountQuote {
    pub amount: String,
    pub quote: String,
}

#[derive(Debug, Serialize)]
pub struct TokenDetailResponse {
    pub coin_type: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: i16,
    pub image_url: Option<String>,
    pub price_usd: Option<String>,
    pub price_quote: Option<AmountQuote>,
    pub volume_24h: Option<AmountQuote>,
    pub txns_24h: Option<i64>,
    pub holder_count: Option<String>,
    pub pools_count: i64,
}

#[derive(Debug, Serialize)]
pub struct PoolSummary {
    pub pool_id: String,
    pub protocol: String,
    pub coin_type_a: String,
    pub coin_type_b: String,
    pub tvl_quote: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TokenPoolsResponse {
    pub coin_type: String,
    pub pools: Vec<PoolSummary>,
}

#[derive(Debug, Serialize)]
pub struct OhlcBarDto {
    pub bucket: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume_quote: String,
    pub trade_count: i32,
}

#[derive(Debug, Serialize)]
pub struct OhlcResponse {
    pub pool_id: String,
    pub interval: String,
    pub bars: Vec<OhlcBarDto>,
}

#[derive(Debug, Serialize)]
pub struct SwapDto {
    pub time: String,
    pub tx_digest: String,
    pub event_seq: i32,
    pub protocol: String,
    pub pool_id: String,
    pub amount_base: String,
    pub amount_quote: String,
    pub price_quote_per_base: String,
}

#[derive(Debug, Serialize)]
pub struct SwapsResponse {
    pub coin_type: String,
    pub swaps: Vec<SwapDto>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
