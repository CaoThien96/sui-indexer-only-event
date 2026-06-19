# Week 5–6 Runbook — FlowX/Magma + Token Metadata

**Scope:** FlowX + Magma CLMM bindings, `token_metadata` pipeline, optional Kafka reconciliation tool.

## Prerequisites

- Week 3–4 infra and DEX pipelines running
- `cmake` installed (Kafka / rdkafka)
- Network on first `cargo build -p event-bindings` after adding Magma linkage

## 1. Build and test

```bash
cargo build --workspace
cargo test -p event-bindings    # 19 tests (FlowX manual + Magma + coin_metadata)
cargo build -p sui-token-indexer
```

## 2. Registered pipelines

| Pipeline | Watermark row | Kafka topic |
|----------|---------------|-------------|
| `dex_swap` | `pipeline = 'dex_swap'` | `dex.swap.raw.v1` |
| `dex_pool` | `pipeline = 'dex_pool'` | `dex.pool.raw.v1` |
| `token_metadata` | `pipeline = 'token_metadata'` | `token.metadata.raw.v1` |

## 3. Run indexer

```bash
cargo run -p sui-token-indexer
```

## 4. Verify Kafka — DEX (FlowX + Magma)

```bash
/opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic dex.swap.raw.v1 \
  --from-beginning --max-messages 50 | grep -E '"protocol":"(flowx|magma)"'
```

Expected event types:

| Protocol | Swap | Pool create |
|----------|------|-------------|
| FlowX | `0x25929…::pool::Swap` | `0x25929…::pool_manager::PoolCreated` |
| Magma | `0x4a35d3…::pool::SwapEvent` | `0x4a35d3…::factory::CreatePoolEvent` |

## 5. Verify Kafka — token metadata

```bash
/opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic token.metadata.raw.v1 \
  --from-beginning --max-messages 5
```

Payload fields: `coin_type`, `name`, `symbol`, `decimals`, `creator`, `checkpoint_sequence_number`, `tx_digest`.

## 6. Optional — reconciliation (Kafka vs fullnode)

```bash
cp tools/reconciliation/.env.example tools/reconciliation/.env
# Edit RECON_MOVE_EVENT_TYPE / window as needed

cargo run -p sui-indexer-reconciliation
```

Exit code 0 = counts and keys within tolerance for the sample window.

## 7. Sprint gate checklist

- [ ] `cargo test -p event-bindings` — 19 tests pass
- [ ] Indexer registers 3 pipelines; independent watermarks in Postgres
- [ ] Kafka shows FlowX/Magma swap + pool facts
- [ ] At least one `token.metadata.raw.v1` message with decoded name/symbol/decimals
- [ ] Phase 1 gate: ≥ 6 DEX protocols on Kafka (Cetus, Turbos, Bluefin, MMT, FlowX, Magma)

## Notes

- **FlowX decode:** manual BCS in `crates/event-bindings/src/flowx_manual.rs` (avoids `sui::table` linkage from full module codegen).
- **Magma linkage:** requires `magma_integer_mate` package (`0x659c0e9c…`) registered in `event-bindings/src/lib.rs`.
- **Token metadata:** scans `tx.created_objects()` for new `0x2::coin::CoinMetadata<T>` objects per checkpoint.
