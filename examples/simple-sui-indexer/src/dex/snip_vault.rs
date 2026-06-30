use anyhow::{Context, Result, bail};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;
use sui_types::digests::ChainIdentifier;
use sui_types::base_types::{ObjectID, ObjectRef, SuiAddress};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{
    Argument, Command, GasData, ObjectArg, TransactionData, TransactionDataV1,
    TransactionExpiration, TransactionKind,
};
use tracing::info;

use crate::bot::config::BotRuntime;
use crate::bot::pool_shared::persist_pool_shared_version;
use crate::bot::state::{BotStateStore, Dex};
use crate::bot::token_type::{normalize_coin_type, parse_type_tag};
use crate::dex::agg_swap::{clock_arg};
use crate::dex::turbos_contract::{self, MINT_DEADLINE_MS};
use crate::dex::turbos_math;

// #region agent log
use crate::bot::debug_log::agent_log;
// #endregion

const CETUS_GLOBAL_CONFIG: &str =
    "0xdaa46292632c3c4d8f31f23ea0f9b36a28ff3677e9684980e4438403a67a3d8f";
const CETUS_PARTNER: &str =
    "0x639b5e433da31739e800cd085f356e64cae222966d0f1b11bd9dc76b322ff58b";
const SUI_TYPE: &str = "0x2::sui::SUI";

const CETUS_TICK_LOWER: u32 = 37_680;
const CETUS_TICK_UPPER: u32 = 69_120;
const CETUS_TOKEN_FOR_LP: u64 = 1_000_000_000;
const TURBOS_SLIPPAGE_PERCENT: u64 = 5;

pub struct SnipVaultClient {
    package: ObjectID,
    vault_id: ObjectID,
    gas_budget: u64,
    sell_nonce: AtomicU32,
}

impl SnipVaultClient {
    pub async fn sell_with_metrics(
        &self,
        runtime: &BotRuntime,
        store: Option<&BotStateStore>,
        dex: Dex,
        token: &str,
        pool: &str,
        amount: u64,
        dry_run: bool,
        sell_detected_at: Option<Instant>,
        allow_pool_rpc_fallback: bool,
    ) -> Result<(String, crate::dex::agg_swap::SwapExecMetrics)> {
        let token = normalize_coin_type(token);
        match dex {
            Dex::Cetus => {
                self.sell_cetus(
                    runtime,
                    store,
                    &token,
                    pool,
                    amount,
                    dry_run,
                    sell_detected_at,
                    allow_pool_rpc_fallback,
                )
                .await
            }
            Dex::Turbos => {
                self.sell_turbos(
                    runtime,
                    store,
                    &token,
                    pool,
                    amount,
                    dry_run,
                    sell_detected_at,
                    allow_pool_rpc_fallback,
                )
                .await
            }
        }
    }

    pub fn from_env() -> Result<Option<Self>> {
        let enabled = std::env::var("USE_SNIP_VAULT")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if !enabled {
            return Ok(None);
        }
        let package = std::env::var("SNIP_VAULT_PACKAGE")
            .context("USE_SNIP_VAULT=true requires SNIP_VAULT_PACKAGE")?;
        let vault_id = std::env::var("SNIP_VAULT_OBJECT_ID")
            .context("USE_SNIP_VAULT=true requires SNIP_VAULT_OBJECT_ID")?;
        let gas_budget = std::env::var("SNIP_VAULT_GAS_BUDGET")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(500_000_000);
        Ok(Some(Self {
            package: package.parse().context("SNIP_VAULT_PACKAGE")?,
            vault_id: vault_id.parse().context("SNIP_VAULT_OBJECT_ID")?,
            gas_budget,
            sell_nonce: AtomicU32::new(0),
        }))
    }

    pub async fn deposit_token(
        &self,
        runtime: &BotRuntime,
        token: &str,
        amount: Option<u64>,
        dry_run: bool,
    ) -> Result<String> {
        let rpc = &runtime.rpc;
        let vault_kp = &runtime.vault;
        let sender = vault_kp.address();
        let token = normalize_coin_type(token);

        let mut ptb = ProgrammableTransactionBuilder::new();
        let coin_arg = wallet_coin_for_deposit(&mut ptb, rpc, sender, &token, amount).await?;
        let vault_arg = ptb.obj(rpc.object_arg(&self.vault_id.to_string(), true).await?)?;

        ptb.programmable_move_call(
            self.package,
            "vault".parse()?,
            "deposit".parse()?,
            vec![parse_type_tag(&token)?],
            vec![vault_arg, coin_arg],
        );

        self.execute_ptb(runtime, sender, ptb, "deposit", dry_run, false, None)
            .await
            .map(|(digest, _)| digest)
    }

    pub async fn withdraw_token(
        &self,
        runtime: &BotRuntime,
        token: &str,
        amount: Option<u64>,
        dry_run: bool,
    ) -> Result<String> {
        let rpc = &runtime.rpc;
        let vault_kp = &runtime.vault;
        let sender = vault_kp.address();
        let token = normalize_coin_type(token);

        let vault_balance = self.read_token_balance(runtime, &token).await?;
        if vault_balance == 0 {
            bail!("vault has zero balance for {token}");
        }

        let withdraw_amount = match amount {
            Some(a) => {
                if a > vault_balance {
                    bail!(
                        "vault balance {vault_balance} < requested withdraw {a} for {token}"
                    );
                }
                if a == 0 {
                    bail!("withdraw amount must be positive");
                }
                a
            }
            None => vault_balance,
        };

        let mut ptb = ProgrammableTransactionBuilder::new();
        let vault_arg = ptb.obj(rpc.object_arg(&self.vault_id.to_string(), true).await?)?;
        let amount_arg = ptb.pure(withdraw_amount)?;

        let coin_out = ptb.programmable_move_call(
            self.package,
            "vault".parse()?,
            "withdraw".parse()?,
            vec![parse_type_tag(&token)?],
            vec![vault_arg, amount_arg],
        );

        ptb.transfer_arg(sender, coin_out);
        self.execute_ptb(runtime, sender, ptb, "withdraw", dry_run, false, None)
            .await
            .map(|(digest, _)| digest)
    }

    pub async fn snip_and_lp(
        &self,
        runtime: &BotRuntime,
        store: Option<&BotStateStore>,
        dex: Dex,
        token: &str,
        pool: &str,
        buy_amount: u64,
        dry_run: bool,
    ) -> Result<String> {
        match dex {
            Dex::Cetus => {
                self.snip_and_lp_cetus(runtime, store, token, pool, buy_amount, dry_run)
                    .await
            }
            Dex::Turbos => {
                self.snip_and_lp_turbos(runtime, store, token, pool, buy_amount, dry_run)
                    .await
            }
        }
    }

    pub async fn sell(
        &self,
        runtime: &BotRuntime,
        store: Option<&BotStateStore>,
        dex: Dex,
        token: &str,
        pool: &str,
        amount: u64,
        dry_run: bool,
    ) -> Result<String> {
        self.sell_with_metrics(
            runtime,
            store,
            dex,
            token,
            pool,
            amount,
            dry_run,
            None,
            store.is_none(),
        )
        .await
        .map(|(digest, _)| digest)
    }

    async fn read_token_balance(&self, runtime: &BotRuntime, token: &str) -> Result<u64> {
        runtime
            .rpc
            .dev_inspect_vault_token_balance(
                runtime.vault.address(),
                self.package,
                &self.vault_id.to_string(),
                token,
            )
            .await
    }

    async fn snip_and_lp_cetus(
        &self,
        runtime: &BotRuntime,
        store: Option<&BotStateStore>,
        token: &str,
        pool: &str,
        buy_amount: u64,
        dry_run: bool,
    ) -> Result<String> {
        let rpc = &runtime.rpc;
        let vault = &runtime.vault;
        let sender = vault.address();
        let token = normalize_coin_type(token);

        let mut ptb = ProgrammableTransactionBuilder::new();
        let buy_amt = ptb.pure(buy_amount)?;
        let sui_coin = ptb.command(Command::SplitCoins(Argument::GasCoin, vec![buy_amt]));

        let vault_arg = ptb.obj(rpc.object_arg(&self.vault_id.to_string(), true).await?)?;
        let config = ptb.obj(rpc.object_arg(CETUS_GLOBAL_CONFIG, false).await?)?;
        let pool_raw = rpc.object_arg(pool, true).await?;
        persist_pool_shared_version(store, pool, &pool_raw).await?;
        let pool_arg = ptb.obj(pool_raw)?;
        let partner = ptb.obj(rpc.object_arg(CETUS_PARTNER, true).await?)?;
        let tick_lower = ptb.pure(CETUS_TICK_LOWER)?;
        let tick_upper = ptb.pure(CETUS_TICK_UPPER)?;
        let token_for_lp = ptb.pure(CETUS_TOKEN_FOR_LP)?;
        let clock = clock_arg(&mut ptb)?;

        ptb.programmable_move_call(
            self.package,
            "cetus_ops".parse()?,
            "snip_and_lp_cetus".parse()?,
            vec![parse_type_tag(&token)?],
            vec![
                vault_arg,
                config,
                pool_arg,
                partner,
                sui_coin,
                tick_lower,
                tick_upper,
                token_for_lp,
                clock,
            ],
        );

        self.execute_ptb(runtime, sender, ptb, "snip_and_lp_cetus", dry_run, false, None)
            .await
            .map(|(digest, _)| digest)
    }

    async fn snip_and_lp_turbos(
        &self,
        runtime: &BotRuntime,
        store: Option<&BotStateStore>,
        token: &str,
        pool: &str,
        buy_amount: u64,
        dry_run: bool,
    ) -> Result<String> {
        let rpc = &runtime.rpc;
        let vault = &runtime.vault;
        let sender = vault.address();
        let token = normalize_coin_type(token);

        let (_, coin_type_a, coin_type_b, fee_type) = {
            let (_, a, b, fee) = rpc.get_turbos_pool_generics(pool).await?;
            ((), a, b, fee)
        };
        if coin_type_b != SUI_TYPE {
            bail!("snip vault turbos expects SUI as coin B; pool has {coin_type_b}");
        }
        if normalize_coin_type(&coin_type_a) != token {
            bail!("token type mismatch for turbos pool");
        }

        let (sqrt_price, fee_bps) = rpc.get_turbos_pool_sqrt_price_and_fee(pool).await?;
        let (amount_a, amount_b) = turbos_math::snip_lp_amounts(&sqrt_price, fee_bps);
        let (tick_lower, tick_upper) = turbos_math::aligned_ticks(fee_bps);
        let amount_a_min = turbos_math::minimum_amount(amount_a, TURBOS_SLIPPAGE_PERCENT);
        let amount_b_min = turbos_math::minimum_amount(amount_b, TURBOS_SLIPPAGE_PERCENT);
        let deadline = deadline_ms(MINT_DEADLINE_MS)?;

        // #region agent log
        agent_log(
            "H2",
            "snip_vault.rs:snip_and_lp_turbos",
            "ptb inputs",
            serde_json::json!({
                "buy_amount": buy_amount,
                "amount_a": amount_a,
                "amount_b": amount_b,
                "gas_budget": self.gas_budget,
                "dry_run": dry_run,
                "coin_type_a": coin_type_a,
                "coin_type_b": coin_type_b,
            }),
        );
        // #endregion

        let mut ptb = ProgrammableTransactionBuilder::new();
        let buy_amt = ptb.pure(buy_amount)?;
        let buy_sui = ptb.command(Command::SplitCoins(Argument::GasCoin, vec![buy_amt]));
        let lp_amt = ptb.pure(amount_b)?;
        let lp_sui = ptb.command(Command::SplitCoins(Argument::GasCoin, vec![lp_amt]));

        let vault_arg = ptb.obj(rpc.object_arg(&self.vault_id.to_string(), true).await?)?;
        let pool_raw = rpc.object_arg(pool, true).await?;
        persist_pool_shared_version(store, pool, &pool_raw).await?;
        let pool_arg = ptb.obj(pool_raw)?;
        let positions = ptb.obj(rpc.object_arg(turbos_contract::POSITIONS, true).await?)?;
        let versioned = ptb.obj(rpc.object_arg(turbos_contract::VERSIONED, false).await?)?;

        let tick_lower_abs = ptb.pure(u32::try_from(tick_lower.unsigned_abs()).context("tick lower")?)?;
        let tick_lower_neg = ptb.pure(tick_lower < 0)?;
        let tick_upper_abs = ptb.pure(u32::try_from(tick_upper.unsigned_abs()).context("tick upper")?)?;
        let tick_upper_neg = ptb.pure(tick_upper < 0)?;
        let amount_a_arg = ptb.pure(amount_a)?;
        let amount_b_arg = ptb.pure(amount_b)?;
        let amount_a_min_arg = ptb.pure(amount_a_min)?;
        let amount_b_min_arg = ptb.pure(amount_b_min)?;
        let deadline_arg = ptb.pure(deadline)?;
        let clock = clock_arg(&mut ptb)?;

        ptb.programmable_move_call(
            self.package,
            "turbos_ops".parse()?,
            "snip_and_lp_turbos".parse()?,
            vec![
                parse_type_tag(&coin_type_a)?,
                parse_type_tag(&fee_type)?,
            ],
            vec![
                vault_arg,
                pool_arg,
                positions,
                versioned,
                buy_sui,
                lp_sui,
                tick_lower_abs,
                tick_lower_neg,
                tick_upper_abs,
                tick_upper_neg,
                amount_a_arg,
                amount_b_arg,
                amount_a_min_arg,
                amount_b_min_arg,
                deadline_arg,
                clock,
            ],
        );

        self.execute_ptb(runtime, sender, ptb, "snip_and_lp_turbos", dry_run, false, None)
            .await
            .map(|(digest, _)| digest)
    }

    async fn sell_cetus(
        &self,
        runtime: &BotRuntime,
        store: Option<&BotStateStore>,
        token: &str,
        pool: &str,
        amount: u64,
        dry_run: bool,
        sell_detected_at: Option<Instant>,
        allow_pool_rpc_fallback: bool,
    ) -> Result<(String, crate::dex::agg_swap::SwapExecMetrics)> {
        use crate::bot::pool_shared::{pool_arg_for_sell, pool_arg_for_sell_with_fallback};

        let rpc = &runtime.rpc;
        let vault = &runtime.vault;
        let sender = vault.address();
        let token = normalize_coin_type(token);
        let vault_id = self.vault_id.to_string();

        let pool_arg = if allow_pool_rpc_fallback {
            pool_arg_for_sell_with_fallback(store, rpc, pool, true).await?
        } else {
            pool_arg_for_sell(store.context("sell requires bot state store")?, pool).await?
        };

        let (vault_arg, config, partner) = tokio::try_join!(
            rpc.object_arg_cached(&vault_id, true),
            rpc.object_arg_cached(CETUS_GLOBAL_CONFIG, false),
            rpc.object_arg_cached(CETUS_PARTNER, true),
        )?;
        // #region agent log
        agent_log(
            "H1",
            "snip_vault.rs:sell_cetus",
            "sell object args resolved",
            serde_json::json!({
                "partner_mutable": true,
                "pool": pool,
            }),
        );
        // #endregion

        let mut ptb = ProgrammableTransactionBuilder::new();
        let vault_arg = ptb.obj(vault_arg)?;
        let config = ptb.obj(config)?;
        let pool_arg = ptb.obj(pool_arg)?;
        let partner = ptb.obj(partner)?;
        let amount_arg = ptb.pure(amount)?;
        let clock = clock_arg(&mut ptb)?;

        let sui_out = ptb.programmable_move_call(
            self.package,
            "cetus_ops".parse()?,
            "sell_cetus".parse()?,
            vec![parse_type_tag(&token)?],
            vec![
                vault_arg,
                config,
                pool_arg,
                partner,
                amount_arg,
                clock,
            ],
        );

        ptb.command(Command::MergeCoins(Argument::GasCoin, vec![sui_out]));
        self.execute_ptb(
            runtime,
            sender,
            ptb,
            "sell_cetus",
            dry_run,
            true,
            sell_detected_at,
        )
            .await
    }

    async fn sell_turbos(
        &self,
        runtime: &BotRuntime,
        store: Option<&BotStateStore>,
        token: &str,
        pool: &str,
        amount: u64,
        dry_run: bool,
        sell_detected_at: Option<Instant>,
        allow_pool_rpc_fallback: bool,
    ) -> Result<(String, crate::dex::agg_swap::SwapExecMetrics)> {
        use crate::bot::pool_shared::{pool_arg_for_sell, pool_arg_for_sell_with_fallback};

        let rpc = &runtime.rpc;
        let vault = &runtime.vault;
        let sender = vault.address();
        let token = normalize_coin_type(token);
        let vault_id = self.vault_id.to_string();

        let pool_arg = if allow_pool_rpc_fallback {
            pool_arg_for_sell_with_fallback(store, rpc, pool, true).await?
        } else {
            pool_arg_for_sell(store.context("sell requires bot state store")?, pool).await?
        };

        let (generics, vault_arg, versioned) = tokio::try_join!(
            rpc.get_turbos_pool_generics(pool),
            rpc.object_arg_cached(&vault_id, true),
            rpc.object_arg_cached(turbos_contract::VERSIONED, false),
        )?;
        let (_, coin_type_a, coin_type_b, fee_type) = generics;
        if coin_type_b != SUI_TYPE || normalize_coin_type(&coin_type_a) != token {
            bail!("turbos pool orientation mismatch for vault sell");
        }

        let mut ptb = ProgrammableTransactionBuilder::new();
        let vault_arg = ptb.obj(vault_arg)?;
        let pool_arg = ptb.obj(pool_arg)?;
        let versioned = ptb.obj(versioned)?;
        let amount_arg = ptb.pure(amount)?;
        let clock = clock_arg(&mut ptb)?;

        let sui_out = ptb.programmable_move_call(
            self.package,
            "turbos_ops".parse()?,
            "sell_turbos".parse()?,
            vec![
                parse_type_tag(&coin_type_a)?,
                parse_type_tag(&fee_type)?,
            ],
            vec![vault_arg, pool_arg, versioned, amount_arg, clock],
        );

        ptb.command(Command::MergeCoins(Argument::GasCoin, vec![sui_out]));
        self.execute_ptb(
            runtime,
            sender,
            ptb,
            "sell_turbos",
            dry_run,
            true,
            sell_detected_at,
        )
            .await
    }

    async fn execute_ptb(
        &self,
        runtime: &BotRuntime,
        sender: SuiAddress,
        ptb: ProgrammableTransactionBuilder,
        label: &str,
        dry_run: bool,
        fast_submit: bool,
        sell_detected_at: Option<Instant>,
    ) -> Result<(String, crate::dex::agg_swap::SwapExecMetrics)> {
        let prep_start = Instant::now();
        let tx_data = if fast_submit {
            let (gas_price, current_epoch, chain_id) = tokio::try_join!(
                runtime.rpc.get_reference_gas_price_cached(),
                runtime.rpc.get_current_epoch(),
                runtime.rpc.get_chain_identifier(),
            )?;
            let nonce = self.sell_nonce.fetch_add(1, Ordering::Relaxed);
            // #region agent log
            agent_log(
                "H2",
                "snip_vault.rs:execute_ptb",
                "sell uses address balance gas",
                serde_json::json!({
                    "label": label,
                    "fast_submit": fast_submit,
                    "gas_price": gas_price,
                    "current_epoch": current_epoch,
                    "nonce": nonce,
                }),
            );
            // #endregion
            let chain = ChainIdentifier::from_chain_short_id(&chain_id)
                .context("unsupported chain identifier for address-balance gas")?;
            TransactionData::V1(TransactionDataV1 {
                kind: TransactionKind::ProgrammableTransaction(ptb.finish()),
                sender,
                gas_data: GasData {
                    payment: vec![],
                    owner: sender,
                    price: gas_price,
                    budget: self.gas_budget,
                },
                expiration: TransactionExpiration::ValidDuring {
                    min_epoch: Some(current_epoch),
                    max_epoch: Some(current_epoch.saturating_add(1)),
                    min_timestamp: None,
                    max_timestamp: None,
                    chain,
                    nonce,
                },
            })
        } else {
            // Gas coin ref must be fresh — caching ObjectRef causes stale version after each tx.
            let (gas, gas_price) = tokio::try_join!(
                select_gas_coin(&runtime.rpc, sender, self.gas_budget),
                runtime.rpc.get_reference_gas_price_cached(),
            )?;
            TransactionData::new_programmable(
                sender,
                vec![gas],
                ptb.finish(),
                self.gas_budget,
                gas_price,
            )
        };
        let prep_ms = prep_start.elapsed().as_millis() as u64;
        let sig = runtime.vault.sign_transaction(&tx_data);
        let request_type = fast_submit.then(|| runtime.config.sell_tx_request_type.as_str());
        let submit_at = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let detect_to_submit_ms = sell_detected_at.map(|t| t.elapsed().as_millis() as u64);
        let submit_start = Instant::now();
        let digest = runtime
            .rpc
            .execute_or_dry_run(tx_data, sig, dry_run, request_type)
            .await?;
        let confirm_ms = submit_start.elapsed().as_millis() as u64;
        info!(
            result = %digest,
            label,
            dry_run,
            fast_submit,
            detect_to_submit_ms,
            prep_ms,
            confirm_ms,
            "snip vault tx"
        );
        Ok((
            digest,
            crate::dex::agg_swap::SwapExecMetrics {
                build_ms: prep_ms,
                detect_to_build_ms: None,
                detect_to_submit_ms,
                submit_at,
                confirm_ms,
            },
        ))
    }
}

fn deadline_ms(offset_ms: u64) -> Result<u64> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .context("clock")?
        .as_millis();
    u64::try_from(now)
        .context("deadline")?
        .checked_add(offset_ms)
        .context("deadline overflow")
}

async fn wallet_coin_for_deposit(
    ptb: &mut ProgrammableTransactionBuilder,
    rpc: &crate::provider::SuiRpcClient,
    owner: SuiAddress,
    coin_type: &str,
    amount: Option<u64>,
) -> Result<Argument> {
    if coin_type == SUI_TYPE {
        bail!("use token coin type, not SUI (gas coin cannot be deposited this way)");
    }

    let coins = rpc.get_coins(owner, coin_type).await?;
    if coins.is_empty() {
        bail!("no {coin_type} coins in wallet {owner}");
    }

    let total: u128 = coins.iter().map(|c| c.balance).sum();
    let deposit_amount = match amount {
        Some(a) => {
            if u128::from(a) > total {
                bail!("wallet balance {total} < requested deposit {a}");
            }
            a
        }
        None => u64::try_from(total).context("wallet balance exceeds u64")?,
    };
    if deposit_amount == 0 {
        bail!("deposit amount must be positive");
    }

    let refs: Vec<ObjectRef> = {
        let mut out = Vec::new();
        for coin in &coins {
            out.push(rpc.get_object_ref(&coin.coin_object_id).await?);
        }
        out
    };

    let primary = ptb.obj(ObjectArg::ImmOrOwnedObject(refs[0]))?;
    if refs.len() > 1 {
        let merge_args: Vec<Argument> = refs[1..]
            .iter()
            .map(|r| ptb.obj(ObjectArg::ImmOrOwnedObject(*r)).unwrap())
            .collect();
        ptb.command(Command::MergeCoins(primary, merge_args));
    }

    if u128::from(deposit_amount) == total {
        return Ok(primary);
    }

    let amt = ptb.pure(deposit_amount)?;
    Ok(ptb.command(Command::SplitCoins(primary, vec![amt])))
}

async fn select_gas_coin(
    rpc: &crate::provider::SuiRpcClient,
    owner: SuiAddress,
    min_balance: u64,
) -> Result<ObjectRef> {
    rpc.select_gas_coin(owner, min_balance).await
}

pub fn try_load_vault_client() -> Result<Option<Arc<SnipVaultClient>>> {
    Ok(SnipVaultClient::from_env()?.map(Arc::new))
}
