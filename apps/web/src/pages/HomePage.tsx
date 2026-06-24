import { useQuery } from "@tanstack/react-query";
import { Link, useSearchParams } from "react-router-dom";
import { fetchTokenList } from "../api/client";
import { AppShell, tokenDetailHref } from "../components/Layout";
import { TokenSearch } from "../components/TokenSearch";
import { shortCoinType } from "../lib/format";

export function HomePage() {
  const [searchParams, setSearchParams] = useSearchParams();
  const q = searchParams.get("q") ?? "";

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["tokens", q],
    queryFn: () => fetchTokenList({ q: q || undefined, limit: 100 }),
  });

  const setQ = (value: string) => {
    if (value.trim()) {
      setSearchParams({ q: value.trim() });
    } else {
      setSearchParams({});
    }
  };

  return (
    <AppShell>
      <div className="mb-6 space-y-2">
        <h1 className="text-2xl font-bold">Tokens</h1>
        <p className="text-sm text-zinc-400">
          Indexed tokens from catalog. Search by symbol or navigate with full
          coin_type.
        </p>
        <TokenSearch
          value={q}
          onChange={setQ}
          onSubmit={() => {
            if (q.trim() && !q.includes("::")) {
              // list filter only; full coin_type navigates in TokenSearch
            }
          }}
        />
      </div>

      {isLoading && (
        <p className="text-sm text-zinc-400">Loading tokens…</p>
      )}
      {isError && (
        <p className="text-sm text-red-400">
          {(error as Error).message ?? "Failed to load tokens"}
        </p>
      )}

      {data && (
        <div className="overflow-hidden rounded-lg border border-zinc-800">
          <table className="w-full text-left text-sm">
            <thead className="bg-zinc-900 text-zinc-400">
              <tr>
                <th className="px-4 py-3 font-medium">Token</th>
                <th className="px-4 py-3 font-medium">Symbol</th>
                <th className="px-4 py-3 font-medium">Coin type</th>
                <th className="px-4 py-3 font-medium">Decimals</th>
              </tr>
            </thead>
            <tbody>
              {data.tokens.length === 0 ? (
                <tr>
                  <td colSpan={4} className="px-4 py-8 text-center text-zinc-500">
                    No tokens found. Ensure indexer and catalog-worker are running.
                  </td>
                </tr>
              ) : (
                data.tokens.map((token) => (
                  <tr
                    key={token.coin_type}
                    className="border-t border-zinc-800 hover:bg-zinc-900/60"
                  >
                    <td className="px-4 py-3">
                      <Link
                        to={tokenDetailHref(token.coin_type)}
                        className="flex items-center gap-2 font-medium text-emerald-400 hover:text-emerald-300"
                      >
                        {token.image_url ? (
                          <img
                            src={token.image_url}
                            alt=""
                            className="h-6 w-6 rounded-full"
                          />
                        ) : (
                          <span className="flex h-6 w-6 items-center justify-center rounded-full bg-zinc-800 text-xs">
                            {(token.symbol ?? "?")[0]}
                          </span>
                        )}
                        {token.name ?? "Unknown"}
                      </Link>
                    </td>
                    <td className="px-4 py-3 text-zinc-300">
                      {token.symbol ?? "—"}
                    </td>
                    <td className="px-4 py-3 font-mono text-xs text-zinc-500">
                      {shortCoinType(token.coin_type, 40)}
                    </td>
                    <td className="px-4 py-3 text-zinc-400">{token.decimals}</td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      )}
    </AppShell>
  );
}
