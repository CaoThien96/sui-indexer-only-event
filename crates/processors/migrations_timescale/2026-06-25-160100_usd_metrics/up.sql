ALTER TABLE swaps_fact
    ADD COLUMN IF NOT EXISTS amount_usd NUMERIC,
    ADD COLUMN IF NOT EXISTS price_usd_per_base NUMERIC;

ALTER TABLE ohlc_1m
    ADD COLUMN IF NOT EXISTS open_usd NUMERIC,
    ADD COLUMN IF NOT EXISTS high_usd NUMERIC,
    ADD COLUMN IF NOT EXISTS low_usd NUMERIC,
    ADD COLUMN IF NOT EXISTS close_usd NUMERIC,
    ADD COLUMN IF NOT EXISTS volume_usd NUMERIC;

ALTER TABLE pool_liquidity
    ADD COLUMN IF NOT EXISTS tvl_usd NUMERIC;

CREATE TABLE IF NOT EXISTS sui_usd_1m (
    bucket          TIMESTAMPTZ PRIMARY KEY,
    price_usd       NUMERIC NOT NULL,
    source_type     TEXT NOT NULL,
    source_pool_id  TEXT,
    quality_score   NUMERIC NOT NULL DEFAULT 0,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

SELECT create_hypertable('sui_usd_1m', 'bucket', if_not_exists => TRUE);
SELECT add_retention_policy('sui_usd_1m', INTERVAL '30 days', if_not_exists => TRUE);

CREATE TABLE IF NOT EXISTS bootstrap_state (
    run_id               TEXT PRIMARY KEY,
    target_first_checkpoint BIGINT NOT NULL,
    status               TEXT NOT NULL,
    iteration            INTEGER NOT NULL DEFAULT 0,
    window_start_checkpoint BIGINT NOT NULL,
    window_end_checkpoint BIGINT NOT NULL,
    metrics_json         JSONB NOT NULL DEFAULT '{}'::jsonb,
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE MATERIALIZED VIEW IF NOT EXISTS token_ohlc_usd_1m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 minute', time) AS bucket,
    base_coin_type,
    first(price_usd_per_base, time) AS open_usd,
    max(price_usd_per_base) AS high_usd,
    min(price_usd_per_base) AS low_usd,
    last(price_usd_per_base, time) AS close_usd,
    sum(amount_usd) AS volume_usd,
    count(*) AS trade_count
FROM swaps_fact
WHERE price_usd_per_base IS NOT NULL
  AND amount_usd IS NOT NULL
GROUP BY 1, 2
WITH NO DATA;

SELECT add_continuous_aggregate_policy('token_ohlc_usd_1m',
    start_offset => INTERVAL '2 hours',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute',
    if_not_exists => TRUE);

CREATE MATERIALIZED VIEW IF NOT EXISTS token_ohlc_usd_5m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('5 minutes', bucket) AS bucket,
    base_coin_type,
    first(open_usd, bucket) AS open_usd,
    max(high_usd) AS high_usd,
    min(low_usd) AS low_usd,
    last(close_usd, bucket) AS close_usd,
    sum(volume_usd) AS volume_usd,
    sum(trade_count) AS trade_count
FROM token_ohlc_usd_1m
GROUP BY 1, 2
WITH NO DATA;

SELECT add_continuous_aggregate_policy('token_ohlc_usd_5m',
    start_offset => INTERVAL '6 hours',
    end_offset => INTERVAL '5 minutes',
    schedule_interval => INTERVAL '5 minutes',
    if_not_exists => TRUE);

CREATE MATERIALIZED VIEW IF NOT EXISTS token_ohlc_usd_1h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', bucket) AS bucket,
    base_coin_type,
    first(open_usd, bucket) AS open_usd,
    max(high_usd) AS high_usd,
    min(low_usd) AS low_usd,
    last(close_usd, bucket) AS close_usd,
    sum(volume_usd) AS volume_usd,
    sum(trade_count) AS trade_count
FROM token_ohlc_usd_1m
GROUP BY 1, 2
WITH NO DATA;

SELECT add_continuous_aggregate_policy('token_ohlc_usd_1h',
    start_offset => INTERVAL '2 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour',
    if_not_exists => TRUE);

CREATE MATERIALIZED VIEW IF NOT EXISTS token_ohlc_usd_4h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('4 hours', bucket) AS bucket,
    base_coin_type,
    first(open_usd, bucket) AS open_usd,
    max(high_usd) AS high_usd,
    min(low_usd) AS low_usd,
    last(close_usd, bucket) AS close_usd,
    sum(volume_usd) AS volume_usd,
    sum(trade_count) AS trade_count
FROM token_ohlc_usd_1m
GROUP BY 1, 2
WITH NO DATA;

SELECT add_continuous_aggregate_policy('token_ohlc_usd_4h',
    start_offset => INTERVAL '5 days',
    end_offset => INTERVAL '4 hours',
    schedule_interval => INTERVAL '4 hours',
    if_not_exists => TRUE);

CREATE MATERIALIZED VIEW IF NOT EXISTS token_ohlc_usd_24h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('24 hours', bucket) AS bucket,
    base_coin_type,
    first(open_usd, bucket) AS open_usd,
    max(high_usd) AS high_usd,
    min(low_usd) AS low_usd,
    last(close_usd, bucket) AS close_usd,
    sum(volume_usd) AS volume_usd,
    sum(trade_count) AS trade_count
FROM token_ohlc_usd_1m
GROUP BY 1, 2
WITH NO DATA;

SELECT add_continuous_aggregate_policy('token_ohlc_usd_24h',
    start_offset => INTERVAL '30 days',
    end_offset => INTERVAL '24 hours',
    schedule_interval => INTERVAL '24 hours',
    if_not_exists => TRUE);
