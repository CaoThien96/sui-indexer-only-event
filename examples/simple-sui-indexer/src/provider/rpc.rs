use anyhow::{Context, Result, bail};
use fastcrypto::encoding::{Base64, Encoding};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use sui_types::base_types::{ObjectID, ObjectRef, SequenceNumber, SuiAddress};
use sui_types::transaction::{ObjectArg, SharedObjectMutability};
use sui_types::transaction::TransactionData;
use sui_types::signature::GenericSignature;
use std::convert::AsRef;

#[derive(Clone)]
pub struct SuiRpcClient {
    http: reqwest::Client,
    urls: Vec<String>,
    cursor: Arc<AtomicUsize>,
}

impl SuiRpcClient {
    pub async fn from_env() -> Result<Self> {
        let main = std::env::var("MAIN_RPC").context("MAIN_RPC must be set")?;
        let mut urls = vec![main];
        if let Ok(fallbacks) = std::env::var("FALLBACK_RPCS") {
            urls.extend(
                fallbacks
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string),
            );
        }
        Ok(Self {
            http: reqwest::Client::new(),
            urls,
            cursor: Arc::new(AtomicUsize::new(0)),
        })
    }

    fn next_url(&self) -> &str {
        let idx = self.cursor.fetch_add(1, Ordering::Relaxed) % self.urls.len();
        &self.urls[idx]
    }

    async fn rpc(&self, method: &str, params: Value) -> Result<Value> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });
        let resp = self
            .http
            .post(self.next_url())
            .json(&body)
            .send()
            .await?;
        let text = resp.text().await?;
        let parsed: RpcResponse = serde_json::from_str(&text).with_context(|| {
            format!(
                "decode rpc {method} response (first 300 chars): {}",
                &text[..text.len().min(300)]
            )
        })?;
        if let Some(err) = parsed.error {
            bail!("rpc {method} error: {err}");
        }
        parsed.result.context("rpc result missing")
    }

    pub async fn get_object_json(&self, object_id: &str) -> Result<Value> {
        self.rpc(
            "sui_getObject",
            json!([object_id, { "showContent": true, "showType": true }]),
        )
        .await
    }

    pub async fn get_pool_coin_b(&self, pool_id: &str) -> Result<u128> {
        let value = self.get_object_json(pool_id).await?;
        let fields = value
            .pointer("/data/content/fields")
            .context("pool fields missing")?;
        json_as_u128(
            fields
                .get("coin_b")
                .or_else(|| fields.get("coinB"))
                .context("pool coin_b missing")?,
        )
    }

    pub async fn get_pool_token_type(&self, pool_id: &str) -> Result<String> {
        let value = self.get_object_json(pool_id).await?;
        let pool_type = value
            .pointer("/data/type")
            .and_then(|v| v.as_str())
            .context("pool type missing")?;
        crate::bot::token_type::meme_token_from_pool_type(pool_type)
            .context("no non-SUI token in pool type")
    }

    pub async fn get_turbos_pool_generics(
        &self,
        pool_id: &str,
    ) -> Result<(String, String, String, String)> {
        let value = self.get_object_json(pool_id).await?;
        let pool_type = value
            .pointer("/data/type")
            .and_then(|v| v.as_str())
            .context("pool type missing")?;
        crate::bot::token_type::parse_pool_generics(pool_type)
            .context("parse turbos pool generics")
    }

    pub async fn get_turbos_fee_type(&self, pool_id: &str) -> Result<String> {
        let value = self.get_object_json(pool_id).await?;
        let pool_type = value
            .pointer("/data/type")
            .and_then(|v| v.as_str())
            .context("pool type missing")?;
        pool_type
            .split(',')
            .nth(2)
            .map(|s| s.trim().trim_end_matches('>').to_string())
            .context("parse turbos fee type")
    }

    pub async fn get_turbos_pool_sqrt_price_and_fee(&self, pool_id: &str) -> Result<(String, u32)> {
        let value = self.get_object_json(pool_id).await?;
        let sqrt_price = value
            .pointer("/data/content/fields/sqrt_price")
            .map(json_as_u128)
            .transpose()?
            .context("pool sqrt_price missing")?
            .to_string();
        let fee = value
            .pointer("/data/content/fields/fee")
            .and_then(|v| v.as_u64())
            .context("pool fee missing")? as u32;
        Ok((sqrt_price, fee))
    }

    pub async fn object_arg(&self, object_id: &str, mutable: bool) -> Result<ObjectArg> {
        let value = self
            .rpc(
                "sui_getObject",
                json!([object_id, { "showOwner": true }]),
            )
            .await?;
        let data = value.get("data").context("object data")?;
        let id: ObjectID = data
            .get("objectId")
            .and_then(|v| v.as_str())
            .context("objectId")?
            .parse()?;

        if let Some(version_val) = data.pointer("/owner/Shared/initial_shared_version") {
            let initial_shared_version: SequenceNumber = json_as_u64(version_val)?
                .into();
            return Ok(ObjectArg::SharedObject {
                id,
                initial_shared_version,
                mutability: if mutable {
                    SharedObjectMutability::Mutable
                } else {
                    SharedObjectMutability::Immutable
                },
            });
        }

        let version: SequenceNumber = json_as_u64(data.get("version").context("version")?)?.into();
        let digest = data
            .get("digest")
            .and_then(|v| v.as_str())
            .context("digest")?
            .parse()?;
        Ok(ObjectArg::ImmOrOwnedObject((id, version, digest)))
    }

    pub async fn get_object_ref(&self, object_id: &str) -> Result<ObjectRef> {
        let value = self
            .rpc("sui_getObject", json!([object_id, { "showOwner": true }]))
            .await?;
        let data = value.get("data").context("object data")?;
        let id: ObjectID = data
            .get("objectId")
            .and_then(|v| v.as_str())
            .context("objectId")?
            .parse()?;
        let version: SequenceNumber = json_as_u64(data.get("version").context("version")?)?.into();
        let digest = data
            .get("digest")
            .and_then(|v| v.as_str())
            .context("digest")?
            .parse()?;
        Ok((id, version, digest))
    }

    pub async fn get_coins(&self, owner: SuiAddress, coin_type: &str) -> Result<Vec<CoinEntry>> {
        let mut coins = Vec::new();
        let mut cursor: Option<Value> = None;
        loop {
            let params = match &cursor {
                Some(c) => json!([owner.to_string(), coin_type, c, Value::Null]),
                None => json!([owner.to_string(), coin_type]),
            };
            let page = self.rpc("suix_getCoins", params).await?;
            if let Some(data) = page.get("data").and_then(|v| v.as_array()) {
                for item in data {
                    coins.push(CoinEntry {
                        coin_object_id: item
                            .get("coinObjectId")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string(),
                        balance: json_as_u128(item.get("balance").unwrap_or(&Value::Null))?,
                    });
                }
            }
            let has_next = page
                .get("hasNextPage")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if !has_next {
                break;
            }
            cursor = page.get("nextCursor").filter(|v| !v.is_null()).cloned();
        }
        Ok(coins)
    }

    pub async fn get_reference_gas_price(&self) -> Result<u64> {
        let value = self.rpc("suix_getReferenceGasPrice", json!([])).await?;
        value
            .as_str()
            .and_then(|s| s.parse().ok())
            .or_else(|| value.as_u64())
            .context("gas price")
    }

    pub async fn execute_transaction(
        &self,
        tx_data: TransactionData,
        sig: GenericSignature,
    ) -> Result<String> {
        let tx_bytes = bcs::to_bytes(&tx_data)?;
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, tx_bytes);
        let value = self
            .rpc(
                "sui_executeTransactionBlock",
                json!([
                    b64,
                    [Base64::encode(sig.as_ref())],
                    { "showEffects": true },
                    "WaitForLocalExecution"
                ]),
            )
            .await?;
        let digest = value
            .pointer("/digest")
            .and_then(|v| v.as_str())
            .context("digest")?
            .to_string();
        let status = value
            .pointer("/effects/status/status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        if status != "success" {
            let err = value
                .pointer("/effects/status/error")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            bail!("tx failed: {err}");
        }
        Ok(digest)
    }
}

#[derive(Debug, Clone)]
pub struct CoinEntry {
    pub coin_object_id: String,
    pub balance: u128,
}

#[derive(Debug, Deserialize)]
struct RpcResponse {
    result: Option<Value>,
    error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
struct RpcError {
    code: Option<i64>,
    message: Option<String>,
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.code, &self.message) {
            (Some(code), Some(msg)) => write!(f, "{code}: {msg}"),
            (None, Some(msg)) => write!(f, "{msg}"),
            (Some(code), None) => write!(f, "{code}"),
            (None, None) => write!(f, "unknown rpc error"),
        }
    }
}

fn json_as_u64(value: &Value) -> Result<u64> {
    if let Some(s) = value.as_str() {
        return s.parse().context("parse u64 string");
    }
    value
        .as_u64()
        .context("parse u64")
}

fn json_as_u128(value: &Value) -> Result<u128> {
    if let Some(obj) = value.as_object() {
        if let Some(inner) = obj.get("value").or_else(|| obj.get("fields")) {
            return json_as_u128(inner);
        }
    }
    if let Some(s) = value.as_str() {
        return s.parse().context("parse u128 string");
    }
    value
        .as_u64()
        .map(|n| n as u128)
        .context("parse u128")
}
