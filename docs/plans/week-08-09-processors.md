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

### Anti-patterns (token_metadata lessons)

- Do **not** use `to_canonical_string(true)` prefix matching for coin types
- Do **not** add catalog/OHLC writes inside `crates/indexer` pipelines
- Every skip path must increment a Prometheus counter

---

## 1. Startup order

```bash
# 1. Infra
docker compose -f infra/docker-compose.yml --env-file .env up -d

# 2. Topics (includes dex.swap.normalized.v1)
bash infra/kafka/create-topics.sh

# 3. Indexer (Phase 1 — produces raw topics)
cargo run -p sui-token-indexer

# 4. Catalog first (pools + tokens → Postgres)
cargo run -p sui-processors --bin catalog-worker

# 5. Swap normalizer (needs pools in PG for coin types)
cargo run -p sui-processors --bin swap-normalizer
```

**Backfill:** set `KAFKA_AUTO_OFFSET_RESET=earliest` in `.env`. Start catalog-worker before swap-normalizer.

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
| `PROCESSOR_METRICS_ADDRESS` | `0.0.0.0:9185` | Prometheus scrape |

---

## 3. Verify catalog

```bash
PGPASSWORD='...' psql -h localhost -U postgres -d sui_indexer -c "
SELECT COUNT(*) AS protocols FROM protocols;
SELECT COUNT(*) AS tokens FROM tokens;
SELECT COUNT(*) AS pools FROM pools;
SELECT COUNT(*) AS watchlist FROM token_watchlist;
"
```

```bash
curl -s localhost:9185/metrics | grep processor_catalog
```

---

## 4. Verify normalized swaps

```bash
curl -s localhost:9185/metrics | grep -E 'processor_swap|processor_kafka'

docker compose -f infra/docker-compose.yml exec -T kafka \
  /opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic dex.swap.normalized.v1 \
  --from-beginning --max-messages 3
```

---

## 5. Phase 2 gate checklist

- [ ] `protocols` seeded (6 DEX rows)
- [ ] `tokens` populated from `token.metadata.raw.v1`
- [ ] `pools` populated from `dex.pool.raw.v1`
- [ ] `token_watchlist` auto-seeded on pool discovery
- [ ] `dex.swap.normalized.v1` messages match [docs/04-data-contracts.md](../04-data-contracts.md) §3
- [ ] Replay from `earliest` does not duplicate catalog rows
- [ ] `processor_swap_missing_pool_total` near zero after catalog catches up
