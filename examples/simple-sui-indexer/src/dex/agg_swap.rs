use anyhow::{Context, Result, bail};
use std::sync::Arc;
use std::time::Instant;
use sui_types::base_types::{ObjectID, ObjectRef, SuiAddress};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{Argument, Command, ObjectArg, SharedObjectMutability, TransactionData};
use sui_types::SUI_CLOCK_OBJECT_ID;

use crate::bot::state::Dex;
use crate::provider::rpc::SuiRpcClient;
use crate::provider::VaultKeypair;

const AGG_PACKAGE: &str =
    "0x3a7fa58adcd7ff474ca0330c93068b139f5263c0cf9c64e702f5c4b17996ff10";
const AGG_GLOBAL_CONFIG: &str =
    "0xdaa46292632c3c4d8f31f23ea0f9b36a28ff3677e9684980e4438403a67a3d8f";
const AGG_PARTNER_CONFIG: &str =
    "0x639b5e433da31739e800cd085f356e64cae222966d0f1b11bd9dc76b322ff58b";
const AGG_TURBOS_VERSION: &str =
    "0xf1cf0e81048df168ebeb1b8030fad24b3e0b53ae827c25053fff0779c1445b6f";
const SUI_TYPE: &str = "0x2::sui::SUI";

#[derive(Clone, Copy, Debug)]
pub enum SwapMode {
    Safe,
    Fast,
    Superfast,
}

#[derive(Debug, Clone)]
pub struct SwapExecMetrics {
    /// RPC + PTB build time inside `swap_exact_amount` (ms).
    pub build_ms: u64,
    /// Detect → build done (ms), when sell passes `sell_detected_at`.
    pub detect_to_build_ms: Option<u64>,
    /// Detect → submit call to RPC started (ms).
    pub detect_to_submit_ms: Option<u64>,
    pub submit_at: String,
    /// Submit → local execution confirmed (ms).
    pub confirm_ms: u64,
}

pub struct AggSwap {
    rpc: Arc<SuiRpcClient>,
    vault: Arc<VaultKeypair>,
    sell_tx_request_type: Option<String>,
}

impl AggSwap {
    pub fn new(
        rpc: Arc<SuiRpcClient>,
        vault: Arc<VaultKeypair>,
        sell_tx_request_type: Option<String>,
    ) -> Self {
        Self {
            rpc,
            vault,
            sell_tx_request_type,
        }
    }

    pub async fn swap_exact_amount(
        &self,
        dex: Dex,
        a2b: bool,
        token: &str,
        pool: &str,
        amount: u64,
        input_is_coin: bool,
        mode: SwapMode,
        sell_detected_at: Option<Instant>,
        dry_run: bool,
    ) -> Result<(String, SwapExecMetrics)> {
        let build_start = Instant::now();
        let sender = self.vault.address();
        let gas_price_ref = self.rpc.get_reference_gas_price_cached().await?;
        let (budget, price) = match mode {
            // Safe: bot-snip leaves gas budget to SDK auto-estimate (~0.1–0.5 SUI on mainnet).
            // 50M was too low for Turbos agg swap (InsufficientGas at MergeCoins).
            SwapMode::Safe => (500_000_000, gas_price_ref),
            SwapMode::Fast => (2_000_000_000, 1000),
            SwapMode::Superfast => (4_000_000_000, 2000),
        };

        let mut ptb = ProgrammableTransactionBuilder::new();
        let normalized_token = crate::bot::token_type::normalize_coin_type(token);
        let coin_type = if a2b {
            normalized_token.as_str()
        } else {
            SUI_TYPE
        };
        let input_amount = if a2b && !input_is_coin {
            self.estimate_token_amount_for_sui(dex, token, pool, amount)
                .await?
        } else {
            amount
        };

        let coin_arg = self
            .take_amount_from_coins(&mut ptb, sender, coin_type, input_amount)
            .await?;

        let swap_result = match dex {
            Dex::Cetus => {
                self.build_cetus_swap(&mut ptb, a2b, &normalized_token, pool, coin_arg)
                    .await?
            }
            Dex::Turbos => {
                self.build_turbos_swap(&mut ptb, a2b, &normalized_token, pool, coin_arg)
                    .await?
            }
        };

        if a2b {
            ptb.command(Command::MergeCoins(Argument::GasCoin, vec![swap_result]));
        } else {
            ptb.transfer_arg(sender, swap_result);
        }

        let gas = self.rpc.select_gas_coin(sender, budget).await?;
        let tx_data =
            TransactionData::new_programmable(sender, vec![gas], ptb.finish(), budget, price);
        let sig = self.vault.sign_transaction(&tx_data);

        let build_ms = build_start.elapsed().as_millis() as u64;
        let detect_to_build_ms = sell_detected_at.map(|t| t.elapsed().as_millis() as u64);
        let submit_at = chrono::Utc::now()
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let submit_start = Instant::now();
        let detect_to_submit_ms = sell_detected_at.map(|t| t.elapsed().as_millis() as u64);
        let sell_request = a2b
            .then(|| self.sell_tx_request_type.as_deref())
            .flatten();

        match self
            .rpc
            .execute_or_dry_run(tx_data, sig, dry_run, sell_request)
            .await
        {
            Ok(digest) => {
                let confirm_ms = submit_start.elapsed().as_millis() as u64;
                let metrics = SwapExecMetrics {
                    build_ms,
                    detect_to_build_ms,
                    detect_to_submit_ms,
                    submit_at: submit_at.clone(),
                    confirm_ms,
                };
                tracing::info!(
                    ?dex,
                    a2b,
                    ?mode,
                    digest = %digest,
                    build_ms,
                    detect_to_build_ms,
                    submit_at = %submit_at,
                    confirm_ms,
                    "swap tx confirmed"
                );
                Ok((digest, metrics))
            }
            Err(err) => {
                tracing::error!(
                    ?dex,
                    a2b,
                    ?mode,
                    gas_budget = budget,
                    gas_price = price,
                    build_ms,
                    detect_to_build_ms,
                    submit_at = %submit_at,
                    confirm_ms = submit_start.elapsed().as_millis() as u64,
                    "swap transaction failed"
                );
                Err(err)
            }
        }
    }

    async fn build_cetus_swap(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        a2b: bool,
        token: &str,
        pool: &str,
        coins: Argument,
    ) -> Result<Argument> {
        let method = if a2b { "swap_a2b" } else { "swap_b2a" };
        let global = obj_arg(self.rpc.as_ref(), ptb, AGG_GLOBAL_CONFIG, false).await?;
        let pool_arg = obj_arg(self.rpc.as_ref(), ptb, pool, true).await?;
        let partner = obj_arg(self.rpc.as_ref(), ptb, AGG_PARTNER_CONFIG, true).await?;
        let clock = clock_arg(ptb)?;
        Ok(ptb.programmable_move_call(
            parse_pkg(AGG_PACKAGE)?,
            parse_ident("cetus")?,
            parse_ident(method)?,
            vec![parse_type_tag(token)?, parse_type_tag(SUI_TYPE)?],
            vec![global, pool_arg, partner, coins, clock],
        ))
    }

    async fn build_turbos_swap(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        a2b: bool,
        token: &str,
        pool: &str,
        coins: Argument,
    ) -> Result<Argument> {
        let method = if a2b { "swap_a2b" } else { "swap_b2a" };
        let fee_type = self.rpc.get_turbos_fee_type(pool).await?;
        let pool_arg = obj_arg(self.rpc.as_ref(), ptb, pool, true).await?;
        let clock = clock_arg(ptb)?;
        let version = obj_arg(self.rpc.as_ref(), ptb, AGG_TURBOS_VERSION, false).await?;
        Ok(ptb.programmable_move_call(
            parse_pkg(AGG_PACKAGE)?,
            parse_ident("turbos")?,
            parse_ident(method)?,
            vec![
                parse_type_tag(token)?,
                parse_type_tag(SUI_TYPE)?,
                parse_type_tag(&fee_type)?,
            ],
            vec![pool_arg, coins, clock, version],
        ))
    }

    async fn take_amount_from_coins(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        owner: SuiAddress,
        coin_type: &str,
        amount: u64,
    ) -> Result<Argument> {
        if coin_type == SUI_TYPE {
            let amt = ptb.pure(amount)?;
            return Ok(ptb.command(Command::SplitCoins(Argument::GasCoin, vec![amt])));
        }

        let coins = self.rpc.get_coins(owner, &crate::bot::token_type::normalize_coin_type(coin_type)).await?;
        if coins.is_empty() {
            bail!("no coins for {coin_type}");
        }
        let refs: Vec<ObjectRef> = {
            let mut out = Vec::new();
            for coin in &coins {
                out.push(self.rpc.get_object_ref(&coin.coin_object_id).await?);
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
        let amt = ptb.pure(amount)?;
        Ok(ptb.command(Command::SplitCoins(primary, vec![amt])))
    }

    async fn estimate_token_amount_for_sui(
        &self,
        _dex: Dex,
        _token: &str,
        _pool: &str,
        sui_amount: u64,
    ) -> Result<u64> {
        Ok(sui_amount.saturating_mul(1_000))
    }
}

pub async fn obj_arg(
    rpc: &SuiRpcClient,
    ptb: &mut ProgrammableTransactionBuilder,
    object_id: &str,
    mutable: bool,
) -> Result<Argument> {
    Ok(ptb.obj(rpc.object_arg(object_id, mutable).await?)?)
}

pub fn clock_arg(ptb: &mut ProgrammableTransactionBuilder) -> Result<Argument> {
    Ok(ptb.obj(ObjectArg::SharedObject {
        id: SUI_CLOCK_OBJECT_ID,
        initial_shared_version: 1.into(),
        mutability: SharedObjectMutability::Immutable,
    })?)
}

fn parse_pkg(s: &str) -> Result<ObjectID> {
    s.parse().context("parse package id")
}

fn parse_ident(s: &str) -> Result<sui_types::Identifier> {
    s.parse().context("parse identifier")
}

use crate::bot::token_type::parse_type_tag;
