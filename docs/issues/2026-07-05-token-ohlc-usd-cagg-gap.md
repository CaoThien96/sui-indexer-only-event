# token_ohlc_usd_1m CAGG misses early bars for new tokens

**Status:** Resolved (2026-07-05)  
**Priority:** Medium  
**Discovered:** 2026-07-04  
**Component:** TimescaleDB, volume-engine, API OHLC chart

## Resolution

Implemented **Option C**: removed pool `ohlc_*` tables and `ohlc-aggregator`; `volume-engine` now upserts all `token_ohlc_usd_*` intervals (1m–24h) directly on each inserted swap with USD fields. Migration `2026-07-05-120000_token_ohlc_usd_direct` drops CAGGs, creates hypertables, and backfills from `swaps_fact`.

Cold storage: rolloff-job copies all `token_ohlc_usd_*` tables to ClickHouse; API merges hot (Timescale) + cold (CH) by interval.

## Original summary

The token USD OHLC chart could start **later** than the first swap because `token_ohlc_usd_1m` was a continuous aggregate with a narrow refresh window, not written directly by volume-engine.

## Acceptance criteria

- [x] New token’s first `token_ohlc_usd_1m` bucket aligns with first swap minute (within 1 bucket), without manual `refresh_continuous_aggregate`
- [x] Chart `/api/v1/tokens/{coin_type}/ohlc?interval=1m` first bar matches first trade time for test tokens
- [x] Pool OHLC removed (`GET /v1/pools/{id}/ohlc` deleted; `ohlc-aggregator` removed)

## Files

- Migration: `crates/processors/migrations_timescale/2026-07-05-120000_token_ohlc_usd_direct/`
- Upsert: `crates/processors/src/timescale/mod.rs` → `upsert_token_ohlc_usd_all_intervals`
- Writer: `crates/processors/src/volume/mod.rs`
- Module: `crates/processors/src/token_ohlc/`
