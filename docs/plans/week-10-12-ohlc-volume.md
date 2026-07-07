# Week 10–12 Runbook — Token USD OHLC + Volume (TimescaleDB + Redis)

**Scope:** `volume-engine` consuming `dex.swap.normalized.v1` — writes `swaps_fact`, `token_ohlc_usd_*` (all intervals), Redis cache, and `pool_liquidity`.

Pool `ohlc_*` and `ohlc-aggregator` were removed (2026-07-05). Charts are TOKEN/USD only.

---

## 0. Pattern decisions

| Decision | Source | Applied |
|----------|--------|---------|
| OHLC/volume outside indexer | [docs/02-system-architecture.md](../02-system-architecture.md) §5 | `volume-engine` Kafka consumer group |
| Input topic | [docs/04-data-contracts.md](../04-data-contracts.md) §3 | Only `dex.swap.normalized.v1` |
| Idempotency | §3 | `swaps_fact` PK; OHLC upsert on conflict per `(bucket, base_coin_type)` |
| Event-first indexing | Architecture §4.6 | No checkpoint re-parse for volume/OHLC |
| Storage split | §8 / §10 | `TIMESCALE_URL` for metrics; `DATABASE_URL` for catalog only |

### Anti-patterns

- Do **not** add OHLC/volume writes inside `crates/indexer` pipelines
- Do **not** run catalog migrations against TimescaleDB
- Every skip path increments `processor_volume_skipped_total`

---

## 1. Startup order

```bash
docker compose -f infra/docker-compose.yml --env-file .env up -d
bash infra/kafka/create-topics.sh

cargo run -p sui-processors --bin catalog-worker
cargo run -p sui-processors --bin swap-normalizer
cargo run -p sui-processors --bin volume-engine
```

**Backfill:** `KAFKA_AUTO_OFFSET_RESET=earliest`. Start normalizer before volume-engine.

---

## 2. Env vars

| Variable | Default | Purpose |
|----------|---------|---------|
| `TIMESCALE_URL` | — | TimescaleDB (port 5433 local) |
| `REDIS_URL` | `redis://localhost:6379` | Hot cache |
| `VOLUME_ENGINE_CONSUMER_GROUP` | `volume-engine` | Kafka group |
| `VOLUME_ENGINE_WORKERS` | `6` | Parallel consumers |
| `VOLUME_METRICS_ADDRESS` | `0.0.0.0:9186` | volume-engine metrics |

---

## 3. Verify TimescaleDB

```bash
psql "$TIMESCALE_URL" -c "SELECT count(*) FROM swaps_fact;"
psql "$TIMESCALE_URL" -c "SELECT * FROM token_ohlc_usd_1m ORDER BY bucket DESC LIMIT 5;"
psql "$TIMESCALE_URL" -c "SELECT * FROM pool_liquidity ORDER BY time DESC LIMIT 5;"
```

---

## 4. Metrics

```bash
curl -s localhost:9186/metrics | grep processor_token_ohlc_usd_upserts
curl -s localhost:9186/metrics | grep processor_swaps_fact_inserted
```

| Metric | Labels | Meaning |
|--------|--------|---------|
| `processor_token_ohlc_usd_upserts_total` | `interval` | Token USD OHLC buckets upserted |
| `processor_swaps_fact_inserted_total` | — | New swap rows |
| `processor_volume_skipped_total` | `reason` | Skipped swaps |

---

## 5. API

```bash
curl -s 'localhost:8081/v1/tokens/0x2::sui::SUI/ohlc?interval=1h&from=2026-06-01T00:00:00Z&to=2026-06-21T00:00:00Z' | jq
```

Cold path (> `HOT_STORAGE_DAYS`): ClickHouse `token_ohlc_usd_{interval}` via rolloff-job.
