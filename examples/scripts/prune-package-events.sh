#!/usr/bin/env bash
# DEPRECATED: package_events moved to ClickHouse with TTL tier (hot SSD + cold HDD).
# Retention is managed by ClickHouse TTL MOVE — no PostgreSQL DELETE needed.
set -euo pipefail
echo "DEPRECATED: prune-package-events.sh is no longer used." >&2
echo "Events are stored in ClickHouse with 3-day TTL MOVE to local cold volume." >&2
echo "See examples/command.md (ClickHouse section)." >&2
exit 0
