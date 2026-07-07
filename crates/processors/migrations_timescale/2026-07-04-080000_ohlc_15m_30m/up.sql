-- Pool-level 15-minute OHLC (rolls up from ohlc_5m)
CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_15m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('15 minutes', bucket) AS bucket,
    pool_id,
    base_coin_type,
    quote_coin_type,
    first(open, bucket) AS open,
    max(high) AS high,
    min(low) AS low,
    last(close, bucket) AS close,
    sum(volume_quote) AS volume_quote,
    sum(trade_count) AS trade_count
FROM ohlc_5m
GROUP BY 1, 2, 3, 4
WITH NO DATA;

SELECT add_continuous_aggregate_policy('ohlc_15m',
    start_offset => INTERVAL '2 hours',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '15 minutes',
    if_not_exists => TRUE);

-- Pool-level 30-minute OHLC (rolls up from ohlc_5m)
CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_30m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('30 minutes', bucket) AS bucket,
    pool_id,
    base_coin_type,
    quote_coin_type,
    first(open, bucket) AS open,
    max(high) AS high,
    min(low) AS low,
    last(close, bucket) AS close,
    sum(volume_quote) AS volume_quote,
    sum(trade_count) AS trade_count
FROM ohlc_5m
GROUP BY 1, 2, 3, 4
WITH NO DATA;

SELECT add_continuous_aggregate_policy('ohlc_30m',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '30 minutes',
    schedule_interval => INTERVAL '30 minutes',
    if_not_exists => TRUE);

-- Token-level USD 15-minute OHLC (rolls up from token_ohlc_usd_5m)
CREATE MATERIALIZED VIEW IF NOT EXISTS token_ohlc_usd_15m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('15 minutes', bucket) AS bucket,
    base_coin_type,
    first(open_usd, bucket) AS open_usd,
    max(high_usd) AS high_usd,
    min(low_usd) AS low_usd,
    last(close_usd, bucket) AS close_usd,
    sum(volume_usd) AS volume_usd,
    sum(trade_count) AS trade_count
FROM token_ohlc_usd_5m
GROUP BY 1, 2
WITH NO DATA;

SELECT add_continuous_aggregate_policy('token_ohlc_usd_15m',
    start_offset => INTERVAL '6 hours',
    end_offset => INTERVAL '15 minutes',
    schedule_interval => INTERVAL '15 minutes',
    if_not_exists => TRUE);

-- Token-level USD 30-minute OHLC (rolls up from token_ohlc_usd_5m)
CREATE MATERIALIZED VIEW IF NOT EXISTS token_ohlc_usd_30m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('30 minutes', bucket) AS bucket,
    base_coin_type,
    first(open_usd, bucket) AS open_usd,
    max(high_usd) AS high_usd,
    min(low_usd) AS low_usd,
    last(close_usd, bucket) AS close_usd,
    sum(volume_usd) AS volume_usd,
    sum(trade_count) AS trade_count
FROM token_ohlc_usd_5m
GROUP BY 1, 2
WITH NO DATA;

SELECT add_continuous_aggregate_policy('token_ohlc_usd_30m',
    start_offset => INTERVAL '12 hours',
    end_offset => INTERVAL '30 minutes',
    schedule_interval => INTERVAL '30 minutes',
    if_not_exists => TRUE);
