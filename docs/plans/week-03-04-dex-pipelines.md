# Week 3–4 Runbook — DEX Pipelines + Event Bindings

**Scope:** `dex_swap` + `dex_pool` pipelines, `crates/event-bindings` for Cetus/Turbos/Bluefin/MMT on **mainnet**.

## Prerequisites

- Week 1–2 infra running (Postgres, Kafka, topics)
- `cmake` installed (Kafka / rdkafka)
- Network on **first** `cargo build -p event-bindings` (GraphQL package fetch for `move_contract!`)

## 1. Configure mainnet `.env`

```bash
cp .env.example .env
```

Set (defaults in `.env.example`):

```bash
REMOTE_STORE_URL=https://checkpoints.mainnet.sui.io
STREAMING_URL=https://fullnode.mainnet.sui.io:443
# Optional: start from a recent checkpoint for faster dev sync
# FIRST_CHECKPOINT=288400000
```

## 2. Build

```bash
cargo build --workspace          # first event-bindings build needs network
cargo test -p event-bindings     # 13 decode/classification tests
```

## 3. Run indexer

```bash
cargo run -p sui-token-indexer
```

Registered pipelines:

| Pipeline | Watermark row | Kafka topic |
|----------|---------------|-------------|
| `dex_swap` | `pipeline = 'dex_swap'` | `dex.swap.raw.v1` |
| `dex_pool` | `pipeline = 'dex_pool'` | `dex.pool.raw.v1` |

## 4. Verify Kafka

Image `apache/kafka:3.7.0` — scripts live under `/opt/kafka/bin/*.sh` (not on `$PATH`).

```bash
# Swap facts (partition key = pool_id)
docker compose -f infra/docker-compose.yml exec kafka \
  /opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic dex.swap.raw.v1 \
  --max-messages 3

# Pool create facts
docker compose -f infra/docker-compose.yml exec kafka \
  /opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic dex.pool.raw.v1 \
  --from-beginning \
  --max-messages 3
```

Or browse messages in **Kafka UI**: http://localhost:8080 (Topics → `dex.swap.raw.v1` → Messages → Load, offset **Latest**)

If UI shows `500 AdminClient` / `Connection to localhost:9092`, recreate Kafka after the dual-listener fix in `infra/docker-compose.yml`:

```bash
docker compose -f infra/docker-compose.yml --env-file .env up -d --force-recreate kafka kafka-ui
```

**Note:** `--from-beginning` reads old messages first. If the topic still has Week 1–2 `stub_events` heartbeats, either consume more messages, reset the topic, or use `--offset latest` (omit `--from-beginning`) to see only new `dex_swap` facts after re-indexing.

Expect `payload.protocol` in `cetus|turbos|bluefin|mmt` and `payload.parsed_json` on swap topic (not heartbeat stubs).

## 5. Verify watermarks

```sql
SELECT pipeline, checkpoint_hi_inclusive, updated_at
FROM watermarks
WHERE pipeline IN ('dex_swap', 'dex_pool');
```

Both rows should advance independently.

## 6. Metrics

Prometheus (`METRICS_ADDRESS`, default `0.0.0.0:9184`):

- `indexer_decode_errors_total{pipeline,protocol,event_type}`
- `indexer_events_matched_total{pipeline,protocol,event_type}`
- `indexer_kafka_rows_published_total{pipeline,topic}`
- Framework: `watermark_checkpoint_in_db{pipeline="dex_swap"}` / `dex_pool`

## Filter strategy

Pipelines match **full canonical `event_type`** allowlists (case-insensitive). Classification uses the defining package embedded in `event_type` (not `event.package_id`, which may differ after package upgrades).

| Protocol | Swap event type |
|----------|-----------------|
| Cetus | `0x1eab…::pool::SwapEvent` |
| Turbos | `0x91bf…::pool::SwapEvent` |
| Bluefin | `0x3492…::events::AssetSwap` |
| MMT | `0x7028…::trade::SwapEvent` |

Pool-create types: `factory::CreatePoolEvent`, `pool_factory::PoolCreatedEvent`, `events::PoolCreated`, `create_pool::PoolCreatedEvent`.

## Sprint gate checklist

- [ ] `cargo build --workspace` clean
- [ ] `cargo test -p event-bindings` — all pass
- [ ] `rg examples crates/` — no prod imports from spike
- [ ] `dex_swap` + `dex_pool` watermarks advance
- [ ] `dex.swap.raw.v1` carries real `parsed_json`
- [ ] `stub_events` not registered in `main.rs`

## Troubleshooting

| Issue | Action |
|-------|--------|
| `move_contract! failed` on build | Check network; verify GraphQL reachable |
| Decode error stops checkpoint | Check `indexer_decode_errors_total`; fix binding or BCS |
| No swap messages | Confirm `FIRST_CHECKPOINT` is recent mainnet; wait for DEX activity |
| Kafka produce fail | Watermark won't advance; check broker logs |

## Out of scope (Week 5+)

FlowX/Magma bindings, `token_metadata` pipeline, GCS mainnet backfill, Phase 2 normalizers.
