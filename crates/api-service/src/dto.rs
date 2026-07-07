use chrono::{DateTime, TimeDelta, Utc};
use serde::Serialize;
use std::collections::HashMap;

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
pub struct TokenListItem {
    pub coin_type: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: i16,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TokenListResponse {
    pub tokens: Vec<TokenListItem>,
    pub next_cursor: Option<String>,
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
    pub source_type: Option<String>,
    pub is_stale: Option<bool>,
    pub confidence_score: Option<String>,
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
pub struct TokenOhlcResponse {
    pub coin_type: String,
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

/// Normalize OHLC bars: carry-forward opens (`open[n] = close[n-1]`)
/// and fill missing buckets with flat candles.
pub fn normalize_ohlc_bars(bars: Vec<OhlcBarDto>, interval: &str) -> Vec<OhlcBarDto> {
    if bars.len() < 2 {
        return bars;
    }

    let step = match interval {
        "1m" => TimeDelta::minutes(1),
        "5m" => TimeDelta::minutes(5),
        "15m" => TimeDelta::minutes(15),
        "30m" => TimeDelta::minutes(30),
        "1h" => TimeDelta::hours(1),
        "4h" => TimeDelta::hours(4),
        "24h" => TimeDelta::hours(24),
        _ => return bars,
    };

    let parse = |s: &str| -> Option<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    };

    let first_bucket = match parse(&bars.first().unwrap().bucket) {
        Some(t) => t,
        None => return bars,
    };
    let last_bucket = match parse(&bars.last().unwrap().bucket) {
        Some(t) => t,
        None => return bars,
    };

    let bar_map: HashMap<DateTime<Utc>, OhlcBarDto> = bars
        .into_iter()
        .filter_map(|b| parse(&b.bucket).map(|t| (t, b)))
        .collect();

    let mut result = Vec::new();
    let mut last_close: Option<String> = None;
    let mut t = first_bucket;

    while t <= last_bucket {
        let rfc = t.to_rfc3339();
        if let Some(bar) = bar_map.get(&t) {
            result.push(OhlcBarDto {
                bucket: rfc,
                open: last_close.clone().unwrap_or_else(|| bar.open.clone()),
                high: bar.high.clone(),
                low: bar.low.clone(),
                close: bar.close.clone(),
                volume_quote: bar.volume_quote.clone(),
                trade_count: bar.trade_count,
            });
            last_close = Some(bar.close.clone());
        } else if let Some(ref close) = last_close {
            result.push(OhlcBarDto {
                bucket: rfc,
                open: close.clone(),
                high: close.clone(),
                low: close.clone(),
                close: close.clone(),
                volume_quote: "0".to_string(),
                trade_count: 0,
            });
        }
        t += step;
    }

    result
}
