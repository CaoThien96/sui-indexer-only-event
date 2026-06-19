# 07 — Indexer Optimization Checklist (Official Sui Docs)

**Source:** [Optimize Runtime and Performance](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-runtime-perf), [BYOS](https://docs.sui.io/develop/accessing-data/custom-indexer/bring-your-own-store), [Pipeline Architecture](https://docs.sui.io/develop/accessing-data/custom-indexer/pipeline-architecture), [Integrate Data Sources](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-data-integration)

**Status:** Frozen for greenfield `crates/` implementation.

---

## Checklist — architecture decisions

| # | Optimization | Doc decision | Where applied |
|---|--------------|--------------|---------------|
| 1 | Kafka BYOS as **primary** `commit()` target | ✅ Frozen | [02-system-architecture.md](./02-system-architecture.md) §4.4 |
| 2 | Manual `Indexer` (not `IndexerCluster`) for BYOS | ✅ Frozen | [02](./02-system-architecture.md) §4.1 |
| 3 | `CompositeStore`: Kafka facts + Postgres watermarks only | ✅ Frozen | [02](./02-system-architecture.md) §4.1, `crates/indexer-store` |
| 4 | GCS backfill `gs://mysten-mainnet-checkpoints-use4` | ⏸ Phase 2+ | Deferred — HTTPS-only for Phase 1 (≤30d) |
| 5 | gRPC streaming steady state | ✅ Frozen | [02](./02-system-architecture.md) §3 |
| 6 | HTTPS remote store = fallback only (30d) | ✅ Frozen | [06-reference-examples.md](./06-reference-examples.md) |
| 7 | Multiple pipelines (`dex_swap`, `dex_pool`, `token_metadata`) | ✅ Frozen | [02](./02-system-architecture.md) §4.2 |
| 8 | Sequential pipeline first | ✅ Frozen | [02](./02-system-architecture.md) §4.1 |
| 9 | No dual-write Postgres → Kafka | ✅ Frozen | [06](./06-reference-examples.md) |
| 10 | No `package_events` staging in prod | ✅ Frozen | [02](./02-system-architecture.md) §6, [04](./04-data-contracts.md) §7 |
| 11 | Derived metrics outside indexer (Phase 2 processors) | ✅ Frozen | [02](./02-system-architecture.md) §5 |
| 12 | ClickHouse cold via roll-off or official BYOS pattern | ✅ Frozen | [02](./02-system-architecture.md) §6 |

---

## Checklist — runtime tuning (implement in `crates/indexer`)

| # | Knob | Backfill | Steady state | Official ref |
|---|------|----------|--------------|--------------|
| 1 | `ingest_concurrency` | Fixed ~200 or Adaptive max 500 | Adaptive default | Runtime perf § Ingestion |
| 2 | `collect_interval_ms` | 500–1000 | 200–500 | Pipeline Architecture § Sequential tuning |
| 3 | `fanout` (processor) | Raise adaptive `max` if Kafka IO-bound | Default adaptive | Pipeline Architecture |
| 4 | `subscriber_channel_size` | Raise per pipeline if burst | Default | Pipeline Architecture |
| 5 | `pipeline_depth` | Default or +1 for slow commit | Default | Sequential tuning |
| 6 | `db_connection_pool_size` | Match pipeline count | Default 100+ | Runtime perf § Database |
| 7 | `streaming_url` + HTTPS remote store | Both wired | GCS optional later | Integrate Data Sources |
| 8 | Prometheus `:9184/metrics` | Scrape + alert lag | Scrape + alert lag | Runtime perf § Metrics |

---

## Checklist — when to upgrade sequential → concurrent

Per official decision framework — **only if metrics prove need:**

| Trigger | Action |
|---------|--------|
| Sequential lag > SLA at tip after tuning | Benchmark → switch to `concurrent_pipeline` |
| Need built-in `Pruner` on indexer raw store | Concurrent + `Handler::prune()` — or keep Kafka TTL + ClickHouse (our default) |
| Single checkpoint produces very large row batches | Concurrent chunking |

**Default prod path:** Kafka retention (7–14d) + ClickHouse cold — **no indexer Pruner required.**

---

## Checklist — Phase 1 gate (must pass)

- [x] Production code in `crates/` only (zero deploy from `examples/`)
- [ ] GCS backfill — **deferred** (HTTPS + gRPC accepted; >30d history when budget allows)
- [x] gRPC streaming + HTTPS remote store wired
- [x] Kafka = BYOS primary commit
- [x] ≥ 2 separate pipelines with separate watermarks (3: dex_swap, dex_pool, token_metadata)
- [x] Prometheus scraped; watermark lag alert configured (`infra/prometheus/alerts.yml`)
- [ ] Steady-state lag < 30s at mainnet tip (verify at runtime)

**Runbook:** [plans/week-07-hardening.md](./plans/week-07-hardening.md)

---

## Implemented (Week 7)

Runtime tuning: `crates/indexer/src/runtime_tuning.rs` — env-driven `INDEXER_RUNTIME_MODE`, collect interval, ingest concurrency, DB pool.
