# Command Cheatsheet — sui-indexer

Quick reference for local dev and ops. Run commands from **repo root** unless noted.

**Project:** `/Users/thiencao/Desktop/sui-indexer`  
**Compose project name:** `sui-indexer-local`

---

## 1. First-time setup

```bash
cp .env.example .env
# Edit .env — keep POSTGRES_* and DATABASE_URL in sync (see §10)
```

**Prerequisites:** Rust ≥ 1.85, Docker, `brew install cmake libpq` (macOS, for `rdkafka` + `psql`).

```bash
cargo build --workspace
```

---

## 2. Start infrastructure (Docker)

Full stack: Postgres, Kafka, TimescaleDB, Redis, ClickHouse, processors, API, Prometheus.

```bash
docker compose -f infra/docker-compose.yml --env-file .env up -d
docker compose -f infra/docker-compose.yml ps
```

**UI / ports**

| Service        | URL / port        |
|----------------|-------------------|
| Kafka UI       | http://localhost:8080 |
| Prometheus     | http://localhost:9090 |
| API (REST)     | http://localhost:8081 |
| Postgres       | localhost:5432    |
| TimescaleDB    | localhost:5433    |
| Redis          | localhost:6379    |
| ClickHouse HTTP| localhost:8123    |

Create Kafka topics (required once, or after `docker compose down -v`):

```bash
chmod +x infra/kafka/create-topics.sh
./infra/kafka/create-topics.sh
```

---

## 3. Recommended dev layout

| Component | Where to run | Why |
|-----------|--------------|-----|
| Infra + processors + API | Docker Compose | Same network (`postgres`, `kafka:29092`, `timescaledb:5432`) |
| **Indexer** (`sui-token-indexer`) | **Host** (`cargo run`) | Heavy backfill; logs in your terminal |

```bash
# Terminal 1 — infra + processors (already up via compose)
docker compose -f infra/docker-compose.yml --env-file .env up -d

# Terminal 2 — indexer on host
RUST_LOG=info cargo run -p sui-token-indexer
```

Processors in Docker use internal hostnames (`postgres`, `kafka:29092`). Host indexer uses `.env` (`localhost:9092`, `localhost:5432`).

---

## 4. Build & run (host)

```bash
# Indexer (ingestion → Kafka)
RUST_LOG=info cargo run -p sui-token-indexer

# Processors (if not using Docker)
RUST_LOG=info cargo run -p sui-processors --bin catalog-worker
RUST_LOG=info cargo run -p sui-processors --bin swap-normalizer
RUST_LOG=info cargo run -p sui-processors --bin volume-engine
RUST_LOG=info cargo run -p sui-processors --bin ohlc-aggregator

# API (if not using Docker)
RUST_LOG=info cargo run -p sui-api-service

# Roll-off (if not using Docker)
RUST_LOG=info cargo run -p sui-processors --bin rolloff-job
```

**Backfill mode** (`.env`):

```bash
# INDEXER_RUNTIME_MODE=backfill   # optional; higher ingest concurrency at catch-up
# FIRST_CHECKPOINT=288461467      # already in .env for mainnet backfill
```

---

## 5. Rebuild Docker images (after code changes)

```bash
# All processor binaries + rolloff-job
docker compose -f infra/docker-compose.yml build catalog-worker swap-normalizer volume-engine ohlc-aggregator rolloff-job

# API only
docker compose -f infra/docker-compose.yml build api-service

# Recreate services
docker compose -f infra/docker-compose.yml up -d catalog-worker swap-normalizer volume-engine ohlc-aggregator api-service rolloff-job
```

---

## 6. Service status & logs

```bash
# Status
docker compose -f infra/docker-compose.yml ps

# Logs (follow)
docker logs -f sui-indexer-local-catalog-worker-1
docker logs -f sui-indexer-local-swap-normalizer-1
docker logs -f sui-indexer-local-volume-engine-1
docker logs -f sui-indexer-local-ohlc-aggregator-1
docker logs -f sui-indexer-local-api-service-1
docker logs -f sui-indexer-local-rolloff-job-1
```

Indexer logs: terminal where `cargo run -p sui-token-indexer` is running.

---

## 7. API smoke tests

```bash
curl -s localhost:8081/health | jq

curl -s 'localhost:8081/v1/tokens/0x2::sui::SUI' | jq

curl -s 'localhost:8081/v1/tokens/0x2::sui::SUI/pools' | jq

curl -s 'localhost:8081/v1/pools/{pool_id}/ohlc?interval=1h&from=2026-06-01T00:00:00Z&to=2026-06-21T00:00:00Z' | jq

curl -s 'localhost:8081/v1/tokens/0x2::sui::SUI/swaps?limit=20' | jq

# Example memecoin (URL path uses catch-all for `::`)
curl -s 'localhost:8081/v1/tokens/0x15a837268acd6d5f1f02784048e129393cff48b9cd55b6b2839cbd60e31faa27::dogtrain::DOGTRAIN' | jq
```

---

## 8. Metrics

```bash
curl -s localhost:9184/metrics | head          # indexer (host)
curl -s localhost:9185/metrics | head          # catalog-worker
curl -s localhost:9186/metrics | head          # volume-engine
curl -s localhost:9187/metrics | head          # ohlc-aggregator
curl -s localhost:9188/metrics | grep api_     # api-service
curl -s localhost:9189/metrics | grep rolloff  # rolloff-job
```

Prometheus UI: http://localhost:9090

---

## 9. Database & cache queries

Use `PGPASSWORD` if password contains `@` (do not rely on broken `DATABASE_URL` in shell):

```bash
export PGPASSWORD='your_password'
psql -h localhost -U postgres -d sui_indexer
```

**Watermarks (indexer progress)**

```sql
SELECT pipeline, checkpoint_hi_inclusive, reader_lo, timestamp_ms_hi_inclusive
FROM watermarks
ORDER BY pipeline;
```

**Catalog**

```sql
SELECT COUNT(*) FROM tokens;
SELECT COUNT(*) FROM pools;
SELECT pool_id, protocol, coin_type_a, coin_type_b FROM pools LIMIT 10;

SELECT * FROM tokens WHERE coin_type ILIKE '%dogtrain%';
SELECT * FROM pools WHERE coin_type_a ILIKE '%dogtrain%' OR coin_type_b ILIKE '%dogtrain%';
```

**Timescale (hot metrics)**

```bash
psql "postgres://postgres:postgres@localhost:5433/sui_metrics" -c \
  "SELECT COUNT(*) FROM swaps_fact;"

psql "postgres://postgres:postgres@localhost:5433/sui_metrics" -c \
  "SELECT base_coin_type, COUNT(*) c FROM swaps_fact GROUP BY 1 ORDER BY c DESC LIMIT 10;"
```

**Redis**

```bash
docker exec sui-indexer-local-redis-1 redis-cli DBSIZE
docker exec sui-indexer-local-redis-1 redis-cli KEYS 'token:*:price' | head
docker exec sui-indexer-local-redis-1 redis-cli GET 'token:0x2::sui::SUI:price'
```

**ClickHouse**

```bash
curl -s 'http://localhost:8123/?query=SELECT+count()+FROM+sui_metrics.swaps_fact'
```

---

## 10. Environment gotchas

| Issue | Cause | Fix |
|-------|--------|-----|
| `password authentication failed` in Docker processors | `POSTGRES_PASSWORD` contains `@`; compose used to break `DATABASE_URL` interpolation | Use current compose + `POSTGRES_HOST=postgres`; prefer `POSTGRES_*` vars (see `postgres_url.rs`) |
| Timescale `error connecting to server` in Docker | `.env` has `TIMESCALE_PORT=5433` (host port); inside Docker network use **5432** | Compose overrides `TIMESCALE_PORT=5432` and internal `TIMESCALE_URL` |
| `psql "$DATABASE_URL"` fails with `@` in password | `@` is a URL delimiter | Use `PGPASSWORD=... psql -h localhost -U postgres -d sui_indexer` or encode `@` as `%40` in URL |
| Processor can't connect from host | Wrong broker URL | Host: `KAFKA_BROKERS=localhost:9092`; Docker: `kafka:29092` |

---

## 11. Kafka debugging

```bash
# List topics
docker compose -f infra/docker-compose.yml exec kafka \
  /opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --list

# Peek pool facts
docker compose -f infra/docker-compose.yml exec kafka \
  /opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic dex.pool.raw.v1 \
  --from-beginning --max-messages 3

# Peek normalized swaps
docker compose -f infra/docker-compose.yml exec kafka \
  /opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic dex.swap.normalized.v1 \
  --from-beginning --max-messages 3
```

---

## 12. Reset databases & messaging (Postgres, Timescale, ClickHouse, Kafka, Redis)

**⚠️ Destructive.** Stops ingestion and wipes local state. Always stop the host indexer first (`Ctrl+C` on `cargo run -p sui-token-indexer`).

Compose does **not** use named volumes — data lives in container filesystems. Removing containers = empty stores on next start.

### 12a. Full reset (recommended)

Wipes **all** local data and consumer offsets. Use after schema bugs, bad backfill, or “start clean from `FIRST_CHECKPOINT`”.

```bash
# 1. Stop host indexer (Ctrl+C)

# 2. Stop and remove all containers
docker compose -f infra/docker-compose.yml --env-file .env down

# 3. (Optional) Force-remove data containers if any linger
docker rm -f sui-indexer-local-postgres-1 \
  sui-indexer-local-timescaledb-1 \
  sui-indexer-local-kafka-1 \
  sui-indexer-local-redis-1 \
  sui-indexer-local-clickhouse-1 2>/dev/null || true

# 4. Start infra only — wait until healthy (~30s)
docker compose -f infra/docker-compose.yml --env-file .env up -d \
  postgres timescaledb kafka redis clickhouse prometheus kafka-ui

docker compose -f infra/docker-compose.yml ps

# 5. Recreate Kafka topics
./infra/kafka/create-topics.sh

# 6. Start processors + API (migrations run on startup)
docker compose -f infra/docker-compose.yml --env-file .env up -d \
  catalog-worker swap-normalizer volume-engine ohlc-aggregator api-service rolloff-job

# 7. Restart indexer from FIRST_CHECKPOINT in .env
RUST_LOG=info cargo run -p sui-token-indexer
```

**Verify empty state:**

```bash
# Postgres catalog — no watermarks until indexer commits
export PGPASSWORD="${POSTGRES_PASSWORD:-postgres}"
psql -h localhost -U postgres -d sui_indexer -c "SELECT COUNT(*) FROM watermarks;"
psql -h localhost -U postgres -d sui_indexer -c "SELECT COUNT(*) FROM pools;"

# Timescale hot store
psql "postgres://postgres:postgres@localhost:5433/sui_metrics" -c "SELECT COUNT(*) FROM swaps_fact;"

# Redis
docker exec sui-indexer-local-redis-1 redis-cli DBSIZE

# ClickHouse cold store
curl -s 'http://localhost:8123/?query=SELECT+count()+FROM+sui_metrics.swaps_fact'

# Kafka — topics exist, no messages yet
docker compose -f infra/docker-compose.yml exec kafka \
  /opt/kafka/bin/kafka-run-class.sh kafka.tools.GetOffsetShell \
  --broker-list localhost:9092 --topic dex.swap.raw.v1
```

---

### 12b. Per-store reset (stack still running)

Use when you only need to clear one layer. Stop the **host indexer** and the **processors** that write to that store first.

```bash
docker compose -f infra/docker-compose.yml stop \
  catalog-worker swap-normalizer volume-engine ohlc-aggregator api-service rolloff-job
```

#### Postgres (catalog — `sui_indexer`)

Watermarks, `tokens`, `pools`, processor state.

```bash
export PGPASSWORD="${POSTGRES_PASSWORD:-postgres}"

psql -h localhost -U postgres -d sui_indexer <<'SQL'
TRUNCATE watermarks, tokens, pools, token_watchlist, protocols RESTART IDENTITY CASCADE;
SQL
```

Or recreate the container (empty DB, migrations re-applied when catalog-worker starts):

```bash
docker compose -f infra/docker-compose.yml stop postgres catalog-worker swap-normalizer
docker rm -f sui-indexer-local-postgres-1
docker compose -f infra/docker-compose.yml --env-file .env up -d postgres
# wait healthy, then restart processors
```

#### TimescaleDB (hot metrics — `sui_metrics`)

`swaps_fact`, `ohlc_*`, roll-off watermarks.

```bash
psql "postgres://postgres:postgres@localhost:5433/sui_metrics" <<'SQL'
DROP SCHEMA public CASCADE;
CREATE SCHEMA public;
GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO public;
SQL
```

Or recreate container:

```bash
docker compose -f infra/docker-compose.yml stop timescaledb volume-engine ohlc-aggregator rolloff-job api-service
docker rm -f sui-indexer-local-timescaledb-1
docker compose -f infra/docker-compose.yml --env-file .env up -d timescaledb
```

Restart `volume-engine` / `ohlc-aggregator` — they run Timescale migrations on boot.

#### ClickHouse (cold storage)

```bash
curl -s 'http://localhost:8123/' --data 'DROP DATABASE IF EXISTS sui_metrics'

# Re-apply DDL (restart api-service or rolloff-job — they run CH migrations on startup)
docker compose -f infra/docker-compose.yml restart api-service rolloff-job
```

Or recreate container:

```bash
docker compose -f infra/docker-compose.yml stop clickhouse api-service rolloff-job
docker rm -f sui-indexer-local-clickhouse-1
docker compose -f infra/docker-compose.yml --env-file .env up -d clickhouse
docker compose -f infra/docker-compose.yml up -d api-service rolloff-job
```

#### Kafka (topics + consumer offsets)

Delete topics and recreate (clears all messages and consumer group progress):

```bash
TOPICS=(dex.swap.raw.v1 dex.pool.raw.v1 token.metadata.raw.v1 dex.swap.normalized.v1)

for topic in "${TOPICS[@]}"; do
  docker compose -f infra/docker-compose.yml exec -T kafka \
    /opt/kafka/bin/kafka-topics.sh \
    --bootstrap-server localhost:9092 \
    --delete --topic "$topic" || true
done

./infra/kafka/create-topics.sh
```

Reset consumer groups without deleting topics (re-read from beginning):

```bash
GROUPS=(catalog-worker swap-normalizer volume-engine ohlc-aggregator)

for group in "${GROUPS[@]}"; do
  docker compose -f infra/docker-compose.yml exec -T kafka \
    /opt/kafka/bin/kafka-consumer-groups.sh \
    --bootstrap-server localhost:9092 \
    --group "$group" \
    --reset-offsets --to-earliest --all-topics --execute
done
```

Requires processors **stopped** before `--execute`. Set `KAFKA_AUTO_OFFSET_RESET=earliest` in `.env` for new consumer groups.

Or recreate Kafka container (nuclear for Kafka only):

```bash
docker compose -f infra/docker-compose.yml stop kafka catalog-worker swap-normalizer volume-engine ohlc-aggregator kafka-ui
docker rm -f sui-indexer-local-kafka-1
docker compose -f infra/docker-compose.yml --env-file .env up -d kafka kafka-ui
./infra/kafka/create-topics.sh
```

#### Redis (price / volume cache)

```bash
docker exec sui-indexer-local-redis-1 redis-cli FLUSHALL
```

Or recreate container:

```bash
docker compose -f infra/docker-compose.yml stop redis volume-engine api-service
docker rm -f sui-indexer-local-redis-1
docker compose -f infra/docker-compose.yml --env-file .env up -d redis
```

---

### 12c. After any reset — restart pipeline

```bash
docker compose -f infra/docker-compose.yml --env-file .env up -d \
  catalog-worker swap-normalizer volume-engine ohlc-aggregator api-service rolloff-job

RUST_LOG=info cargo run -p sui-token-indexer
```

| Store        | What you lose                          | Re-filled by                          |
|--------------|----------------------------------------|---------------------------------------|
| Postgres     | Watermarks, tokens, pools              | Indexer + catalog-worker              |
| TimescaleDB  | swaps_fact, ohlc, roll-off watermarks  | volume-engine, ohlc-aggregator        |
| ClickHouse   | Cold swaps_fact, ohlc mirrors          | rolloff-job (aged data)               |
| Kafka        | All topic messages, consumer offsets   | Indexer + processors (re-consume)     |
| Redis        | token price/vol, pool TVL cache        | volume-engine (on new swaps)          |

---

## 13. Replay / reset watermarks (partial, no full wipe)

Use when fixing pipeline bugs (e.g. Turbos pool coin types) or re-processing a checkpoint range **without** deleting all data.

**⚠️** Rewinding `dex_pool` re-publishes pool facts; reset catalog consumer or use idempotent upserts.

```bash
# 1. Stop host indexer (Ctrl+C)

# 2. Rewind dex_pool watermark (example: before DOGTRAIN pool at 289677385)
export PGPASSWORD="${POSTGRES_PASSWORD:-postgres}"
psql -h localhost -U postgres -d sui_indexer -c \
  "UPDATE watermarks SET checkpoint_hi_inclusive = 289677300 WHERE pipeline = 'dex_pool';"

# 3. Rebuild indexer if code changed, then restart
cargo build -p sui-token-indexer
RUST_LOG=info cargo run -p sui-token-indexer

# 4. Reset catalog-worker Kafka offset to re-consume pool facts (processors stopped first)
docker compose -f infra/docker-compose.yml stop catalog-worker
docker compose -f infra/docker-compose.yml exec -T kafka \
  /opt/kafka/bin/kafka-consumer-groups.sh \
  --bootstrap-server localhost:9092 \
  --group catalog-worker \
  --reset-offsets --to-earliest --topic dex.pool.raw.v1 --execute
docker compose -f infra/docker-compose.yml start catalog-worker
```

**Swap pipeline only** (pool coin-type fix without re-normalizing swaps):

```sql
UPDATE watermarks SET checkpoint_hi_inclusive = 289677300 WHERE pipeline = 'dex_pool';
-- Do NOT reset dex_swap unless you intend to re-process all swaps
```

---

## 14. Stop infra (no data wipe)

```bash
docker compose -f infra/docker-compose.yml down
```

Containers removed; anonymous container data is discarded. Next `up` starts fresh DBs (same as §12a steps 4–7).

---

## 15. Tests

```bash
cargo test -p sui-processors
cargo test -p sui-api-service
cargo test -p indexer-store
cargo test -p event-bindings pool_id
cargo build --workspace
```

---

## 16. Related docs

| Doc | Purpose |
|-----|---------|
| [docs/plans/week-01-02-runbook.md](docs/plans/week-01-02-runbook.md) | Greenfield indexer |
| [docs/plans/week-08-09-processors.md](docs/plans/week-08-09-processors.md) | Catalog + swap-normalizer |
| [docs/plans/week-10-12-ohlc-volume.md](docs/plans/week-10-12-ohlc-volume.md) | Timescale + Redis |
| [docs/plans/week-13-15-api-clickhouse.md](docs/plans/week-13-15-api-clickhouse.md) | API + ClickHouse |
| [docs/04-data-contracts.md](docs/04-data-contracts.md) | Kafka topics, API shapes |
| [infra/kafka/create-topics.sh](infra/kafka/create-topics.sh) | Topic bootstrap script |
