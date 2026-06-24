import { useMemo, useState } from "react";
import { useQuery, useInfiniteQuery } from "@tanstack/react-query";
import { Link, useParams } from "react-router-dom";
import {
  ApiError,
  decodeCoinTypePath,
  fetchPoolOhlc,
  fetchTokenDetail,
  fetchTokenPools,
  fetchTokenSwaps,
} from "../api/client";
import type { OhlcInterval } from "../api/types";
import { AppShell, PhasePlaceholder } from "../components/Layout";
import { OhlcChart } from "../components/OhlcChart";
import { PoolsTable } from "../components/PoolsTable";
import { SwapsTable } from "../components/SwapsTable";
import { TokenHeader } from "../components/TokenHeader";
import { isoRange } from "../lib/format";

export function TokenPage() {
  const params = useParams();
  const coinType = decodeCoinTypePath(params["*"] ?? "");

  const [selectedPoolId, setSelectedPoolId] = useState<string | null>(null);
  const [ohlcInterval, setOhlcInterval] = useState<OhlcInterval>("1h");
  const [rangeHours, setRangeHours] = useState(24 * 7);

  const detailQuery = useQuery({
    queryKey: ["token", coinType],
    queryFn: () => fetchTokenDetail(coinType),
    enabled: Boolean(coinType),
    retry: false,
  });

  const poolsQuery = useQuery({
    queryKey: ["token-pools", coinType],
    queryFn: () => fetchTokenPools(coinType),
    enabled: Boolean(coinType) && detailQuery.isSuccess,
  });

  const effectivePoolId =
    selectedPoolId ?? poolsQuery.data?.pools[0]?.pool_id ?? null;

  const range = useMemo(() => isoRange(rangeHours), [rangeHours]);

  const ohlcQuery = useQuery({
    queryKey: [
      "ohlc",
      effectivePoolId,
      ohlcInterval,
      range.from,
      range.to,
      coinType,
    ],
    queryFn: () =>
      fetchPoolOhlc(effectivePoolId!, {
        interval: ohlcInterval,
        from: range.from,
        to: range.to,
        base_coin_type: coinType,
      }),
    enabled: Boolean(effectivePoolId),
  });

  const swapsQuery = useInfiniteQuery({
    queryKey: ["token-swaps", coinType, effectivePoolId],
    queryFn: ({ pageParam }) =>
      fetchTokenSwaps(coinType, {
        pool_id: effectivePoolId ?? undefined,
        limit: 20,
        cursor: pageParam as string | undefined,
      }),
    initialPageParam: undefined as string | undefined,
    getNextPageParam: (last) => last.next_cursor ?? undefined,
    enabled: Boolean(coinType) && detailQuery.isSuccess,
  });

  const allSwaps = swapsQuery.data?.pages.flatMap((p) => p.swaps) ?? [];

  if (!coinType) {
    return (
      <AppShell>
        <p className="text-zinc-400">Missing coin type in URL.</p>
      </AppShell>
    );
  }

  if (detailQuery.isLoading) {
    return (
      <AppShell>
        <p className="text-zinc-400">Loading token…</p>
      </AppShell>
    );
  }

  if (detailQuery.isError) {
    const err = detailQuery.error;
    const is404 = err instanceof ApiError && err.status === 404;
    return (
      <AppShell>
        <Link to="/" className="text-sm text-emerald-500 hover:underline">
          ← Back to tokens
        </Link>
        <div className="mt-4 rounded-lg border border-red-900/50 bg-red-950/20 p-4">
          <h1 className="font-semibold text-red-300">
            {is404 ? "Token not indexed" : "Failed to load token"}
          </h1>
          <p className="mt-1 font-mono text-sm text-zinc-400">{coinType}</p>
          <p className="mt-2 text-sm text-zinc-500">
            {is404
              ? "This coin_type is not in the catalog yet. Wait for indexer or check spelling."
              : (err as Error).message}
          </p>
        </div>
      </AppShell>
    );
  }

  const token = detailQuery.data!;

  return (
    <AppShell>
      <Link to="/" className="text-sm text-emerald-500 hover:underline">
        ← Back to tokens
      </Link>

      <div className="mt-4 space-y-8">
        <TokenHeader token={token} />

        <section>
          <h2 className="mb-3 text-lg font-semibold">Pools</h2>
          <PoolsTable
            pools={poolsQuery.data?.pools ?? []}
            selectedPoolId={effectivePoolId}
            onSelectPool={setSelectedPoolId}
          />
        </section>

        <section>
          <h2 className="mb-3 text-lg font-semibold">Price chart</h2>
          <OhlcChart
            poolId={effectivePoolId}
            baseCoinType={coinType}
            bars={ohlcQuery.data?.bars ?? []}
            interval={ohlcInterval}
            rangeHours={rangeHours}
            isLoading={ohlcQuery.isLoading}
            onIntervalChange={setOhlcInterval}
            onRangeChange={setRangeHours}
          />
        </section>

        <section>
          <h2 className="mb-3 text-lg font-semibold">Recent swaps</h2>
          <SwapsTable
            swaps={allSwaps}
            hasMore={Boolean(swapsQuery.hasNextPage)}
            isLoadingMore={swapsQuery.isFetchingNextPage}
            onLoadMore={() => void swapsQuery.fetchNextPage()}
          />
        </section>

        <section className="grid gap-4 sm:grid-cols-3">
          <PhasePlaceholder
            phase="Phase 3"
            title="Liquidity depth"
            description="CLMM tick depth and accurate TVL from pool snapshots."
          />
          <PhasePlaceholder
            phase="Phase 4"
            title="Holders"
            description="Holder count and top holders from coin balance pipeline."
          />
          <PhasePlaceholder
            phase="Phase 5"
            title="Bubble map"
            description="Transfer graph visualization and wallet clusters."
          />
        </section>
      </div>
    </AppShell>
  );
}
