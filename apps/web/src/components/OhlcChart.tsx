import { useEffect, useRef } from "react";
import { createChart, type IChartApi, type ISeriesApi } from "lightweight-charts";
import type { OhlcBar, OhlcInterval } from "../api/types";
import { OHLC_INTERVALS, RANGE_PRESETS, isoRange } from "../lib/format";

interface Props {
  poolId: string | null;
  baseCoinType?: string;
  bars: OhlcBar[];
  interval: OhlcInterval;
  rangeHours: number;
  isLoading: boolean;
  onIntervalChange: (interval: OhlcInterval) => void;
  onRangeChange: (hours: number) => void;
}

export function OhlcChart({
  poolId,
  bars,
  interval,
  rangeHours,
  isLoading,
  onIntervalChange,
  onRangeChange,
}: Props) {
  const containerRef = useRef<HTMLDivElement>(null);
  const chartRef = useRef<IChartApi | null>(null);
  const seriesRef = useRef<ISeriesApi<"Candlestick"> | null>(null);

  useEffect(() => {
    if (!containerRef.current) return;
    const chart = createChart(containerRef.current, {
      layout: {
        background: { color: "#09090b" },
        textColor: "#a1a1aa",
      },
      grid: {
        vertLines: { color: "#27272a" },
        horzLines: { color: "#27272a" },
      },
      width: containerRef.current.clientWidth,
      height: 320,
    });
    const series = chart.addCandlestickSeries({
      upColor: "#10b981",
      downColor: "#ef4444",
      borderVisible: false,
      wickUpColor: "#10b981",
      wickDownColor: "#ef4444",
    });
    chartRef.current = chart;
    seriesRef.current = series;

    const ro = new ResizeObserver(() => {
      if (containerRef.current) {
        chart.applyOptions({ width: containerRef.current.clientWidth });
      }
    });
    ro.observe(containerRef.current);

    return () => {
      ro.disconnect();
      chart.remove();
      chartRef.current = null;
      seriesRef.current = null;
    };
  }, []);

  useEffect(() => {
    if (!seriesRef.current) return;
    const data = bars.map((b) => ({
      time: Math.floor(new Date(b.bucket).getTime() / 1000) as import("lightweight-charts").UTCTimestamp,
      open: Number(b.open),
      high: Number(b.high),
      low: Number(b.low),
      close: Number(b.close),
    }));
    seriesRef.current.setData(data);
    chartRef.current?.timeScale().fitContent();
  }, [bars]);

  if (!poolId) {
    return (
      <p className="text-sm text-zinc-500">Select a pool to view OHLC chart.</p>
    );
  }

  const range = isoRange(rangeHours);

  return (
    <div className="space-y-3">
      <div className="flex flex-wrap items-center gap-2">
        <div className="flex rounded-lg border border-zinc-700 p-0.5">
          {OHLC_INTERVALS.map((iv) => (
            <button
              key={iv}
              type="button"
              onClick={() => onIntervalChange(iv)}
              className={`rounded-md px-3 py-1 text-xs font-medium ${
                interval === iv
                  ? "bg-emerald-800 text-white"
                  : "text-zinc-400 hover:text-zinc-200"
              }`}
            >
              {iv}
            </button>
          ))}
        </div>
        <div className="flex rounded-lg border border-zinc-700 p-0.5">
          {RANGE_PRESETS.map((preset) => (
            <button
              key={preset.label}
              type="button"
              onClick={() => onRangeChange(preset.hours)}
              className={`rounded-md px-3 py-1 text-xs font-medium ${
                rangeHours === preset.hours
                  ? "bg-zinc-700 text-white"
                  : "text-zinc-400 hover:text-zinc-200"
              }`}
            >
              {preset.label}
            </button>
          ))}
        </div>
        {isLoading && (
          <span className="text-xs text-zinc-500">Loading chart…</span>
        )}
      </div>
      <p className="font-mono text-xs text-zinc-600">
        {range.from.slice(0, 19)} → {range.to.slice(0, 19)} UTC
      </p>
      <div ref={containerRef} className="w-full rounded-lg border border-zinc-800" />
      {bars.length === 0 && !isLoading && (
        <p className="text-sm text-zinc-500">
          No OHLC bars in this range. Data appears after swaps are indexed in
          Timescale.
        </p>
      )}
    </div>
  );
}
