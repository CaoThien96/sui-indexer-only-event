# 06 — Reference Examples (Not Production)

**Last updated:** 2026-06-03  
**Rule:** Code under `examples/` is for learning and spike validation only. **Production will be rebuilt from scratch** in `crates/` + `infra/`.

---

## 1. What lives in `examples/`

| Path | What it demonstrates | Use in production? |
|------|----------------------|-------------------|
| `examples/simple-sui-indexer/` | `IndexerCluster` + sequential pipeline + Diesel/Postgres + `move-binding` decode | ❌ No — patterns only |
| `examples/reconciliation/` | Compare indexer DB vs fullnode events | ❌ No — rewrite as `tools/reconciliation` if needed |
| `examples/rpc-service/` | `suix_queryEvents` over Postgres | ❌ No — prod uses `api-service` REST |
| `examples/scripts/` | Shell helpers for local testing | ❌ No |

---

## 2. Patterns worth reusing (ideas, not copy-paste)

| Pattern | Source example | Production location |
|---------|----------------|---------------------|
| `move_contract!` static event decode | `examples/simple-sui-indexer/event-bindings/` | `crates/event-bindings/` (new crate) |
| Prefix-based event filter | `examples/simple-sui-indexer/src/prefix.rs` | `crates/indexer/src/filter.rs` |
| Sequential pipeline handler shape | `examples/simple-sui-indexer/src/handlers.rs` | Multiple handlers in `crates/indexer/` |
| Reconciliation event-key logic | `examples/reconciliation/` | Phase 1 QA tool, not prod path |
| Sample event BCS hex for tests | `events.md` + example unit tests | `crates/event-bindings/tests/` |

---

## 3. Patterns NOT to carry into production

| Example approach | Why skip | Production approach |
|------------------|----------|---------------------|
| Single monolithic `EventTypeHandler` | Official docs: multiple pipelines | `dex_swap`, `dex_pool`, `token_metadata` pipelines |
| Postgres `package_events` as primary store | BYOS: Kafka is primary commit target | Kafka BYOS `commit()` + watermarks in minimal Postgres |
| `https://checkpoints.mainnet.sui.io` backfill | 30-day retention only | GCS `gs://mysten-mainnet-checkpoints-use4` |
| Dual-write Postgres then Kafka | Extra latency, drift risk | Kafka primary via BYOS Store |
| `suix_queryEvents` RPC as product API | Analytics need REST/OHLC | `api-service` only |
| `IndexerCluster` + full Postgres ORM | BYOS for Kafka needs manual `Indexer` | Manual `Indexer` + `CompositeStore` (Kafka + watermark PG) |

---

## 4. Official references (prefer over examples)

| Need | Read this first |
|------|-----------------|
| Indexer skeleton | [Build a Custom Indexer](https://docs.sui.io/develop/accessing-data/custom-indexer/build) |
| Kafka / non-Postgres store | [BYOS](https://docs.sui.io/develop/accessing-data/custom-indexer/bring-your-own-store) |
| ClickHouse cold path | [clickhouse-sui-indexer](https://github.com/MystenLabs/sui/tree/main/examples/rust/clickhouse-sui-indexer) (official Mysten example) |
| Performance tuning | [Optimize Runtime and Performance](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-runtime-perf) |
| gRPC + GCS ingestion | [Integrate Data Sources](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-data-integration) |

---

## 5. Contract references (production dependency)

DEX Move interfaces for `event-bindings` codegen live under `docs/contracts/` — these **are** used in production:

- `docs/contracts/cetus-contracts-main/`
- `docs/contracts/turbos-sui-move-interface-main/`
- `docs/contracts/bluefin-spot-contract-interface-main/`
- `docs/contracts/mmt-contract-interface-mainnet/`
- `docs/contracts/flowx-clmm-contracts-main/`
- `docs/contracts/magma_core_clmm_interface-main/`
