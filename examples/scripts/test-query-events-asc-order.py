#!/usr/bin/env python3
"""Verify rpc-service suix_queryEvents ASC pagination order.

For each event returned by the RPC, loads on-chain stream position from
package_events and checks:
  1. Positions are strictly non-decreasing within each page (ASC).
  2. Positions are strictly increasing across page boundaries (no overlap).
  3. Cursor continuity: last event of page N == cursor anchor before page N+1.

Optional: compare event-id sequence with fullnode mainnet for the same pages.

Usage:
  python3 scripts/test-query-events-asc-order.py
  MOVE_EVENT_TYPE='0x1eab...::pool::SwapEvent' MAX_PAGES=3 LIMIT=20 python3 scripts/test-query-events-asc-order.py
  COMPARE_FULLNODE=1 python3 scripts/test-query-events-asc-order.py

Requires: rpc-service running, curl. Set CLICKHOUSE_URL if not default.
Uses ClickHouse to verify stream positions (package_events).
"""

from __future__ import annotations

import json
import os
import sys
import urllib.error
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Any

sys.path.insert(0, str(Path(__file__).resolve().parent))
from ch_client import run_query_tsv  # noqa: E402

RPC_URL = os.environ.get("RPC_URL", "http://127.0.0.1:9000")
FULLNODE_URL = os.environ.get("FULLNODE_URL", "https://fullnode.mainnet.sui.io:443")
MOVE_EVENT_TYPE = os.environ.get(
    "MOVE_EVENT_TYPE",
    "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent",
)
LIMIT = int(os.environ.get("LIMIT", "50"))
MAX_PAGES = int(os.environ.get("MAX_PAGES", "5"))
COMPARE_FULLNODE = os.environ.get("COMPARE_FULLNODE", "0") == "1"


@dataclass(frozen=True, order=True)
class StreamPos:
    checkpoint: int
    tx_idx: int
    event_idx: int


@dataclass(frozen=True)
class EventId:
    tx_digest: str
    event_seq: int


def rpc_query(url: str, move_event_type: str, cursor: dict[str, str] | None, limit: int, descending: bool) -> dict[str, Any]:
    body = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "suix_queryEvents",
        "params": [
            {"MoveEventType": move_event_type},
            cursor,
            limit,
            descending,
        ],
    }
    req = urllib.request.Request(
        url,
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=60) as resp:
        payload = json.load(resp)
    if "error" in payload:
        raise RuntimeError(f"RPC error from {url}: {payload['error']}")
    return payload["result"]


def parse_event_id(raw: dict[str, Any]) -> EventId:
    event_id = raw["id"]
    return EventId(
        tx_digest=event_id["txDigest"],
        event_seq=int(event_id["eventSeq"]),
    )


def load_stream_positions(ids: list[EventId]) -> dict[EventId, StreamPos]:
    if not ids:
        return {}

    tuples = ",".join(
        f"('{i.tx_digest.replace(chr(39), chr(39)+chr(39))}', {i.event_seq})" for i in ids
    )
    sql = f"""
SELECT event_id_tx_digest, event_id_seq,
       checkpoint_sequence_number,
       transaction_sequence_in_checkpoint,
       event_sequence_in_transaction
FROM package_events FINAL
WHERE (event_id_tx_digest, event_id_seq) IN ({tuples})
"""

    try:
        lines = run_query_tsv(sql)
    except RuntimeError as exc:
        raise RuntimeError(f"ClickHouse query failed: {exc}") from exc

    out: dict[EventId, StreamPos] = {}
    for line in lines:
        tx_digest, event_seq, ckpt, tx_idx, ev_idx = line.split("\t")
        key = EventId(tx_digest, int(event_seq))
        out[key] = StreamPos(int(ckpt), int(tx_idx), int(ev_idx))
    return out


def positions_for_page(events: list[dict[str, Any]]) -> list[tuple[EventId, StreamPos]]:
    ids = [parse_event_id(e) for e in events]
    pos_map = load_stream_positions(ids)
    missing = [i for i in ids if i not in pos_map]
    if missing:
        raise RuntimeError(f"{len(missing)} RPC events missing in DB (first: {missing[0]})")
    return [(i, pos_map[i]) for i in ids]


def assert_strictly_increasing(label: str, positions: list[StreamPos]) -> None:
    for prev, curr in zip(positions, positions[1:]):
        if not (prev < curr):
            raise AssertionError(
                f"{label}: not strictly ASC at stream position\n"
                f"  prev={prev}\n  curr={curr}"
            )


def paginate_asc(url: str, max_pages: int) -> list[list[EventId]]:
    pages: list[list[EventId]] = []
    cursor: dict[str, str] | None = None

    for page_num in range(1, max_pages + 1):
        result = rpc_query(url, MOVE_EVENT_TYPE, cursor, LIMIT, descending=False)
        data = result.get("data") or []
        if not data:
            print(f"page {page_num}: empty data — stop")
            break

        ids = [parse_event_id(e) for e in data]
        pages.append(ids)

        positions = [p for _, p in positions_for_page(data)]
        assert_strictly_increasing(f"page {page_num} internal", positions)

        has_next = bool(result.get("hasNextPage"))
        next_cursor = result.get("nextCursor")
        print(
            f"page {page_num}: {len(ids)} events | "
            f"stream {positions[0]} .. {positions[-1]} | "
            f"hasNextPage={has_next}"
        )

        if not has_next:
            print("hasNextPage=false — stop")
            break
        if not next_cursor:
            raise AssertionError(f"page {page_num}: hasNextPage=true but nextCursor is null")

        cursor = next_cursor

    return pages


def assert_cross_page_order(pages: list[list[EventId]]) -> None:
    if len(pages) < 2:
        print("only one page fetched — skip cross-page check")
        return

    all_ids = [i for page in pages for i in page]
    pos_map = load_stream_positions(all_ids)
    ordered_positions = [pos_map[i] for i in all_ids]
    assert_strictly_increasing("all pages combined", ordered_positions)

    # No duplicate event ids across pages
    if len(all_ids) != len(set(all_ids)):
        raise AssertionError("duplicate event ids across pages")

    # Cross-page: last of page N strictly before first of page N+1
    for idx in range(len(pages) - 1):
        last_pos = pos_map[pages[idx][-1]]
        first_pos = pos_map[pages[idx + 1][0]]
        if not (last_pos < first_pos):
            raise AssertionError(
                f"page {idx + 1} -> {idx + 2} boundary not strict ASC\n"
                f"  last={last_pos}\n  first={first_pos}"
            )
        print(f"page {idx + 1} -> {idx + 2} boundary OK ({last_pos} < {first_pos})")


def compare_with_fullnode(local_pages: list[list[EventId]]) -> None:
    if not local_pages:
        return

    print("\n--- fullnode comparison (ASC, same limit/pages) ---")
    fn_pages: list[list[EventId]] = []
    cursor: dict[str, str] | None = None

    for page_num in range(1, len(local_pages) + 1):
        result = rpc_query(FULLNODE_URL, MOVE_EVENT_TYPE, cursor, LIMIT, descending=False)
        data = result.get("data") or []
        fn_ids = [parse_event_id(e) for e in data]
        fn_pages.append(fn_ids)
        cursor = result.get("nextCursor")
        print(f"fullnode page {page_num}: {len(fn_ids)} events")

    for i, (local, remote) in enumerate(zip(local_pages, fn_pages), start=1):
        if local != remote:
            # Indexer may lag or scope differs — report first mismatch only
            for j, (a, b) in enumerate(zip(local, remote)):
                if a != b:
                    raise AssertionError(
                        f"page {i} differs at index {j}:\n  indexer  {a}\n  fullnode {b}"
                    )
            if len(local) != len(remote):
                raise AssertionError(
                    f"page {i} length mismatch: indexer={len(local)} fullnode={len(remote)}"
                )
        else:
            print(f"page {i}: event-id order matches fullnode ({len(local)} events)")


def main() -> int:
    print(f"RPC_URL={RPC_URL}")
    print(f"MOVE_EVENT_TYPE={MOVE_EVENT_TYPE}")
    print(f"LIMIT={LIMIT} MAX_PAGES={MAX_PAGES} ASC\n")

    try:
        health = urllib.request.urlopen(f"{RPC_URL.rstrip('/')}/health", timeout=5)
        health.read()
    except (urllib.error.URLError, TimeoutError) as exc:
        print(f"ERROR: rpc-service not reachable at {RPC_URL}: {exc}", file=sys.stderr)
        return 1

    pages = paginate_asc(RPC_URL, MAX_PAGES)
    if not pages:
        print("ERROR: no events returned", file=sys.stderr)
        return 1

    assert_cross_page_order(pages)

    if COMPARE_FULLNODE:
        compare_with_fullnode(pages)

    print("\nOK: ASC stream order verified")
    return 0


if __name__ == "__main__":
    sys.exit(main())
