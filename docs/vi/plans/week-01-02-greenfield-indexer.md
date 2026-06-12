# Kế hoạch Tuần 1–2 — Khung Indexer Greenfield

**Thuộc:** [03-roadmap-timeline.md](../../03-roadmap-timeline.md) Phase 1  
**Thời gian:** 2026-06-03 → 2026-06-17 (2 tuần)  
**Mục tiêu:** Chạy được `crates/indexer` trên **testnet** với manual `Indexer`, BYOS Kafka commit, Postgres watermarks, và observability.  
**Ngoài phạm vi sprint:** Decode multi-DEX, pipeline `dex_swap`/`dex_pool` production (Tuần 3–4), GCS mainnet backfill (Tuần 7).

> **English:** [../../plans/week-01-02-greenfield-indexer.md](../../plans/week-01-02-greenfield-indexer.md)

---

## 1. Kết quả sprint

Cuối Tuần 2, developer có thể:

```bash
cd infra && docker compose up -d
cargo run -p sui-token-indexer -- ...
curl localhost:9184/metrics
kafka-console-consumer ... dex.swap.raw.v1
psql ... -c "SELECT * FROM watermarks"
```

| Deliverable | Crate / path | Hoàn thành khi |
|-------------|--------------|----------------|
| Cargo workspace | `/Cargo.toml` | `cargo build --workspace` pass |
| Infra local | `infra/docker-compose.yml` | Kafka + Postgres healthy |
| Watermark store | `crates/indexer-store` | Trait `Store` + `Connection` |
| Kafka BYOS writer | `crates/indexer-store` | Produce idempotent trong `commit()` |
| Binary indexer | `crates/indexer` | Manual `Indexer::new()` chạy testnet |
| Pipeline stub | `crates/indexer` | Một sequential pipeline end-to-end |
| Shell event-bindings | `crates/event-bindings` | Crate compile; logic Tuần 3 |
| Metrics | `crates/indexer` | `:9184/metrics` hoạt động |
| Runbook | `docs/plans/week-01-02-runbook.md` | Lệnh start/stop/verify |

---

## 2. Non-goals (rõ ràng)

| Hạng mục | Hoãn đến |
|----------|----------|
| Decode Cetus/Turbos `move_contract!` | Tuần 3–4 |
| Pipeline `dex_swap` + `dex_pool` tách biệt | Tuần 3–4 |
| Mainnet + GCS Requester Pays | Tuần 7 |
| Pipeline `token_metadata` | Tuần 5–6 |
| `tools/reconciliation` | Tuần 5–6 |
| Copy từ `examples/` | Không bao giờ |

---

## 3. Cấu trúc repo (cuối Tuần 2)

```
sui-indexer/
├── Cargo.toml
├── .env.example
├── crates/
│   ├── indexer/           # main.rs, config, pipelines/stub_events.rs
│   ├── indexer-store/     # composite, kafka, postgres
│   └── event-bindings/    # shell Tuần 2
├── infra/
│   ├── docker-compose.yml
│   ├── kafka/create-topics.sh
│   └── prometheus/prometheus.yml
└── docs/plans/
```

---

## 4. Tuần 1 — Nền tảng (Ngày 1–5)

### Ngày 1 — Workspace + infra

| Task | Chi tiết | Verify |
|------|----------|--------|
| Workspace gốc | Members: `indexer`, `indexer-store`, `event-bindings` | `cargo check --workspace` |
| `docker-compose.yml` | Postgres 16, Kafka KRaft, optional Kafka UI | `docker compose ps` |
| Tạo Kafka topics | `dex.swap.raw.v1`, `dex.pool.raw.v1`, `token.metadata.raw.v1` | `kafka-topics.sh --list` |
| `.env.example` | Mọi biến môi trường (§8) | Review |

---

### Ngày 2 — Postgres watermarks (`indexer-store`)

| Task | Chi tiết | Verify |
|------|----------|--------|
| Implement `PostgresStore` | Trait `Store` + `Connection` theo official BYOS | Test round-trip watermark |
| Migration | Bảng `watermarks` only | `diesel migration run` |

Tham khảo: [sui-pg-db store.rs](https://github.com/MystenLabs/sui/blob/main/crates/sui-pg-db/src/store.rs)

---

### Ngày 3 — Kafka producer

| Task | Chi tiết | Verify |
|------|----------|--------|
| `rdkafka` | Producer idempotent, `acks=all` | 100 message test |
| Message envelope | Theo [04-data-contracts.md](../../04-data-contracts.md) | JSON đúng schema |

---

### Ngày 4 — `CompositeStore`

| Task | Chi tiết | Verify |
|------|----------|--------|
| Facade Kafka + PG | `commit()` ghi Kafka rồi cập nhật watermark | Kafka fail → watermark không lùi/sai |
| Error handling | Không nuốt lỗi | Test fail path |

---

### Ngày 5 — Shell `crates/indexer`

| Task | Chi tiết | Verify |
|------|----------|--------|
| `Indexer::new()` | **Không** dùng `IndexerCluster` | Compile + start testnet |
| `ClientArgs` | HTTPS testnet + gRPC streaming | Checkpoint log |
| Prometheus | Registry + `:9184/metrics` | `curl` metrics |
| `event-bindings` shell | Crate rỗng compile được | `cargo build` |

---

## 5. Tuần 2 — Chứng minh end-to-end (Ngày 6–10)

### Ngày 6 — Pipeline stub

| Task | Chi tiết | Verify |
|------|----------|--------|
| `StubEventHandler` | `process()` → row đơn giản per checkpoint | Row có data |
| `commit()` | Ghi `dex.swap.raw.v1` | Consumer thấy message |
| Watermark | Pipeline `stub_events` | Bảng `watermarks` tăng |

Dùng payload `checkpoint_heartbeat` tạm — thay bằng swap event Tuần 3.

---

### Ngày 7 — Soak testnet (≥ 4h)

| Task | Verify |
|------|--------|
| Chạy liên tục testnet | Lag < 500 checkpoint |
| Kafka consumer lag | ~ 0 |
| Restart | Resume đúng watermark |

---

### Ngày 8 — Observability

| Task | Verify |
|------|--------|
| Prometheus scrape config | Target UP |
| Custom counter lỗi Kafka | Tăng khi inject fail |
| Structured logging | `checkpoint_seq`, `pipeline` |

---

### Ngày 9 — Hardening + runbook

| Task | Verify |
|------|--------|
| `week-01-02-runbook.md` | Peer review |
| `cargo fmt/clippy/test` | Pass local |
| Backpressure smoke | Không OOM khi Kafka chậm |

---

### Ngày 10 — Sprint review

| Cổng | Pass? |
|------|-------|
| `cargo build --workspace` | ☐ |
| Không import từ `examples/` | ☐ |
| Kafka = output chính | ☐ |
| Watermark survive restart | ☐ |
| Prometheus OK | ☐ |
| Soak testnet ≥ 4h | ☐ |
| Runbook xong | ☐ |

---

## 6. Dependencies

Pin Mysten `mainnet` branch — xem bản EN §6 cho `Cargo.toml` đầy đủ. Rust 2024, toolchain ≥ 1.85.

---

## 7. Biến môi trường (Tuần 1–2)

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/sui_indexer
KAFKA_BROKERS=localhost:9092
KAFKA_CLIENT_ID=sui-token-indexer
METRICS_ADDRESS=0.0.0.0:9184
REMOTE_STORE_URL=https://checkpoints.testnet.sui.io
STREAMING_URL=https://fullnode.testnet.sui.io:443
```

Mainnet GCS — **chưa dùng** sprint này.

---

## 8. Rủi ro

| Rủi ro | Giảm thiểu |
|--------|------------|
| BYOS phức tạp | Đọc clickhouse-sui-indexer Ngày 2 |
| `rdkafka` macOS | `brew install cmake` trong runbook |
| API drift | Pin git rev sau build đầu |
| Scope creep DEX | Chỉ stub pipeline Tuần 2 |

---

## 9. Bàn giao Tuần 3–4

Khi pass cổng Tuần 2:

1. Thay `stub_events` → `dex_swap` + `dex_pool`
2. Implement `event-bindings` Cetus + Turbos
3. `parsed_json` thật trên `dex.swap.raw.v1`

---

## 10. Checklist (issue tracker)

### Tuần 1
- [ ] W1-1 Workspace
- [ ] W1-2 docker-compose + topics
- [ ] W1-3 Migration watermarks
- [ ] W1-4 PostgresStore traits
- [ ] W1-5 KafkaFactWriter
- [ ] W1-6 CompositeStore
- [ ] W1-7 Binary shell + testnet
- [ ] W1-8 Prometheus

### Tuần 2
- [ ] W2-1 Pipeline stub
- [ ] W2-2 Message trên Kafka
- [ ] W2-3 Watermark advance
- [ ] W2-4 Soak ≥ 4h
- [ ] W2-5 Restart test
- [ ] W2-6 Prometheus config
- [ ] W2-7 Runbook
- [ ] W2-8 Demo sprint gate
