import type { SuiClient } from '@mysten/sui/client';
import type { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import type { Transaction } from '@mysten/sui/transactions';
import {
  buildBaitBuyTx,
  buildBurstSellTx,
  buildTrapSingleTx,
} from './cetus-flash-swap.js';
import { planWalletSwap } from './coin-picker.js';
import type { TrapConfig } from './config.js';
import { prepareWalletCoins } from './coin-merge.js';
import { readPoolState } from './pool-reader.js';
import type { BuySwapBounds } from './swap-bounds.js';
import {
  computeBaitBuyBounds,
  isBaitTightBoundsEnabled,
} from './swap-bounds.js';
import type { ExecResult } from './tx-executor.js';
import { executeTx } from './tx-executor.js';

export type TrapRoundResult = {
  mode: TrapConfig['trapMode'];
  results: ExecResult[];
};

function logResults(label: string, results: ExecResult[]): void {
  for (const r of results) {
    console.log(
      `${label} digest=${r.digest} success=${r.success} abort=${r.abortCode ?? '-'} gas=${r.gasUsed ?? '-'}`,
    );
    if (r.error) console.log(`  error: ${r.error}`);
  }
}

async function attachGasPayment(
  client: SuiClient,
  tx: Transaction,
  gasCoinId: string,
): Promise<void> {
  const obj = await client.getObject({ id: gasCoinId });
  if (!obj.data?.version || !obj.data?.digest) {
    throw new Error(`gas coin ${gasCoinId} not found`);
  }
  tx.setGasPayment([
    {
      objectId: gasCoinId,
      version: obj.data.version,
      digest: obj.data.digest,
    },
  ]);
}

async function resolveBaitBuyBounds(
  client: SuiClient,
  config: TrapConfig,
  sender: string,
): Promise<BuySwapBounds | undefined> {
  if (!isBaitTightBoundsEnabled(config)) return undefined;

  const pool = await readPoolState(client, config);
  const bounds = await computeBaitBuyBounds(client, config, sender, pool);
  const bps = config.baitTightBounds?.toleranceBps ?? 50;
  console.log(
    `baitTightBounds ON tolerance=${bps}bps ` +
      `quotedOut=${bounds.expectedAmountOut} minOut=${bounds.minAmountOut} ` +
      `sqrtLimit=${bounds.sqrtPriceLimit}`,
  );
  return bounds;
}

async function buildTxFromPlan(
  client: SuiClient,
  sender: string,
  config: TrapConfig,
  mode: 'bait' | 'dump' | 'trap',
  buyBounds?: BuySwapBounds,
): Promise<Transaction> {
  const plan = await planWalletSwap(client, sender, config, mode);

  const tx =
    mode === 'bait'
      ? buildBaitBuyTx(config, sender, plan.inputs, buyBounds)
      : mode === 'dump'
        ? buildBurstSellTx(config, sender, plan.inputs)
        : buildTrapSingleTx(config, sender, plan.inputs, buyBounds);

  if (plan.explicitGas && plan.gasCoinId) {
    await attachGasPayment(client, tx, plan.gasCoinId);
  }

  return tx;
}

export async function executeBait(
  client: SuiClient,
  keypair: Ed25519Keypair,
  config: TrapConfig,
): Promise<ExecResult> {
  await prepareWalletCoins(client, keypair, config, 'bait');
  const sender = keypair.getPublicKey().toSuiAddress();
  const buyBounds = await resolveBaitBuyBounds(client, config, sender);
  const tx = await buildTxFromPlan(client, sender, config, 'bait', buyBounds);
  return executeTx(client, keypair, tx);
}

export async function executeDump(
  client: SuiClient,
  keypair: Ed25519Keypair,
  config: TrapConfig,
): Promise<ExecResult[]> {
  await prepareWalletCoins(client, keypair, config, 'dump');
  const sender = keypair.getPublicKey().toSuiAddress();
  const tx = await buildTxFromPlan(client, sender, config, 'dump');
  const result = await executeTx(client, keypair, tx);
  return [result];
}

export async function executeTrapSingleTx(
  client: SuiClient,
  keypair: Ed25519Keypair,
  config: TrapConfig,
): Promise<TrapRoundResult> {
  await prepareWalletCoins(client, keypair, config, 'trap');
  const sender = keypair.getPublicKey().toSuiAddress();
  const buyBounds = await resolveBaitBuyBounds(client, config, sender);
  const tx = await buildTxFromPlan(client, sender, config, 'trap', buyBounds);
  const result = await executeTx(client, keypair, tx);
  logResults('trap-single', [result]);
  return { mode: 'single-tx', results: [result] };
}

export async function executeTrapParallel(
  client: SuiClient,
  baitKeypair: Ed25519Keypair,
  dumpKeypair: Ed25519Keypair,
  config: TrapConfig,
): Promise<TrapRoundResult> {
  await Promise.all([
    prepareWalletCoins(client, baitKeypair, config, 'bait'),
    prepareWalletCoins(client, dumpKeypair, config, 'dump'),
  ]);

  const baitSender = baitKeypair.getPublicKey().toSuiAddress();
  const dumpSender = dumpKeypair.getPublicKey().toSuiAddress();

  const buyBounds = await resolveBaitBuyBounds(client, config, baitSender);

  const [baitTx, dumpTx] = await Promise.all([
    buildTxFromPlan(client, baitSender, config, 'bait', buyBounds),
    buildTxFromPlan(client, dumpSender, config, 'dump'),
  ]);

  console.log('trap-parallel: bait + dump concurrent (post-merge tx.gas when 1 SUI)');

  const [baitResult, dumpResult] = await Promise.all([
    executeTx(client, baitKeypair, baitTx),
    executeTx(client, dumpKeypair, dumpTx),
  ]);

  logResults('trap-parallel-bait', [baitResult]);
  logResults('trap-parallel-dump', [dumpResult]);

  return { mode: 'parallel-tx', results: [baitResult, dumpResult] };
}

export async function executeTrapRound(
  client: SuiClient,
  keypairs: {
    single?: Ed25519Keypair;
    bait?: Ed25519Keypair;
    dump?: Ed25519Keypair;
  },
  config: TrapConfig,
): Promise<TrapRoundResult> {
  if (config.trapMode === 'single-tx') {
    if (!keypairs.single) throw new Error('single keypair required');
    return executeTrapSingleTx(client, keypairs.single, config);
  }

  if (!keypairs.bait || !keypairs.dump) {
    throw new Error('bait and dump keypairs required for parallel-tx mode');
  }
  return executeTrapParallel(client, keypairs.bait, keypairs.dump, config);
}
