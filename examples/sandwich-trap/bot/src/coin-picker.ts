import type { SuiClient } from '@mysten/sui/client';
import type { TransactionObjectArgument } from '@mysten/sui/transactions';

/** Cetus tick_math::min_sqrt_price / max_sqrt_price (mainnet CLMM). */
export const MIN_SQRT_PRICE = 4295048016n;
export const MAX_SQRT_PRICE = 79226673515401279992447579055n;

/** SUI coin used in PTB: separate object, or tx.gas when only one coin. */
export type SuiCoinRef = { kind: 'gas' } | { kind: 'object'; id: string };

export type SwapCoinInputs = {
  suiCoin: SuiCoinRef;
  tokenCoinId: string;
};

export type CoinEntry = { id: string; balance: bigint };

export async function listCoins(
  client: SuiClient,
  owner: string,
  coinType: string,
): Promise<CoinEntry[]> {
  let cursor: string | null | undefined = null;
  const coins: CoinEntry[] = [];

  do {
    const page = await client.getCoins({
      owner,
      coinType,
      cursor,
      limit: 50,
    });

    for (const coin of page.data) {
      coins.push({ id: coin.coinObjectId, balance: BigInt(coin.balance) });
    }

    cursor = page.hasNextPage ? page.nextCursor : null;
  } while (cursor);

  coins.sort((a, b) => (a.balance > b.balance ? -1 : 1));
  return coins;
}

/** Pick the largest coin object with balance >= minAmount. */
export async function pickLargestCoin(
  client: SuiClient,
  owner: string,
  coinType: string,
  minAmount: bigint,
  excludeIds: ReadonlySet<string> = new Set(),
): Promise<string> {
  const coins = await listCoins(client, owner, coinType);

  for (const coin of coins) {
    if (excludeIds.has(coin.id)) continue;
    if (coin.balance >= minAmount) return coin.id;
  }

  throw new Error(
    `no ${coinType} coin with balance >= ${minAmount} for ${owner}`,
  );
}

/** Pick any coin of type (largest available). */
export async function pickAnyCoin(
  client: SuiClient,
  owner: string,
  coinType: string,
  excludeIds: ReadonlySet<string> = new Set(),
): Promise<string> {
  const coins = await listCoins(client, owner, coinType);
  const pick = coins.find((c) => !excludeIds.has(c.id));
  if (!pick) {
    throw new Error(`no ${coinType} coin found for ${owner}`);
  }
  return pick.id;
}

/**
 * SUI coin to merge swap output into (SELL leg). Must NOT be tx.gas — gas coin
 * only pays fees and ParallelExecutor may assign a coin < gasBudget if it is
 * also used as a PTB input via mergeCoins(tx.gas, ...).
 */
export async function pickSuiReceiveCoin(
  client: SuiClient,
  owner: string,
  coinType: string,
  excludeIds: ReadonlySet<string> = new Set(),
): Promise<string> {
  const coins = await listCoins(client, owner, coinType);
  const candidates = coins
    .filter((c) => !excludeIds.has(c.id))
    .sort((a, b) => (a.balance < b.balance ? -1 : 1));

  if (candidates.length === 0) {
    throw new Error(`no ${coinType} receive coin for ${owner}`);
  }

  return candidates[0]!.id;
}

/** N distinct SUI coins for parallel dump receive (smallest first). */
export async function pickMultipleReceiveSuiCoins(
  client: SuiClient,
  owner: string,
  coinType: string,
  count: number,
  excludeIds: ReadonlySet<string> = new Set(),
): Promise<string[]> {
  const coins = await listCoins(client, owner, coinType);
  const candidates = coins
    .filter((c) => !excludeIds.has(c.id))
    .sort((a, b) => (a.balance < b.balance ? -1 : 1));

  const ids = candidates.slice(0, count).map((c) => c.id);
  if (ids.length < count) {
    throw new Error(
      `need ${count} ${coinType} receive coins, found ${ids.length}`,
    );
  }
  return ids;
}

/**
 * Pick SUI for swap input without stealing the gas coin.
 * Falls back to tx.gas when wallet has a single SUI coin.
 */
export async function resolveSuiForSwap(
  client: SuiClient,
  owner: string,
  coinType: string,
  minSwapAmount: bigint,
  gasBudget: bigint,
  reservedIds: ReadonlySet<string> = new Set(),
): Promise<SuiCoinRef> {
  const coins = await listCoins(client, owner, coinType);
  const available = coins.filter((c) => !reservedIds.has(c.id));
  if (available.length === 0) {
    throw new Error(`no ${coinType} coin found for ${owner}`);
  }

  const swapCandidates = available.filter((c) => c.balance >= minSwapAmount);

  if (swapCandidates.length === 0) {
    throw new Error(
      `no ${coinType} coin with balance >= ${minSwapAmount} for swap`,
    );
  }

  const gasCandidates = available.filter((c) => c.balance >= gasBudget);
  const gasId = gasCandidates[0]?.id;
  const separateSwap = swapCandidates.find((c) => c.id !== gasId);
  if (separateSwap) {
    return { kind: 'object', id: separateSwap.id };
  }

  const only = available[0]!;
  if (only.balance < minSwapAmount + gasBudget) {
    throw new Error(
      `single SUI coin ${only.id} balance ${only.balance} < swap ${minSwapAmount} + gas ${gasBudget}`,
    );
  }

  return { kind: 'gas' };
}

export async function resolveBaitCoinInputs(
  client: SuiClient,
  owner: string,
  config: { coinTypeB: string; baitSuiMist: number; gasBudget: number },
): Promise<{ suiCoin: SuiCoinRef }> {
  const suiCoin = await resolveSuiForSwap(
    client,
    owner,
    config.coinTypeB,
    BigInt(config.baitSuiMist),
    BigInt(config.gasBudget),
  );
  return { suiCoin };
}

export async function resolveDumpCoinInputs(
  client: SuiClient,
  owner: string,
  config: { coinTypeA: string; coinTypeB: string; dumpTokenAmount: number },
): Promise<SwapCoinInputs> {
  const [tokenCoinId, receiveSuiId] = await Promise.all([
    pickLargestCoin(
      client,
      owner,
      config.coinTypeA,
      BigInt(config.dumpTokenAmount),
    ),
    pickSuiReceiveCoin(client, owner, config.coinTypeB),
  ]);
  return { suiCoin: { kind: 'object', id: receiveSuiId }, tokenCoinId };
}

/** Pick up to `count` distinct coins each with balance >= minAmount. */
export async function pickMultipleCoins(
  client: SuiClient,
  owner: string,
  coinType: string,
  minAmount: bigint,
  count: number,
  excludeIds: ReadonlySet<string> = new Set(),
): Promise<string[]> {
  const coins = await listCoins(client, owner, coinType);
  const eligible = coins.filter(
    (c) => !excludeIds.has(c.id) && c.balance >= minAmount,
  );
  const ids = eligible.slice(0, count).map((c) => c.id);

  if (ids.length < count) {
    throw new Error(
      `need ${count} ${coinType} coins with balance >= ${minAmount}, found ${ids.length}`,
    );
  }

  return ids;
}

/** Gas coin — must not overlap PTB input objects. */
export async function pickGasCoin(
  client: SuiClient,
  owner: string,
  coinType: string,
  gasBudget: bigint,
  excludeIds: ReadonlySet<string> = new Set(),
): Promise<string> {
  return pickLargestCoin(client, owner, coinType, gasBudget, excludeIds);
}

export function suiCoinObjectId(ref: SuiCoinRef): string | null {
  return ref.kind === 'object' ? ref.id : null;
}

export type WalletSwapPlan = {
  inputs: SwapCoinInputs;
  /** When false, SDK uses tx.gas (post-merge single SUI coin). */
  explicitGas: boolean;
  gasCoinId?: string;
};

/**
 * Plan swap inputs after optional coin merge.
 * Single SUI coin → suiCoin.kind = 'gas', no setGasPayment.
 * Multiple SUI coins → separate gas + swap/receive objects.
 */
export async function planWalletSwap(
  client: SuiClient,
  owner: string,
  config: {
    coinTypeA: string;
    coinTypeB: string;
    gasBudget: number;
    baitSuiMist?: number;
    dumpTokenAmount?: number;
  },
  mode: 'bait' | 'dump' | 'trap',
): Promise<WalletSwapPlan> {
  const suiCoins = await listCoins(client, owner, config.coinTypeB);
  const gasBudget = BigInt(config.gasBudget);

  if (suiCoins.length === 0) {
    throw new Error(`no ${config.coinTypeB} coin for ${owner}`);
  }

  const tokenCoinId =
    mode === 'bait'
      ? await pickAnyCoin(client, owner, config.coinTypeA)
      : await pickLargestCoin(
          client,
          owner,
          config.coinTypeA,
          BigInt(config.dumpTokenAmount!),
        );

  if (suiCoins.length <= 1) {
    const only = suiCoins[0]!;
    const minNeeded =
      mode === 'bait' || mode === 'trap'
        ? BigInt(config.baitSuiMist ?? 0) + gasBudget
        : gasBudget;
    if (only.balance < minNeeded) {
      throw new Error(
        `SUI balance ${only.balance} < required ${minNeeded} for ${owner}`,
      );
    }
    return {
      inputs: { suiCoin: { kind: 'gas' }, tokenCoinId },
      explicitGas: false,
    };
  }

  const gasCoinId = await pickGasCoin(
    client,
    owner,
    config.coinTypeB,
    gasBudget,
  );
  const reserved = new Set([gasCoinId]);

  const suiCoin =
    mode === 'dump'
      ? {
          kind: 'object' as const,
          id: await pickSuiReceiveCoin(
            client,
            owner,
            config.coinTypeB,
            reserved,
          ),
        }
      : await resolveSuiForSwap(
          client,
          owner,
          config.coinTypeB,
          BigInt(config.baitSuiMist ?? 0),
          gasBudget,
          reserved,
        );

  return {
    inputs: { suiCoin, tokenCoinId },
    explicitGas: true,
    gasCoinId,
  };
}

export async function resolveTrapCoinInputs(
  client: SuiClient,
  owner: string,
  config: {
    coinTypeA: string;
    coinTypeB: string;
    baitSuiMist: number;
    dumpTokenAmount: number;
    gasBudget: number;
  },
): Promise<SwapCoinInputs> {
  const [suiCoin, tokenCoinId] = await Promise.all([
    resolveSuiForSwap(
      client,
      owner,
      config.coinTypeB,
      BigInt(config.baitSuiMist),
      BigInt(config.gasBudget),
    ),
    pickLargestCoin(client, owner, config.coinTypeA, BigInt(config.dumpTokenAmount)),
  ]);
  return { suiCoin, tokenCoinId };
}

export type SwapLegResult = {
  suiCoin: TransactionObjectArgument;
  tokenCoin: TransactionObjectArgument;
  suiIsGas: boolean;
};
