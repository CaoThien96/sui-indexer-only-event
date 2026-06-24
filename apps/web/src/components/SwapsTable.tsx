import type { SwapDto } from "../api/types";
import { formatAmount, txExplorerUrl } from "../lib/format";

interface Props {
  swaps: SwapDto[];
  hasMore: boolean;
  isLoadingMore: boolean;
  onLoadMore: () => void;
}

export function SwapsTable({
  swaps,
  hasMore,
  isLoadingMore,
  onLoadMore,
}: Props) {
  if (swaps.length === 0) {
    return (
      <p className="text-sm text-zinc-500">No swaps indexed yet for this token.</p>
    );
  }

  return (
    <div>
      <div className="overflow-hidden rounded-lg border border-zinc-800">
        <table className="w-full text-left text-sm">
          <thead className="bg-zinc-900 text-zinc-400">
            <tr>
              <th className="px-4 py-3 font-medium">Time</th>
              <th className="px-4 py-3 font-medium">Protocol</th>
              <th className="px-4 py-3 font-medium">Amount base</th>
              <th className="px-4 py-3 font-medium">Amount quote</th>
              <th className="px-4 py-3 font-medium">Price</th>
              <th className="px-4 py-3 font-medium">Tx</th>
            </tr>
          </thead>
          <tbody>
            {swaps.map((swap) => (
              <tr
                key={`${swap.tx_digest}-${swap.event_seq}`}
                className="border-t border-zinc-800"
              >
                <td className="px-4 py-3 text-zinc-400">
                  {new Date(swap.time).toLocaleString()}
                </td>
                <td className="px-4 py-3 capitalize">{swap.protocol}</td>
                <td className="px-4 py-3 font-mono text-xs">
                  {formatAmount(swap.amount_base)}
                </td>
                <td className="px-4 py-3 font-mono text-xs">
                  {formatAmount(swap.amount_quote)}
                </td>
                <td className="px-4 py-3 font-mono text-xs">
                  {formatAmount(swap.price_quote_per_base, 6)}
                </td>
                <td className="px-4 py-3">
                  <a
                    href={txExplorerUrl(swap.tx_digest)}
                    target="_blank"
                    rel="noreferrer"
                    className="text-xs text-emerald-500 hover:underline"
                  >
                    {swap.tx_digest.slice(0, 8)}…
                  </a>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      {hasMore && (
        <button
          type="button"
          onClick={onLoadMore}
          disabled={isLoadingMore}
          className="mt-3 rounded-lg border border-zinc-700 px-4 py-2 text-sm text-zinc-300 hover:bg-zinc-900 disabled:opacity-50"
        >
          {isLoadingMore ? "Loading…" : "Load more"}
        </button>
      )}
    </div>
  );
}
