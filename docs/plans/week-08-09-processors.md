# Week 8–9 Runbook — Processors (Normalizer + Catalog)

**Scope:** `crates/processors` — independent Kafka consumers for catalog + swap normalization.

---

## 0. Official pattern decisions (MCP + docs)

| Decision | Source | Applied |
|----------|--------|---------|
| Derived metrics outside indexer | [docs/02-system-architecture.md](../02-system-architecture.md) §1 #6, BYOS docs | Processors are **not** `sui-indexer-alt` pipelines; separate binaries with Kafka consumer groups |
| Catalog from Kafka facts | [docs/04-data-contracts.md](../04-data-contracts.md) | No checkpoint re-parse in processors |
| CoinMetadata fields only | Sui Coin Standard, GraphQL `CoinMetadata` | Indexer filters `CoinMetadata<T>`; catalog-worker upserts from `token.metadata.raw.v1` |
| Coin type PK format | GraphQL examples (`0x2::sui::SUI`) | `processors::coin_type::normalize()` — short hex addresses |
| Object creation detection | [Walrus custom indexer](https://docs.sui.io/guides/developer/advanced/custom-indexer) | Fixed in indexer (`checkpoint_output_objects`); processors consume pre-indexed Kafka |
| Lazy pool hydration | swap-normalizer gRPC | Missing pools hydrated at runtime; catalog-worker still preferred for pool-create events |

### Anti-patterns (token_metadata lessons)

- Do **not** use `to_canonical_string(true)` prefix matching for coin types
- Do **not** add catalog/OHLC writes inside `crates/indexer` pipelines
- Every skip path must increment a Prometheus counter

---

## 1. Startup order

**Greenfield from `FIRST_CHECKPOINT` (with oracle bootstrap):**

```bash
# 1. Infra + topics
docker compose -f infra/docker-compose.yml --env-file .env up -d
bash infra/kafka/create-topics.sh

# 2. Oracle bootstrap (seeds sui_usd_1m before indexer)
cargo run -p sui-processors --bin oracle-bootstrap

# 3. Processors
cargo run -p sui-processors --bin catalog-worker
cargo run -p sui-processors --bin swap-normalizer   # lazy hydration if pool missing

# 4. Indexer (waits for bootstrap_state READY when ORACLE_BOOTSTRAP_GATE=true)
cargo run -p sui-token-indexer
```

**Legacy order (no oracle gate):**

```bash
docker compose -f infra/docker-compose.yml --env-file .env up -d
bash infra/kafka/create-topics.sh
cargo run -p sui-token-indexer
cargo run -p sui-processors --bin catalog-worker
cargo run -p sui-processors --bin swap-normalizer
```

**Backfill:** set `KAFKA_AUTO_OFFSET_RESET=earliest` in `.env`. Start catalog-worker before swap-normalizer when replaying historical pool-create events; swap-normalizer can hydrate pools on demand when events were never indexed.

**Docker Compose (optional):** processor services use `KAFKA_BROKERS=kafka:29092` and `DATABASE_URL=...@postgres:5432/...` overrides. Run on host with `cargo run` uses `.env` defaults (`localhost:9092`).

```bash
docker compose -f infra/docker-compose.yml --env-file .env up -d catalog-worker swap-normalizer
```

---

## 2. Env vars

| Variable | Default | Purpose |
|----------|---------|---------|
| `DATABASE_URL` | — | Same Postgres as indexer (URL-encode `@` in password as `%40`) |
| `KAFKA_BROKERS` | `localhost:9092` | Kafka bootstrap |
| `KAFKA_AUTO_OFFSET_RESET` | `earliest` | `latest` for prod steady-state only |
| `CATALOG_CONSUMER_GROUP` | `catalog-worker` | Pool + metadata consumer groups |
| `SWAP_NORMALIZER_CONSUMER_GROUP` | `swap-normalizer` | Raw swap consumer |
| `PROCESSOR_METRICS_ADDRESS` | `0.0.0.0:9185` | Prometheus scrape (catalog-worker / swap-normalizer) |
| `TIMESCALE_URL` | — | `swap-normalizer` reads `sui_usd_1m` for USD enrichment on normalized Kafka output |
| `SUI_GRPC_URL` | `STREAMING_URL` | gRPC fullnode for lazy pool/token hydration |
| `SWAP_HYDRATION_ENABLED` | `true` | Disable to skip gRPC hydration (swaps without pool row are dropped) |
| `SWAP_HYDRATION_RPC_TIMEOUT_MS` | `3000` | gRPC timeout per hydration call |
| `SWAP_HYDRATION_POOL_CACHE_SIZE` | `10000` | LRU pool cache in swap-normalizer |
| `SWAP_HYDRATION_DEFER_MAX_RETRIES` | `5` | In-process retries before leaving Kafka offset uncommitted |
| `SWAP_HYDRATION_DEFER_BACKOFF_MS` | `500` | Exponential backoff base for defer retries |
| `ORACLE_BOOTSTRAP_METRICS_ADDRESS` | `0.0.0.0:9190` | oracle-bootstrap Prometheus endpoint |
| `ORACLE_BOOTSTRAP_METRICS_HOLD_SECS` | `30` | Seconds to keep metrics HTTP open after bootstrap exits |

---

## 3. Verify catalog

```bash
PGPASSWORD='...' psql -h localhost -U postgres -d sui_indexer -c "
SELECT COUNT(*) AS protocols FROM protocols;
SELECT COUNT(*) AS tokens FROM tokens;
SELECT COUNT(*) AS pools FROM pools;
SELECT discovery_source, COUNT(*) FROM pools GROUP BY 1;
SELECT COUNT(*) AS watchlist FROM token_watchlist;
"
```

```bash
curl -s localhost:9185/metrics | grep processor_catalog
```

---

## 4. Verify normalized swaps

```bash
curl -s localhost:9185/metrics | grep -E 'processor_swap|processor_pool_hydrated|processor_token_metadata'

docker compose -f infra/docker-compose.yml exec -T kafka \
  /opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic dex.swap.normalized.v1 \
  --from-beginning --max-messages 3
```

### Prometheus counters (swap-normalizer)

| Metric | Labels | When incremented |
|--------|--------|------------------|
| `processor_swap_normalized_total` | `protocol` | Swap published to normalized topic |
| `processor_swap_skipped_total` | `reason` | Permanent skip (`bad_parse`, `missing_pool_permanent`, `hydration_disabled`) |
| `processor_swap_deferred_total` | `reason` | Transient failure (`pool_rpc`, `metadata_rpc`, `db_error`, `oracle_missing`) |
| `processor_swap_defer_retries_total` | — | Retry attempt for deferred swap |
| `processor_pool_hydrated_total` | `result` | gRPC pool hydration (`ok`, `not_found`, `error`) |
| `processor_token_metadata_hydrated_total` | `result` | gRPC coin metadata fetch |
| `processor_catalog_skipped_total` | `reason` | catalog-worker skip (not swap-normalizer) |

---

## 5. Phase 2 gate checklist

- [ ] `protocols` seeded (6 DEX rows)
- [ ] `tokens` populated from `token.metadata.raw.v1` and/or `rpc_metadata` (hydration)
- [ ] `pools` populated from `dex.pool.raw.v1` (`discovery_source=pool_create`) and/or `swap_hydration`
- [ ] `token_watchlist` auto-seeded on pool discovery / hydration
- [ ] `dex.swap.normalized.v1` includes `price_usd_per_base` / `amount_usd` for SUI/USDC quote pairs when oracle is ready
- [ ] Replay from `earliest` does not duplicate catalog rows
- [ ] `processor_swap_skipped_total{reason="missing_pool_permanent"}` near zero when hydration enabled
- [ ] `processor_pool_hydrated_total{result="ok"}` increases for greenfield `FIRST_CHECKPOINT` starts
- [ ] `oracle_bootstrap_last_run_success` = 1 before indexer start (when gate enabled)

---

## 6. Oracle bootstrap metrics

One-shot job; metrics served on `:9190` until process exits (after `ORACLE_BOOTSTRAP_METRICS_HOLD_SECS`).

```bash
cargo run -p sui-processors --bin oracle-bootstrap
curl -s localhost:9190/metrics | grep oracle_bootstrap
```

| Metric | Meaning |
|--------|---------|
| `oracle_bootstrap_checkpoints_scanned_total` | Checkpoints scanned backward |
| `oracle_bootstrap_swaps_matched_total` | Trusted SUI/USDC swaps seen |
| `oracle_bootstrap_buckets_seeded_total` | `sui_usd_1m` rows written |
| `oracle_bootstrap_last_run_success` | `1` = READY, `0` = FAILED |
