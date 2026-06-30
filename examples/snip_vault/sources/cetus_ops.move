/// Cetus snip (buy + LP) and sell — atomic swap + liquidity on Cetus CLMM.
module snip_vault::cetus_ops;

use cetusclmm::config::GlobalConfig;
use cetusclmm::partner::Partner;
use cetusclmm::pool::{Self, Pool, FlashSwapReceipt};
use cetusclmm::tick_math;
use snip_vault::vault::{Self, Vault};
use sui::balance::{Self, Balance};
use sui::clock::Clock;
use sui::coin::{Self, Coin};
use sui::event;
use sui::sui::SUI;

const DEFAULT_PARTNER_ID: address =
    @0x639b5e433da31739e800cd085f356e64cae222966d0f1b11bd9dc76b322ff58b;

#[error]
const ESwapPayMismatch: vector<u8> = b"flash swap repay amount does not match input";
#[error]
const ETokenForLpZero: vector<u8> = b"token amount for LP must be positive";
#[error]
const ELpNeedsSui: vector<u8> = b"Cetus LP requires token-only deposit (pay_b must be zero)";

public struct CetusSniped has copy, drop {
    vault_id: ID,
    pool_id: ID,
    buy_sui: u64,
    token_received: u64,
    token_for_lp: u64,
}

public struct CetusSold has copy, drop {
    vault_id: ID,
    pool_id: ID,
    token_in: u64,
    sui_out: u64,
}

/// Buy token with SUI then open a Cetus position + add liquidity in one transaction.
public fun snip_and_lp_cetus<Token>(
    vault: &mut Vault,
    config: &GlobalConfig,
    pool: &mut Pool<Token, SUI>,
    partner: &mut Partner,
    sui_in: Coin<SUI>,
    tick_lower: u32,
    tick_upper: u32,
    token_for_lp: u64,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    vault::assert_authorized(vault, ctx);
    let buy_sui = coin::value(&sui_in);
    let pool_id = object::id(pool);

    let token_coin = swap_b2a(vault, config, pool, partner, sui_in, clock, ctx);
    let token_received = coin::value(&token_coin);

    let lp_amount = if (token_for_lp > token_received) {
        token_received
    } else {
        token_for_lp
    };
    assert!(lp_amount > 0, ETokenForLpZero);

    let mut token_coin = token_coin;
    let lp_coin = token_coin.split(lp_amount, ctx);
    if (coin::value(&token_coin) > 0) {
        vault::deposit_coin(vault, token_coin);
    } else {
        coin::destroy_zero(token_coin);
    };

    let mut position = pool::open_position<Token, SUI>(
        config,
        pool,
        tick_lower,
        tick_upper,
        ctx,
    );
    let receipt = pool::add_liquidity_fix_coin<Token, SUI>(
        config,
        pool,
        &mut position,
        lp_amount,
        true,
        clock,
    );
    let (pay_a, pay_b) = pool::add_liquidity_pay_amount(&receipt);
    let mut lp_coin = lp_coin;
    let pay_a_bal = lp_coin.split(pay_a, ctx).into_balance();
    let rem_token = if (coin::value(&lp_coin) > 0) {
        coin::into_balance(lp_coin)
    } else {
        coin::destroy_zero(lp_coin);
        balance::zero<Token>()
    };
    let pay_b_bal = balance::zero<SUI>();
    pool::repay_add_liquidity<Token, SUI>(
        config,
        pool,
        pay_a_bal,
        pay_b_bal,
        receipt,
    );
    assert!(pay_b == 0, ELpNeedsSui);
    if (balance::value(&rem_token) > 0) {
        vault::deposit_balance(vault, rem_token);
    } else {
        balance::destroy_zero(rem_token);
    };

    vault::store_position_object(vault, pool_id, position);

    event::emit(CetusSniped {
        vault_id: object::id(vault),
        pool_id,
        buy_sui,
        token_received,
        token_for_lp: lp_amount,
    });
}

/// Sell token from vault; returns SUI for the bot PTB to merge into gas coin.
public fun sell_cetus<Token>(
    vault: &mut Vault,
    config: &GlobalConfig,
    pool: &mut Pool<Token, SUI>,
    partner: &mut Partner,
    amount: u64,
    clock: &Clock,
    ctx: &mut TxContext,
): Coin<SUI> {
    vault::assert_authorized(vault, ctx);
    let token_coin = vault::withdraw_coin<Token>(vault, amount, ctx);
    let token_in = coin::value(&token_coin);
    let sui_out = swap_a2b(vault, config, pool, partner, token_coin, clock, ctx);
    event::emit(CetusSold {
        vault_id: object::id(vault),
        pool_id: object::id(pool),
        token_in,
        sui_out: coin::value(&sui_out),
    });
    sui_out
}

fun swap_a2b<Token>(
    vault: &mut Vault,
    config: &GlobalConfig,
    pool: &mut Pool<Token, SUI>,
    partner: &mut Partner,
    coin_a: Coin<Token>,
    clock: &Clock,
    ctx: &mut TxContext,
): Coin<SUI> {
    let amount_in = coin_a.value();
    let (receive_b, flash_receipt, pay_amount) = flash_swap_a2b<Token, SUI>(
        config,
        pool,
        partner,
        amount_in,
        true,
        tick_math::min_sqrt_price(),
        clock,
        ctx,
    );
    assert!(pay_amount == amount_in, ESwapPayMismatch);
    let rem_a = repay_flash_swap_a2b(vault, config, pool, partner, coin_a, flash_receipt, ctx);
    deposit_optional(vault, rem_a);
    receive_b
}

fun swap_b2a<Token>(
    vault: &mut Vault,
    config: &GlobalConfig,
    pool: &mut Pool<Token, SUI>,
    partner: &mut Partner,
    coin_b: Coin<SUI>,
    clock: &Clock,
    ctx: &mut TxContext,
): Coin<Token> {
    let amount_in = coin_b.value();
    let (receive_a, flash_receipt, pay_amount) = flash_swap_b2a<Token, SUI>(
        config,
        pool,
        partner,
        amount_in,
        true,
        tick_math::max_sqrt_price(),
        clock,
        ctx,
    );
    assert!(pay_amount == amount_in, ESwapPayMismatch);
    let rem_b = repay_flash_swap_b2a(vault, config, pool, partner, coin_b, flash_receipt, ctx);
    deposit_optional(vault, rem_b);
    receive_a
}

fun flash_swap_a2b<CoinA, CoinB>(
    config: &GlobalConfig,
    pool: &mut Pool<CoinA, CoinB>,
    partner: &Partner,
    amount: u64,
    by_amount_in: bool,
    sqrt_price_limit: u128,
    clock: &Clock,
    ctx: &mut TxContext,
): (Coin<CoinB>, FlashSwapReceipt<CoinA, CoinB>, u64) {
    let (bal_a, bal_b, flash_receipt) = flash_swap_balances(
        config,
        pool,
        partner,
        amount,
        true,
        by_amount_in,
        sqrt_price_limit,
        clock,
    );
    destroy_balance_zero(bal_a);
    let coin_b = coin::from_balance(bal_b, ctx);
    let repay_amount = pool::swap_pay_amount(&flash_receipt);
    (coin_b, flash_receipt, repay_amount)
}

fun flash_swap_b2a<CoinA, CoinB>(
    config: &GlobalConfig,
    pool: &mut Pool<CoinA, CoinB>,
    partner: &Partner,
    amount: u64,
    by_amount_in: bool,
    sqrt_price_limit: u128,
    clock: &Clock,
    ctx: &mut TxContext,
): (Coin<CoinA>, FlashSwapReceipt<CoinA, CoinB>, u64) {
    let (bal_a, bal_b, flash_receipt) = flash_swap_balances(
        config,
        pool,
        partner,
        amount,
        false,
        by_amount_in,
        sqrt_price_limit,
        clock,
    );
    destroy_balance_zero(bal_b);
    let coin_a = coin::from_balance(bal_a, ctx);
    let repay_amount = pool::swap_pay_amount(&flash_receipt);
    (coin_a, flash_receipt, repay_amount)
}

fun flash_swap_balances<CoinA, CoinB>(
    config: &GlobalConfig,
    pool: &mut Pool<CoinA, CoinB>,
    partner: &Partner,
    amount: u64,
    a2b: bool,
    by_amount_in: bool,
    sqrt_price_limit: u128,
    clock: &Clock,
): (Balance<CoinA>, Balance<CoinB>, FlashSwapReceipt<CoinA, CoinB>) {
    if (object::id_address(partner) == DEFAULT_PARTNER_ID) {
        pool::flash_swap<CoinA, CoinB>(
            config,
            pool,
            a2b,
            by_amount_in,
            amount,
            sqrt_price_limit,
            clock,
        )
    } else {
        pool::flash_swap_with_partner<CoinA, CoinB>(
            config,
            pool,
            partner,
            a2b,
            by_amount_in,
            amount,
            sqrt_price_limit,
            clock,
        )
    }
}

fun repay_flash_swap_a2b<CoinA, CoinB>(
    vault: &mut Vault,
    config: &GlobalConfig,
    pool: &mut Pool<CoinA, CoinB>,
    partner: &mut Partner,
    coin_a: Coin<CoinA>,
    receipt: FlashSwapReceipt<CoinA, CoinB>,
    ctx: &mut TxContext,
): Coin<CoinA> {
    let (rem_a, rem_b) = repay_flash_swap(
        config,
        pool,
        partner,
        true,
        coin_a,
        coin::zero<CoinB>(ctx),
        receipt,
        ctx,
    );
    deposit_optional(vault, rem_b);
    rem_a
}

fun repay_flash_swap_b2a<CoinA, CoinB>(
    vault: &mut Vault,
    config: &GlobalConfig,
    pool: &mut Pool<CoinA, CoinB>,
    partner: &mut Partner,
    coin_b: Coin<CoinB>,
    receipt: FlashSwapReceipt<CoinA, CoinB>,
    ctx: &mut TxContext,
): Coin<CoinB> {
    let (rem_a, rem_b) = repay_flash_swap(
        config,
        pool,
        partner,
        false,
        coin::zero<CoinA>(ctx),
        coin_b,
        receipt,
        ctx,
    );
    deposit_optional(vault, rem_a);
    rem_b
}

fun repay_flash_swap<CoinA, CoinB>(
    config: &GlobalConfig,
    pool: &mut Pool<CoinA, CoinB>,
    partner: &mut Partner,
    a2b: bool,
    coin_a: Coin<CoinA>,
    coin_b: Coin<CoinB>,
    receipt: FlashSwapReceipt<CoinA, CoinB>,
    ctx: &mut TxContext,
): (Coin<CoinA>, Coin<CoinB>) {
    let repay_amount = pool::swap_pay_amount(&receipt);
    let mut coin_a = coin_a;
    let mut coin_b = coin_b;
    let (pay_a, pay_b) = if (a2b) {
        (
            coin_a.split(repay_amount, ctx).into_balance(),
            balance::zero<CoinB>(),
        )
    } else {
        (
            balance::zero<CoinA>(),
            coin_b.split(repay_amount, ctx).into_balance(),
        )
    };
    if (object::id_address(partner) == DEFAULT_PARTNER_ID) {
        pool::repay_flash_swap<CoinA, CoinB>(config, pool, pay_a, pay_b, receipt);
    } else {
        pool::repay_flash_swap_with_partner<CoinA, CoinB>(
            config,
            pool,
            partner,
            pay_a,
            pay_b,
            receipt,
        );
    };
    (coin_a, coin_b)
}

fun deposit_optional<T>(vault: &mut Vault, coin: Coin<T>) {
    if (coin.value() == 0) {
        coin.destroy_zero();
    } else {
        vault::deposit_coin(vault, coin);
    }
}

fun destroy_balance_zero<T>(bal: Balance<T>) {
    bal.destroy_zero();
}
