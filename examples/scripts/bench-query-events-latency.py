#!/usr/bin/env python3
"""Compare suix_queryEvents latency: local rpc-service vs Sui fullnode.

Uses the same filter/limit/descending on both. Each endpoint paginates with
its own nextCursor chain (realistic client usage).

Hot/cold SLA (ClickHouse local tier):
  BENCH_HOT_COLD=1 — paginate from a recent cursor (hot SSD) vs old cursor (cold HDD)

Usage:
  python3 scripts/bench-query-events-latency.py
  MAX_PAGES=10 WARMUP=1 FILTER='{"Sender":"0x..."}' python3 scripts/bench-query-events-latency.py
  BENCH_HOT_COLD=1 python3 scripts/bench-query-events-latency.py
"""

from __future__ import annotations

import json
import os
import statistics
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any

sys.path.insert(0, str(Path(__file__).resolve().parent))
from ch_client import run_query_tsv  # noqa: E402

LOCAL_URL = os.environ.get("LOCAL_RPC_URL", "http://127.0.0.1:9000")
FULLNODE_URL = os.environ.get("FULLNODE_URL", "https://fullnode.mainnet.sui.io:443")
LIMIT = int(os.environ.get("LIMIT", "50"))
MAX_PAGES = int(os.environ.get("MAX_PAGES", "5"))
DESCENDING = os.environ.get("DESCENDING", "true").lower() in ("1", "true", "yes")
WARMUP = os.environ.get("WARMUP", "1") in ("1", "true", "yes")
TIMEOUT = float(os.environ.get("TIMEOUT", "120"))
BENCH_HOT_COLD = os.environ.get("BENCH_HOT_COLD", "0") in ("1", "true", "yes")
HOT_MAX_AGE_DAYS = int(os.environ.get("HOT_MAX_AGE_DAYS", "2"))
COLD_MIN_AGE_DAYS = int(os.environ.get("COLD_MIN_AGE_DAYS", "4"))

DEFAULT_FILTER = {
    "MoveEventType": (
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent"
    )
}


def load_filter() -> dict[str, Any]:
    raw = os.environ.get("FILTER")
    if not raw:
        return DEFAULT_FILTER
    return json.loads(raw)


def rpc_query(
    url: str,
    event_filter: dict[str, Any],
    cursor: dict[str, str] | None,
    limit: int,
    descending: bool,
) -> tuple[dict[str, Any], float]:
    body = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "suix_queryEvents",
        "params": [event_filter, cursor, limit, descending],
    }
    data = json.dumps(body).encode()
    req = urllib.request.Request(
        url,
        data=data,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    start = time.perf_counter()
    with urllib.request.urlopen(req, timeout=TIMEOUT) as resp:
        payload = json.load(resp)
    elapsed_ms = (time.perf_counter() - start) * 1000
    if "error" in payload:
        raise RuntimeError(f"RPC error from {url}: {payload['error']}")
    return payload["result"], elapsed_ms


def paginate(
    label: str,
    url: str,
    event_filter: dict[str, Any],
    start_cursor: dict[str, str] | None = None,
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    cursor: dict[str, str] | None = start_cursor

    if WARMUP:
        try:
            rpc_query(url, event_filter, start_cursor, LIMIT, DESCENDING)
        except Exception as e:
            print(f"[{label}] warmup failed: {e}", file=sys.stderr)

    for page in range(1, MAX_PAGES + 1):
        result, elapsed_ms = rpc_query(url, event_filter, cursor, LIMIT, DESCENDING)
        count = len(result.get("data") or [])
        has_next = bool(result.get("hasNextPage"))
        cursor = result.get("nextCursor")
        rows.append(
            {
                "page": page,
                "ms": elapsed_ms,
                "count": count,
                "hasNextPage": has_next,
                "cursor": cursor,
            }
        )
        print(
            f"[{label}] page {page}: {elapsed_ms:7.1f} ms | "
            f"events={count:2d} hasNextPage={has_next}"
        )
        if not has_next:
            break
        if cursor is None:
            break

    return rows


def summarize(label: str, rows: list[dict[str, Any]]) -> None:
    if not rows:
        print(f"[{label}] no samples")
        return
    times = [r["ms"] for r in rows]
    total_events = sum(r["count"] for r in rows)
    print(
        f"[{label}] pages={len(rows)} events={total_events} | "
        f"min={min(times):.1f}ms avg={statistics.mean(times):.1f}ms "
        f"median={statistics.median(times):.1f}ms max={max(times):.1f}ms "
        f"total={sum(times):.1f}ms"
    )


def find_tier_cursor(event_type: str, hot: bool) -> dict[str, str] | None:
    age_clause = (
        f"timestamp_ms >= toUnixTimestamp64Milli(now64() - INTERVAL {HOT_MAX_AGE_DAYS} DAY)"
        if hot
        else f"timestamp_ms <= toUnixTimestamp64Milli(now64() - INTERVAL {COLD_MIN_AGE_DAYS} DAY)"
    )
    sql = f"""
SELECT event_id_tx_digest, event_id_seq
FROM package_events FINAL
WHERE event_type ILIKE '{event_type.replace("'", "''")}'
  AND {age_clause}
ORDER BY checkpoint_sequence_number DESC,
         transaction_sequence_in_checkpoint DESC,
         event_sequence_in_transaction DESC
LIMIT 1
"""
    lines = run_query_tsv(sql)
    if not lines:
        return None
    tx_digest, event_seq = lines[0].split("\t")
    return {"txDigest": tx_digest, "eventSeq": event_seq}


def bench_hot_cold_tiers(event_filter: dict[str, Any]) -> int:
    if "MoveEventType" not in event_filter:
        print("BENCH_HOT_COLD requires MoveEventType filter", file=sys.stderr)
        return 1

    event_type = event_filter["MoveEventType"]
    hot_cursor = find_tier_cursor(event_type, hot=True)
    cold_cursor = find_tier_cursor(event_type, hot=False)

    print("--- ClickHouse hot vs cold tier (local rpc-service) ---")
    print(f"hot: events <= {HOT_MAX_AGE_DAYS}d old | cold: events >= {COLD_MIN_AGE_DAYS}d old")
    print()

    if hot_cursor:
        print(f"hot start cursor: {hot_cursor}")
        hot_rows = paginate("hot-tier", LOCAL_URL, event_filter, hot_cursor)
        print()
        summarize("hot-tier", hot_rows)
    else:
        print("[hot-tier] no cursor found — index more recent data or lower HOT_MAX_AGE_DAYS")

    print()
    if cold_cursor:
        print(f"cold start cursor: {cold_cursor}")
        cold_rows = paginate("cold-tier", LOCAL_URL, event_filter, cold_cursor)
        print()
        summarize("cold-tier", cold_rows)
        if hot_cursor and hot_rows and cold_rows:
            hot_avg = statistics.mean(r["ms"] for r in hot_rows)
            cold_avg = statistics.mean(r["ms"] for r in cold_rows)
            print()
            print(
                f"SLA snapshot: hot avg {hot_avg:.1f}ms vs cold avg {cold_avg:.1f}ms "
                f"({cold_avg / hot_avg:.1f}x slower)" if hot_avg > 0 else "SLA snapshot unavailable"
            )
    else:
        print(
            f"[cold-tier] no cursor found — need data older than {COLD_MIN_AGE_DAYS}d "
            "(TTL MOVE runs after 3 days)"
        )

    return 0


def main() -> int:
    event_filter = load_filter()
    order = "DESC" if DESCENDING else "ASC"
    print(f"Filter: {json.dumps(event_filter)}")
    print(f"limit={LIMIT} pages={MAX_PAGES} order={order} warmup={WARMUP}")
    print(f"local   -> {LOCAL_URL}")
    print(f"fullnode-> {FULLNODE_URL}")
    print()

    if BENCH_HOT_COLD:
        return bench_hot_cold_tiers(event_filter)

    try:
        local_rows = paginate("local", LOCAL_URL, event_filter)
    except urllib.error.URLError as e:
        print(f"[local] unreachable: {e}", file=sys.stderr)
        return 1
    except RuntimeError as e:
        print(f"[local] {e}", file=sys.stderr)
        return 1

    print()
    try:
        fullnode_rows = paginate("fullnode", FULLNODE_URL, event_filter)
    except urllib.error.URLError as e:
        print(f"[fullnode] unreachable: {e}", file=sys.stderr)
        return 1
    except RuntimeError as e:
        print(f"[fullnode] {e}", file=sys.stderr)
        return 1

    print()
    summarize("local", local_rows)
    summarize("fullnode", fullnode_rows)

    pages = min(len(local_rows), len(fullnode_rows))
    if pages:
        print()
        print("page-by-page delta (local - fullnode, negative = local faster):")
        for i in range(pages):
            l = local_rows[i]
            f = fullnode_rows[i]
            delta = l["ms"] - f["ms"]
            ratio = l["ms"] / f["ms"] if f["ms"] > 0 else float("inf")
            print(
                f"  page {l['page']}: local {l['ms']:7.1f} ms | "
                f"fullnode {f['ms']:7.1f} ms | delta {delta:+7.1f} ms ({ratio:.2f}x)"
            )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
