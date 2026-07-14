#!/usr/bin/env bash
# Drop package_events partitions older than RETENTION_DAYS (default 7).
# Default: dry-run. Pass --execute to actually DROP.
#
# Usage:
#   ./scripts/clean-clickhouse-cold.sh              # preview
#   ./scripts/clean-clickhouse-cold.sh --execute    # delete
#   RETENTION_DAYS=15 ./scripts/clean-clickhouse-cold.sh --execute

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RETENTION_DAYS="${RETENTION_DAYS:-7}"
EXECUTE=0
[[ "${1:-}" == "--execute" ]] && EXECUTE=1

# Load credentials
for f in "$ROOT/clickhouse/.env" "$ROOT/simple-sui-indexer/.env"; do
  if [[ -f "$f" ]]; then
    set -a
    # shellcheck disable=SC1090
    source "$f"
    set +a
  fi
done

CLICKHOUSE_URL="${CLICKHOUSE_URL:-http://127.0.0.1:8123}"
DB="${CLICKHOUSE_DATABASE:-sui_indexer}"

curl_ch() {
  local auth=()
  if [[ -n "${CLICKHOUSE_USER:-}" ]]; then
    auth=(-u "${CLICKHOUSE_USER}:${CLICKHOUSE_PASSWORD:-}")
  fi
  curl -sf "${auth[@]}" "${CLICKHOUSE_URL}/?database=${DB}" --data-binary "$1"
}

CUTOFF="$(date -u -v-"${RETENTION_DAYS}"d +%Y%m%d 2>/dev/null \
  || date -u -d "${RETENTION_DAYS} days ago" +%Y%m%d)"

echo "Retention: keep last ${RETENTION_DAYS} days (drop partition < ${CUTOFF})"
echo "Mode: $([ "$EXECUTE" -eq 1 ] && echo EXECUTE || echo DRY-RUN)"
echo

# List cold partitions that would be dropped
PREVIEW="$(curl_ch "
SELECT
  partition,
  formatReadableSize(sum(bytes_on_disk)) AS size,
  sum(rows) AS rows
FROM system.parts
WHERE database = '${DB}'
  AND table = 'package_events'
  AND active
  AND disk_name = 'cold_disk'
  AND partition < '${CUTOFF}'
GROUP BY partition
ORDER BY partition
FORMAT TSV
")"

if [[ -z "${PREVIEW}" ]]; then
  echo "Nothing to drop (cold partitions all >= ${CUTOFF})."
  exit 0
fi

echo "Partitions to drop:"
echo -e "partition\tsize\trows"
echo "$PREVIEW"
echo

TOTAL="$(curl_ch "
SELECT
  count(DISTINCT partition) AS partitions,
  formatReadableSize(sum(bytes_on_disk)) AS size,
  sum(rows) AS rows
FROM system.parts
WHERE database = '${DB}'
  AND table = 'package_events'
  AND active
  AND disk_name = 'cold_disk'
  AND partition < '${CUTOFF}'
FORMAT TSV
")"
echo "Total: ${TOTAL} (partitions / size / rows)"
echo

if [[ "$EXECUTE" -ne 1 ]]; then
  echo "Dry-run only. Re-run with --execute to DROP."
  exit 0
fi

# Drop each partition
while IFS=$'\t' read -r partition _size _rows; do
  [[ -z "$partition" ]] && continue
  echo "DROP PARTITION ${partition} ..."
  curl_ch "ALTER TABLE package_events DROP PARTITION '${partition}'"
done <<< "$PREVIEW"

echo
echo "Done. Current cold usage:"
curl_ch "
SELECT
  disk_name,
  formatReadableSize(sum(bytes_on_disk)) AS size,
  sum(rows) AS rows
FROM system.parts
WHERE database = '${DB}'
  AND table = 'package_events'
  AND active
GROUP BY disk_name
ORDER BY disk_name
FORMAT PrettyCompact
"
echo
