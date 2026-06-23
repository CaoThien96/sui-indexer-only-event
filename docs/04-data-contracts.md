# 04 — Data Contracts

Kafka topics, normalized schemas, and core database tables.  
**Version:** 1.0 — frozen with Phase 1 start

---

## 1. Kafka topics

| Topic | Phase | Producer | Consumers | Retention |
|-------|-------|----------|-----------|-----------|
| `dex.swap.raw.v1` | 1 | indexer | swap-normalizer | 7d |
| `dex.pool.raw.v1` | 1 | indexer | catalog-worker | 7d |
| `token.metadata.raw.v1` | 1 | indexer | catalog-worker | 7d |
| `dex.swap.normalized.v1` | 2 | swap-normalizer | ohlc, volume, liquidity | 7d |
| `coin.balance_change.v1` | 4 | balance-extractor | holder-aggregator, graph-builder | 14d |

**Partition key:**
- Swap topics: `pool_id`
- Metadata: `coin_type`
- Balance change: `coin_type`

**Message envelope (all topics):**

```json
{
  "schema_version": 1,
  "message_id": "sha256(tx_digest + event_seq + topic)",
  "produced_at_ms": 1710000000000,
  "payload": { }
}
```

---

## 2. `dex.swap.raw.v1` payload

Indexer publishes after BCS decode. `parsed_json` is protocol-native field names.

```json
{
  "protocol": "cetus",
  "package_id": "0x1eab...b2fb",
  "event_type": "0x1eab...::pool::SwapEvent",
  "checkpoint_sequence_number": 12345678,
  "timestamp_ms": 1710000000000,
  "tx_digest": "ABC...",
  "event_sequence_in_transaction": 0,
  "transaction_sequence_in_checkpoint": 3,
  "sender": "0xsender...",
  "parsed_json": {
    "pool": "0x9661...",
    "atob": false,
    "amount_in": "2230721817",
    "amount_out": "101397162635",
    "fee_amount": "223073",
    "vault_a_amount": "1318800564561393",
    "vault_b_amount": "28977577658125",
    "before_sqrt_price": "2735842178263405965",
    "after_sqrt_price": "2736053497612767632"
  }
}
```

---

## 3. `dex.swap.normalized.v1` payload

Produced by `swap-normalizer`. All downstream engines consume only this topic.

```json
{
  "protocol": "cetus",
  "pool_id": "0x9661cca01a5b9b3536883568fa967a2943e237de11a97976795f5adb293892e9",
  "coin_type_a": "0x2::sui::SUI",
  "coin_type_b": "0x...::coin::TOKEN",
  "amount_in_raw": "2230721817",
  "amount_out_raw": "101397162635",
  "amount_in_decimal": "2.230721817",
  "amount_out_decimal": "101.397162635",
  "amount_base_raw": "101397162635",
  "amount_quote_raw": "2230721817",
  "amount_base_decimal": "101.397162635",
  "amount_quote_decimal": "2.230721817",
  "fee_amount_raw": "223073",
  "a_to_b": false,
  "sqrt_price_before": "2735842178263405965",
  "sqrt_price_after": "2736053497612767632",
  "price_quote_per_base": "0.001234",
  "quote_coin_type": "0x2::sui::SUI",
  "base_coin_type": "0x...::coin::TOKEN",
  "vault_a_raw": "1318800564561393",
  "vault_b_raw": "28977577658125",
  "checkpoint_sequence_number": 12345678,
  "timestamp_ms": 1710000000000,
  "tx_digest": "ABC...",
  "event_seq": 0,
  "sender": "0xsender..."
}
```

**Idempotency key:** `(tx_digest, event_seq, protocol)`

**Price rule (frozen):**
- `quote_coin_type` = SUI or USDC if pool contains either; else `coin_type_a` as quote
- `price_quote_per_base` computed from `sqrt_price_after`, decimals, and `a_to_b`

---

## 4. `dex.pool.raw.v1` payload

```json
{
  "protocol": "cetus",
  "event_type": "0x1eab...::factory::CreatePoolEvent",
  "pool_id": "0x9661...",
  "coin_type_a": "0x2::sui::SUI",
  "coin_type_b": "0x...::token::TOKEN",
  "tick_spacing": 60,
  "checkpoint_sequence_number": 12345678,
  "timestamp_ms": 1710000000000,
  "tx_digest": "ABC...",
  "event_seq": 1
}
```

---

## 5. `token.metadata.raw.v1` payload

```json
{
  "coin_type": "0x...::coin::TOKEN",
  "name": "Example Token",
  "symbol": "EXT",
  "decimals": 9,
  "description": "...",
  "image_url": "https://...",
  "creator": "0x...",
  "created_at_ms": 1710000000000,
  "checkpoint_sequence_number": 12345678,
  "tx_digest": "ABC..."
}
```

---

## 6. `coin.balance_change.v1` payload (Phase 4)

```json
{
  "coin_type": "0x...::coin::TOKEN",
  "owner": "0xabc...",
  "delta_raw": "-1000000",
  "balance_after_raw": "5000000",
  "counterparty": "0xdef...",
  "checkpoint_sequence_number": 12345678,
  "timestamp_ms": 1710000000000,
  "tx_digest": "ABC...",
  "effect_index": 2
}
```

---

## 7. Postgres tables (production)

**No `package_events` table in production.** Raw facts live in Kafka (BYOS primary). Postgres holds watermarks + catalog only.

### `watermarks` (required by sui-indexer-alt)

Managed by `crates/indexer-store` — one row per pipeline. See [BYOS](https://docs.sui.io/develop/accessing-data/custom-indexer/bring-your-own-store).

### `protocols`

```sql
CREATE TABLE protocols (
  id            TEXT PRIMARY KEY,  -- cetus, turbos, bluefin, ...
  package_id    TEXT NOT NULL,
  name          TEXT NOT NULL,
  kind          TEXT NOT NULL,      -- clmm
  is_active     BOOLEAN DEFAULT true,
  created_at    TIMESTAMPTZ DEFAULT now()
);
```

### `tokens`

```sql
CREATE TABLE tokens (
  coin_type       TEXT PRIMARY KEY,
  name            TEXT,
  symbol          TEXT,
  decimals        SMALLINT NOT NULL,
  description     TEXT,
  image_url       TEXT,
  creator         TEXT,
  created_at_ms   BIGINT,
  first_seen_cp   BIGINT,
  updated_at      TIMESTAMPTZ DEFAULT now()
);
```

### `pools`

```sql
CREATE TABLE pools (
  pool_id         TEXT PRIMARY KEY,
  protocol        TEXT NOT NULL REFERENCES protocols(id),
  coin_type_a     TEXT NOT NULL,
  coin_type_b     TEXT NOT NULL,
  tick_spacing    INTEGER,
  created_at_ms   BIGINT,
  created_tx      TEXT,
  created_cp      BIGINT,
  is_active       BOOLEAN DEFAULT true
);

CREATE INDEX pools_coin_a_idx ON pools(coin_type_a);
CREATE INDEX pools_coin_b_idx ON pools(coin_type_b);
```

### `token_watchlist`

```sql
CREATE TABLE token_watchlist (
  coin_type   TEXT PRIMARY KEY REFERENCES tokens(coin_type),
  source      TEXT NOT NULL,  -- pool_discovery, manual, trending
  priority    INTEGER DEFAULT 0,
  added_at    TIMESTAMPTZ DEFAULT now()
);
```

---

## 8. TimescaleDB tables (hot)

### `swaps_fact`

```sql
CREATE TABLE swaps_fact (
  time                TIMESTAMPTZ NOT NULL,
  tx_digest           TEXT NOT NULL,
  event_seq           INTEGER NOT NULL,
  protocol            TEXT NOT NULL,
  pool_id             TEXT NOT NULL,
  base_coin_type      TEXT NOT NULL,
  quote_coin_type     TEXT NOT NULL,
  amount_base         NUMERIC NOT NULL,
  amount_quote        NUMERIC NOT NULL,
  price_quote_per_base NUMERIC NOT NULL,
  fee_amount          NUMERIC,
  sender              TEXT,
  checkpoint_seq      BIGINT NOT NULL,
  PRIMARY KEY (time, tx_digest, event_seq, protocol)
);
SELECT create_hypertable('swaps_fact', 'time');
```

### `ohlc_1m`

```sql
CREATE TABLE ohlc_1m (
  bucket              TIMESTAMPTZ NOT NULL,
  pool_id             TEXT NOT NULL,
  base_coin_type      TEXT NOT NULL,
  quote_coin_type     TEXT NOT NULL,
  open                NUMERIC NOT NULL,
  high                NUMERIC NOT NULL,
  low                 NUMERIC NOT NULL,
  close               NUMERIC NOT NULL,
  volume_quote        NUMERIC NOT NULL,
  trade_count         INTEGER NOT NULL,
  PRIMARY KEY (bucket, pool_id, base_coin_type, quote_coin_type)
);
SELECT create_hypertable('ohlc_1m', 'bucket');
```

### `pool_liquidity`

```sql
CREATE TABLE pool_liquidity (
  time            TIMESTAMPTZ NOT NULL,
  pool_id         TEXT NOT NULL,
  vault_a_raw     NUMERIC NOT NULL,
  vault_b_raw     NUMERIC NOT NULL,
  tvl_quote       NUMERIC,
  source          TEXT NOT NULL,  -- swap_event, pool_snapshot
  PRIMARY KEY (time, pool_id, source)
);
SELECT create_hypertable('pool_liquidity', 'time');
```

---

## 9. ClickHouse tables (cold)

Mirror `swaps_fact`, `ohlc_1m`, and (Phase 5) `transfer_edges`.

### `swaps_fact`

```sql
CREATE TABLE swaps_fact (
  time DateTime64(3),
  tx_digest String,
  event_seq Int32,
  protocol String,
  pool_id String,
  base_coin_type String,
  quote_coin_type String,
  amount_base String,
  amount_quote String,
  price_quote_per_base String,
  fee_amount Nullable(String),
  sender Nullable(String),
  checkpoint_seq Int64
) ENGINE = ReplacingMergeTree()
ORDER BY (base_coin_type, time, tx_digest, event_seq, protocol);
```

### `ohlc_1m`

```sql
CREATE TABLE ohlc_1m (
  bucket DateTime64(3),
  pool_id String,
  base_coin_type String,
  quote_coin_type String,
  open String,
  high String,
  low String,
  close String,
  volume_quote String,
  trade_count Int32
) ENGINE = ReplacingMergeTree()
ORDER BY (base_coin_type, pool_id, bucket);
```

### `transfer_edges` (Phase 5)

```sql
CREATE TABLE transfer_edges (
  ts DateTime64(3),
  coin_type String,
  from_addr String,
  to_addr String,
  amount_raw Int128,
  amount_decimal Float64,
  tx_digest String,
  checkpoint_seq UInt64
) ENGINE = MergeTree()
ORDER BY (coin_type, ts, tx_digest);
```

---

## 10. Redis keys

| Key pattern | TTL | Content |
|-------------|-----|---------|
| `token:{coin_type}:price` | 60s | last price + pool_id |
| `token:{coin_type}:vol:24h` | 120s | volume + tx count |
| `pool:{pool_id}:tvl` | 300s | TVL estimate |

---

## 11. API response shapes (Phase 2)

### `GET /v1/tokens/{coin_type}`

```json
{
  "coin_type": "0x...::coin::TOKEN",
  "name": "Token",
  "symbol": "TKN",
  "decimals": 9,
  "image_url": "https://...",
  "price_usd": null,
  "price_quote": { "amount": "0.001", "quote": "SUI" },
  "volume_24h": { "amount": "125000", "quote": "SUI" },
  "txns_24h": 1842,
  "holder_count": null,
  "pools_count": 5
}
```

`holder_count` populated in Phase 4.

### `GET /v1/tokens/{coin_type}/pools`

```json
{
  "coin_type": "0x...::coin::TOKEN",
  "pools": [
    {
      "pool_id": "0x...",
      "protocol": "cetus",
      "coin_type_a": "0x2::sui::SUI",
      "coin_type_b": "0x...::coin::TOKEN",
      "tvl_quote": "125000.5"
    }
  ]
}
```

### `GET /v1/pools/{pool_id}/ohlc`

Query: `interval` = `1m|5m|1h|4h|24h`, `from`, `to` (ISO-8601), optional `base_coin_type`.

```json
{
  "pool_id": "0x...",
  "interval": "1h",
  "bars": [
    {
      "bucket": "2026-06-21T10:00:00Z",
      "open": "1.0",
      "high": "1.2",
      "low": "0.9",
      "close": "1.1",
      "volume_quote": "50000",
      "trade_count": 42
    }
  ]
}
```

### `GET /v1/tokens/{coin_type}/swaps`

Query: `pool_id?`, `limit` (default 50, max 200), `cursor?` (opaque; last `time` + `tx_digest`).

```json
{
  "coin_type": "0x...::coin::TOKEN",
  "swaps": [
    {
      "time": "2026-06-21T10:15:00Z",
      "tx_digest": "ABC...",
      "event_seq": 0,
      "protocol": "cetus",
      "pool_id": "0x...",
      "amount_base": "100",
      "amount_quote": "0.5",
      "price_quote_per_base": "0.005"
    }
  ],
  "next_cursor": null
}
```
