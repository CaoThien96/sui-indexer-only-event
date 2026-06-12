# 01 — Phạm vi sản phẩm (Đã chốt)

**Sản phẩm:** Nền tảng phân tích DEX theo token trên Sui  
**Cập nhật:** 2026-06-03  
**Triển khai:** Greenfield — `examples/` chỉ để tham khảo ([06-reference-examples.md](./06-reference-examples.md))

> **English:** [../01-product-scope.md](../01-product-scope.md)

---

## 1. Tầm nhìn

Hệ thống trả lời, cho mỗi token trên Sui:

- Token là gì? (metadata)
- Giao dịch ở đâu? (pool trên các DEX)
- Mức độ hoạt động? (volume, số giao dịch, giá, thanh khoản)
- Ai đang nắm giữ? (holders — Phase 4)
- Ví liên quan thế nào? (bubble map — Phase 5)

---

## 2. Phạm vi theo phase

### Phase 0 — Lập kế hoạch & spike tham khảo ✅

| Hạng mục | Trạng thái |
|----------|------------|
| Tài liệu sản phẩm / kiến trúc / timeline | ✅ |
| Contract DEX trong `docs/contracts/` | ✅ |
| Code spike trong `examples/` | ✅ Chỉ tham khảo |
| Code production | ⬜ Chưa bắt đầu |

**Tiêu chí hoàn thành:** Tài liệu đã chốt; team thống nhất xây greenfield trong `crates/`.

---

### Phase 1 — Lớp thu thập dữ liệu (greenfield)

**Mục tiêu:** Indexer production → Kafka (fact bus). Không UI. Không aggregation trong hot path.

| ID | Tính năng | Ưu tiên |
|----|-----------|---------|
| P1-1 | `crates/indexer` — manual `Indexer` + BYOS Kafka store | P0 |
| P1-2 | GCS backfill + gRPC streaming (nguồn dữ liệu official) | P0 |
| P1-3 | Nhiều pipeline: `dex_swap`, `dex_pool` | P0 |
| P1-4 | `crates/event-bindings` — Cetus, Turbos, Bluefin, MMT, FlowX, Magma | P0 |
| P1-5 | Kafka topics + schema ([04-data-contracts.md](./04-data-contracts.md)) | P0 |
| P1-6 | Postgres chỉ lưu watermarks (không staging toàn bộ event) | P0 |
| P1-7 | Prometheus metrics + cảnh báo watermark lag | P1 |
| P1-8 | Pipeline `token_metadata` | P1 |
| P1-9 | `tools/reconciliation` (QA tùy chọn) | P2 |

**Ngoài phạm vi Phase 1:** OHLC, API, `suix_queryEvents`, mở rộng `examples/`.

**Tiêu chí hoàn thành:**
- Lag steady-state < 30s tại chain tip
- ≥ 4 giao thức DEX trên Kafka
- Replay từ offset tái tạo đúng số lượng fact

---

### Phase 2 — Lớp xử lý & lưu trữ

#### Catalog
| ID | Tính năng |
|----|-----------|
| P2-1 | Hồ sơ token tĩnh (name, symbol, decimals, image, …) |
| P2-2 | Registry pool + chỉ mục token ↔ pool |
| P2-3 | Watchlist token tự động từ pool |

#### Chỉ số real-time
| ID | Tính năng |
|----|-----------|
| P2-4 | Volume 1h / 6h / 24h |
| P2-5 | Số giao dịch 1h / 6h / 24h |
| P2-6 | OHLC 1m / 5m / 1h / 4h / 24h |
| P2-7 | Giá mới nhất, lịch sử swap |
| P2-8 | Snapshot TVL pool |

#### Storage
| ID | Tính năng |
|----|-----------|
| P2-9 | TimescaleDB hot (< 30 ngày) |
| P2-10 | ClickHouse cold (> 30 ngày) |
| P2-11 | Redis hot cache |
| P2-12 | `crates/api-service` REST |

**MVP = hoàn thành Phase 2.**

---

### Phase 3 — Thanh khoản nâng cao

Snapshot object pool, độ chính xác TVL, độ sâu tick CLMM (batch).

---

### Phase 4 — Holders

Pipeline `coin_balance` → số holder + top holders.

---

### Phase 5 — Bubble map

Đồ thị chuyển token → subgraph API + cache layout.

---

## 3. Ngoài phạm vi (mọi phase)

| Hạng mục | Lý do |
|----------|-------|
| Deploy `examples/` lên production | Chỉ là spike tham khảo |
| `suix_queryEvents` làm API sản phẩm | Dùng REST analytics API |
| Full state diff toàn mạng | Chi phí cao / không cần |
| Trading / execution | Chỉ đọc dữ liệu phân tích |

---

## 4. Giao thức DEX hỗ trợ

Xem [05-dex-coverage.md](./05-dex-coverage.md).

---

## 5. Quy tắc sản phẩm (đã chốt)

- **Swap → giá & volume.** **Coin effects → holders & bubble map.**
- OHLC keyed theo `(pool_id, base_coin_type, quote_coin_type, interval)`.
- Quote mặc định: SUI + USDC.
- Idempotency: `(tx_digest, event_seq, protocol)`.

---

## 6. Yêu cầu phi chức năng

| Yêu cầu | Mục tiêu |
|---------|----------|
| Indexer lag steady-state | < 30s |
| API p95 (token detail có cache) | < 200ms |
| Kafka retention | ≥ 7 ngày |
| Replay | Rebuild aggregator từ Kafka hoặc ClickHouse |
