#!/usr/bin/env bash
# Apply storage policy + package_events schema to a running ClickHouse instance.
set -euo pipefail
cd "$(dirname "$0")"

if [[ -f .env ]]; then
  set -a
  # shellcheck disable=SC1091
  source .env
  set +a
fi

CLICKHOUSE_URL="${CLICKHOUSE_URL:-http://127.0.0.1:8123}"
SCHEMA_FILE="${SCHEMA_FILE:-init/01_schema.sql}"

if [[ -z "${CLICKHOUSE_USER:-}" || -z "${CLICKHOUSE_PASSWORD:-}" ]]; then
  echo "ERROR: set CLICKHOUSE_USER and CLICKHOUSE_PASSWORD in clickhouse/.env" >&2
  echo "  cp .env.example .env   # then edit password" >&2
  exit 1
fi

CURL_AUTH=(--user "${CLICKHOUSE_USER}:${CLICKHOUSE_PASSWORD}")

run_sql() {
  local sql="$1"
  local label="$2"
  local response
  response="$(curl -sS "${CURL_AUTH[@]}" "${CLICKHOUSE_URL}/" --data-binary "${sql}" 2>&1)" || {
    echo "ERROR: ${label} failed (curl)" >&2
    exit 1
  }
  if [[ "${response}" == Code:* ]]; then
    echo "ERROR: ${label} failed:" >&2
    echo "${response}" >&2
    exit 1
  fi
}

echo "CLICKHOUSE_URL=${CLICKHOUSE_URL}"
echo "Applying ${SCHEMA_FILE} ..."

if ! curl -sf "${CURL_AUTH[@]}" "${CLICKHOUSE_URL}/ping" >/dev/null; then
  echo "ERROR: ClickHouse not reachable at ${CLICKHOUSE_URL}" >&2
  echo "Start it first: cd clickhouse && docker compose up -d" >&2
  exit 1
fi

# HTTP interface does not allow multi-statement queries — run one statement per request.
run_sql "CREATE DATABASE IF NOT EXISTS sui_indexer;" "CREATE DATABASE"

CREATE_TABLE_SQL="$(awk 'BEGIN{p=0} /^CREATE TABLE/{p=1} p{print}' "${SCHEMA_FILE}")"
run_sql "${CREATE_TABLE_SQL}" "CREATE TABLE package_events"

echo "Schema applied."
count="$(curl -sf "${CURL_AUTH[@]}" "${CLICKHOUSE_URL}/?database=sui_indexer" --data "SELECT count() FROM package_events" || echo "?")"
echo "package_events row count: ${count}"
