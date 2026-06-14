import type { SuiClient } from '@mysten/sui/client';
import type { TrapConfig } from './config.js';

export type PoolState = {
  sqrtPrice: bigint;
  liquidity: bigint;
  balanceA: bigint;
  balanceB: bigint;
};

type BalanceField = { fields?: { value?: string } };

/** Read pool state from on-chain Cetus Pool object. */
export async function readPoolState(
  client: SuiClient,
  config: TrapConfig,
): Promise<PoolState> {
  const obj = await client.getObject({
    id: config.poolId,
    options: { showContent: true },
  });

  const content = obj.data?.content;
  if (!content || content.dataType !== 'moveObject') {
    throw new Error(`pool ${config.poolId} not found or not a move object`);
  }

  const fields = content.fields as Record<string, unknown>;
  const sqrtPrice = BigInt(String(fields.current_sqrt_price));
  const liquidity = BigInt(String(fields.liquidity));
  const balanceA = BigInt(
    String(
      typeof fields.coin_a === 'object'
        ? (fields.coin_a as BalanceField)?.fields?.value ?? 0
        : fields.coin_a ?? 0,
    ),
  );
  const balanceB = BigInt(
    String(
      typeof fields.coin_b === 'object'
        ? (fields.coin_b as BalanceField)?.fields?.value ?? 0
        : fields.coin_b ?? 0,
    ),
  );

  return { sqrtPrice, liquidity, balanceA, balanceB };
}
