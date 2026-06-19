# 05 — DEX Event Coverage

Checklist of protocols, package IDs, and event types. Update when adding a new DEX.

**Legend:** 🔬 spike in `examples/` only · ✅ prod (`crates/event-bindings` + indexer pipelines)

Production decode crate: `crates/event-bindings/` — **implemented Week 3–4**.

---

## 1. Cetus CLMM

| Item | Value |
|------|-------|
| Package (mainnet type prefix) | `0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb` |
| Contracts path | `docs/contracts/cetus-contracts-main/` |
| Decode crate (prod) | `crates/event-bindings` (`pkg_1eab`) — ✅ |
| Spike reference | `examples/simple-sui-indexer/event-bindings/` 🔬 |

| Event | Spike | Prod decode | Prod Kafka |
|-------|-------|-------------|------------|
| `pool::SwapEvent` | 🔬 | ✅ | ✅ `dex_swap` |
| `factory::CreatePoolEvent` | 🔬 | ✅ | ✅ `dex_pool` |
| `pool::AddLiquidityEvent` | 🔬 | ✅ | — |
| `pool::RemoveLiquidityEvent` | 🔬 | ✅ | — |
| `pool::OpenPositionEvent` | 🔬 | ✅ | — |
| `pool::ClosePositionEvent` | 🔬 | ✅ | — |
| `pool::CollectFeeEvent` | 🔬 | ✅ | — |
| `pool::CollectProtocolFeeEvent` | 🔬 | ✅ | — |
| `partner::ReceiveRefFeeEvent` | 🔬 | ✅ | — |
| `partner::ClaimRefFeeEvent` | 🔬 | ✅ | — |

---

## 2. Turbos CLMM

| Item | Value |
|------|-------|
| Package (mainnet) | `0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1` |
| Contracts path | `docs/contracts/turbos-sui-move-interface-main/` |
| Decode crate (prod) | `crates/event-bindings` (`pkg_91bf`) — ✅ |
| Spike reference | `examples/simple-sui-indexer/event-bindings/` 🔬 |

| Event | Spike | Prod decode | Prod Kafka |
|-------|-------|-------------|------------|
| `pool::SwapEvent` | 🔬 | ✅ | ✅ `dex_swap` |
| `pool_factory::PoolCreatedEvent` | 🔬 | ✅ | ✅ `dex_pool` |
| `pool::MintEvent` | 🔬 | ✅ | — |
| `pool::BurnEvent` | 🔬 | ✅ | — |
| `position_manager::IncreaseLiquidityEvent` | 🔬 | ✅ | — |
| `position_manager::DecreaseLiquidityEvent` | 🔬 | ✅ | — |
| `pool::CollectProtocolFeeEvent` | 🔬 | ✅ | — |

---

## 3. Bluefin Spot

| Item | Value |
|------|-------|
| Package (mainnet) | `0x3492c874c1e3b3e2984e8c41b589e642d4d0a5d6459e5a9cfc2d52fd7c89c267` |
| Contracts path | `docs/contracts/bluefin-spot-contract-interface-main/` |
| Sample event | `events::AssetSwap` — see [events.md](../examples/events.md) |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| `events::AssetSwap` | ✅ | ✅ | ✅ `dex_swap` |
| `events::PoolCreated` | ✅ | ✅ | ✅ `dex_pool` |

**Note:** Bluefin event structs lack `Event` suffix; bindings use `emit_mode = "module_structs"` for `events` module.

---

## 4. MMT v3

| Item | Value |
|------|-------|
| Contracts path | `docs/contracts/mmt-contract-interface-mainnet/` |
| Type prefix (on-chain events) | `0x70285592c97965e811e0c6f98dccc3a9c2b4ad854b3594faab9597ada267b860` |
| `published-at` (Move.toml) | `0xcf60a40f45d46fc1e828871a647c1e25a0915dec860d2662eb10fdb382c3c1d1` |
| Decode crate | `pkg_7028` — ✅ |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| `trade::SwapEvent` | ✅ | ✅ | ✅ `dex_swap` |
| `create_pool::PoolCreatedEvent` | ✅ | ✅ | ✅ `dex_pool` |

**Note:** On-chain `event_type` uses the original package id (`0x7028…`), not `published-at`. `event.package_id` may be a newer upgrade id.

---

## 5. FlowX CLMM

| Item | Value |
|------|-------|
| Contracts path | `docs/contracts/flowx-clmm-contracts-main/` |
| Type package (on-chain events) | `0x25929e7f29e0a30eb4e692952ba1b5b65a3a4d65ab5f2a32e1ba3edcb587f26d` |
| `published-at` (Move.toml) | `0xde2c47eb0da8c74e4d0f6a220c41619681221b9c2590518095f0f0c2d3f3c772` |
| Decode crate | `config::flowx` — manual BCS (`flowx_manual.rs`) — ✅ |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| `pool::Swap` | ✅ | ✅ | ✅ `dex_swap` |
| `pool_manager::PoolCreated` | ✅ | ✅ | ✅ `dex_pool` |

**Note:** Event structs lack `Event` suffix; decode uses manual BCS (not `move_contract!`) because `pool_manager::PoolRegistry` references `sui::table::Table`.

---

## 6. Magma CLMM

| Item | Value |
|------|-------|
| Contracts path | `docs/contracts/magma_core_clmm_interface-main/` |
| Type package | `0x4a35d3dfef55ed3631b7158544c6322a23bc434fe4fca1234cb680ce0505f82d` |
| `published-at` (Move.toml) | `0x183af2adf115f331105825ae63e1d7d3c848d67beb4d60bc36208a90a5e92f4b` |
| Decode crate | `pkg_4a35` — ✅ |
| IntegerMate dep | `0x659c0e9c4c8a416f040fa758d4fc4073a5fdd1fed97edadcd5cba5180fb36246` |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| `pool::SwapEvent` | ✅ | ✅ | ✅ `dex_swap` |
| `factory::CreatePoolEvent` | ✅ | ✅ | ✅ `dex_pool` |

---

## 7. Adding a new DEX — checklist

1. Add contract interface to `docs/contracts/`
2. Add `move_contract!` block in `crates/event-bindings/src/lib.rs`
3. Add decode arms in `decode_parsed_json()`
4. Add unit test with real BCS hex (from `events.md` or fullnode)
5. Register event types in `crates/event-bindings/src/protocol.rs`
6. Add row to `protocols` table (Phase 2 catalog)
7. Implement normalizer mapping in `crates/processors` (Phase 2)
8. Run `tools/reconciliation` for 24h window (optional QA)
9. Update this file

---

## 8. Normalizer field mapping (reference)

| Protocol | pool id field | direction | amount in | sqrt price |
|----------|---------------|-----------|-----------|------------|
| Cetus | `pool` | `atob` | `amount_in` | `after_sqrt_price` |
| Turbos | `pool` | `a_to_b` | `amount_a` / `amount_b` | per event struct |
| Bluefin | `pool_id` | `a2b` | `amount_in` | `after_sqrt_price` |
| MMT | `pool_id` | `x_for_y` | `amount_x` / `amount_y` | `sqrt_price_after` |
| FlowX | `pool_id` | `x_for_y` | `amount_x` / `amount_y` | `sqrt_price_after` |
| Magma | `pool` | `atob` | `amount_in` | `after_sqrt_price` |

---

## 9. Indexer filter (Week 3–6)

Prod pipelines use **full `event_type` allowlists** (6 swap + 6 pool-create types). See [week-03-04-dex-pipelines.md](./plans/week-03-04-dex-pipelines.md) and [week-05-06-remaining-dexes-metadata.md](./plans/week-05-06-remaining-dexes-metadata.md).

Spike prefix filter (`examples/simple-sui-indexer/src/prefix.rs`) remains reference only.
