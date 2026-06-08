# 05 — DEX Event Coverage

Checklist of protocols, package IDs, and event types. Update when adding a new DEX.

**Legend:** 🔬 spike in `examples/` only · ⬜ prod todo (`crates/event-bindings`)

Production decode crate: `crates/event-bindings/` (greenfield — not started).

---

## 1. Cetus CLMM

| Item | Value |
|------|-------|
| Package (mainnet) | `0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb` |
| Contracts path | `docs/contracts/cetus-contracts-main/` |
| Decode crate (prod) | `crates/event-bindings` (`pkg_1eab`) — ⬜ |
| Spike reference | `examples/simple-sui-indexer/event-bindings/` 🔬 |

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

**Env prefix example:**
```
EVENT_TYPE_PREFIXES=0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::
```

---

## 2. Turbos CLMM

| Item | Value |
|------|-------|
| Package (mainnet) | `0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1` |
| Contracts path | `docs/contracts/turbos-sui-move-interface-main/` |
| Decode crate (prod) | `crates/event-bindings` (`pkg_91bf`) — ⬜ |
| Spike reference | `examples/simple-sui-indexer/event-bindings/` 🔬 |

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

| Item | Value |
|------|-------|
| Package (mainnet) | `0x3492c874c1e3b3e2984e8c41b589e642d4d0a5d6459e5a9cfc2d52fd7c89c267` |
| Contracts path | `docs/contracts/bluefin-spot-contract-interface-main/` |
| Sample event | `events::AssetSwap` — see [events.md](../events.md) |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| `events::AssetSwap` | ⬜ | ⬜ | ⬜ |
| Pool creation event | ⬜ | ⬜ | ⬜ |

**Phase 1 Week 3 deliverable.**

---

## 4. MMT v3

| Item | Value |
|------|-------|
| Contracts path | `docs/contracts/mmt-contract-interface-mainnet/` |
| Package ID | Confirm on mainnet before binding |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| Swap event | ⬜ | ⬜ | ⬜ |
| Pool create event | ⬜ | ⬜ | ⬜ |

---

## 5. FlowX CLMM

| Item | Value |
|------|-------|
| Contracts path | `docs/contracts/flowx-clmm-contracts-main/` |
| Package ID | Confirm on mainnet before binding |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| Swap event | ⬜ | ⬜ | ⬜ |
| Pool create event | ⬜ | ⬜ | ⬜ |

---

## 6. Magma CLMM

| Item | Value |
|------|-------|
| Contracts path | `docs/contracts/magma_core_clmm_interface-main/` |
| Package ID | Confirm on mainnet before binding |

| Event | Index | Decode | Kafka |
|-------|-------|--------|-------|
| Swap event | ⬜ | ⬜ | ⬜ |
| Pool create event | ⬜ | ⬜ | ⬜ |

---

## 7. Adding a new DEX — checklist

1. Add contract interface to `docs/contracts/`
2. Add `move_contract!` block in `crates/event-bindings/src/lib.rs`
3. Add decode arms in `decode_parsed_json()`
4. Add unit test with real BCS hex (from `events.md` or fullnode)
5. Register protocol in indexer pipeline config
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

Exact Turbos/Bluefin field names finalized when bindings are added.
