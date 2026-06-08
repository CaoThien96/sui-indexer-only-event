# 01 — Product Scope (Frozen)

**Product:** Sui token-centric DEX analytics platform  
**Last updated:** 2026-06-03  
**Implementation:** Greenfield — `examples/` is reference only ([06-reference-examples.md](./06-reference-examples.md))

---

## 1. Vision

A system that answers, per token on Sui:

- What is it? (metadata)
- Where does it trade? (pools across DEXes)
- How active is it? (volume, tx count, price, liquidity)
- Who holds it? (holders — Phase 4)
- How do wallets relate? (bubble map — Phase 5)

---

## 2. Scope by phase

### Phase 0 — Planning & reference spikes ✅

| Deliverable | Status |
|-------------|--------|
| Product / architecture / timeline docs | ✅ |
| DEX contract interfaces in `docs/contracts/` | ✅ |
| Spike code in `examples/` (indexer, rpc, reconciliation) | ✅ Reference only |
| Production code | ⬜ Not started |

**Exit criteria:** Docs frozen; team agrees greenfield build in `crates/`.

---

### Phase 1 — Data Ingestion Layer (greenfield)

**Goal:** Production indexer → Kafka fact bus. No UI. No aggregations.

| ID | Feature | Priority |
|----|---------|----------|
| P1-1 | `crates/indexer` — manual `Indexer` + BYOS Kafka store | P0 |
| P1-2 | GCS backfill + gRPC streaming (official data sources) | P0 |
| P1-3 | Multiple pipelines: `dex_swap`, `dex_pool` | P0 |
| P1-4 | `crates/event-bindings` — Cetus, Turbos, Bluefin, MMT, FlowX, Magma | P0 |
| P1-5 | Kafka topics + schemas ([04-data-contracts.md](./04-data-contracts.md)) | P0 |
| P1-6 | Postgres watermarks only (not full event staging) | P0 |
| P1-7 | Prometheus metrics + watermark lag alerts | P1 |
| P1-8 | `token_metadata` pipeline | P1 |
| P1-9 | `tools/reconciliation` (optional QA) | P2 |

**Out of scope Phase 1:** OHLC, API, `suix_queryEvents`, extending `examples/`.

**Exit criteria:**
- Steady-state lag < 30s at chain tip
- ≥ 4 DEX protocols on Kafka
- Replay from offset reproduces same fact count

---

### Phase 2 — Processing & Storage Layer

#### Catalog
| ID | Feature |
|----|---------|
| P2-1 | Token static profile (name, symbol, decimals, image, …) |
| P2-2 | Pool registry + token ↔ pool index |
| P2-3 | Token watchlist auto-seed |

#### Real-time metrics
| ID | Feature |
|----|---------|
| P2-4 | Volume 1h / 6h / 24h |
| P2-5 | Tx count 1h / 6h / 24h |
| P2-6 | OHLC 1m / 5m / 1h / 4h / 24h |
| P2-7 | Last price, swap history |
| P2-8 | Pool TVL snapshot |

#### Storage
| ID | Feature |
|----|---------|
| P2-9 | TimescaleDB hot (< 30d) |
| P2-10 | ClickHouse cold (> 30d) |
| P2-11 | Redis hot cache |
| P2-12 | `crates/api-service` REST |

**MVP = Phase 2 complete.**

---

### Phase 3 — Advanced liquidity

Pool object snapshots, TVL accuracy, CLMM tick depth (batch).

---

### Phase 4 — Holders

`coin_balance` pipeline → holder count + top holders.

---

### Phase 5 — Bubble map

Transfer graph → subgraph API + layout cache.

---

## 3. Explicitly out of scope

| Item | Reason |
|------|--------|
| Deploying `examples/` to production | Reference spikes only |
| `suix_queryEvents` as product API | REST analytics API instead |
| Full-network object state diff | Cost / unnecessary |
| Trading / execution | Read-only analytics |

---

## 4. Supported DEX protocols

See [05-dex-coverage.md](./05-dex-coverage.md).

---

## 5. Key product rules (frozen)

- **Swaps → price & volume.** **Coin effects → holders & bubble map.**
- OHLC keyed by `(pool_id, base_coin_type, quote_coin_type, interval)`.
- Default quote: SUI + USDC.
- Idempotency: `(tx_digest, event_seq, protocol)`.

---

## 6. Non-functional requirements

| Requirement | Target |
|-------------|--------|
| Indexer steady-state lag | < 30s |
| API p95 (cached token detail) | < 200ms |
| Kafka retention | ≥ 7d |
| Replay | Rebuild aggregators from Kafka or ClickHouse |
