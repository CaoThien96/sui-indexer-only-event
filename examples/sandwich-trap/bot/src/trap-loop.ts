import type { SuiClient } from '@mysten/sui/client';
import type { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import type { TrapConfig } from './config.js';
import { readPoolState } from './pool-reader.js';
import { executeTrapRound } from './trap-executor.js';

export type TrapLoopOptions = {
  intervalMs?: number;
  maxRounds?: number;
  signal?: AbortSignal;
};

export function createShutdownSignal(): AbortSignal {
  const controller = new AbortController();
  for (const sig of ['SIGINT', 'SIGTERM'] as const) {
    process.on(sig, () => controller.abort());
  }
  return controller.signal;
}

export function parseMaxRounds(argv: string[]): number | undefined {
  const arg = argv.find((a) => a.startsWith('--max-rounds='));
  if (!arg) return undefined;
  const n = Number(arg.split('=')[1]);
  return Number.isFinite(n) && n > 0 ? n : undefined;
}

function jsonReplacer(_key: string, value: unknown) {
  return typeof value === 'bigint' ? value.toString() : value;
}

export async function runTrapLoop(
  client: SuiClient,
  keypairs: {
    single?: Ed25519Keypair;
    bait?: Ed25519Keypair;
    dump?: Ed25519Keypair;
  },
  config: TrapConfig,
  options: TrapLoopOptions = {},
): Promise<void> {
  const intervalMs = options.intervalMs ?? config.loopIntervalMs;
  const maxRounds = options.maxRounds;
  let round = 0;

  while (!options.signal?.aborted) {
    round += 1;
    if (maxRounds !== undefined && round > maxRounds) break;

    const pool = await readPoolState(client, config);
    console.log(
      `\n--- trap round ${round} mode=${config.trapMode} ---`,
    );
    console.log(JSON.stringify({ pool }, jsonReplacer, 2));

    const result = await executeTrapRound(client, keypairs, config);
    const ok = result.results.filter((r) => r.success).length;
    console.log(`round ${round} done: ${ok}/${result.results.length} txs succeeded`);

    if (options.signal?.aborted) break;
    if (maxRounds !== undefined && round >= maxRounds) break;

    await sleep(intervalMs, options.signal);
  }
}

function sleep(ms: number, signal?: AbortSignal): Promise<void> {
  return new Promise((resolve) => {
    if (signal?.aborted) {
      resolve();
      return;
    }
    const timer = setTimeout(resolve, ms);
    signal?.addEventListener(
      'abort',
      () => {
        clearTimeout(timer);
        resolve();
      },
      { once: true },
    );
  });
}
