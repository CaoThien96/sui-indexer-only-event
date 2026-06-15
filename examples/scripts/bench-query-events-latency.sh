#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# BENCH_HOT_COLD=1 loads CLICKHOUSE_* from simple-sui-indexer/.env via scripts/ch_client.py
exec python3 scripts/bench-query-events-latency.py "$@"
