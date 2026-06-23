# Week 13–15 Runbook — API Service + ClickHouse roll-off

**Scope:** `crates/api-service` REST + `rolloff-job` (TimescaleDB → ClickHouse).

---

## 0. Pattern decisions

| Decision | Source | Applied |
|----------|--------|---------|
| REST only | [docs/02-system-architecture.md](../02-system-architecture.md) §7 | Axum; no JSON-RPC |
| Read-only API | Architecture §1 #6 | No aggregation in api-service |
| Hot/cold at 30d | P2-9/P2-10 | Timescale hot; ClickHouse cold |
| Redis mapping | [docs/04-data-contracts.md](../04-data-contracts.md) §10–11 | price/vol keys → API JSON |
| Coin type PK | Week 8–9 | `processors::coin_type::normalize()` |

### Anti-patterns

- Do **not** deploy `examples/rpc-service` to production
- Do **not** use `package_events` ClickHouse schema
- Do **not** add OHLC/volume computation in api-service

---

## 1. Startup order

```bash
docker compose -f infra/docker-compose.yml --env-file .env up -d
# indexer + processors (catalog, normalizer, volume, ohlc) already running

cargo run -p sui-api-service
# rolloff-job runs in compose (daily loop) or:
cargo run -p sui-processors --bin rolloff-job
```

---

## 2. Env vars

| Variable | Default | Purpose |
|----------|---------|---------|
| `DATABASE_URL` | — | Postgres catalog |
| `TIMESCALE_URL` | — | Hot metrics |
| `REDIS_URL` | `redis://localhost:6379` | Price/vol cache |
| `CLICKHOUSE_URL` | `http://localhost:8123` | Cold storage HTTP |
| `CLICKHOUSE_DATABASE` | `sui_metrics` | CH database |
| `API_PORT` | `8081` | REST listen |
| `API_METRICS_ADDRESS` | `0.0.0.0:9188` | Prometheus |
| `HOT_STORAGE_DAYS` | `30` | Hot/cold boundary |
| `ROLLOFF_BATCH_SIZE` | `10000` | Rows per batch |
| `ROLLOFF_INTERVAL_SECS` | `3600` | Sleep between rolloff ticks |

---

## 3. Verify API (curl)

```bash
curl -s localhost:8081/health
curl -s 'localhost:8081/v1/tokens/0x2::sui::SUI' | jq
curl -s 'localhost:8081/v1/tokens/0x2::sui::SUI/pools' | jq
curl -s 'localhost:8081/v1/pools/{pool_id}/ohlc?interval=1h&from=2026-06-01T00:00:00Z&to=2026-06-21T00:00:00Z' | jq
curl -s 'localhost:8081/v1/tokens/0x2::sui::SUI/swaps?limit=20' | jq
curl -s localhost:9188/metrics | grep api_request
```

URL-encode coin types with `::` when needed (`0x2::sui::SUI` works unencoded in most shells).

---

## 4. Verify roll-off

```bash
curl -s localhost:9189/metrics | grep processor_rolloff
clickhouse-client --query "SELECT count() FROM sui_metrics.swaps_fact"
```

With &lt;30d of local data, roll-off is idle until rows age past `HOT_STORAGE_DAYS`.

---

## 5. Hot/cold routing

| Query range | Source |
|-------------|--------|
| `from` and `to` within last 30d | TimescaleDB only |
| `to` older than 30d | ClickHouse only |
| Spans boundary | Both; merge + dedupe by PK |
