# 06 — Ví dụ tham khảo (Không phải Production)

**Cập nhật:** 2026-06-03  
**Quy tắc:** Code trong `examples/` chỉ để học và validate spike. **Production xây lại từ đầu** trong `crates/` + `infra/`.

> **English:** [../06-reference-examples.md](../06-reference-examples.md)

---

## 1. Nội dung trong `examples/`

| Đường dẫn | Minh họa gì | Dùng production? |
|-----------|---------------|------------------|
| `examples/simple-sui-indexer/` | `IndexerCluster` + sequential pipeline + Diesel/Postgres + `move-binding` | ❌ Không — chỉ pattern |
| `examples/reconciliation/` | So sánh DB indexer vs fullnode | ❌ Không — viết lại `tools/reconciliation` nếu cần |
| `examples/rpc-service/` | `suix_queryEvents` qua Postgres | ❌ Không — prod dùng `api-service` REST |
| `examples/scripts/` | Shell helper test local | ❌ Không |

---

## 2. Pattern nên học (ý tưởng, không copy-paste)

| Pattern | Nguồn example | Vị trí production |
|---------|---------------|-------------------|
| Decode event tĩnh `move_contract!` | `examples/simple-sui-indexer/event-bindings/` | `crates/event-bindings/` |
| Lọc event theo prefix | `examples/simple-sui-indexer/src/prefix.rs` | `crates/indexer/src/filter.rs` |
| Hình dạng sequential handler | `examples/simple-sui-indexer/src/handlers.rs` | Nhiều handler trong `crates/indexer/` |
| Logic reconciliation event-key | `examples/reconciliation/` | Công cụ QA Phase 1 |
| BCS hex mẫu cho test | `events.md` + unit test example | `crates/event-bindings/tests/` |

---

## 3. Pattern KHÔNG mang sang production

| Cách làm trong example | Vì sao bỏ | Cách production |
|------------------------|-----------|-----------------|
| Một `EventTypeHandler` monolithic | Official: nhiều pipeline | `dex_swap`, `dex_pool`, `token_metadata` |
| Postgres `package_events` làm store chính | BYOS: Kafka là target `commit()` | Kafka BYOS + watermark Postgres tối thiểu |
| Backfill `https://checkpoints.mainnet.sui.io` | Chỉ giữ 30 ngày | GCS `gs://mysten-mainnet-checkpoints-use4` |
| Dual-write Postgres rồi Kafka | Latency, drift | Kafka chính qua BYOS Store |
| `suix_queryEvents` làm API sản phẩm | Analytics cần REST/OHLC | Chỉ `api-service` |
| `IndexerCluster` + ORM Postgres đầy đủ | BYOS Kafka cần manual `Indexer` | Manual `Indexer` + `CompositeStore` |

---

## 4. Tham chiếu official (ưu tiên hơn examples)

| Nhu cầu | Đọc trước |
|---------|-----------|
| Khung indexer | [Build a Custom Indexer](https://docs.sui.io/develop/accessing-data/custom-indexer/build) |
| Store không phải Postgres (Kafka) | [BYOS](https://docs.sui.io/develop/accessing-data/custom-indexer/bring-your-own-store) |
| ClickHouse cold | [clickhouse-sui-indexer](https://github.com/MystenLabs/sui/tree/main/examples/rust/clickhouse-sui-indexer) |
| Tuning hiệu năng | [Optimize Runtime and Performance](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-runtime-perf) |
| gRPC + GCS ingestion | [Integrate Data Sources](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-data-integration) |

---

## 5. Contract tham chiếu (dùng trong production)

Interface Move DEX cho codegen `event-bindings` nằm trong `docs/contracts/` — **được dùng** trong production:

- `docs/contracts/cetus-contracts-main/`
- `docs/contracts/turbos-sui-move-interface-main/`
- `docs/contracts/bluefin-spot-contract-interface-main/`
- `docs/contracts/mmt-contract-interface-mainnet/`
- `docs/contracts/flowx-clmm-contracts-main/`
- `docs/contracts/magma_core_clmm_interface-main/`
