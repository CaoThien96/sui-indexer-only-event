import type { PoolSummary } from "../api/types";
import { formatAmount, poolExplorerUrl, symbolFromCoinType } from "../lib/format";

interface Props {
  pools: PoolSummary[];
  selectedPoolId: string | null;
  onSelectPool: (poolId: string) => void;
}

export function PoolsTable({ pools, selectedPoolId, onSelectPool }: Props) {
  if (pools.length === 0) {
    return (
      <p className="text-sm text-zinc-500">No pools indexed for this token yet.</p>
    );
  }

  return (
    <div className="overflow-hidden rounded-lg border border-zinc-800">
      <table className="w-full text-left text-sm">
        <thead className="bg-zinc-900 text-zinc-400">
          <tr>
            <th className="px-4 py-3 font-medium">Protocol</th>
            <th className="px-4 py-3 font-medium">Pair</th>
            <th className="px-4 py-3 font-medium">TVL (quote)</th>
            <th className="px-4 py-3 font-medium">Pool</th>
          </tr>
        </thead>
        <tbody>
          {pools.map((pool) => {
            const selected = pool.pool_id === selectedPoolId;
            return (
              <tr
                key={pool.pool_id}
                onClick={() => onSelectPool(pool.pool_id)}
                className={`cursor-pointer border-t border-zinc-800 ${
                  selected ? "bg-emerald-950/40" : "hover:bg-zinc-900/60"
                }`}
              >
                <td className="px-4 py-3 capitalize text-zinc-200">
                  {pool.protocol}
                </td>
                <td className="px-4 py-3 text-zinc-300">
                  {symbolFromCoinType(pool.coin_type_a)} /{" "}
                  {symbolFromCoinType(pool.coin_type_b)}
                </td>
                <td className="px-4 py-3 text-zinc-300">
                  {pool.tvl_quote ? formatAmount(pool.tvl_quote) : "—"}
                </td>
                <td className="px-4 py-3">
                  <a
                    href={poolExplorerUrl(pool.pool_id)}
                    target="_blank"
                    rel="noreferrer"
                    onClick={(e) => e.stopPropagation()}
                    className="font-mono text-xs text-emerald-500 hover:underline"
                  >
                    {pool.pool_id.slice(0, 10)}…
                  </a>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
