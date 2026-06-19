use std::collections::HashSet;

use anyhow::{Context, Result};
use serde_json::{json, Value};

use crate::event_key::EventKey;

const DEFAULT_PAGE_SIZE: u64 = 50;
const MAX_PAGES: u32 = 50_000;

pub struct FullnodeClient {
    http: reqwest::Client,
    url: String,
}

impl FullnodeClient {
    pub fn new(url: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            url,
        }
    }

    pub async fn list_event_keys_in_window(
        &self,
        move_event_type: &str,
        start_ms: i64,
        end_ms: i64,
    ) -> Result<HashSet<EventKey>> {
        let filter = json!({ "MoveEventType": move_event_type });
        let mut cursor: Option<Value> = None;
        let mut keys = HashSet::new();
        let mut pages = 0u32;

        loop {
            pages += 1;
            let page = self
                .query_events_page(&filter, cursor.as_ref(), DEFAULT_PAGE_SIZE, true)
                .await?;

            let data = page
                .get("data")
                .and_then(|v| v.as_array())
                .context("fullnode result missing data array")?;

            if data.is_empty() {
                break;
            }

            let mut page_entirely_before_window = true;

            for event in data {
                let Some(ts) = parse_timestamp_ms(event) else {
                    page_entirely_before_window = false;
                    continue;
                };

                if ts > end_ms {
                    page_entirely_before_window = false;
                    continue;
                }

                if ts < start_ms {
                    continue;
                }

                page_entirely_before_window = false;

                if let Some(key) = EventKey::from_fullnode_event(event) {
                    keys.insert(key);
                }
            }

            if page_entirely_before_window {
                break;
            }

            let has_next = page
                .get("hasNextPage")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if !has_next {
                break;
            }

            cursor = page.get("nextCursor").cloned();
            if cursor.is_none() {
                break;
            }

            if pages > MAX_PAGES {
                anyhow::bail!("fullnode pagination exceeded safety limit ({MAX_PAGES} pages)");
            }
        }

        Ok(keys)
    }

    async fn query_events_page(
        &self,
        filter: &Value,
        cursor: Option<&Value>,
        limit: u64,
        descending: bool,
    ) -> Result<Value> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "suix_queryEvents",
            "params": [
                filter,
                cursor,
                limit,
                descending
            ]
        });

        let response = self
            .http
            .post(&self.url)
            .json(&body)
            .send()
            .await
            .context("fullnode HTTP request failed")?
            .error_for_status()
            .context("fullnode returned non-success status")?
            .json::<Value>()
            .await
            .context("fullnode response is not valid JSON")?;

        if let Some(error) = response.get("error") {
            anyhow::bail!("fullnode JSON-RPC error: {error}");
        }

        response
            .get("result")
            .cloned()
            .context("fullnode response missing result")
    }
}

fn parse_timestamp_ms(event: &Value) -> Option<i64> {
    event.get("timestampMs").and_then(|v| {
        v.as_str()
            .and_then(|s| s.parse().ok())
            .or_else(|| v.as_i64())
    })
}
