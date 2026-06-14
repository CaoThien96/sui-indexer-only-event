import type { SuiTransactionBlockResponse } from '@mysten/sui/client';
import type { SuiClient } from '@mysten/sui/client';
import type { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import type { ParallelTransactionExecutor, Transaction } from '@mysten/sui/transactions';
import { ParallelTransactionExecutor as ParallelExecutor } from '@mysten/sui/transactions';
import type { TrapConfig } from './config.js';

export type ExecResult = {
  digest: string;
  success: boolean;
  abortCode?: number;
  abortModule?: string;
  error?: string;
  gasUsed?: string;
  timestampMs?: number;
};

export function parseTxResponse(result: SuiTransactionBlockResponse): ExecResult {
  const success = result.effects?.status?.status === 'success';
  const parsed = success ? {} : parseAbortCode(result.effects ?? {});

  return {
    digest: result.digest,
    success,
    abortCode: parsed.code,
    abortModule: parsed.module,
    error: parsed.error,
    gasUsed: result.effects?.gasUsed?.computationCost,
    timestampMs: result.timestampMs ? Number(result.timestampMs) : undefined,
  };
}

export function parseAbortCode(effects: {
  status?: { status?: string; error?: string };
}): { code?: number; module?: string; error?: string } {
  const error = effects.status?.error ?? '';
  if (!error) return {};

  const moveAbort = error.match(
    /MoveAbort\(MoveLocation \{ module: ModuleId \{ address: [^,]+, name: Identifier\("([^"]+)"\) \}[^}]*\}, (\d+)\)/,
  );
  if (moveAbort) {
    return { code: Number(moveAbort[2]), module: moveAbort[1], error };
  }

  return { error };
}
export const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

export async function executeTx(
  client: SuiClient,
  keypair: Ed25519Keypair,
  tx: Transaction,
): Promise<ExecResult> {
  // await sleep(50)
  const sender = keypair.getPublicKey().toSuiAddress();
  tx.setSender(sender);

  const result = await client.signAndExecuteTransaction({
    signer: keypair,
    transaction: tx,
    options: { showEffects: true, showEvents: true },
  });

  return parseTxResponse(result);
}

export type TrapTxExecutor = ParallelTransactionExecutor;

export function createTxExecutor(
  client: SuiClient,
  keypair: Ed25519Keypair,
  config: TrapConfig,
): TrapTxExecutor {
  const gasBudget = BigInt(config.gasBudget);
  return new ParallelExecutor({
    client,
    signer: keypair,
    defaultGasBudget: gasBudget,
    minimumCoinBalance: gasBudget,
    maxPoolSize: config.parallelMaxPoolSize ?? 10,
  });
}

export async function executeTxExecutor(
  executor: TrapTxExecutor,
  tx: Transaction,
): Promise<ExecResult> {
  const result = await executor.executeTransaction(tx, {
    showEffects: true,
    showEvents: true,
  });
  return parseTxResponse(result.data);
}
