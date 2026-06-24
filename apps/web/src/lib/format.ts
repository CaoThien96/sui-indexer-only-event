export function shortCoinType(coinType: string, maxLen = 24): string {
  if (coinType.length <= maxLen) return coinType;
  return `${coinType.slice(0, 10)}…${coinType.slice(-8)}`;
}

export function symbolFromCoinType(coinType: string): string {
  const parts = coinType.split("::");
  return parts[parts.length - 1] ?? coinType;
}

export function formatAmount(value: string | null | undefined, decimals = 4): string {
  if (!value) return "—";
  const n = Number(value);
  if (Number.isNaN(n)) return value;
  if (n === 0) return "0";
  if (Math.abs(n) >= 1_000_000) return `${(n / 1_000_000).toFixed(2)}M`;
  if (Math.abs(n) >= 1_000) return `${(n / 1_000).toFixed(2)}K`;
  return n.toFixed(decimals);
}

export function txExplorerUrl(digest: string): string {
  return `https://suiscan.xyz/mainnet/tx/${digest}`;
}

export function poolExplorerUrl(poolId: string): string {
  return `https://suiscan.xyz/mainnet/object/${poolId}`;
}

export function isoRange(hours: number): { from: string; to: string } {
  const to = new Date();
  const from = new Date(to.getTime() - hours * 60 * 60 * 1000);
  return { from: from.toISOString(), to: to.toISOString() };
}

export const RANGE_PRESETS = [
  { label: "24h", hours: 24 },
  { label: "7d", hours: 24 * 7 },
  { label: "30d", hours: 24 * 30 },
] as const;

export const OHLC_INTERVALS = ["1h", "4h", "24h"] as const;
