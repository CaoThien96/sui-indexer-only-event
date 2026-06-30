/// Turbos snip (buy + LP) and sell — Pool<Token, SUI, Fee> (token is coin A).
module snip_vault::turbos_ops;

use cetusclmm::tick_math;
use snip_vault::vault::{Self, Vault};
use sui::clock::{Self, Clock};
use sui::coin::{Self, Coin};
use sui::event;
use sui::sui::SUI;
use turbos_clmm::pool::{Pool, Versioned};
use turbos_clmm::position_manager::{Self, Positions};
use turbos_clmm::swap_router::{swap_a_b_with_return_, swap_b_a_with_return_};

const SWAP_DEADLINE_MS: u64 = 18_000;

public struct TurbosSniped has copy, drop {
    vault_id: ID,
    pool_id: ID,
    buy_sui: u64,
}

public struct TurbosSold has copy, drop {
    vault_id: ID,
    pool_id: ID,
    token_in: u64,
    sui_out: u64,
}

/// Token = coin A, SUI = coin B. Bot splits gas into `buy_sui` + optional `lp_sui`.
public fun snip_and_lp_turbos<Token, Fee>(
    vault: &mut Vault,
    pool: &mut Pool<Token, SUI, Fee>,
    positions: &mut Positions,
    versioned: &Versioned,
    buy_sui: Coin<SUI>,
    lp_sui: Coin<SUI>,
    tick_lower: u32,
    tick_lower_neg: bool,
    tick_upper: u32,
    tick_upper_neg: bool,
    amount_a: u64,
    amount_b: u64,
    amount_a_min: u64,
    amount_b_min: u64,
    deadline: u64,
    clock: &Clock,
    ctx: &mut TxContext,
) {
    vault::assert_authorized(vault, ctx);
    let pool_id = object::id(pool);
    let buy_amount = buy_sui.value();

    let swapped_token = swap_sui_to_token(vault, pool, buy_sui, clock, versioned, ctx);

    let mut coins_a = vector<Coin<Token>>[];
    let mut coins_b = vector<Coin<SUI>>[];
    push_coin(&mut coins_a, swapped_token);
    push_coin(&mut coins_b, lp_sui);

    let (nft, rem_a, rem_b) = position_manager::mint_with_return_<Token, SUI, Fee>(
        pool,
        positions,
        coins_a,
        coins_b,
        tick_lower,
        tick_lower_neg,
        tick_upper,
        tick_upper_neg,
        amount_a,
        amount_b,
        amount_a_min,
        amount_b_min,
        deadline,
        clock,
        versioned,
        ctx,
    );

    deposit_optional(vault, rem_a);
    deposit_optional(vault, rem_b);
    vault::store_position_object(vault, pool_id, nft);

    event::emit(TurbosSniped {
        vault_id: object::id(vault),
        pool_id,
        buy_sui: buy_amount,
    });
}

/// Sell token (coin A) from vault; returns SUI (coin B).
public fun sell_turbos<Token, Fee>(
    vault: &mut Vault,
    pool: &mut Pool<Token, SUI, Fee>,
    versioned: &Versioned,
    amount: u64,
    clock: &Clock,
    ctx: &mut TxContext,
): Coin<SUI> {
    vault::assert_authorized(vault, ctx);
    let token = vault::withdraw_coin<Token>(vault, amount, ctx);
    let token_in = token.value();
    let sui = swap_token_to_sui(vault, pool, token, clock, versioned, ctx);
    event::emit(TurbosSold {
        vault_id: object::id(vault),
        pool_id: object::id(pool),
        token_in,
        sui_out: sui.value(),
    });
    sui
}

fun swap_sui_to_token<Token, Fee>(
    vault: &mut Vault,
    pool: &mut Pool<Token, SUI, Fee>,
    sui_in: Coin<SUI>,
    clock: &Clock,
    versioned: &Versioned,
    ctx: &mut TxContext,
): Coin<Token> {
    let amount_in = sui_in.value();
    let deadline = clock.timestamp_ms() + SWAP_DEADLINE_MS;
    let coins_b = vector[sui_in];
    let (received_a, received_b) = swap_b_a_with_return_<Token, SUI, Fee>(
        pool,
        coins_b,
        amount_in,
        0,
        tick_math::max_sqrt_price(),
        true,
        ctx.sender(),
        deadline,
        clock,
        versioned,
        ctx,
    );
    deposit_optional(vault, received_b);
    received_a
}

fun swap_token_to_sui<Token, Fee>(
    vault: &mut Vault,
    pool: &mut Pool<Token, SUI, Fee>,
    token: Coin<Token>,
    clock: &Clock,
    versioned: &Versioned,
    ctx: &mut TxContext,
): Coin<SUI> {
    let amount_in = token.value();
    let deadline = clock.timestamp_ms() + SWAP_DEADLINE_MS;
    let coins_a = vector[token];
    let (received_b, received_a) = swap_a_b_with_return_<Token, SUI, Fee>(
        pool,
        coins_a,
        amount_in,
        0,
        tick_math::min_sqrt_price(),
        true,
        ctx.sender(),
        deadline,
        clock,
        versioned,
        ctx,
    );
    deposit_optional(vault, received_a);
    received_b
}

fun push_coin<T>(vec: &mut vector<Coin<T>>, coin: Coin<T>) {
    if (coin.value() > 0) {
        vec.push_back(coin);
    } else {
        coin.destroy_zero();
    }
}

fun deposit_optional<T>(vault: &mut Vault, coin: Coin<T>) {
    if (coin.value() == 0) {
        coin.destroy_zero();
    } else {
        vault::deposit_coin(vault, coin);
    }
}
