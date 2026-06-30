use anyhow::{Context, Result, bail};
use fastcrypto::encoding::{Base64, Encoding};
use move_core_types::identifier::Identifier;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use sui_types::base_types::{ObjectID, ObjectRef, SequenceNumber, SuiAddress};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{ObjectArg, SharedObjectMutability, TransactionData, TransactionKind};
use sui_types::signature::GenericSignature;
use std::convert::AsRef;

const HOT_CACHE_TTL: Duration = Duration::from_millis(2_000);
const SUI_TYPE: &str = "0x2::sui::SUI";

#[derive(Clone)]
pub struct SuiRpcClient {
    http: reqwest::Client,
    urls: Vec<String>,
    cursor: Arc<AtomicUsize>,
    hot_cache: Arc<Mutex<RpcHotCache>>,
}

#[derive(Default)]
struct RpcHotCache {
    gas_price: Option<Timed<u64>>,
    object_args: HashMap<String, ObjectArg>,
}

struct Timed<T> {
    value: T,
    at: Instant,
}

impl RpcHotCache {
    fn gas_price_if_fresh(&self) -> Option<u64> {
        self.gas_price
            .as_ref()
            .filter(|t| t.at.elapsed() < HOT_CACHE_TTL)
            .map(|t| t.value)
    }

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
            hot_cache: Arc::new(Mutex::new(RpcHotCache::default())),
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
        self.fetch_object_arg(object_id, mutable).await
    }

    /// Fetch `initial_shared_version` for a shared object (backfill / one-off tooling).
    pub async fn get_shared_initial_version(&self, object_id: &str) -> Result<u64> {
        let arg = self.fetch_object_arg(object_id, false).await?;
        initial_shared_version_from_arg(&arg)
    }

    /// Cached `object_arg` for static on-chain objects (vault, global config, partner, …).
    pub async fn object_arg_cached(&self, object_id: &str, mutable: bool) -> Result<ObjectArg> {
        let key = format!("{object_id}:{}", if mutable { "m" } else { "i" });
        if let Some(arg) = self
            .hot_cache
            .lock()
            .expect("rpc hot cache")
            .object_args
            .get(&key)
            .cloned()
        {
            return Ok(arg);
        }
        let arg = self.fetch_object_arg(object_id, mutable).await?;
        self.hot_cache
            .lock()
            .expect("rpc hot cache")
            .object_args
            .insert(key, arg.clone());
        Ok(arg)
    }

    async fn fetch_object_arg(&self, object_id: &str, mutable: bool) -> Result<ObjectArg> {
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

    pub async fn get_reference_gas_price_cached(&self) -> Result<u64> {
        if let Some(price) = self
            .hot_cache
            .lock()
            .expect("rpc hot cache")
            .gas_price_if_fresh()
        {
            return Ok(price);
        }
        let price = self.get_reference_gas_price().await?;
        let mut cache = self.hot_cache.lock().expect("rpc hot cache");
        cache.gas_price = Some(Timed {
            value: price,
            at: Instant::now(),
        });
        Ok(price)
    }

    pub async fn get_current_epoch(&self) -> Result<u64> {
        let value = self.rpc("suix_getLatestSuiSystemState", json!([])).await?;
        let epoch = value
            .get("epoch")
            .context("latest system state epoch missing")?;
        json_as_u64(epoch)
    }

    pub async fn get_chain_identifier(&self) -> Result<String> {
        let value = self.rpc("sui_getChainIdentifier", json!([])).await?;
        value
            .as_str()
            .map(str::to_string)
            .filter(|v| !v.is_empty())
            .context("chain identifier missing")
    }

    pub async fn select_gas_coin(
        &self,
        owner: SuiAddress,
        min_balance: u64,
    ) -> Result<ObjectRef> {
        let coins = self.get_coins(owner, SUI_TYPE).await?;
        if coins.is_empty() {
            bail!("no gas coin");
        }
        for coin in &coins {
            if coin.balance >= min_balance as u128 {
                return self.get_object_ref(&coin.coin_object_id).await;
            }
        }
        self.get_object_ref(&coins[0].coin_object_id).await
    }

    /// Dev-inspect `snip_vault::vault::token_balance<T>` on the shared vault (read-only).
    pub async fn dev_inspect_vault_token_balance(
        &self,
        sender: SuiAddress,
        package: ObjectID,
        vault_id: &str,
        token_type: &str,
    ) -> Result<u64> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let vault_arg = ptb.obj(self.object_arg(vault_id, false).await?)?;
        ptb.programmable_move_call(
            package,
            Identifier::new("vault").context("vault module")?,
            Identifier::new("token_balance").context("token_balance")?,
            vec![crate::bot::token_type::parse_type_tag(token_type)?],
            vec![vault_arg],
        );

        let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());
        let tx_bytes = bcs::to_bytes(&tx_kind)?;
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &tx_bytes);

        let value = self
            .rpc(
                "sui_devInspectTransactionBlock",
                json!([sender.to_string(), b64, Value::Null, Value::Null]),
            )
            .await?;

        let status = value
            .pointer("/effects/status/status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        if status != "success" {
            let err = value
                .pointer("/effects/status/error")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            bail!("devInspect token_balance failed: {err}");
        }

        let return_entry = value
            .pointer("/results/0/returnValues/0")
            .context("token_balance return value missing")?;
        let raw = parse_devinspect_return_bytes(return_entry).context("return value bytes")?;
        bcs::from_bytes::<u64>(&raw).context("decode u64 return")
    }

    /// Simulate a PTB without submitting (no signature required).
    pub async fn dry_run_transaction(&self, tx_data: &TransactionData) -> Result<DryRunOutcome> {
        let tx_bytes = bcs::to_bytes(tx_data)?;
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, tx_bytes);
        let value = self
            .rpc("sui_dryRunTransactionBlock", json!([b64, Value::Null, Value::Null]))
            .await?;

        let status = value
            .pointer("/effects/status/status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let error = value
            .pointer("/effects/status/error")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let computation_cost = json_u64(value.pointer("/effects/gasUsed/computationCost"));
        let storage_cost = json_u64(value.pointer("/effects/gasUsed/storageCost"));
        let storage_rebate = json_u64(value.pointer("/effects/gasUsed/storageRebate"));
        let total_gas = computation_cost
            .saturating_add(storage_cost)
            .saturating_sub(storage_rebate);

        Ok(DryRunOutcome {
            status,
            error,
            computation_cost,
            storage_cost,
            storage_rebate,
            total_gas,
        })
    }

    pub async fn execute_or_dry_run(
        &self,
        tx_data: TransactionData,
        sig: GenericSignature,
        dry_run: bool,
        request_type: Option<&str>,
    ) -> Result<String> {
        if dry_run {
            let outcome = self.dry_run_transaction(&tx_data).await?;
            if outcome.status != "success" {
                bail!(
                    "dry-run failed: {}",
                    outcome.error.unwrap_or_else(|| outcome.status.clone())
                );
            }
            Ok(outcome.summary())
        } else {
            self.execute_transaction(tx_data, sig, request_type).await
        }
    }

    pub async fn execute_transaction(
        &self,
        tx_data: TransactionData,
        sig: GenericSignature,
        request_type: Option<&str>,
    ) -> Result<String> {
        let tx_bytes = bcs::to_bytes(&tx_data)?;
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, tx_bytes);
        let execution = request_type.unwrap_or("WaitForLocalExecution");
        let value = self
            .rpc(
                "sui_executeTransactionBlock",
                json!([
                    b64,
                    [Base64::encode(sig.as_ref())],
                    { "showEffects": true },
                    execution
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
pub struct DryRunOutcome {
    pub status: String,
    pub error: Option<String>,
    pub computation_cost: u64,
    pub storage_cost: u64,
    pub storage_rebate: u64,
    pub total_gas: u64,
}

impl DryRunOutcome {
    pub fn summary(&self) -> String {
        format!(
            "dry-run:success gas={} (computation={} storage={} rebate={})",
            self.total_gas, self.computation_cost, self.storage_cost, self.storage_rebate
        )
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

fn json_u64(value: Option<&Value>) -> u64 {
    value
        .and_then(|v| json_as_u64(v).ok())
        .unwrap_or(0)
}

/// Parse first element of a devInspect `returnValues` entry: `[bytes, typeTag]`.
/// Bytes may be base64 string (legacy) or JSON array of uint8 (current RPC).
fn parse_devinspect_return_bytes(return_entry: &Value) -> Result<Vec<u8>> {
    let bytes_value = return_entry
        .get(0)
        .context("return value entry missing bytes field")?;
    if let Some(b64) = bytes_value.as_str() {
        return base64::Engine::decode(&base64::engine::general_purpose::STANDARD, b64)
            .context("decode base64 return value");
    }
    if let Some(arr) = bytes_value.as_array() {
        return arr
            .iter()
            .map(|v| {
                v.as_u64()
                    .and_then(|n| u8::try_from(n).ok())
                    .context("return value byte")
            })
            .collect();
    }
    bail!("unexpected devInspect return value bytes format: {bytes_value}")
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

pub fn initial_shared_version_from_arg(arg: &ObjectArg) -> Result<u64> {
    match arg {
        ObjectArg::SharedObject {
            initial_shared_version,
            ..
        } => Ok(initial_shared_version.value()),
        _ => bail!("expected shared object arg for pool"),
    }
}

pub fn shared_pool_arg_mutable(pool_id: &str, initial_shared_version: u64) -> Result<ObjectArg> {
    Ok(ObjectArg::SharedObject {
        id: pool_id.parse().context("pool id")?,
        initial_shared_version: initial_shared_version.into(),
        mutability: SharedObjectMutability::Mutable,
    })
}
