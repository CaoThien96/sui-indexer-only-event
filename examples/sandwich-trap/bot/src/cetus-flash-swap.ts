import type { TransactionObjectArgument } from '@mysten/sui/transactions';
import { Transaction } from '@mysten/sui/transactions';
import type { TrapConfig } from './config.js';
import { MAX_SQRT_PRICE, MIN_SQRT_PRICE } from './coin-picker.js';
import type { SuiCoinRef, SwapCoinInputs, SwapLegResult } from './coin-picker.js';
import type { BuySwapBounds } from './swap-bounds.js';

type LegKind = 'buy' | 'sell';

function poolTarget(config: TrapConfig, fn: string): string {
  return `${config.cetusPackageId}::pool::${fn}`;
}

function suiCoinArg(tx: Transaction, ref: SuiCoinRef): TransactionObjectArgument {
  return ref.kind === 'gas' ? tx.gas : tx.object(ref.id);
}

function zeroBalance(
  tx: Transaction,
  coinType: string,
): TransactionObjectArgument {
  return tx.moveCall({
    target: '0x2::balance::zero',
    typeArguments: [coinType],
    arguments: [],
  });
}

function intoBalance(
  tx: Transaction,
  coinType: string,
  coin: TransactionObjectArgument,
): TransactionObjectArgument {
  return tx.moveCall({
    target: '0x2::coin::into_balance',
    typeArguments: [coinType],
    arguments: [coin],
  });
}

function fromBalance(
  tx: Transaction,
  coinType: string,
  balance: TransactionObjectArgument,
): TransactionObjectArgument {
  return tx.moveCall({
    target: '0x2::coin::from_balance',
    typeArguments: [coinType],
    arguments: [balance],
  });
}

function destroyZero(
  tx: Transaction,
  coinType: string,
  balance: TransactionObjectArgument,
): void {
  tx.moveCall({
    target: '0x2::balance::destroy_zero',
    typeArguments: [coinType],
    arguments: [balance],
  });
}

function transferSwapOutputs(
  tx: Transaction,
  sender: string,
  after: SwapLegResult,
): void {
  const coins: TransactionObjectArgument[] = [];
  if (!after.suiIsGas) coins.push(after.suiCoin);
  coins.push(after.tokenCoin);
  if (coins.length > 0) {
    tx.transferObjects(coins, sender);
  }
}

/**
 * Append one Cetus flash_swap leg to an existing PTB.
 * BUY: a2b=false (SUI -> token). SELL: a2b=true (token -> SUI).
 */
export function appendFlashSwapLeg(
  tx: Transaction,
  config: TrapConfig,
  leg: LegKind,
  amount: number,
  suiCoin: TransactionObjectArgument,
  tokenCoin: TransactionObjectArgument,
  suiIsGas: boolean,
  buyBounds?: BuySwapBounds,
): SwapLegResult {
  const a2b = leg === 'sell';
  const sqrtLimit = a2b
    ? MIN_SQRT_PRICE
    : (buyBounds?.sqrtPriceLimit ?? MAX_SQRT_PRICE);

  const [recvA, recvB, receipt] = tx.moveCall({
    target: poolTarget(config, 'flash_swap'),
    typeArguments: [config.coinTypeA, config.coinTypeB],
    arguments: [
      tx.object(config.globalConfigId),
      tx.object(config.poolId),
      tx.pure.bool(a2b),
      tx.pure.bool(true),
      tx.pure.u64(amount),
      tx.pure.u128(sqrtLimit),
      tx.object('0x6'),
    ],
  });

  const payAmount = tx.moveCall({
    target: poolTarget(config, 'swap_pay_amount'),
    typeArguments: [config.coinTypeA, config.coinTypeB],
    arguments: [receipt],
  });

  if (a2b) {
    const [payToken] = tx.splitCoins(tokenCoin, [payAmount]);
    const payA = intoBalance(tx, config.coinTypeA, payToken);
    const zeroB = zeroBalance(tx, config.coinTypeB);
    tx.moveCall({
      target: poolTarget(config, 'repay_flash_swap'),
      typeArguments: [config.coinTypeA, config.coinTypeB],
      arguments: [
        tx.object(config.globalConfigId),
        tx.object(config.poolId),
        payA,
        zeroB,
        receipt,
      ],
    });
    const outSui = fromBalance(tx, config.coinTypeB, recvB);
    destroyZero(tx, config.coinTypeA, recvA);
    tx.mergeCoins(suiCoin, [outSui]);
    return { suiCoin, tokenCoin, suiIsGas };
  }

  const [paySui] = tx.splitCoins(suiCoin, [payAmount]);
  const zeroA = zeroBalance(tx, config.coinTypeA);
  const payB = intoBalance(tx, config.coinTypeB, paySui);
  tx.moveCall({
    target: poolTarget(config, 'repay_flash_swap'),
    typeArguments: [config.coinTypeA, config.coinTypeB],
    arguments: [
      tx.object(config.globalConfigId),
      tx.object(config.poolId),
      zeroA,
      payB,
      receipt,
    ],
  });
  const outToken = fromBalance(tx, config.coinTypeA, recvA);
  destroyZero(tx, config.coinTypeB, recvB);
  if (buyBounds && buyBounds.minAmountOut > 0) {
    // splitCoins aborts if output < minAmountOut (sandwich / front-run protection).
    const [minCheck] = tx.splitCoins(outToken, [buyBounds.minAmountOut]);
    tx.mergeCoins(tokenCoin, [minCheck, outToken]);
  } else {
    tx.mergeCoins(tokenCoin, [outToken]);
  }
  return { suiCoin, tokenCoin, suiIsGas };
}

function baseTx(config: TrapConfig, gasPrice: number): Transaction {
  const tx = new Transaction();
  tx.setGasBudget(config.gasBudget);
  tx.setGasPrice(gasPrice);
  return tx;
}

/** Bait BUY: small SUI -> token (a2b=false). */
export function buildBaitBuyTx(
  config: TrapConfig,
  sender: string,
  inputs: SwapCoinInputs,
  buyBounds?: BuySwapBounds,
): Transaction {
  const tx = baseTx(config, config.baitGasPrice);
  const suiIsGas = inputs.suiCoin.kind === 'gas';
  const suiCoin = suiCoinArg(tx, inputs.suiCoin);
  const tokenCoin = tx.object(inputs.tokenCoinId);
  const after = appendFlashSwapLeg(
    tx,
    config,
    'buy',
    config.baitSuiMist,
    suiCoin,
    tokenCoin,
    suiIsGas,
    buyBounds,
  );
  transferSwapOutputs(tx, sender, after);
  return tx;
}

/** Burst SELL: large token -> SUI (a2b=true). */
export function buildBurstSellTx(
  config: TrapConfig,
  sender: string,
  inputs: SwapCoinInputs,
): Transaction {
  const tx = baseTx(config, config.dumpGasPrice);
  const suiIsGas = inputs.suiCoin.kind === 'gas';
  const suiCoin = suiCoinArg(tx, inputs.suiCoin);
  const tokenCoin = tx.object(inputs.tokenCoinId);
  const after = appendFlashSwapLeg(
    tx,
    config,
    'sell',
    config.dumpTokenAmount,
    suiCoin,
    tokenCoin,
    suiIsGas,
  );
  transferSwapOutputs(tx, sender, after);
  return tx;
}

/** Single PTB: bait BUY then burst SELL sequential. */
export function buildTrapSingleTx(
  config: TrapConfig,
  sender: string,
  inputs: SwapCoinInputs,
  buyBounds?: BuySwapBounds,
): Transaction {
  const tx = baseTx(config, config.dumpGasPrice);
  const suiIsGas = inputs.suiCoin.kind === 'gas';
  const suiCoin = suiCoinArg(tx, inputs.suiCoin);
  const tokenCoin = tx.object(inputs.tokenCoinId);
  const afterBuy = appendFlashSwapLeg(
    tx,
    config,
    'buy',
    config.baitSuiMist,
    suiCoin,
    tokenCoin,
    suiIsGas,
    buyBounds,
  );
  const afterSell = appendFlashSwapLeg(
    tx,
    config,
    'sell',
    config.dumpTokenAmount,
    afterBuy.suiCoin,
    afterBuy.tokenCoin,
    afterBuy.suiIsGas,
  );
  transferSwapOutputs(tx, sender, afterSell);
  return tx;
}
