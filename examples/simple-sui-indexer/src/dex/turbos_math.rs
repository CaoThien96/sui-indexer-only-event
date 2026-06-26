use num_bigint::BigUint;
use num_traits::Zero;

const MIN_TICK_INDEX: i32 = -443_636;
const MAX_TICK_INDEX: i32 = 443_636;
const LP_REFERENCE_LIQUIDITY: u128 = 100_000_000;

/// Align full-range ticks to fee tier (matches turbos-clmm-sdk + bot-snip `openPoolPositionWithLPFixed`).
pub fn aligned_ticks(fee_bps: u32) -> (i32, i32) {
    let fee = i32::try_from(fee_bps).unwrap_or(100);
    let lower_rem = MIN_TICK_INDEX % fee;
    let upper_rem = MAX_TICK_INDEX % fee;
    (MIN_TICK_INDEX - lower_rem, MAX_TICK_INDEX - upper_rem)
}

fn sqrt_prices_for_fee(fee_bps: u32) -> (&'static str, &'static str) {
    match fee_bps {
        500 => (
            "4324352399",
            "78689786464075983699238871887",
        ),
        3000 => (
            "4900102608",
            "69443926813843921247854992145",
        ),
        10000 => (
            "5151323364",
            "66057271670166509943169781762",
        ),
        _ => (
            "4302785677",
            "79084200890414257525634219231",
        ),
    }
}

fn parse_u(s: &str) -> BigUint {
    BigUint::parse_bytes(s.as_bytes(), 10).unwrap_or_else(BigUint::zero)
}

fn to_x64(n: &BigUint) -> BigUint {
    n << 64
}

fn from_x64(n: &BigUint) -> BigUint {
    n >> 64
}

fn ceil_div(num: BigUint, den: &BigUint) -> BigUint {
    if den.is_zero() {
        return BigUint::zero();
    }
    (num + den - 1u8) / den
}

/// Port of `turbos-clmm-sdk` `pool.getTokenAmountsFromLiquidity`.
pub fn token_amounts_from_liquidity(
    liquidity: u128,
    current_sqrt_price: &str,
    lower_sqrt_price: &str,
    upper_sqrt_price: &str,
) -> (u128, u128) {
    let liquidity = BigUint::from(liquidity);
    let current = parse_u(current_sqrt_price);
    let lower = parse_u(lower_sqrt_price);
    let upper = parse_u(upper_sqrt_price);

    let (amount_a, amount_b) = if current < lower {
        let num = to_x64(&liquidity) * (&upper - &lower);
        let den = &lower * &upper;
        (ceil_div(num, &den), BigUint::zero())
    } else if current < upper {
        let num_a = to_x64(&liquidity) * (&upper - &current);
        let den_a = &current * &upper;
        let amount_a = ceil_div(num_a, &den_a);
        let amount_b = from_x64(&(&liquidity * (&current - &lower)));
        (amount_a, amount_b)
    } else {
        let amount_b = from_x64(&(&liquidity * (&upper - &lower)));
        (BigUint::zero(), amount_b)
    };

    (
        biguint_to_u128(&amount_a),
        biguint_to_u128(&amount_b),
    )
}

fn biguint_to_u128(v: &BigUint) -> u128 {
    v.to_string().parse().unwrap_or(0)
}

/// Compute `(amount_token, amount_sui)` for snip LP — mirrors bot-snip `turbos.service.ts`.
pub fn snip_lp_amounts(sqrt_price: &str, fee_bps: u32) -> (u64, u64) {
    let (lower_sqrt, upper_sqrt) = sqrt_prices_for_fee(fee_bps);
    let (big_amount_a, big_amount_b) = token_amounts_from_liquidity(
        LP_REFERENCE_LIQUIDITY,
        sqrt_price,
        lower_sqrt,
        upper_sqrt,
    );

    let sui_amount = 1u64;
    let amount_a = if big_amount_b == 0 {
        1u64
    } else {
        let num = BigUint::from(sui_amount) * BigUint::from(big_amount_a);
        let den = BigUint::from(big_amount_b);
        biguint_to_u128(&ceil_div(num, &den)).max(1) as u64
    };

    (amount_a, sui_amount)
}

/// `getMinimumAmountBySlippage` — slippage is percent (e.g. 5 = 5%).
pub fn minimum_amount(amount: u64, slippage_percent: u64) -> u64 {
    let kept = 100u64.saturating_sub(slippage_percent);
    amount.saturating_mul(kept) / 100
}
