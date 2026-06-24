import type { TokenDetailResponse } from "../api/types";
import { formatAmount } from "../lib/format";

interface Props {
  token: TokenDetailResponse;
}

export function TokenHeader({ token }: Props) {
  const copyCoinType = () => {
    void navigator.clipboard.writeText(token.coin_type);
  };

  return (
    <div className="flex flex-col gap-4 rounded-lg border border-zinc-800 bg-zinc-900/50 p-5 sm:flex-row sm:items-start">
      {token.image_url ? (
        <img
          src={token.image_url}
          alt=""
          className="h-16 w-16 rounded-full border border-zinc-700"
        />
      ) : (
        <div className="flex h-16 w-16 items-center justify-center rounded-full bg-zinc-800 text-2xl font-bold">
          {(token.symbol ?? "?")[0]}
        </div>
      )}
      <div className="min-w-0 flex-1">
        <div className="flex flex-wrap items-center gap-2">
          <h1 className="text-2xl font-bold">
            {token.symbol ?? "Unknown"}
          </h1>
          {token.name && (
            <span className="text-zinc-400">{token.name}</span>
          )}
        </div>
        <button
          type="button"
          onClick={copyCoinType}
          className="mt-1 max-w-full truncate font-mono text-xs text-zinc-500 hover:text-zinc-300"
          title="Click to copy"
        >
          {token.coin_type}
        </button>
        <div className="mt-4 grid grid-cols-2 gap-3 sm:grid-cols-4">
          <Stat
            label="Price"
            value={
              token.price_quote
                ? `${formatAmount(token.price_quote.amount)} ${token.price_quote.quote}`
                : "—"
            }
          />
          <Stat
            label="Volume 24h"
            value={
              token.volume_24h
                ? `${formatAmount(token.volume_24h.amount)} ${token.volume_24h.quote}`
                : "—"
            }
          />
          <Stat
            label="Txns 24h"
            value={token.txns_24h?.toLocaleString() ?? "—"}
          />
          <Stat label="Pools" value={String(token.pools_count)} />
        </div>
      </div>
    </div>
  );
}

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-md bg-zinc-950/60 px-3 py-2">
      <div className="text-xs text-zinc-500">{label}</div>
      <div className="mt-0.5 text-sm font-semibold text-zinc-100">{value}</div>
    </div>
  );
}
