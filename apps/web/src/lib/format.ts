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
  { label: "1h", hours: 1 },
  { label: "4h", hours: 4 },
  { label: "12h", hours: 12 },
  { label: "24h", hours: 24 },
  { label: "7d", hours: 24 * 7 },
  { label: "30d", hours: 24 * 30 },
] as const;

export const OHLC_INTERVALS = ["1m", "5m", "15m", "30m", "1h", "4h", "24h"] as const;

export const MAX_RANGE_HOURS: Record<string, number> = {
  // 1m must allow the same 7d window as 5m — otherwise tokens with no trades in the
  // last 24h show an empty 1m chart while coarser intervals still have bars.
  "1m": 24 * 7,
  "5m": 24 * 7,
  "15m": 24 * 7,
  "30m": 24 * 30,
  "1h": 24 * 30,
  "4h": 24 * 30,
  "24h": 24 * 30,
};
