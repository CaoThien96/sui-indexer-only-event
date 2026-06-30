use anyhow::Result;
use std::sync::Arc;

use crate::bot::event_types;
use crate::dex::{AggSwap, SnipVaultClient};
use crate::provider::{SuiRpcClient, VaultKeypair};

#[derive(Clone, Debug)]
pub struct BotConfig {
    pub enabled: bool,
    pub snip_buy_amount: u64,
    pub snip_delay_ms_min: u64,
    pub snip_delay_ms_max: u64,
    pub snip_lp_wait_ms: u64,
    /// When true, failed vault snip falls back to agg_swap + separate LP.
    pub snip_vault_fallback_agg: bool,
    pub min_pool_reserve_sui: u128,
    pub remove_reserve_threshold: u128,
    pub sell_buy_threshold: u128,
    /// `sui_executeTransactionBlock` request type for sell hot path (e.g. WaitForEffectsCert).
    pub sell_tx_request_type: String,
    pub processed_swaps_ttl_days: u32,
    pub cleanup_interval_secs: u64,
    pub blacklist_tokens: Vec<String>,
}

impl BotConfig {
    pub fn from_env() -> Result<Arc<Self>> {
        let enabled = std::env::var("BOT_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        let blacklist_tokens = std::env::var("BLACKLIST_TOKEN")
            .ok()
            .map(|raw| {
                raw.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect()
            })
            .filter(|v: &Vec<String>| !v.is_empty())
            .unwrap_or_else(default_blacklist);

        Ok(Arc::new(Self {
            enabled,
            snip_buy_amount: env_u64("SNIP_BUY_AMOUNT", 10_000_000),
            snip_delay_ms_min: env_u64("SNIP_DELAY_MS_MIN", 5_000),
            snip_delay_ms_max: env_u64("SNIP_DELAY_MS_MAX", 10_000),
            snip_lp_wait_ms: env_u64("SNIP_LP_WAIT_MS", 60_000),
            snip_vault_fallback_agg: env_bool("SNIP_VAULT_FALLBACK_AGG", true),
            min_pool_reserve_sui: env_u128("MIN_POOL_RESERVE_SUI", 10_000_000_000),
            remove_reserve_threshold: env_u128("REMOVE_RESERVE_THRESHOLD", 2_000_000_000),
            sell_buy_threshold: env_u128("SELL_BUY_THRESHOLD", 500_000_000),
            sell_tx_request_type: std::env::var("SELL_TX_REQUEST_TYPE")
                .ok()
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
                .unwrap_or_else(|| "WaitForEffectsCert".into()),
            processed_swaps_ttl_days: env_u32("BOT_PROCESSED_SWAPS_TTL_DAYS", 7),
            cleanup_interval_secs: env_u64("BOT_CLEANUP_INTERVAL_SECS", 86_400),
            blacklist_tokens,
        }))
    }

    pub fn is_blacklisted(&self, token: &str) -> bool {
        if crate::bot::token_type::is_sui_coin_type(token) {
            return true;
        }
        let normalized = crate::bot::token_type::normalize_coin_type(token);
        self.blacklist_tokens
            .iter()
            .any(|t| crate::bot::token_type::normalize_coin_type(t) == normalized)
    }
}

fn env_u32(key: &str, default: u32) -> u32 {
    env_parse(key, default)
}

fn env_u64(key: &str, default: u64) -> u64 {
    env_parse(key, default)
}

fn env_u128(key: &str, default: u128) -> u128 {
    env_parse(key, default)
}

fn env_bool(key: &str, default: bool) -> bool {
    std::env::var(key)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(default)
}

fn env_parse<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn default_blacklist() -> Vec<String> {
    vec![
        "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC".into(),
        "0x5145494a5f5100e645e4b0aa950fa6b68f614e8c59e17bc5ded3495123a79178::ns::NS".into(),
        "0xbde4ba4c2e274a60ce15c1cfff9e5c42e41654ac8b6d906a57efa4bd3c29f47d::hasui::HASUI".into(),
        "0x356a26eb9e012a68958082340d4c4116e7f55615cf27affcff209cf0ae544f59::wal::WAL".into(),
        "0xdeeb7a4662eec9f2f3def03fb937a663dddaa2e215b8078a284d026b7946c270::deep::DEEP".into(),
        "0x06864a6f921804860930db6ddbe2e16acdf8504495ea7481637a1c8b9a8fe54b::cetus::CETUS".into(),
        "0xf325ce1300e8dac124071d3152c5c5ee6174914f8bc2161e88329cf579246efc::afsui::AFSUI".into(),
        "0xf22da9a24ad027cccb5f2d496cbe91de953d363513db08a3a734d361c7c17503::LOFI::LOFI".into(),
        "0x8993129d72e733985f7f1a00396cbd055bad6f817fee36576ce483c8bbb8b87b::sudeng::SUDENG".into(),
        "0xfa7ac3951fdca92c5200d468d31a365eb03b2be9936fde615e69f0c1274ad3a0::BLUB::BLUB".into(),
        "0xa99b8952d4f7d947ea77fe0ecdcc9e5fc0bcab2841d6e2a5aa00c3044e5544b5::navx::NAVX".into(),
        "0x3a304c7feba2d819ea57c3542d68439ca2c386ba02159c740f7b406e592c62ea::haedal::HAEDAL".into(),
        "0x4c981f3ff786cdb9e514da897ab8a953647dae2ace9679e8358eec1e3e8871ac::dmc::DMC".into(),
        "0xe4239cd951f6c53d9c41e25270d80d31f925ad1655e5ba5b543843d4a66975ee::SUIP::SUIP".into(),
        "0x9f854b3ad20f8161ec0886f15f4a1752bf75d22261556f14cc8d3a1c5d50e529::magma::MAGMA".into(),
        "0x2fa23e8c2994b34f5d2bae9cc877ade7bf4a2f62575611d60c006fa162c302ec::pawtato_coin_iron_q_rune::PAWTATO_COIN_IRON_Q_RUNE".into(),
    ]
}

pub struct BotRuntime {
    pub config: Arc<BotConfig>,
    pub rpc: Arc<SuiRpcClient>,
    pub vault: Arc<VaultKeypair>,
    pub agg: Arc<AggSwap>,
    pub snip_vault: Option<Arc<SnipVaultClient>>,
}

impl BotRuntime {
    pub async fn try_from_env() -> Result<Option<Arc<Self>>> {
        let config = BotConfig::from_env()?;
        if !config.enabled {
            return Ok(None);
        }
        Self::init().await.map(Some)
    }

    /// Load vault + RPC for manual CLI tests (ignores `BOT_ENABLED`).
    pub async fn init() -> Result<Arc<Self>> {
        let config = BotConfig::from_env()?;
        let rpc = Arc::new(SuiRpcClient::from_env().await?);
        let vault = VaultKeypair::from_env()?;
        let agg = Arc::new(AggSwap::new(
            Arc::clone(&rpc),
            Arc::clone(&vault),
            Some(config.sell_tx_request_type.clone()),
        ));
        let snip_vault = SnipVaultClient::from_env()?.map(Arc::new);
        Ok(Arc::new(Self {
            config,
            rpc,
            vault,
            agg,
            snip_vault,
        }))
    }

    pub fn dex_from_event_type(event_type: &str) -> Option<crate::bot::state::Dex> {
        if event_type.starts_with(event_types::CETUS_PKG) {
            Some(crate::bot::state::Dex::Cetus)
        } else if event_type.starts_with(event_types::TURBOS_PKG) {
            Some(crate::bot::state::Dex::Turbos)
        } else {
            None
        }
    }
}
