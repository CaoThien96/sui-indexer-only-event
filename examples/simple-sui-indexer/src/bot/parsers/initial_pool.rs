use anyhow::Result;
use serde_json::Value;

use crate::bot::event_types;
use crate::bot::state::Dex;
use crate::bot::token_type::{
    is_sui_coin_type, normalize_coin_type, pick_non_sui_token, symbol_from_coin_type,
};
use crate::provider::SuiRpcClient;

use crate::bot::reactor::BotEventContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitialPoolCandidate {
    pub dex: Dex,
    pub pool: String,
    pub token: String,
    pub symbol: String,
    pub reserve_sui: u128,
    pub create_event_seq: usize,
    pub liquidity_event_seq: usize,
}

/// Detect new pools only when create + initial liquidity events appear in the same tx.
pub async fn find_initial_pool_candidates(
    rpc: &SuiRpcClient,
    events: &[BotEventContext],
) -> Result<Vec<InitialPoolCandidate>> {
    let mut out = Vec::new();
    out.extend(find_cetus_candidates(events));
    for candidate in find_turbos_pairs(events) {
        if let Some(parsed) = parse_turbos_candidate(rpc, &candidate).await? {
            out.push(parsed);
        }
    }
    Ok(out)
}

#[derive(Debug, Clone)]
struct TurbosPair {
    pool: String,
    create_event_seq: usize,
    liquidity_event_seq: usize,
    amount_a: u128,
    amount_b: u128,
}

fn find_cetus_candidates(events: &[BotEventContext]) -> Vec<InitialPoolCandidate> {
    let create_type = event_types::cetus_create_pool();
    let add_type = event_types::cetus_add_liquidity();

    struct CetusCreate {
        pool: String,
        event_seq: usize,
        coin_a: String,
        coin_b: String,
        token: String,
        symbol: String,
    }

    let mut creates: Vec<CetusCreate> = Vec::new();
    let mut adds: Vec<(String, usize, u128, u128)> = Vec::new();

    for event in events {
        if event.event_type == create_type {
            let Some(pool) = event
                .parsed_json
                .get("pool_id")
                .or_else(|| event.parsed_json.get("pool"))
                .and_then(|v| v.as_str())
            else {
                continue;
            };
            let coin_a = event
                .parsed_json
                .get("coin_type_a")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let coin_b = event
                .parsed_json
                .get("coin_type_b")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let Some(token) = pick_non_sui_token(Some(coin_a), Some(coin_b)) else {
                continue;
            };
            let symbol = symbol_from_coin_type(&token);
            creates.push(CetusCreate {
                pool: pool.to_string(),
                event_seq: event.event_seq,
                coin_a: normalize_coin_type(coin_a),
                coin_b: normalize_coin_type(coin_b),
                token,
                symbol,
            });
        } else if event.event_type == add_type {
            let Some(pool) = event.parsed_json.get("pool").and_then(|v| v.as_str()) else {
                continue;
            };
            let amount_a = json_u128(event.parsed_json.get("amount_a"));
            let amount_b = json_u128(event.parsed_json.get("amount_b"));
            if amount_a == 0 && amount_b == 0 {
                continue;
            }
            adds.push((pool.to_string(), event.event_seq, amount_a, amount_b));
        }
    }

    let mut out = Vec::new();
    for create in creates {
        let Some((_, liquidity_seq, amount_a, amount_b)) =
            adds.iter().find(|(p, ..)| *p == create.pool).cloned()
        else {
            continue;
        };
        let reserve_sui =
            sui_amount_from_pair(&create.coin_a, &create.coin_b, amount_a, amount_b);
        if reserve_sui == 0 {
            continue;
        }
        out.push(InitialPoolCandidate {
            dex: Dex::Cetus,
            pool: create.pool,
            token: create.token,
            symbol: create.symbol,
            reserve_sui,
            create_event_seq: create.event_seq,
            liquidity_event_seq: liquidity_seq,
        });
    }
    out
}

fn find_turbos_pairs(events: &[BotEventContext]) -> Vec<TurbosPair> {
    let create_type = event_types::turbos_create_pool();
    let mint_type = event_types::turbos_mint_liquidity();

    let mut creates: Vec<(String, usize)> = Vec::new();
    let mut mints: Vec<(String, usize, u128, u128)> = Vec::new();

    for event in events {
        if event.event_type == create_type {
            let Some(pool) = event.parsed_json.get("pool").and_then(|v| v.as_str()) else {
                continue;
            };
            creates.push((pool.to_string(), event.event_seq));
        } else if event.event_type == mint_type {
            let Some(pool) = event.parsed_json.get("pool").and_then(|v| v.as_str()) else {
                continue;
            };
            let amount_a = json_u128(event.parsed_json.get("amount_a"));
            let amount_b = json_u128(event.parsed_json.get("amount_b"));
            if amount_a == 0 && amount_b == 0 {
                continue;
            }
            mints.push((pool.to_string(), event.event_seq, amount_a, amount_b));
        }
    }

    let mut out = Vec::new();
    for (pool, create_seq) in creates {
        let Some((_, liquidity_seq, amount_a, amount_b)) =
            mints.iter().find(|(p, ..)| *p == pool).cloned()
        else {
            continue;
        };
        out.push(TurbosPair {
            pool,
            create_event_seq: create_seq,
            liquidity_event_seq: liquidity_seq,
            amount_a,
            amount_b,
        });
    }
    out
}

async fn parse_turbos_candidate(
    rpc: &SuiRpcClient,
    pair: &TurbosPair,
) -> Result<Option<InitialPoolCandidate>> {
    let (_, coin_a, coin_b, _) = rpc.get_turbos_pool_generics(&pair.pool).await?;
    let Some(token) = pick_non_sui_token(Some(&coin_a), Some(&coin_b)) else {
        return Ok(None);
    };
    let reserve_sui = sui_amount_from_pair(&coin_a, &coin_b, pair.amount_a, pair.amount_b);
    if reserve_sui == 0 {
        return Ok(None);
    }
    let symbol = symbol_from_coin_type(&token);
    Ok(Some(InitialPoolCandidate {
        dex: Dex::Turbos,
        pool: pair.pool.clone(),
        token,
        symbol,
        reserve_sui,
        create_event_seq: pair.create_event_seq,
        liquidity_event_seq: pair.liquidity_event_seq,
    }))
}

fn sui_amount_from_pair(
    coin_a: &str,
    coin_b: &str,
    amount_a: u128,
    amount_b: u128,
) -> u128 {
    if is_sui_coin_type(coin_a) {
        amount_a
    } else if is_sui_coin_type(coin_b) {
        amount_b
    } else {
        0
    }
}

fn json_u128(value: Option<&Value>) -> u128 {
    value
        .and_then(|v| v.as_str().and_then(|s| s.parse().ok()))
        .or_else(|| value.and_then(|v| v.as_u64().map(|n| n as u128)))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn tx_event(event_type: &str, seq: usize, parsed: Value) -> BotEventContext {
        BotEventContext {
            event_type: event_type.to_string(),
            tx_digest: "test_tx".into(),
            event_seq: seq,
            sender: "0x1".into(),
            parsed_json: parsed,
        }
    }

    #[test]
    fn cetus_create_plus_add_in_same_tx() {
        let events = vec![
            tx_event(
                &event_types::cetus_create_pool(),
                0,
                json!({
                    "pool_id": "0xpool",
                    "coin_type_a": "ce6159883c082ce77dfb20a19e61ad92f8f1ecd4141762178c97509ba1cd13e0::giftv4::GIFTV4",
                    "coin_type_b": "0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"
                }),
            ),
            tx_event(
                &event_types::cetus_add_liquidity(),
                2,
                json!({
                    "pool": "0xpool",
                    "amount_a": "100000003228672903",
                    "amount_b": "5200000000000"
                }),
            ),
        ];
        let candidates = find_cetus_candidates(&events);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].reserve_sui, 5_200_000_000_000);
        assert!(candidates[0].token.contains("GIFTV4"));
    }

    #[test]
    fn cetus_create_without_add_is_ignored() {
        let events = vec![tx_event(
            &event_types::cetus_create_pool(),
            0,
            json!({
                "pool_id": "0xpool",
                "coin_type_a": "ce6159883c082ce77dfb20a19e61ad92f8f1ecd4141762178c97509ba1cd13e0::giftv4::GIFTV4",
                "coin_type_b": "0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"
            }),
        )];
        assert!(find_cetus_candidates(&events).is_empty());
    }

    #[test]
    fn turbos_create_plus_mint_in_same_tx() {
        let events = vec![
            tx_event(
                &event_types::turbos_create_pool(),
                0,
                json!({ "pool": "0xpool" }),
            ),
            tx_event(
                &event_types::turbos_mint_liquidity(),
                1,
                json!({
                    "pool": "0xpool",
                    "amount_a": "2000000",
                    "amount_b": "2208694"
                }),
            ),
        ];
        let pairs = find_turbos_pairs(&events);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].amount_b, 2_208_694);
    }
}
