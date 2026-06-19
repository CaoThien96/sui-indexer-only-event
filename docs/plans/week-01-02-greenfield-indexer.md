# Week 1–2 Plan — Greenfield Indexer Skeleton

**Parent:** [03-roadmap-timeline.md](../03-roadmap-timeline.md) Phase 1  
**Dates:** 2026-06-03 → 2026-06-17 (2 weeks)  
**Goal:** Runnable `crates/indexer` on **testnet** with manual `Indexer`, BYOS Kafka commit, Postgres watermarks, and observability.  
**Out of scope this sprint:** Multi-DEX decode, `dex_swap`/`dex_pool` production pipelines (Week 3–4), mainnet GCS backfill (Week 7).

> **Tiếng Việt:** [../vi/plans/week-01-02-greenfield-indexer.md](../vi/plans/week-01-02-greenfield-indexer.md)

---

## 1. Sprint outcomes

At end of Week 2, a developer can:

```bash
cd infra && docker compose up -d          # Kafka + Postgres
cargo run -p sui-token-indexer -- ...     # indexer process
curl localhost:9184/metrics               # Prometheus
kafka-console-consumer ... dex.swap.raw.v1  # messages (from stub pipeline)
psql ... -c "SELECT * FROM watermarks"    # watermark advancing
```

| Deliverable | Crate / path | Done when |
|-------------|--------------|-----------|
| Cargo workspace | `/Cargo.toml` | `cargo build --workspace` passes |
| Local infra | `infra/docker-compose.yml` | Kafka + Postgres healthy |
| Watermark store | `crates/indexer-store` | `Store` + `Connection` traits implemented |
| Kafka BYOS writer | `crates/indexer-store` | Idempotent produce in `commit()` path |
| Indexer binary | `crates/indexer` | Manual `Indexer::new()` runs on testnet |
| Stub pipeline | `crates/indexer` | One sequential pipeline proves end-to-end |
| Event-bindings crate shell | `crates/event-bindings` | Empty crate compiles; wired in Week 3 |
| Metrics | `crates/indexer` | `:9184/metrics` exposes framework metrics |
| Runbook | `docs/plans/week-01-02-runbook.md` | Start/stop/verify commands documented |

---

## 2. Non-goals (explicit)

| Item | Deferred to |
|------|-------------|
| Cetus/Turbos `move_contract!` decode | Week 3–4 |
| `dex_swap` + `dex_pool` split pipelines | Week 3–4 |
| Mainnet + GCS Requester Pays | Week 7 hardening |
| `token_metadata` pipeline | Week 5–6 |
| `tools/reconciliation` | Week 5–6 (optional) |
| Copy-paste from `examples/` | Never — patterns only |

---

## 3. Target repo layout (end of Week 2)

```
sui-indexer/
├── Cargo.toml                          # workspace
├── .env.example                        # indexer env template
├── crates/
│   ├── indexer/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs                 # manual Indexer bootstrap
│   │   │   ├── config.rs               # env + CLI merge
│   │   │   └── pipelines/
│   │   │       └── stub_events.rs      # Week 2 proof pipeline
│   │   └── migrations/                 # watermarks only
│   ├── indexer-store/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── composite.rs            # Kafka + PG facade
│   │       ├── kafka.rs                # producer + topic routing
│   │       └── postgres.rs             # watermark Connection
│   └── event-bindings/
│       ├── Cargo.toml                  # shell only Week 2
│       └── src/lib.rs
├── infra/
│   ├── docker-compose.yml
│   ├── kafka/create-topics.sh
│   └── prometheus/prometheus.yml       # stub scrape config
└── docs/plans/
    └── week-01-02-greenfield-indexer.md
```

---

## 4. Week 1 — Foundation (Days 1–5)

### Day 1 — Workspace + infra

| Task | Details | Verify |
|------|---------|--------|
| Root workspace | `Cargo.toml` members: `indexer`, `indexer-store`, `event-bindings` | `cargo check --workspace` |
| `infra/docker-compose.yml` | Services: `postgres:16`, `kafka` (KRaft single node), optional `kafka-ui` | `docker compose ps` all healthy |
| Create Kafka topics | `dex.swap.raw.v1`, `dex.pool.raw.v1`, `token.metadata.raw.v1` — 6 partitions each, retention 7d | `kafka-topics.sh --list` |
| `.env.example` | Document all required env vars (see §8) | Reviewed |

**Postgres schema (watermarks only):**

```sql
-- crates/indexer/migrations/.../up.sql
-- Mirror sui-pg-db watermarks table shape required by framework Store traits
```

Reference: [sui-pg-db store.rs](https://github.com/MystenLabs/sui/blob/main/crates/sui-pg-db/src/store.rs)

---

### Day 2 — `crates/indexer-store` (Postgres watermarks)

| Task | Details | Verify |
|------|---------|--------|
| Add deps | `sui-indexer-alt-framework-store-traits`, `diesel-async`, `async-trait` | Compiles |
| `PostgresStore` | Implement `Store` trait — `connect()` returns connection | Unit test with testcontainers or docker PG |
| `Connection` trait | `init_watermark`, `accepts_chain_id`, `committer_watermark`, `set_committer_watermark` | Insert + read watermark row |
| Migrations | Diesel migrations for `watermarks` table only | `diesel migration run` |

**Exit:** Watermark round-trip test passes against local Postgres.

---

### Day 3 — `crates/indexer-store` (Kafka producer)

| Task | Details | Verify |
|------|---------|--------|
| Add `rdkafka` | `cmake-build` or `dynamic-linking` per platform | Compiles on macOS |
| `KafkaFactWriter` | Async batch produce with `acks=all`, idempotent producer enabled | Integration test → local Kafka |
| Message envelope | Wrap payload per [04-data-contracts.md](../04-data-contracts.md) §1 | JSON schema matches doc |
| Partition key helper | `pool_id` or `tx_digest` for stub pipeline | Logged in test |

**Exit:** Test publishes 100 messages; consumer receives all; no duplicates on retry.

---

### Day 4 — `CompositeStore` facade

| Task | Details | Verify |
|------|---------|--------|
| `CompositeStore` | Implements `Store`; `commit()` path writes Kafka facts + watermark via PG `Connection` | Design doc in code comment |
| Transaction boundary | Watermark update after successful Kafka ack (or same logical batch id) | Failed Kafka → watermark does not advance |
| Error handling | Propagate errors — no silent swallow | Panic policy matches framework guidance |

**Design note:** Handler `commit()` receives rows; `CompositeStore` connection serializes to Kafka JSON and delegates watermark ops to PG `Connection`.

---

### Day 5 — `crates/indexer` shell + config

| Task | Details | Verify |
|------|---------|--------|
| Binary `sui-token-indexer` | `main.rs` with `Indexer::new()` — **not** `IndexerCluster` | Compiles |
| `ClientArgs` | Testnet: HTTPS remote store fallback + gRPC streaming URL | See §8 env |
| `IngestionConfig` | Defaults; document tuning knobs for Week 7 | Comment refs [07-indexer-optimization-checklist.md](../07-indexer-optimization-checklist.md) |
| Prometheus registry | Pass to `Indexer::new()`; expose `:9184/metrics` | `curl` returns metrics |
| `crates/event-bindings` | Empty `lib.rs` + workspace dep placeholder | `cargo build` |

**Exit:** Indexer starts, connects to testnet checkpoint source, exits cleanly or logs ingestion (pipeline not yet registered).

---

## 5. Week 2 — End-to-end proof (Days 6–10)

### Day 6 — Stub sequential pipeline

Purpose: prove `process()` → `commit()` → Kafka + watermark **before** DEX-specific logic.

| Task | Details | Verify |
|------|---------|--------|
| `StubEventHandler` | `Processor::process()` — filter one known testnet event type OR emit checkpoint heartbeat record | Rows produced |
| Sequential config | `collect_interval_ms: 500` (backfill-friendly default per official docs) | Config in code |
| Handler `commit()` | Write to `dex.swap.raw.v1` via `CompositeStore` | Messages in topic |
| Watermark | Pipeline name: `stub_events` — advances per checkpoint | `watermarks` table updates |

**Stub payload (minimal):**

```json
{
  "schema_version": 1,
  "message_id": "...",
  "produced_at_ms": 1710000000000,
  "payload": {
    "kind": "checkpoint_heartbeat",
    "checkpoint_sequence_number": 12345,
    "event_count": 0
  }
}
```

Replace with real swap events in Week 3.

---

### Day 7 — Testnet soak (4h+)

| Task | Details | Verify |
|------|---------|--------|
| Run indexer against testnet | `--remote-store-url https://checkpoints.testnet.sui.io --streaming-url https://fullnode.testnet.sui.io:443` | Logs show checkpoint progress |
| Monitor lag | Compare `watermarks.checkpoint_hi_inclusive` vs public explorer tip | Lag < 500 cp on testnet |
| Kafka volume | Consumer lag near 0 | No unbounded growth |
| Restart test | Kill process; restart; resumes from watermark | No duplicate watermark regression |

---

### Day 8 — Observability

| Task | Details | Verify |
|------|---------|--------|
| `infra/prometheus/prometheus.yml` | Scrape `host.docker.internal:9184` or indexer host | Prometheus UI shows targets UP |
| Grafana dashboard stub (optional) | Panels: `indexer_total_ingested_checkpoints`, pipeline watermark, commit latency | Visual check |
| Custom metric | `indexer_kafka_produce_errors_total` counter | Increments on forced failure test |
| Log structure | `tracing` with `checkpoint_sequence_number`, `pipeline`, `batch_size` | Grep-friendly |

---

### Day 9 — Hardening + docs

| Task | Details | Verify |
|------|---------|--------|
| `docs/plans/week-01-02-runbook.md` | Start infra, run indexer, verify Kafka, verify watermark, common failures | Peer review |
| CI stub | `cargo fmt --check`, `cargo clippy`, `cargo test` (unit only) | Local pass |
| Fix backpressure smoke test | Slow Kafka → indexer logs throttle (no OOM) | Manual test |

---

### Day 10 — Sprint review + gate

| Gate | Pass? |
|------|-------|
| `cargo build --workspace` clean | ☐ |
| Zero imports from `examples/` crates | ☐ |
| Kafka is primary fact output (no `package_events` table) | ☐ |
| Watermark survives restart | ☐ |
| Prometheus metrics reachable | ☐ |
| Testnet indexer ran ≥ 4h without crash | ☐ |
| Runbook complete | ☐ |

**Demo script for review:**

1. `docker compose up -d`
2. `./infra/kafka/create-topics.sh`
3. `cargo run -p sui-token-indexer -- --first-checkpoint ...` (or env-driven)
4. Show Kafka messages + watermark row + metrics endpoint

---

## 6. Dependency baseline

Pin to Mysten `mainnet` branch (match `examples/` spike):

```toml
# workspace Cargo.toml
[workspace.dependencies]
sui-indexer-alt-framework = { git = "https://github.com/MystenLabs/sui.git", branch = "mainnet" }
sui-indexer-alt-framework-store-traits = { git = "https://github.com/MystenLabs/sui.git", branch = "mainnet" }
sui-indexer-alt-metrics = { git = "https://github.com/MystenLabs/sui.git", branch = "mainnet" }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
async-trait = "0.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
clap = { version = "4", features = ["derive"] }
dotenvy = "0.15"
rdkafka = { version = "0.36", features = ["cmake-build"] }
diesel = { version = "2.3", features = ["postgres", "serde_json"] }
diesel-async = { version = "0.8", features = ["bb8", "postgres"] }
diesel_migrations = "2.3"
prometheus = "0.13"
```

Rust **2024** edition; toolchain ≥ 1.85.

---

## 7. Architecture flow (Week 2 target)

```mermaid
sequenceDiagram
  participant GCS as Testnet checkpoint source
  participant IDX as crates/indexer
  participant H as stub_events Handler
  participant CS as CompositeStore
  participant K as Kafka
  participant PG as Postgres watermarks

  GCS->>IDX: checkpoint stream
  IDX->>H: process(checkpoint)
  H->>CS: commit(batch rows)
  CS->>K: produce dex.swap.raw.v1
  CS->>PG: set_committer_watermark
```

---

## 8. Environment variables

```bash
# .env.example
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=sui_indexer
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
DATABASE_URL=postgres://postgres:postgres@localhost:5432/sui_indexer
KAFKA_BROKERS=localhost:9092
KAFKA_CLIENT_ID=sui-token-indexer
METRICS_ADDRESS=0.0.0.0:9184

# Testnet ingestion (Week 1-2)
REMOTE_STORE_URL=https://checkpoints.testnet.sui.io
STREAMING_URL=https://fullnode.testnet.sui.io:443
# FIRST_CHECKPOINT=optional override for dev

# Week 7+ mainnet (not used this sprint)
# REMOTE_STORE_GCS=mysten-mainnet-checkpoints-use4
# GOOGLE_APPLICATION_CREDENTIALS=...
```

---

## 9. Risks & mitigations

| Risk | Mitigation |
|------|------------|
| BYOS `Store` trait complexity | Study [clickhouse-sui-indexer](https://github.com/MystenLabs/sui/tree/main/examples/rust/clickhouse-sui-indexer) `store.rs` Day 2 |
| `rdkafka` build on macOS | Document `brew install cmake` in runbook |
| Framework API drift on `mainnet` branch | Pin git rev after first successful build |
| Kafka watermark vs PG watermark ordering | Week 2: watermark only after Kafka ack; document at-least-once semantics |
| Scope creep into DEX decode | Stub pipeline only; bindings stay empty until Week 3 |

---

## 10. Handoff to Week 3–4

Week 3 starts when all Week 2 gates pass. Next sprint:

1. Replace `stub_events` with `dex_swap` + `dex_pool` pipelines
2. Implement `crates/event-bindings` (Cetus + Turbos)
3. Real `parsed_json` on `dex.swap.raw.v1` per [04-data-contracts.md](../04-data-contracts.md)

---

## 11. Task checklist (copy to issue tracker)

### Week 1
- [ ] W1-1 Root `Cargo.toml` workspace
- [ ] W1-2 `infra/docker-compose.yml` + topic script
- [ ] W1-3 Postgres watermarks migration
- [ ] W1-4 `PostgresStore` + `Connection` traits
- [ ] W1-5 `KafkaFactWriter` + envelope JSON
- [ ] W1-6 `CompositeStore` facade
- [ ] W1-7 `sui-token-indexer` binary shell + testnet `ClientArgs`
- [ ] W1-8 Prometheus registry wired

### Week 2
- [ ] W2-1 `stub_events` sequential pipeline
- [ ] W2-2 Kafka messages visible on `dex.swap.raw.v1`
- [ ] W2-3 Watermark advances with checkpoints
- [ ] W2-4 Testnet soak ≥ 4h
- [ ] W2-5 Restart/resume test
- [ ] W2-6 Prometheus scrape config
- [ ] W2-7 Runbook written
- [ ] W2-8 Sprint gate demo
