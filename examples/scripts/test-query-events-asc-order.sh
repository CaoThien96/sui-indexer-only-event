#!/usr/bin/env bash
# Wrapper for ASC order verification (see test-query-events-asc-order.py).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
if [[ -f "$ROOT/clickhouse/.env" ]]; then
  set -a
  # shellcheck disable=SC1091
  source "$ROOT/clickhouse/.env"
  set +a
fi
export RPC_URL="${RPC_URL:-http://127.0.0.1:9000}"
export MOVE_EVENT_TYPE="${MOVE_EVENT_TYPE:-0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent}"
export LIMIT="${LIMIT:-50}"
export MAX_PAGES="${MAX_PAGES:-5}"

exec python3 "$ROOT/scripts/test-query-events-asc-order.py" "$@"
