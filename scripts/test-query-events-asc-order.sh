#!/usr/bin/env bash
# Wrapper for ASC order verification (see test-query-events-asc-order.py).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export RPC_URL="${RPC_URL:-http://127.0.0.1:9000}"
export DATABASE_URL="${DATABASE_URL:-postgres://postgres:postgres@localhost:5432/sui_indexer}"
export MOVE_EVENT_TYPE="${MOVE_EVENT_TYPE:-0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent}"
export LIMIT="${LIMIT:-50}"
export MAX_PAGES="${MAX_PAGES:-5}"

exec python3 "$ROOT/scripts/test-query-events-asc-order.py" "$@"
