use anyhow::{Context, Result};
use sui_types::base_types::{ObjectRef, SuiAddress};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{Argument, Command, ObjectArg, TransactionData};
use tracing::info;

use crate::bot::config::BotRuntime;
use crate::bot::pool_shared::persist_pool_shared_version;
use crate::bot::state::BotStateStore;
use crate::bot::token_type::{normalize_coin_type, parse_type_tag};
use crate::dex::agg_swap::clock_arg;
use crate::telegram_format::format_add_liquidity_fail;

const CETUS_GLOBAL_CONFIG: &str =
    "0xdaa46292632c3c4d8f31f23ea0f9b36a28ff3677e9684980e4438403a67a3d8f";
const CETUS_POOL_SCRIPT: &str =
    "0xb2db7142fa83210a7d78d9c12ac49c043b3cbbd482224fea6e3da00aa5a5ae2d";

pub async fn open_pool_position_with_lp_fixed(
    runtime: &BotRuntime,
    store: Option<&BotStateStore>,
    pool: &str,
    symbol: &str,
    notify_fail: bool,
    dry_run: bool,
) -> Result<String> {
    let mut attempt = 0;
    let mut last_err: Option<anyhow::Error> = None;
    let max_attempts = if dry_run { 1 } else { 3 };
    while attempt < max_attempts {
        match try_open(runtime, store, pool, dry_run).await {
            Ok(digest) => {
                info!(digest = %digest, pool = %pool, "cetus lp fixed success");
                return Ok(digest);
            }
            Err(err) => {
                last_err = Some(err);
                attempt += 1;
                if attempt >= 3 {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
    let err = last_err.unwrap_or_else(|| anyhow::anyhow!("cetus lp failed with no attempts"));
    if notify_fail {
        crate::telegram_notify::send_bot_message(&format_add_liquidity_fail(
            symbol,
            pool,
            &err.to_string(),
        ))
        .await;
    }
    Err(err)
}

async fn try_open(
    runtime: &BotRuntime,
    store: Option<&BotStateStore>,
    pool: &str,
    dry_run: bool,
) -> Result<String> {
    let rpc = &runtime.rpc;
    let vault = &runtime.vault;
    let sender = vault.address();
    let token_type = rpc.get_pool_token_type(pool).await?;
    let sui_type = "0x2::sui::SUI";

    let mut ptb = ProgrammableTransactionBuilder::new();
    let token_coin = take_coin(rpc, &mut ptb, sender, &token_type, 1_000_000_000).await?;
    let sui_coin = take_coin(rpc, &mut ptb, sender, sui_type, 0).await?;

    let lower_tick: u32 = 37_680;
    let upper_tick: u32 = 69_120;
    let amount_a = 1_000_000_000u64;

    let global = ptb.obj(rpc.object_arg(CETUS_GLOBAL_CONFIG, false).await?)?;
    let pool_raw = rpc.object_arg(pool, true).await?;
    persist_pool_shared_version(store, pool, &pool_raw).await?;
    let pool_obj = ptb.obj(pool_raw)?;
    let lower = ptb.pure(lower_tick)?;
    let upper = ptb.pure(upper_tick)?;
    let amount_a_arg = ptb.pure(amount_a)?;
    let zero = ptb.pure(0u64)?;
    let flag = ptb.pure(true)?;
    let clock = clock_arg(&mut ptb)?;

    ptb.programmable_move_call(
        CETUS_POOL_SCRIPT.parse()?,
        "pool_script_v2".parse()?,
        "open_position_with_liquidity_by_fix_coin".parse()?,
        vec![parse_type_tag(&token_type)?, parse_type_tag(sui_type)?],
        vec![
            global,
            pool_obj,
            lower,
            upper,
            token_coin,
            sui_coin,
            amount_a_arg,
            zero,
            flag,
            clock,
        ],
    );

    let gas = select_gas(rpc, sender).await?;
    let gas_price = rpc.get_reference_gas_price().await?;
    let tx_data = TransactionData::new_programmable(sender, vec![gas], ptb.finish(), 200_000_000, gas_price);
    let sig = vault.sign_transaction(&tx_data);
    rpc.execute_or_dry_run(tx_data, sig, dry_run, None).await
}

async fn take_coin(
    rpc: &crate::provider::SuiRpcClient,
    ptb: &mut ProgrammableTransactionBuilder,
    owner: SuiAddress,
    coin_type: &str,
    amount: u64,
) -> Result<Argument> {
    if coin_type == "0x2::sui::SUI" && amount == 0 {
        let amt = ptb.pure(0u64)?;
        return Ok(ptb.command(Command::SplitCoins(Argument::GasCoin, vec![amt])));
    }
    let coins = rpc.get_coins(owner, &normalize_coin_type(coin_type)).await?;
    let first = coins.first().context("no coin")?;
    let object_ref = rpc.get_object_ref(&first.coin_object_id).await?;
    let arg = ptb.obj(ObjectArg::ImmOrOwnedObject(object_ref))?;
    if amount > 0 {
        let amt = ptb.pure(amount)?;
        Ok(ptb.command(Command::SplitCoins(arg, vec![amt])))
    } else {
        Ok(arg)
    }
}

async fn select_gas(
    rpc: &crate::provider::SuiRpcClient,
    owner: SuiAddress,
) -> Result<ObjectRef> {
    let coins = rpc.get_coins(owner, "0x2::sui::SUI").await?;
    let first = coins.first().context("gas coin")?;
    rpc.get_object_ref(&first.coin_object_id).await
}
