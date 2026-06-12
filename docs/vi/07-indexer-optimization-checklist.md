# 07 — Checklist tối ưu Indexer (Official Sui Docs)

**Nguồn:** [Optimize Runtime and Performance](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-runtime-perf), [BYOS](https://docs.sui.io/develop/accessing-data/custom-indexer/bring-your-own-store), [Pipeline Architecture](https://docs.sui.io/develop/accessing-data/custom-indexer/pipeline-architecture), [Integrate Data Sources](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-data-integration)

**Trạng thái:** Đã chốt cho triển khai greenfield `crates/`.

> **English:** [../07-indexer-optimization-checklist.md](../07-indexer-optimization-checklist.md)

---

## Checklist — quyết định kiến trúc

| # | Tối ưu | Quyết định doc | Áp dụng tại |
|---|--------|----------------|-------------|
| 1 | Kafka BYOS là target `commit()` **chính** | ✅ Đã chốt | [02](./02-system-architecture.md) §4.4 |
| 2 | Manual `Indexer` (không `IndexerCluster`) cho BYOS | ✅ Đã chốt | [02](./02-system-architecture.md) §4.1 |
| 3 | `CompositeStore`: Kafka fact + Postgres watermark | ✅ Đã chốt | `crates/indexer-store` |
| 4 | GCS backfill `gs://mysten-mainnet-checkpoints-use4` | ✅ Đã chốt | [02](./02-system-architecture.md) §3 |
| 5 | gRPC streaming steady state | ✅ Đã chốt | [02](./02-system-architecture.md) §3 |
| 6 | HTTPS remote store = fallback (30 ngày) | ✅ Đã chốt | [06](./06-reference-examples.md) |
| 7 | Nhiều pipeline (`dex_swap`, `dex_pool`, `token_metadata`) | ✅ Đã chốt | [02](./02-system-architecture.md) §4.2 |
| 8 | Sequential pipeline trước | ✅ Đã chốt | [02](./02-system-architecture.md) §4.1 |
| 9 | Không dual-write Postgres → Kafka | ✅ Đã chốt | [06](./06-reference-examples.md) |
| 10 | Không staging `package_events` trong prod | ✅ Đã chốt | [02](./02-system-architecture.md) §6 |
| 11 | Metric dẫn xuất ngoài indexer (Phase 2) | ✅ Đã chốt | [02](./02-system-architecture.md) §5 |
| 12 | ClickHouse cold qua roll-off hoặc BYOS official | ✅ Đã chốt | [02](./02-system-architecture.md) §6 |

---

## Checklist — tuning runtime (implement trong `crates/indexer`)

| # | Tham số | Backfill | Steady state | Tham chiếu official |
|---|---------|----------|--------------|---------------------|
| 1 | `ingest_concurrency` | Fixed ~200 hoặc Adaptive max 500 | Adaptive mặc định | Runtime perf § Ingestion |
| 2 | `collect_interval_ms` | 500–1000 | 200–500 | Pipeline Architecture § Sequential |
| 3 | `fanout` (processor) | Tăng adaptive `max` nếu Kafka IO-bound | Adaptive mặc định | Pipeline Architecture |
| 4 | `subscriber_channel_size` | Tăng theo pipeline nếu burst | Mặc định | Pipeline Architecture |
| 5 | `pipeline_depth` | Mặc định hoặc +1 nếu commit chậm | Mặc định | Sequential tuning |
| 6 | `db_connection_pool_size` | Khớp số pipeline | Mặc định 100+ | Runtime perf § Database |
| 7 | `streaming_url` + GCS fallback | Cả hai bắt buộc | Cả hai bắt buộc | Integrate Data Sources |
| 8 | Prometheus `:9184/metrics` | Scrape + cảnh báo lag | Scrape + cảnh báo lag | Runtime perf § Metrics |

---

## Checklist — khi nào nâng sequential → concurrent

Theo framework official — **chỉ khi metric chứng minh cần:**

| Trigger | Hành động |
|---------|-----------|
| Lag sequential > SLA tại tip sau tuning | Benchmark → chuyển `concurrent_pipeline` |
| Cần `Pruner` built-in trên raw store indexer | Concurrent + `Handler::prune()` — hoặc Kafka TTL + ClickHouse (mặc định của ta) |
| Một checkpoint sinh batch row rất lớn | Chunking concurrent |

**Đường prod mặc định:** Kafka retention (7–14d) + ClickHouse cold — **không cần Pruner trên indexer.**

---

## Checklist — cổng Phase 1 (bắt buộc pass)

- [ ] Code production chỉ trong `crates/` (không deploy từ `examples/`)
- [ ] GCS backfill đã cấu hình (không chỉ HTTPS)
- [ ] gRPC streaming + GCS fallback đều đã wire
- [ ] Kafka = BYOS commit chính
- [ ] ≥ 2 pipeline tách biệt với watermark riêng
- [ ] Prometheus đã scrape; cảnh báo watermark lag
- [ ] Lag steady-state < 30s tại mainnet tip

---

## Chưa implement (code)

Mọi mục trên **mới có trên tài liệu**. Scaffold greenfield `crates/` **chưa bắt đầu**.
