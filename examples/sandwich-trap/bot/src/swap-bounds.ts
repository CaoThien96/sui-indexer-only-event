import type { SuiClient } from '@mysten/sui/client';
import { Transaction } from '@mysten/sui/transactions';
import { MAX_SQRT_PRICE } from './coin-picker.js';
import type { TrapConfig } from './config.js';
import type { PoolState } from './pool-reader.js';

/** Tight execution bounds for bait BUY (a2b=false, SUI -> token). */
export type BuySwapBounds = {
  sqrtPriceLimit: bigint;
  minAmountOut: number;
  expectedAmountOut: bigint;
};

const FEE_RATE_DENOMINATOR = 1_000_000n;
const Q64 = 1n << 64n;

function toleranceBps(config: TrapConfig): number {
  return config.baitTightBounds?.toleranceBps ?? 50;
}

function poolTarget(config: TrapConfig, fn: string): string {
  return `${config.cetusPackageId}::pool::${fn}`;
}

/** sqrt_price_limit for BUY: price may rise at most toleranceBps above spot. */
export function computeBuySqrtPriceLimit(
  currentSqrt: bigint,
  toleranceBps: number,
): bigint {
  const limit = (currentSqrt * BigInt(10000 + toleranceBps)) / 10000n;
  return limit > MAX_SQRT_PRICE ? MAX_SQRT_PRICE : limit;
}

function mulDivFloor(a: bigint, b: bigint, c: bigint): bigint {
  return (a * b) / c;
}

function mulDivCeil(a: bigint, b: bigint, c: bigint): bigint {
  return (a * b + c - 1n) / c;
}

function divRound(numerator: bigint, denominator: bigint, roundUp: boolean): bigint {
  if (denominator === 0n) return 0n;
  const q = numerator / denominator;
  const r = numerator % denominator;
  return roundUp && r > 0n ? q + 1n : q;
}

function getDeltaDownFromOutput(
  currentSqrt: bigint,
  targetSqrt: bigint,
  liquidity: bigint,
  a2b: boolean,
): bigint {
  const sqrtDiff =
    currentSqrt > targetSqrt ? currentSqrt - targetSqrt : targetSqrt - currentSqrt;
  if (sqrtDiff === 0n || liquidity === 0n) return 0n;

  if (a2b) {
    return (liquidity * sqrtDiff) >> 64n;
  }
  const numerator = liquidity * sqrtDiff * Q64;
  const denominator = currentSqrt * targetSqrt;
  return divRound(numerator, denominator, false);
}

function getDeltaUpFromInput(
  currentSqrt: bigint,
  targetSqrt: bigint,
  liquidity: bigint,
  a2b: boolean,
): bigint {
  const sqrtDiff =
    currentSqrt > targetSqrt ? currentSqrt - targetSqrt : targetSqrt - currentSqrt;
  if (sqrtDiff === 0n || liquidity === 0n) return 0n;

  if (a2b) {
    const numerator = liquidity * sqrtDiff * Q64;
    const denominator = currentSqrt * targetSqrt;
    return divRound(numerator, denominator, true);
  }
  const product = liquidity * sqrtDiff;
  const lo64Mask = (1n << 64n) - 1n;
  const shouldRoundUp = (product & lo64Mask) > 0n;
  return shouldRoundUp ? (product >> 64n) + 1n : product >> 64n;
}

function getNextSqrtPriceFromInput(
  sqrtPrice: bigint,
  liquidity: bigint,
  amount: bigint,
  a2b: boolean,
): bigint {
  if (amount === 0n) return sqrtPrice;
  if (a2b) {
    const numerator = sqrtPrice * liquidity * Q64;
    const liquidityShl64 = liquidity * Q64;
    const product = sqrtPrice * amount;
    return divRound(numerator, liquidityShl64 + product, true);
  }
  const delta = divRound(amount * Q64, liquidity, false);
  return sqrtPrice + delta;
}

/** Port of Cetus clmm_math::compute_swap_step (single tick step). */
function computeSwapStep(
  currentSqrt: bigint,
  targetSqrt: bigint,
  liquidity: bigint,
  amount: bigint,
  feeRate: bigint,
  a2b: boolean,
  byAmountIn: boolean,
): { amountIn: bigint; amountOut: bigint; nextSqrt: bigint; feeAmount: bigint } {
  let nextSqrt = targetSqrt;
  let amountIn = 0n;
  let amountOut = 0n;
  let feeAmount = 0n;

  if (liquidity === 0n || currentSqrt === targetSqrt) {
    return { amountIn, amountOut, nextSqrt, feeAmount };
  }
  if (a2b && currentSqrt <= targetSqrt) {
    return { amountIn, amountOut, nextSqrt, feeAmount };
  }
  if (!a2b && currentSqrt >= targetSqrt) {
    return { amountIn, amountOut, nextSqrt, feeAmount };
  }

  if (byAmountIn) {
    const amountRemain = mulDivFloor(
      amount,
      FEE_RATE_DENOMINATOR - feeRate,
      FEE_RATE_DENOMINATOR,
    );
    const maxAmountIn = getDeltaUpFromInput(currentSqrt, targetSqrt, liquidity, a2b);

    if (maxAmountIn > amountRemain) {
      amountIn = amountRemain;
      feeAmount = amount - amountRemain;
      nextSqrt = getNextSqrtPriceFromInput(currentSqrt, liquidity, amountRemain, a2b);
    } else {
      amountIn = maxAmountIn;
      feeAmount = mulDivCeil(
        amountIn,
        feeRate,
        FEE_RATE_DENOMINATOR - feeRate,
      );
      nextSqrt = targetSqrt;
    }

    const out = getDeltaDownFromOutput(currentSqrt, nextSqrt, liquidity, a2b);
    amountOut = out;
  }

  return { amountIn, amountOut, nextSqrt, feeAmount };
}

/** Fallback quote when devInspect is unavailable. */
export function estimateBuyTokenOutWithLimit(
  pool: PoolState,
  suiIn: bigint,
  sqrtPriceLimit: bigint,
  feeRate: bigint = 2500n,
): bigint {
  if (suiIn === 0n) return 0n;

  const { sqrtPrice, liquidity, balanceA, balanceB } = pool;
  if (liquidity === 0n || sqrtPrice === 0n || sqrtPrice >= sqrtPriceLimit) {
    if (balanceB === 0n) return 0n;
    const afterFee = mulDivFloor(suiIn, FEE_RATE_DENOMINATOR - feeRate, FEE_RATE_DENOMINATOR);
    return (afterFee * balanceA) / (balanceB + afterFee);
  }

  const step = computeSwapStep(
    sqrtPrice,
    sqrtPriceLimit,
    liquidity,
    suiIn,
    feeRate,
    false,
    true,
  );
  return step.amountOut;
}

function parseSwapAmountOut(
  events: Array<{ type: string; parsedJson?: unknown }>,
  poolId: string,
): bigint | null {
  for (const event of events) {
    if (!event.type.includes('SwapEvent')) continue;
    const json = event.parsedJson as Record<string, unknown> | undefined;
    if (!json || String(json.pool) !== poolId) continue;
    const out = json.amount_out;
    if (out !== undefined) return BigInt(String(out));
  }
  return null;
}

/**
 * Quote bait BUY output via devInspect with the same sqrt_price_limit used in the real tx.
 * Matches on-chain Cetus swap math (multi-tick included).
 */
export async function quoteBaitBuyOutput(
  client: SuiClient,
  config: TrapConfig,
  sender: string,
  sqrtPriceLimit: bigint,
): Promise<bigint> {
  const tx = new Transaction();
  tx.moveCall({
    target: poolTarget(config, 'flash_swap'),
    typeArguments: [config.coinTypeA, config.coinTypeB],
    arguments: [
      tx.object(config.globalConfigId),
      tx.object(config.poolId),
      tx.pure.bool(false),
      tx.pure.bool(true),
      tx.pure.u64(config.baitSuiMist),
      tx.pure.u128(sqrtPriceLimit),
      tx.object('0x6'),
    ],
  });

  const result = await client.devInspectTransactionBlock({
    sender,
    transactionBlock: tx,
  });

  if (result.error) {
    throw new Error(`baitTightBounds quote failed: ${result.error}`);
  }

  const quoted = parseSwapAmountOut(result.events ?? [], config.poolId);
  if (quoted !== null && quoted > 0n) return quoted;

  throw new Error('baitTightBounds quote: SwapEvent not found in devInspect');
}

export async function computeBaitBuyBounds(
  client: SuiClient,
  config: TrapConfig,
  sender: string,
  pool: PoolState,
): Promise<BuySwapBounds> {
  const bps = toleranceBps(config);
  const sqrtPriceLimit = computeBuySqrtPriceLimit(pool.sqrtPrice, bps);

  let quoted: bigint;
  try {
    quoted = await quoteBaitBuyOutput(client, config, sender, sqrtPriceLimit);
  } catch (quoteErr) {
    console.warn(`baitTightBounds: devInspect quote failed (${quoteErr}), using CLMM fallback`);
    quoted = estimateBuyTokenOutWithLimit(pool, BigInt(config.baitSuiMist), sqrtPriceLimit);
  }

  const minOut = (quoted * BigInt(10000 - bps)) / 10000n;

  if (minOut <= 0n) {
    throw new Error(
      `baitTightBounds: quoted token out is 0 (sqrtPrice=${pool.sqrtPrice}, liquidity=${pool.liquidity})`,
    );
  }
  if (minOut > BigInt(Number.MAX_SAFE_INTEGER)) {
    throw new Error(`baitTightBounds: minAmountOut ${minOut} exceeds u64 safe range`);
  }

  return {
    sqrtPriceLimit,
    minAmountOut: Number(minOut),
    expectedAmountOut: quoted,
  };
}

export function isBaitTightBoundsEnabled(config: TrapConfig): boolean {
  return config.baitTightBounds?.enabled === true;
}
