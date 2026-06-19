# Week 1–2 Runbook — Greenfield Indexer

**Parent:** [week-01-02-greenfield-indexer.md](./week-01-02-greenfield-indexer.md)

## Prerequisites

- Rust ≥ 1.85 (`rustup show`)
- Docker + Docker Compose
- macOS: `brew install cmake` (required for `rdkafka` cmake-build)

## 1. Configure environment

```bash
cp .env.example .env
# Edit POSTGRES_* / DATABASE_URL if needed (keep them in sync)
```

## 2. Start local infra

Run from **repo root** so Compose picks up `.env` for Postgres credentials (project name: `sui-indexer-local`):

```bash
docker compose -f infra/docker-compose.yml --env-file .env up -d
docker compose -f infra/docker-compose.yml ps   # postgres + kafka healthy
```

Create Kafka topics (6 partitions, 7d retention):

```bash
chmod +x infra/kafka/create-topics.sh
./infra/kafka/create-topics.sh
```

## 3. Build and run indexer

From repo root (`.env` loaded by `cargo`/indexer via `dotenvy`):

```bash
cargo build --workspace
cargo run -p sui-token-indexer
```

Optional CLI overrides (merged with env):

```bash
cargo run -p sui-token-indexer -- \
  --remote-store-url https://checkpoints.testnet.sui.io \
  --streaming-url https://fullnode.testnet.sui.io:443 \
  --first-checkpoint 100000000
```

## 4. Verify

**Prometheus metrics**

```bash
curl -s localhost:9184/metrics | head
```

**Kafka messages** (`dex.swap.raw.v1` heartbeat envelopes)

```bash
docker compose -f infra/docker-compose.yml exec kafka \
  /opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic dex.swap.raw.v1 \
  --from-beginning \
  --max-messages 5
```

**Watermark**

```bash
psql "$DATABASE_URL" -c \
  "SELECT pipeline, checkpoint_hi_inclusive, reader_lo FROM watermarks WHERE pipeline = 'stub_events';"
```

**Prometheus UI** (optional): http://localhost:9090 — target `host.docker.internal:9184`

## 5. Restart / resume test

1. Note current `checkpoint_hi_inclusive` from `watermarks`.
2. `Ctrl+C` the indexer.
3. `cargo run -p sui-token-indexer` again.
4. Confirm watermark resumes from last committed checkpoint (no regression).

## 6. Testnet soak (≥ 4h)

Run with testnet URLs in `.env`, monitor:

- Logs show checkpoint progress every 100 checkpoints
- `watermarks.checkpoint_hi_inclusive` advances
- Kafka consumer lag near 0
- No OOM / crash

## Common failures

| Symptom | Fix |
|---------|-----|
| `rdkafka` build error on macOS | `brew install cmake`, retry `cargo build` |
| Kafka connection refused | `docker compose ps`, wait for kafka healthy, check `KAFKA_BROKERS=localhost:9092` |
| Migration error | Drop DB volume or `docker compose -f infra/docker-compose.yml down -v` and recreate |
| `DATABASE_URL` missing | Copy `.env.example` → `.env` |
| Postgres auth failed | Ensure `POSTGRES_*` in `.env` matches `DATABASE_URL`; recreate container after password change |
| HTTPS checkpoint fetch fails | Ensure rustls crypto provider installed (done in `main.rs`) |
| No Kafka messages | Run `./infra/kafka/create-topics.sh`; check producer errors metric `indexer_kafka_produce_errors_total` |

## Semantics

- **At-least-once:** Kafka produce succeeds before handler `commit()` returns OK; watermark advances after commit. A crash between Kafka ack and watermark update may replay messages on restart.
