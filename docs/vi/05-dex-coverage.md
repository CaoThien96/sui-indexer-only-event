# 05 — Phạm vi DEX & Event

Checklist giao thức, package ID, và loại event. Cập nhật khi thêm DEX mới.

**Chú thích:** 🔬 spike trong `examples/` · ⬜ prod chưa làm (`crates/event-bindings`)

Crate decode production: `crates/event-bindings/` (greenfield — chưa bắt đầu).

> **English:** [../05-dex-coverage.md](../05-dex-coverage.md)

---

## 1. Cetus CLMM

| Mục | Giá trị |
|-----|---------|
| Package (mainnet) | `0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb` |
| Đường dẫn contract | `docs/contracts/cetus-contracts-main/` |
| Crate decode (prod) | `crates/event-bindings` (`pkg_1eab`) — ⬜ |
| Tham khảo spike | `examples/simple-sui-indexer/event-bindings/` 🔬 |

| Event | Spike | Prod decode | Prod Kafka |
|-------|-------|-------------|------------|
| `pool::SwapEvent` | 🔬 | ⬜ | ⬜ |
| `factory::CreatePoolEvent` | 🔬 | ⬜ | ⬜ |
| `pool::AddLiquidityEvent` | 🔬 | ⬜ | ⬜ |
| `pool::RemoveLiquidityEvent` | 🔬 | ⬜ | ⬜ |
| `pool::OpenPositionEvent` | 🔬 | ⬜ | ⬜ |
| `pool::ClosePositionEvent` | 🔬 | ⬜ | ⬜ |
| `pool::CollectFeeEvent` | 🔬 | ⬜ | ⬜ |
| `pool::CollectProtocolFeeEvent` | 🔬 | ⬜ | ⬜ |
| `partner::ReceiveRefFeeEvent` | 🔬 | ⬜ | ⬜ |
| `partner::ClaimRefFeeEvent` | 🔬 | ⬜ | ⬜ |

---

## 2. Turbos CLMM

| Mục | Giá trị |
|-----|---------|
| Package (mainnet) | `0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1` |
| Đường dẫn contract | `docs/contracts/turbos-sui-move-interface-main/` |
| Crate decode (prod) | `crates/event-bindings` (`pkg_91bf`) — ⬜ |
| Tham khảo spike | `examples/simple-sui-indexer/event-bindings/` 🔬 |

| Event | Spike | Prod decode | Prod Kafka |
|-------|-------|-------------|------------|
| `pool::SwapEvent` | 🔬 | ⬜ | ⬜ |
| `pool_factory::PoolCreatedEvent` | 🔬 | ⬜ | ⬜ |
| `pool::MintEvent` | 🔬 | ⬜ | ⬜ |
| `pool::BurnEvent` | 🔬 | ⬜ | ⬜ |
| `position_manager::IncreaseLiquidityEvent` | 🔬 | ⬜ | ⬜ |
| `position_manager::DecreaseLiquidityEvent` | 🔬 | ⬜ | ⬜ |
| `pool::CollectProtocolFeeEvent` | 🔬 | ⬜ | ⬜ |

---

## 3. Bluefin Spot

| Mục | Giá trị |
|-----|---------|
| Package (mainnet) | `0x3492c874c1e3b3e2984e8c41b589e642d4d0a5d6459e5a9cfc2d52fd7c89c267` |
| Đường dẫn contract | `docs/contracts/bluefin-spot-contract-interface-main/` |
| Event mẫu | `events::AssetSwap` — xem [events.md](../../events.md) |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| `events::AssetSwap` | ⬜ | ⬜ | ⬜ |
| Pool creation event | ⬜ | ⬜ | ⬜ |

**Deliverable Phase 1 tuần 3.**

---

## 4. MMT v3

| Mục | Giá trị |
|-----|---------|
| Đường dẫn contract | `docs/contracts/mmt-contract-interface-mainnet/` |
| Package ID | Xác nhận trên mainnet trước khi binding |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| Swap event | ⬜ | ⬜ | ⬜ |
| Pool create event | ⬜ | ⬜ | ⬜ |

---

## 5. FlowX CLMM

| Mục | Giá trị |
|-----|---------|
| Đường dẫn contract | `docs/contracts/flowx-clmm-contracts-main/` |
| Package ID | Xác nhận trên mainnet trước khi binding |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| Swap event | ⬜ | ⬜ | ⬜ |
| Pool create event | ⬜ | ⬜ | ⬜ |

---

## 6. Magma CLMM

| Mục | Giá trị |
|-----|---------|
| Đường dẫn contract | `docs/contracts/magma_core_clmm_interface-main/` |
| Package ID | Xác nhận trên mainnet trước khi binding |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| Swap event | ⬜ | ⬜ | ⬜ |
| Pool create event | ⬜ | ⬜ | ⬜ |

---

## 7. Checklist thêm DEX mới

1. Thêm contract interface vào `docs/contracts/`
2. Thêm block `move_contract!` trong `crates/event-bindings/src/lib.rs`
3. Thêm nhánh decode trong `decode_parsed_json()`
4. Unit test với BCS hex thật (từ `events.md` hoặc fullnode)
5. Đăng ký protocol trong cấu hình indexer pipeline
6. Thêm row bảng `protocols` (Phase 2 catalog)
7. Mapping normalizer trong `crates/processors` (Phase 2)
8. Chạy `tools/reconciliation` cửa sổ 24h (QA tùy chọn)
9. Cập nhật file này

---

## 8. Mapping field normalizer (tham khảo)

| Protocol | field pool id | hướng | amount in | sqrt price |
|----------|---------------|-------|-----------|------------|
| Cetus | `pool` | `atob` | `amount_in` | `after_sqrt_price` |
| Turbos | `pool` | `a_to_b` | `amount_a` / `amount_b` | theo struct event |
| Bluefin | `pool_id` | `a2b` | `amount_in` | `after_sqrt_price` |

Tên field Turbos/Bluefin chốt khi thêm bindings.
