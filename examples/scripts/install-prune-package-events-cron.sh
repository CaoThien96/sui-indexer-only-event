#!/usr/bin/env bash
# Install daily cron to prune package_events (default 23:55, keep 3 days).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PRUNE_SCRIPT="$ROOT/scripts/prune-package-events.sh"
LOG_FILE="${PRUNE_LOG_FILE:-$ROOT/simple-sui-indexer/prune-package-events.log}"
CRON_SCHEDULE="${CRON_SCHEDULE:-55 23 * * *}"
RETENTION_DAYS="${RETENTION_DAYS:-3}"
MARKER="# sui-indexer-prune-package-events"

if [[ ! -x "$PRUNE_SCRIPT" ]]; then
  chmod +x "$PRUNE_SCRIPT"
fi

CRON_CMD="$CRON_SCHEDULE cd $ROOT && RETENTION_DAYS=$RETENTION_DAYS $PRUNE_SCRIPT >> $LOG_FILE 2>&1 $MARKER"

EXISTING="$(crontab -l 2>/dev/null || true)"
if echo "$EXISTING" | grep -Fq "$MARKER"; then
  echo "Cron entry already installed ($MARKER)"
  echo "$EXISTING" | grep -F "$MARKER" || true
  exit 0
fi

{
  echo "$EXISTING" | sed '/^$/d'
  echo "$CRON_CMD"
} | crontab -

echo "Installed cron:"
echo "  $CRON_CMD"
echo "Log: $LOG_FILE"
echo ""
echo "Run now: $PRUNE_SCRIPT"
echo "Dry run: $PRUNE_SCRIPT --dry-run"
