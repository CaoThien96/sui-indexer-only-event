#!/usr/bin/env bash
# Prune package_events older than RETENTION_DAYS (default 3).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export DATABASE_URL="${DATABASE_URL:-postgres://postgres:postgres@localhost:5432/sui_indexer}"
export RETENTION_DAYS="${RETENTION_DAYS:-3}"
export BATCH_SIZE="${BATCH_SIZE:-10000}"

if [[ -f "$ROOT/simple-sui-indexer/.env" ]]; then
  set -a
  # shellcheck disable=SC1091
  source "$ROOT/simple-sui-indexer/.env"
  set +a
fi

exec python3 "$ROOT/scripts/prune-package-events.py" "$@"
