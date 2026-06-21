CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_5m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('5 minutes', bucket) AS bucket,
    pool_id,
    base_coin_type,
    quote_coin_type,
    first(open, bucket) AS open,
    max(high) AS high,
    min(low) AS low,
    last(close, bucket) AS close,
    sum(volume_quote) AS volume_quote,
    sum(trade_count) AS trade_count
FROM ohlc_1m
GROUP BY 1, 2, 3, 4
WITH NO DATA;

SELECT add_continuous_aggregate_policy('ohlc_5m',
    start_offset => INTERVAL '1 hour',
    end_offset => INTERVAL '5 minutes',
    schedule_interval => INTERVAL '5 minutes',
    if_not_exists => TRUE);

CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_1h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', bucket) AS bucket,
    pool_id,
    base_coin_type,
    quote_coin_type,
    first(open, bucket) AS open,
    max(high) AS high,
    min(low) AS low,
    last(close, bucket) AS close,
    sum(volume_quote) AS volume_quote,
    sum(trade_count) AS trade_count
FROM ohlc_1m
GROUP BY 1, 2, 3, 4
WITH NO DATA;

SELECT add_continuous_aggregate_policy('ohlc_1h',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour',
    if_not_exists => TRUE);

CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_4h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('4 hours', bucket) AS bucket,
    pool_id,
    base_coin_type,
    quote_coin_type,
    first(open, bucket) AS open,
    max(high) AS high,
    min(low) AS low,
    last(close, bucket) AS close,
    sum(volume_quote) AS volume_quote,
    sum(trade_count) AS trade_count
FROM ohlc_1m
GROUP BY 1, 2, 3, 4
WITH NO DATA;

SELECT add_continuous_aggregate_policy('ohlc_4h',
    start_offset => INTERVAL '12 hours',
    end_offset => INTERVAL '4 hours',
    schedule_interval => INTERVAL '4 hours',
    if_not_exists => TRUE);

CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_24h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('24 hours', bucket) AS bucket,
    pool_id,
    base_coin_type,
    quote_coin_type,
    first(open, bucket) AS open,
    max(high) AS high,
    min(low) AS low,
    last(close, bucket) AS close,
    sum(volume_quote) AS volume_quote,
    sum(trade_count) AS trade_count
FROM ohlc_1m
GROUP BY 1, 2, 3, 4
WITH NO DATA;

SELECT add_continuous_aggregate_policy('ohlc_24h',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '24 hours',
    schedule_interval => INTERVAL '24 hours',
    if_not_exists => TRUE);

CREATE MATERIALIZED VIEW IF NOT EXISTS token_volume_1h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', time) AS bucket,
    base_coin_type,
    quote_coin_type,
    sum(amount_quote) AS volume_quote,
    count(*) AS tx_count
FROM swaps_fact
GROUP BY 1, 2, 3
WITH NO DATA;

SELECT add_continuous_aggregate_policy('token_volume_1h',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour',
    if_not_exists => TRUE);

CREATE MATERIALIZED VIEW IF NOT EXISTS token_volume_6h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('6 hours', time) AS bucket,
    base_coin_type,
    quote_coin_type,
    sum(amount_quote) AS volume_quote,
    count(*) AS tx_count
FROM swaps_fact
GROUP BY 1, 2, 3
WITH NO DATA;

SELECT add_continuous_aggregate_policy('token_volume_6h',
    start_offset => INTERVAL '18 hours',
    end_offset => INTERVAL '6 hours',
    schedule_interval => INTERVAL '6 hours',
    if_not_exists => TRUE);

CREATE MATERIALIZED VIEW IF NOT EXISTS token_volume_24h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('24 hours', time) AS bucket,
    base_coin_type,
    quote_coin_type,
    sum(amount_quote) AS volume_quote,
    count(*) AS tx_count
FROM swaps_fact
GROUP BY 1, 2, 3
WITH NO DATA;

SELECT add_continuous_aggregate_policy('token_volume_24h',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '24 hours',
    schedule_interval => INTERVAL '24 hours',
    if_not_exists => TRUE);
