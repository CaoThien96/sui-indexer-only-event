import type { SuiClient } from '@mysten/sui/client';
import type { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { Transaction } from '@mysten/sui/transactions';
import { listCoins } from './coin-picker.js';
import type { TrapConfig } from './config.js';
import { executeTx, type ExecResult } from './tx-executor.js';

/** Max coins merged per tx (object/command limits). */
const MERGE_BATCH_SIZE = 50;

export type MergeResult = {
  coinType: string;
  before: number;
  after: number;
  merged: boolean;
  results: ExecResult[];
};

/**
 * Merge SUI coins into tx.gas (SDK auto-picks gas coin).
 * Pattern: mergeCoins(txb.gas, allCoins.slice(1)) in a loop until one coin remains.
 */
async function mergeSuiCoins(
  client: SuiClient,
  keypair: Ed25519Keypair,
  coinType: string,
  gasBudget: number,
): Promise<MergeResult> {
  const owner = keypair.getPublicKey().toSuiAddress();
  const results: ExecResult[] = [];

  const beforeCoins = await listCoins(client, owner, coinType);
  const before = beforeCoins.length;

  if (before <= 1) {
    return { coinType, before, after: before, merged: false, results };
  }

  // eslint-disable-next-line no-constant-condition
  while (true) {
    const coins = await listCoins(client, owner, coinType);
    if (coins.length <= 1) break;

    const sources = coins.slice(1, 1 + MERGE_BATCH_SIZE);
    if (sources.length === 0) break;

    const tx = new Transaction();
    // tx.setGasBudget(gasBudget);
    tx.mergeCoins(
      tx.gas,
      sources.map((c) => tx.object(c.id)),
    );

    const result = await executeTx(client, keypair, tx);
    results.push(result);
    if (!result.success) {
      throw new Error(
        `merge SUI failed: ${result.error ?? result.abortCode}`,
      );
    }
  }

  const after = (await listCoins(client, owner, coinType)).length;
  return { coinType, before, after, merged: true, results };
}

/**
 * Merge token coins into the largest coin object.
 * Gas paid via tx.gas (SUI); token objects are the only PTB inputs.
 */
async function mergeTokenCoins(
  client: SuiClient,
  keypair: Ed25519Keypair,
  coinType: string,
  gasBudget: number,
): Promise<MergeResult> {
  const owner = keypair.getPublicKey().toSuiAddress();
  const results: ExecResult[] = [];

  const beforeCoins = await listCoins(client, owner, coinType);
  const before = beforeCoins.length;

  if (before <= 1) {
    return { coinType, before, after: before, merged: false, results };
  }

  // eslint-disable-next-line no-constant-condition
  while (true) {
    const coins = await listCoins(client, owner, coinType);
    if (coins.length <= 1) break;

    const sources = coins.slice(1, 1 + MERGE_BATCH_SIZE);
    if (sources.length === 0) break;

    const tx = new Transaction();
    // tx.setGasBudget(gasBudget);
    const primary = tx.object(coins[0]!.id);
    tx.mergeCoins(
      primary,
      sources.map((c) => tx.object(c.id)),
    );

    const result = await executeTx(client, keypair, tx);
    results.push(result);
    if (!result.success) {
      throw new Error(
        `merge ${coinType} failed: ${result.error ?? result.abortCode}`,
      );
    }
  }

  const after = (await listCoins(client, owner, coinType)).length;
  return { coinType, before, after, merged: true, results };
}

/** Merge all coins of a type (SUI uses tx.gas target, tokens use largest). */
export async function mergeAllCoinsOfType(
  client: SuiClient,
  keypair: Ed25519Keypair,
  coinType: string,
  gasBudget: number,
): Promise<MergeResult> {
  if (coinType === '0x2::sui::SUI' || coinType.endsWith('::sui::SUI')) {
    return mergeSuiCoins(client, keypair, coinType, gasBudget);
  }
  return mergeTokenCoins(client, keypair, coinType, gasBudget);
}

/** Merge SUI + token coin objects for one wallet before trap. */
export async function prepareWalletCoins(
  client: SuiClient,
  keypair: Ed25519Keypair,
  config: TrapConfig,
  label: string,
): Promise<MergeResult[]> {
  const owner = keypair.getPublicKey().toSuiAddress();
  console.log(`prepare-coins [${label}] ${owner}`);

  const suiResult = await mergeSuiCoins(
    client,
    keypair,
    config.coinTypeB,
    config.gasBudget,
  );
  console.log(
    `  SUI: ${suiResult.before} -> ${suiResult.after} objects` +
      (suiResult.merged ? ` (${suiResult.results.length} merge tx)` : ' (skip)'),
  );

  const tokenShort = config.coinTypeA.split('::').pop() ?? config.coinTypeA;
  const tokenResult = await mergeTokenCoins(
    client,
    keypair,
    config.coinTypeA,
    config.gasBudget,
  );
  console.log(
    `  ${tokenShort}: ${tokenResult.before} -> ${tokenResult.after} objects` +
      (tokenResult.merged ? ` (${tokenResult.results.length} merge tx)` : ' (skip)'),
  );

  return [suiResult, tokenResult];
}
