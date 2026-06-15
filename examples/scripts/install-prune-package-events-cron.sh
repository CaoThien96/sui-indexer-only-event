#!/usr/bin/env bash
# DEPRECATED: package_events moved to ClickHouse TTL tier.
set -euo pipefail
echo "DEPRECATED: install-prune-package-events-cron.sh is no longer used." >&2
echo "ClickHouse TTL MOVE handles hot/cold retention (see command.md)." >&2
exit 0
