use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sui_processors::coin_type;
use sui_processors::clickhouse;
use sui_processors::timescale::SwapRow;

use crate::dto::{
    AmountQuote, ErrorResponse, OhlcBarDto, PoolSummary, SwapDto, SwapsResponse,
    TokenDetailResponse, TokenOhlcResponse, TokenPoolsResponse, normalize_ohlc_bars,
};
use crate::query_router::{route_query, StorageTarget};
use crate::state::AppState;

pub async fn token_detail(
    State(state): State<AppState>,
    axum::extract::Path(coin_type): axum::extract::Path<String>,
) -> Response {
    let coin_type = coin_type::normalize(&coin_type);
    let route = "token_detail";

    let token = match state.get_token(&coin_type).await {
        Ok(Some(t)) => t,
        Ok(None) => return not_found(route, &state, "token not found"),
        Err(e) => return internal(route, &state, e),
    };

    let pools_count = match state.count_pools(&coin_type).await {
        Ok(c) => c,
        Err(e) => return internal(route, &state, e),
    };

    let (price_quote, volume_24h, txns_24h, price_usd, source_type, is_stale, confidence_score) =
        match metrics_for_token(&state, &coin_type).await {
            Ok(v) => v,
            Err(e) => return internal(route, &state, e),
        };

    Json(TokenDetailResponse {
        coin_type: token.coin_type,
        name: token.name,
        symbol: token.symbol,
        decimals: token.decimals,
        image_url: token.image_url,
        price_usd,
        price_quote,
        volume_24h,
        txns_24h,
        holder_count: None,
        pools_count,
        source_type,
        is_stale: Some(is_stale),
        confidence_score,
    })
    .into_response()
}

pub async fn token_pools(
    State(state): State<AppState>,
    axum::extract::Path(coin_type): axum::extract::Path<String>,
) -> Response {
    let coin_type = coin_type::normalize(&coin_type);
    let route = "token_pools";

    let pools = match state.list_pools(&coin_type, 200).await {
        Ok(p) => p,
        Err(e) => return internal(route, &state, e),
    };

    let mut summaries = Vec::with_capacity(pools.len());
    for pool in pools {
        let tvl_quote = state.redis_pool_tvl(&pool.pool_id).await.ok().flatten();
        summaries.push(PoolSummary {
            pool_id: pool.pool_id,
            protocol: pool.protocol,
            coin_type_a: pool.coin_type_a,
            coin_type_b: pool.coin_type_b,
            tvl_quote,
        });
    }

    Json(TokenPoolsResponse {
        coin_type,
        pools: summaries,
    })
    .into_response()
}

#[derive(Debug, Deserialize)]
pub struct DispatchQuery {
    pub pool_id: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub interval: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenOhlcQuery {
    pub interval: String,
    pub from: String,
    pub to: String,
}

/// Coin types contain `::` and must be captured with a trailing catch-all segment.
pub async fn token_dispatch(
    State(state): State<AppState>,
    axum::extract::Path(rest): axum::extract::Path<String>,
    Query(query): Query<DispatchQuery>,
) -> Response {
    if let Some(coin) = rest.strip_suffix("/swaps") {
        let coin = coin.trim_end_matches('/');
        return token_swaps(State(state), axum::extract::Path(coin.to_string()), Query(query)).await;
    }
    if let Some(coin) = rest.strip_suffix("/pools") {
        let coin = coin.trim_end_matches('/');
        return token_pools(State(state), axum::extract::Path(coin.to_string())).await;
    }
    if let Some(coin) = rest.strip_suffix("/ohlc") {
        let coin = coin.trim_end_matches('/');
        let ohlc_query = TokenOhlcQuery {
            interval: query.interval.clone().unwrap_or_else(|| "1h".to_string()),
            from: query
                .from
                .clone()
                .unwrap_or_else(|| (Utc::now() - chrono::Duration::days(1)).to_rfc3339()),
            to: query.to.clone().unwrap_or_else(|| Utc::now().to_rfc3339()),
        };
        return token_ohlc(
            State(state),
            axum::extract::Path(coin.to_string()),
            Query(ohlc_query),
        )
        .await;
    }
    token_detail(State(state), axum::extract::Path(rest)).await
}

pub async fn token_swaps(
    State(state): State<AppState>,
    axum::extract::Path(coin_type): axum::extract::Path<String>,
    Query(query): Query<DispatchQuery>,
) -> Response {
    let coin_type = coin_type::normalize(&coin_type);
    let _route = "token_swaps";
    let limit = query.limit.unwrap_or(50).clamp(1, 200);

    let before_time = query
        .cursor
        .as_deref()
        .and_then(parse_cursor_time);

    let from = Utc::now() - chrono::Duration::days(state.hot_storage_days + 365);
    let to = Utc::now();
    let target = route_query(from, to, state.hot_storage_days);

    let mut swaps = Vec::new();
    match target {
        StorageTarget::Hot | StorageTarget::Both => {
            if let Ok(rows) = state
                .timescale
                .list_swaps(
                    &coin_type,
                    query.pool_id.as_deref(),
                    limit + 1,
                    before_time,
                )
                .await
            {
                swaps.extend(rows);
            }
        }
        _ => {}
    }
    if matches!(target, StorageTarget::Cold | StorageTarget::Both) {
        if let Ok(rows) = clickhouse::query_swaps(
            &state.clickhouse,
            &coin_type,
            query.pool_id.as_deref(),
            from,
            to,
            (limit + 1) as u64,
        )
        .await
        {
            for r in rows {
                swaps.push(SwapRow {
                    time: r.time,
                    tx_digest: r.tx_digest,
                    event_seq: r.event_seq,
                    protocol: r.protocol,
                    pool_id: r.pool_id,
                    amount_base: r.amount_base,
                    amount_quote: r.amount_quote,
                    price_quote_per_base: r.price_quote_per_base,
                });
            }
        }
    }

    swaps.sort_by(|a, b| b.time.cmp(&a.time));
    swaps.dedup_by(|a, b| {
        a.time == b.time && a.tx_digest == b.tx_digest && a.event_seq == b.event_seq
    });
    swaps.truncate(limit as usize);

    let next_cursor = swaps.last().map(|s| format!("{}|{}", s.time.to_rfc3339(), s.tx_digest));

    let swap_dtos: Vec<SwapDto> = swaps
        .into_iter()
        .map(|s| SwapDto {
            time: s.time.to_rfc3339(),
            tx_digest: s.tx_digest,
            event_seq: s.event_seq,
            protocol: s.protocol,
            pool_id: s.pool_id,
            amount_base: s.amount_base,
            amount_quote: s.amount_quote,
            price_quote_per_base: s.price_quote_per_base,
        })
        .collect();

    Json(SwapsResponse {
        coin_type,
        swaps: swap_dtos,
        next_cursor,
    })
    .into_response()
}

async fn metrics_for_token(
    state: &AppState,
    coin_type: &str,
) -> anyhow::Result<(
    Option<AmountQuote>,
    Option<AmountQuote>,
    Option<i64>,
    Option<String>,
    Option<String>,
    bool,
    Option<String>,
)> {
    let (price, quote) = if let Some((p, q)) = state.redis_price(coin_type).await? {
        (Some(p), Some(q))
    } else if let Some((p, q)) = state.timescale.latest_price_for_token(coin_type).await? {
        (Some(p), Some(q))
    } else {
        (None, None)
    };

    let price_quote = price
        .zip(quote.clone())
        .map(|(amount, quote)| AmountQuote { amount, quote });

    let (vol, txns, vol_quote) = if let Some((v, t)) = state.redis_vol_usd(coin_type).await? {
        (Some(v), Some(t), Some("USD".to_string()))
    } else if let Some((v, t)) = state.redis_vol(coin_type).await? {
        (Some(v), Some(t), None)
    } else {
        let (v, t) = state.timescale.sum_token_volume_24h(coin_type).await?;
        (Some(v.to_string()), Some(t), None)
    };

    let quote_for_vol = vol_quote.or_else(|| quote.or_else(|| price_quote.as_ref().map(|p| p.quote.clone())));
    let volume_24h = vol.zip(quote_for_vol).map(|(amount, quote)| AmountQuote {
        amount,
        quote,
    });

    let (price_usd, source_type, confidence_score, is_stale) =
        if let Some((p, source, confidence, stale)) = state.redis_price_usd(coin_type).await? {
            (Some(p), source, confidence, stale)
        } else if let Some(p) = state.timescale.latest_price_usd_for_token(coin_type).await? {
            (Some(p), Some("timescale".to_string()), Some("0.8".to_string()), false)
        } else {
            (None, None, None, true)
        };

    Ok((
        price_quote,
        volume_24h,
        txns,
        price_usd,
        source_type,
        is_stale,
        confidence_score,
    ))
}

pub async fn token_ohlc(
    State(state): State<AppState>,
    axum::extract::Path(coin_type): axum::extract::Path<String>,
    Query(query): Query<TokenOhlcQuery>,
) -> Response {
    let coin_type = coin_type::normalize(&coin_type);
    let from = match DateTime::parse_from_rfc3339(&query.from) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "invalid from timestamp".to_string(),
                }),
            )
                .into_response();
        }
    };
    let to = match DateTime::parse_from_rfc3339(&query.to) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "invalid to timestamp".to_string(),
                }),
            )
                .into_response();
        }
    };

    let target = route_query(from, to, state.hot_storage_days);
    let mut raw_bars: Vec<OhlcBarDto> = Vec::new();

    if matches!(target, StorageTarget::Hot | StorageTarget::Both) {
        match state
            .timescale
            .query_token_ohlc_usd(&coin_type, &query.interval, from, to)
            .await
        {
            Ok(rows) => {
                raw_bars.extend(rows.into_iter().map(|r| OhlcBarDto {
                    bucket: r.bucket.to_rfc3339(),
                    open: r.open,
                    high: r.high,
                    low: r.low,
                    close: r.close,
                    volume_quote: r.volume_usd,
                    trade_count: r.trade_count,
                }));
            }
            Err(e) => return internal("token_ohlc", &state, e),
        }
    }

    if matches!(target, StorageTarget::Cold | StorageTarget::Both) {
        if let Some(table) = sui_processors::token_ohlc::token_ohlc_usd_table(&query.interval) {
            if let Ok(rows) = clickhouse::query_token_ohlc_usd(
                &state.clickhouse,
                table,
                &coin_type,
                from,
                to,
            )
            .await
            {
                raw_bars.extend(rows.into_iter().map(|r| OhlcBarDto {
                    bucket: r.bucket.to_rfc3339(),
                    open: r.open_usd,
                    high: r.high_usd,
                    low: r.low_usd,
                    close: r.close_usd,
                    volume_quote: r.volume_usd,
                    trade_count: r.trade_count,
                }));
            }
        }
    }

    raw_bars.sort_by(|a, b| a.bucket.cmp(&b.bucket));
    raw_bars.dedup_by(|a, b| a.bucket == b.bucket);

    let bars = normalize_ohlc_bars(raw_bars, &query.interval);

    Json(TokenOhlcResponse {
        coin_type,
        interval: query.interval,
        bars,
    })
    .into_response()
}

fn parse_cursor_time(cursor: &str) -> Option<DateTime<Utc>> {
    let (time, _) = cursor.split_once('|')?;
    DateTime::parse_from_rfc3339(time)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

fn not_found(route: &str, state: &AppState, msg: &str) -> Response {
    state
        .metrics
        .errors
        .with_label_values(&[route, "404"])
        .inc();
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: msg.to_string(),
        }),
    )
        .into_response()
}

fn internal(route: &str, state: &AppState, err: anyhow::Error) -> Response {
    state
        .metrics
        .errors
        .with_label_values(&[route, "500"])
        .inc();
    tracing::error!(error = %err, route, "API handler error");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: "internal error".to_string(),
        }),
    )
        .into_response()
}
