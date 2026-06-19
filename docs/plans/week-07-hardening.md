# Week 7 Runbook — Hardening (HTTPS-only, GCS deferred)

**Scope:** Runtime tuning, Prometheus watermark alerts, Phase 1 gate verification.

**Decision:** GCS backfill (`gs://mysten-mainnet-checkpoints-use4`) is **deferred**. Production uses HTTPS remote store + gRPC streaming. Historical depth is limited to ~30 days on HTTPS; expand to GCS when budget/ops allow.

---

## 1. Runtime tuning

Tuning lives in `crates/indexer/src/runtime_tuning.rs` and is driven by env vars.

| Mode | When | `INDEXER_RUNTIME_MODE` | Defaults |
|------|------|------------------------|----------|
| Steady | At/near tip via gRPC | `steady` (default) | `collect_interval_ms=300`, adaptive ingest max 500 |
| Backfill | HTTPS catch-up from `FIRST_CHECKPOINT` | `backfill` | `ingest_concurrency=200` fixed, `collect_interval_ms=750` |

### Env overrides

| Variable | Purpose |
|----------|---------|
| `INDEXER_RUNTIME_MODE` | `steady` or `backfill` |
| `COLLECT_INTERVAL_MS` | Sequential committer poll interval |
| `INGEST_CONCURRENCY` | Fixed ingest parallelism (overrides mode default) |
| `INGEST_CONCURRENCY_MAX` | Adaptive ingest ceiling (steady mode) |
| `SUBSCRIBER_CHANNEL_SIZE` | Per-pipeline subscriber channel |
| `PIPELINE_DEPTH` | Collector/committer overlap (default `2` for Kafka IO) |
| `PROCESSOR_FANOUT_MAX` | Adaptive processor concurrency cap |
| `DB_CONNECTION_POOL_SIZE` | Postgres watermark pool (default `15` = 3 pipelines × 5) |

### Backfill workflow (HTTPS-only)

```bash
# .env — start from a recent checkpoint within HTTPS retention (~30d)
FIRST_CHECKPOINT=288461467
INDEXER_RUNTIME_MODE=backfill

cargo run -p sui-token-indexer
```

When lag at tip is < 30s for several minutes, switch to steady:

```bash
# Stop indexer, then:
INDEXER_RUNTIME_MODE=steady
# or unset INDEXER_RUNTIME_MODE

cargo run -p sui-token-indexer
```

---

## 2. Prometheus alerts

Alerts are in `infra/prometheus/alerts.yml` and loaded by the local Prometheus container.

```bash
docker compose -f infra/docker-compose.yml --env-file .env up -d prometheus
```

Open http://localhost:9090/alerts — expect rules **Inactive** when healthy.

| Alert | Condition | Threshold |
|-------|-----------|-----------|
| `IndexerIngestionLagHigh` | `indexer_latest_ingested_checkpoint_timestamp_lag_ms` | > 30s for 2m |
| `IndexerPipelineWatermarkLagHigh` | `indexer_latest_watermarked_checkpoint_timestamp_lag_ms` | > 30s for 2m |
| `IndexerPipelineWatermarkStalled` | `watermark_checkpoint_in_db` unchanged | 10m while ingesting |
| `IndexerDecodeErrorsSpike` | `indexer_decode_errors_total` increase | > 10 in 5m |

**Note:** Metrics use prefix `indexer_` by default. If `METRICS_PREFIX` is set, update `alerts.yml` accordingly.

### Quick lag check (CLI)

```bash
curl -s localhost:9184/metrics | grep -E 'latest_(ingested|watermarked)_checkpoint_timestamp_lag_ms'
```

---

## 3. Phase 1 gate checklist

| Gate | Status | Evidence |
|------|--------|----------|
| Production code in `crates/` only | ✅ | No `examples/` imports in `crates/`; workspace `examples/` is reference-only |
| Checkpoint source | ✅ HTTPS + gRPC | `REMOTE_STORE_URL` + `STREAMING_URL` in `.env` |
| GCS backfill | ⏸ Deferred | Documented; add `REMOTE_STORE_GCS` when budget allows |
| Kafka = BYOS primary commit | ✅ | `CompositeStore` → Kafka then watermark |
| ≥ 4 DEX protocols | ✅ | 6 protocols: Cetus, Turbos, Bluefin, MMT, FlowX, Magma |
| Prometheus watermark alerts | ✅ | `infra/prometheus/alerts.yml` |
| Steady-state lag < 30s | ☐ Verify | Run at tip; check metrics / alerts |

### Verify watermarks

```sql
SELECT pipeline, checkpoint_hi_inclusive, timestamp_ms_hi_inclusive
FROM watermarks
ORDER BY pipeline;
```

### Verify DEX coverage on Kafka

```bash
/opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic dex.swap.raw.v1 \
  --from-beginning --max-messages 100 \
  | jq -r '.protocol' | sort | uniq -c
```

---

## 4. GCS — future path

When ready to index >30 days without HTTPS rate limits:

1. Provision GCS access to `mysten-mainnet-checkpoints-use4`
2. Set `REMOTE_STORE_GCS` + `GOOGLE_APPLICATION_CREDENTIALS` in `.env`
3. Use `INDEXER_RUNTIME_MODE=backfill` with an older `FIRST_CHECKPOINT`
4. Switch to `steady` at tip

No code changes required — framework `ClientArgs` already supports GCS via CLI/env.

---

## 5. Common issues

| Symptom | Fix |
|---------|-----|
| `OutOfOrderSequenceNumber` (Kafka) | `docker compose … restart kafka` or `KAFKA_ENABLE_IDEMPOTENCE=false` |
| Ingestion lag high during backfill | `INDEXER_RUNTIME_MODE=backfill`, raise `INGEST_CONCURRENCY` |
| Watermark lag high at tip | `INDEXER_RUNTIME_MODE=steady`, lower `COLLECT_INTERVAL_MS`, check Kafka |
| Checkpoint not found (HTTPS) | `FIRST_CHECKPOINT` older than ~30d — wait for GCS or pick newer cp |
| Prometheus target DOWN | Indexer must run on host; scrape `host.docker.internal:9184` |
