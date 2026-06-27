use anyhow::{Context, Result, bail};
use std::time::{SystemTime, UNIX_EPOCH};
use sui_types::base_types::{ObjectID, ObjectRef, SuiAddress};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{Argument, Command, ObjectArg, TransactionData};
use tracing::{info, warn};

use crate::bot::config::BotRuntime;
use crate::bot::token_type::parse_type_tag;
use crate::dex::agg_swap::clock_arg;
use crate::dex::turbos_contract::{self, MINT_DEADLINE_MS};
use crate::dex::turbos_math;
use crate::telegram_format::format_add_liquidity_fail;

const SUI_TYPE: &str = "0x2::sui::SUI";
const SLIPPAGE_PERCENT: u64 = 5;

pub async fn open_pool_position_with_lp_fixed(
    runtime: &BotRuntime,
    pool: &str,
    symbol: &str,
    notify_fail: bool,
) -> Result<String> {
    let mut attempt = 0;
    let mut last_err: Option<anyhow::Error> = None;
    while attempt < 3 {
        match try_open(runtime, pool).await {
            Ok(digest) => {
                info!(digest = %digest, pool = %pool, "turbos lp fixed success");
                return Ok(digest);
            }
            Err(err) => {
                warn!(?err, pool = %pool, attempt, "turbos lp fixed retry");
                last_err = Some(err);
                attempt += 1;
                if attempt >= 3 {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
    let err = last_err.unwrap_or_else(|| anyhow::anyhow!("turbos lp failed with no attempts"));
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

async fn try_open(runtime: &BotRuntime, pool: &str) -> Result<String> {
    let rpc = &runtime.rpc;
    let vault = &runtime.vault;
    let sender = vault.address();

    let (coin_type_a, coin_type_b, fee_type) = {
        let (_, a, b, fee) = rpc.get_turbos_pool_generics(pool).await?;
        (a, b, fee)
    };
    let (sqrt_price, fee_bps) = rpc.get_turbos_pool_sqrt_price_and_fee(pool).await?;
    let (amount_a, amount_b) = turbos_math::snip_lp_amounts(&sqrt_price, fee_bps);
    let (tick_lower, tick_upper) = turbos_math::aligned_ticks(fee_bps);

    let amount_a_min = turbos_math::minimum_amount(amount_a, SLIPPAGE_PERCENT);
    let amount_b_min = turbos_math::minimum_amount(amount_b, SLIPPAGE_PERCENT);

    let mint_package: ObjectID = turbos_contract::PACKAGE_ID.parse().context("mint package")?;

    let mut ptb = ProgrammableTransactionBuilder::new();

    let pending_sui_split = if coin_type_b == SUI_TYPE && amount_b > 0 {
        let amt = ptb.pure(amount_b)?;
        Some(ptb.command(Command::SplitCoins(Argument::GasCoin, vec![amt])))
    } else {
        None
    };

    let coin_a_vec = if amount_a == 0 {
        let zero = coin_zero(&mut ptb, &coin_type_a)?;
        ptb.command(Command::MakeMoveVec(None, vec![zero]))
    } else if coin_type_a == SUI_TYPE {
        let amt = ptb.pure(amount_a)?;
        let sui_coin = ptb.command(Command::SplitCoins(Argument::GasCoin, vec![amt]));
        ptb.command(Command::MakeMoveVec(None, vec![sui_coin]))
    } else {
        let coin = take_coin_amount(rpc, &mut ptb, sender, &coin_type_a, amount_a).await?;
        ptb.command(Command::MakeMoveVec(None, vec![coin]))
    };

    let coin_b_vec = if let Some(sui_coin) = pending_sui_split {
        ptb.command(Command::MakeMoveVec(None, vec![sui_coin]))
    } else if amount_b == 0 {
        let zero = coin_zero(&mut ptb, &coin_type_b)?;
        ptb.command(Command::MakeMoveVec(None, vec![zero]))
    } else {
        let coin = take_coin_amount(rpc, &mut ptb, sender, &coin_type_b, amount_b).await?;
        ptb.command(Command::MakeMoveVec(None, vec![coin]))
    };

    let pool_arg = ptb.obj(rpc.object_arg(pool, true).await?)?;
    let positions_arg = ptb.obj(rpc.object_arg(turbos_contract::POSITIONS, true).await?)?;
    let versioned_arg = ptb.obj(rpc.object_arg(turbos_contract::VERSIONED, false).await?)?;
    let clock = clock_arg(&mut ptb)?;

    let tick_lower_abs = ptb.pure(u32::try_from(tick_lower.unsigned_abs()).context("tick lower")?)?;
    let tick_lower_neg = ptb.pure(tick_lower < 0)?;
    let tick_upper_abs = ptb.pure(u32::try_from(tick_upper.unsigned_abs()).context("tick upper")?)?;
    let tick_upper_neg = ptb.pure(tick_upper < 0)?;
    let amount_a_arg = ptb.pure(amount_a)?;
    let amount_b_arg = ptb.pure(amount_b)?;
    let amount_a_min_arg = ptb.pure(amount_a_min)?;
    let amount_b_min_arg = ptb.pure(amount_b_min)?;
    let recipient = ptb.pure(sender)?;
    let deadline = ptb.pure(deadline_ms(MINT_DEADLINE_MS)?)?;

    ptb.programmable_move_call(
        mint_package,
        "position_manager".parse()?,
        "mint".parse()?,
        vec![
            parse_type_tag(&coin_type_a)?,
            parse_type_tag(&coin_type_b)?,
            parse_type_tag(&fee_type)?,
        ],
        vec![
            pool_arg,
            positions_arg,
            coin_a_vec,
            coin_b_vec,
            tick_lower_abs,
            tick_lower_neg,
            tick_upper_abs,
            tick_upper_neg,
            amount_a_arg,
            amount_b_arg,
            amount_a_min_arg,
            amount_b_min_arg,
            recipient,
            deadline,
            clock,
            versioned_arg,
        ],
    );

    let gas = select_gas(rpc, sender).await?;
    let gas_price = rpc.get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas], ptb.finish(), 200_000_000, gas_price);
    let sig = vault.sign_transaction(&tx_data);
    rpc.execute_transaction(tx_data, sig).await
}

fn coin_zero(ptb: &mut ProgrammableTransactionBuilder, coin_type: &str) -> Result<Argument> {
    Ok(ptb.programmable_move_call(
        ObjectID::from_hex_literal("0x2").context("0x2")?,
        "coin".parse()?,
        "zero".parse()?,
        vec![parse_type_tag(coin_type)?],
        vec![],
    ))
}

async fn take_coin_amount(
    rpc: &crate::provider::SuiRpcClient,
    ptb: &mut ProgrammableTransactionBuilder,
    owner: SuiAddress,
    coin_type: &str,
    amount: u64,
) -> Result<Argument> {
    let coins = rpc.get_coins(owner, coin_type).await?;
    if coins.is_empty() {
        bail!("no coins for {coin_type}");
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
    let amt = ptb.pure(amount)?;
    Ok(ptb.command(Command::SplitCoins(primary, vec![amt])))
}

fn deadline_ms(offset_ms: u64) -> Result<u64> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("clock")?
        .as_millis();
    u64::try_from(now).context("deadline")?
        .checked_add(offset_ms)
        .context("deadline overflow")
}

async fn select_gas(
    rpc: &crate::provider::SuiRpcClient,
    owner: SuiAddress,
) -> Result<ObjectRef> {
    let coins = rpc.get_coins(owner, SUI_TYPE).await?;
    let first = coins.first().context("gas coin")?;
    rpc.get_object_ref(&first.coin_object_id).await
}
