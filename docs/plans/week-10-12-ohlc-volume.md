# Week 10–12 Runbook — OHLC + Volume (TimescaleDB + Redis)

**Scope:** `volume-engine` + `ohlc-aggregator` consuming `dex.swap.normalized.v1`.

---

## 0. Pattern decisions

| Decision | Source | Applied |
|----------|--------|---------|
| OHLC/volume outside indexer | [docs/02-system-architecture.md](../02-system-architecture.md) §5 | Independent Kafka consumer groups in `crates/processors` |
| Input topic | [docs/04-data-contracts.md](../04-data-contracts.md) §3 | Only `dex.swap.normalized.v1` |
| Idempotency | §3 | `swaps_fact` PK `(time, tx_digest, event_seq, protocol)`; OHLC dedupe set per swap key |
| Event-first indexing | Architecture §4.6 | No checkpoint re-parse for volume/OHLC |
| Storage split | §8 / §10 | `TIMESCALE_URL` for metrics; `DATABASE_URL` for catalog only |

### Anti-patterns

- Do **not** add OHLC/volume writes inside `crates/indexer` pipelines
- Do **not** run catalog migrations against TimescaleDB
- Every skip path increments `processor_volume_skipped_total` or `processor_ohlc_skipped_total`

---

## 1. Startup order

```bash
docker compose -f infra/docker-compose.yml --env-file .env up -d
bash infra/kafka/create-topics.sh

cargo run -p sui-token-indexer
cargo run -p sui-processors --bin catalog-worker
cargo run -p sui-processors --bin swap-normalizer
cargo run -p sui-processors --bin volume-engine
cargo run -p sui-processors --bin ohlc-aggregator
```

**Backfill:** `KAFKA_AUTO_OFFSET_RESET=earliest`. Start normalizer before volume/OHLC workers.

---

## 2. Env vars

| Variable | Default | Purpose |
|----------|---------|---------|
| `TIMESCALE_URL` | — | TimescaleDB (port 5433 local) |
| `REDIS_URL` | `redis://localhost:6379` | Hot cache |
| `VOLUME_ENGINE_CONSUMER_GROUP` | `volume-engine` | Kafka group |
| `OHLC_AGGREGATOR_CONSUMER_GROUP` | `ohlc-aggregator` | Kafka group |
| `VOLUME_METRICS_ADDRESS` | `0.0.0.0:9186` | volume-engine metrics |
| `OHLC_METRICS_ADDRESS` | `0.0.0.0:9187` | ohlc-aggregator metrics |

---

## 3. Verify TimescaleDB

```bash
psql "$TIMESCALE_URL" -c "SELECT count(*) FROM swaps_fact;"
psql "$TIMESCALE_URL" -c "SELECT * FROM ohlc_1m ORDER BY bucket DESC LIMIT 5;"
psql "$TIMESCALE_URL" -c "SELECT * FROM pool_liquidity ORDER BY time DESC LIMIT 5;"
psql "$TIMESCALE_URL" -c "SELECT * FROM token_volume_1h ORDER BY bucket DESC LIMIT 5;"
```

---

## 4. Verify Redis

```bash
redis-cli GET 'token:0x2::sui::SUI:price'
redis-cli GET 'token:0x...::token::TKN:vol:24h'
redis-cli GET 'pool:0x...:tvl'
```

---

## 5. Metrics

```bash
curl -s localhost:9186/metrics | grep processor_swaps_fact
curl -s localhost:9187/metrics | grep processor_ohlc
```

---

## 6. TVL estimate (swap_event)

`pool_liquidity.tvl_quote` ≈ `vault_quote_decimal + vault_base_decimal * price_quote_per_base` using 9-decimal default scaling on vault raw amounts. Phase 3 improves via pool snapshots.
