import { readFileSync } from 'node:fs';

export type TrapMode = 'single-tx' | 'parallel-tx';

/** Tight slippage bounds for bait BUY only (sqrt_price_limit + min_out). */
export type BaitTightBoundsConfig = {
  enabled: boolean;
  /** Price / output tolerance in basis points (50 = 0.5%). */
  toleranceBps?: number;
};

export type TrapConfig = {
  network: string;
  rpcUrl: string;
  trapMode: TrapMode;
  cetusPackageId: string;
  globalConfigId: string;
  poolId: string;
  coinTypeA: string;
  coinTypeB: string;
  botAddress: string;
  baitSuiMist: number;
  dumpTokenAmount: number;
  dumpBurstCount: number;
  baitGasPrice: number;
  dumpGasPrice: number;
  gasBudget: number;
  loopIntervalMs: number;
  parallelMaxPoolSize?: number;
  baitTightBounds?: BaitTightBoundsConfig;
};

export function loadConfig(path: string): TrapConfig {
  const raw = JSON.parse(readFileSync(path, 'utf8')) as TrapConfig;
  for (const key of ['poolId', 'coinTypeA', 'globalConfigId', 'cetusPackageId'] as const) {
    if (!raw[key]) {
      throw new Error(`config.${key} is required — fill mainnet.json before running`);
    }
  }
  if (raw.trapMode !== 'single-tx' && raw.trapMode !== 'parallel-tx') {
    throw new Error(`config.trapMode must be "single-tx" or "parallel-tx", got ${raw.trapMode}`);
  }
  return raw;
}
