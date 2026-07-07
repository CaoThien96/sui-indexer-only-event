-- Drop token USD OHLC continuous aggregates (dependents first).
DROP MATERIALIZED VIEW IF EXISTS token_ohlc_usd_24h CASCADE;
DROP MATERIALIZED VIEW IF EXISTS token_ohlc_usd_4h CASCADE;
DROP MATERIALIZED VIEW IF EXISTS token_ohlc_usd_1h CASCADE;
DROP MATERIALIZED VIEW IF EXISTS token_ohlc_usd_30m CASCADE;
DROP MATERIALIZED VIEW IF EXISTS token_ohlc_usd_15m CASCADE;
DROP MATERIALIZED VIEW IF EXISTS token_ohlc_usd_5m CASCADE;
DROP MATERIALIZED VIEW IF EXISTS token_ohlc_usd_1m CASCADE;

-- Drop pool OHLC continuous aggregates and base hypertable.
DROP MATERIALIZED VIEW IF EXISTS ohlc_24h CASCADE;
DROP MATERIALIZED VIEW IF EXISTS ohlc_4h CASCADE;
DROP MATERIALIZED VIEW IF EXISTS ohlc_1h CASCADE;
DROP MATERIALIZED VIEW IF EXISTS ohlc_30m CASCADE;
DROP MATERIALIZED VIEW IF EXISTS ohlc_15m CASCADE;
DROP MATERIALIZED VIEW IF EXISTS ohlc_5m CASCADE;
DROP TABLE IF EXISTS ohlc_1m CASCADE;

-- Token USD OHLC hypertables (direct upsert from volume-engine).
CREATE TABLE IF NOT EXISTS token_ohlc_usd_1m (
    bucket          TIMESTAMPTZ NOT NULL,
    base_coin_type  TEXT NOT NULL,
    open_usd        NUMERIC NOT NULL,
    high_usd        NUMERIC NOT NULL,
    low_usd         NUMERIC NOT NULL,
    close_usd       NUMERIC NOT NULL,
    volume_usd      NUMERIC NOT NULL,
    trade_count     INTEGER NOT NULL,
    PRIMARY KEY (bucket, base_coin_type)
);
SELECT create_hypertable('token_ohlc_usd_1m', 'bucket', if_not_exists => TRUE);

CREATE TABLE IF NOT EXISTS token_ohlc_usd_5m (
    bucket          TIMESTAMPTZ NOT NULL,
    base_coin_type  TEXT NOT NULL,
    open_usd        NUMERIC NOT NULL,
    high_usd        NUMERIC NOT NULL,
    low_usd         NUMERIC NOT NULL,
    close_usd       NUMERIC NOT NULL,
    volume_usd      NUMERIC NOT NULL,
    trade_count     INTEGER NOT NULL,
    PRIMARY KEY (bucket, base_coin_type)
);
SELECT create_hypertable('token_ohlc_usd_5m', 'bucket', if_not_exists => TRUE);

CREATE TABLE IF NOT EXISTS token_ohlc_usd_15m (
    bucket          TIMESTAMPTZ NOT NULL,
    base_coin_type  TEXT NOT NULL,
    open_usd        NUMERIC NOT NULL,
    high_usd        NUMERIC NOT NULL,
    low_usd         NUMERIC NOT NULL,
    close_usd       NUMERIC NOT NULL,
    volume_usd      NUMERIC NOT NULL,
    trade_count     INTEGER NOT NULL,
    PRIMARY KEY (bucket, base_coin_type)
);
SELECT create_hypertable('token_ohlc_usd_15m', 'bucket', if_not_exists => TRUE);

CREATE TABLE IF NOT EXISTS token_ohlc_usd_30m (
    bucket          TIMESTAMPTZ NOT NULL,
    base_coin_type  TEXT NOT NULL,
    open_usd        NUMERIC NOT NULL,
    high_usd        NUMERIC NOT NULL,
    low_usd         NUMERIC NOT NULL,
    close_usd       NUMERIC NOT NULL,
    volume_usd      NUMERIC NOT NULL,
    trade_count     INTEGER NOT NULL,
    PRIMARY KEY (bucket, base_coin_type)
);
SELECT create_hypertable('token_ohlc_usd_30m', 'bucket', if_not_exists => TRUE);

CREATE TABLE IF NOT EXISTS token_ohlc_usd_1h (
    bucket          TIMESTAMPTZ NOT NULL,
    base_coin_type  TEXT NOT NULL,
    open_usd        NUMERIC NOT NULL,
    high_usd        NUMERIC NOT NULL,
    low_usd         NUMERIC NOT NULL,
    close_usd       NUMERIC NOT NULL,
    volume_usd      NUMERIC NOT NULL,
    trade_count     INTEGER NOT NULL,
    PRIMARY KEY (bucket, base_coin_type)
);
SELECT create_hypertable('token_ohlc_usd_1h', 'bucket', if_not_exists => TRUE);

CREATE TABLE IF NOT EXISTS token_ohlc_usd_4h (
    bucket          TIMESTAMPTZ NOT NULL,
    base_coin_type  TEXT NOT NULL,
    open_usd        NUMERIC NOT NULL,
    high_usd        NUMERIC NOT NULL,
    low_usd         NUMERIC NOT NULL,
    close_usd       NUMERIC NOT NULL,
    volume_usd      NUMERIC NOT NULL,
    trade_count     INTEGER NOT NULL,
    PRIMARY KEY (bucket, base_coin_type)
);
SELECT create_hypertable('token_ohlc_usd_4h', 'bucket', if_not_exists => TRUE);

CREATE TABLE IF NOT EXISTS token_ohlc_usd_24h (
    bucket          TIMESTAMPTZ NOT NULL,
    base_coin_type  TEXT NOT NULL,
    open_usd        NUMERIC NOT NULL,
    high_usd        NUMERIC NOT NULL,
    low_usd         NUMERIC NOT NULL,
    close_usd       NUMERIC NOT NULL,
    volume_usd      NUMERIC NOT NULL,
    trade_count     INTEGER NOT NULL,
    PRIMARY KEY (bucket, base_coin_type)
);
SELECT create_hypertable('token_ohlc_usd_24h', 'bucket', if_not_exists => TRUE);

-- Backfill from swaps_fact and rollups (idempotent).
INSERT INTO token_ohlc_usd_1m (
    bucket, base_coin_type, open_usd, high_usd, low_usd, close_usd, volume_usd, trade_count
)
SELECT
    time_bucket('1 minute', time) AS bucket,
    base_coin_type,
    first(price_usd_per_base, time),
    max(price_usd_per_base),
    min(price_usd_per_base),
    last(price_usd_per_base, time),
    sum(amount_usd),
    count(*)::integer
FROM swaps_fact
WHERE price_usd_per_base IS NOT NULL
  AND amount_usd IS NOT NULL
GROUP BY 1, 2
ON CONFLICT (bucket, base_coin_type) DO NOTHING;

INSERT INTO token_ohlc_usd_5m (
    bucket, base_coin_type, open_usd, high_usd, low_usd, close_usd, volume_usd, trade_count
)
SELECT
    time_bucket('5 minutes', bucket),
    base_coin_type,
    first(open_usd, bucket),
    max(high_usd),
    min(low_usd),
    last(close_usd, bucket),
    sum(volume_usd),
    sum(trade_count)::integer
FROM token_ohlc_usd_1m
GROUP BY 1, 2
ON CONFLICT (bucket, base_coin_type) DO NOTHING;

INSERT INTO token_ohlc_usd_15m (
    bucket, base_coin_type, open_usd, high_usd, low_usd, close_usd, volume_usd, trade_count
)
SELECT
    time_bucket('15 minutes', bucket),
    base_coin_type,
    first(open_usd, bucket),
    max(high_usd),
    min(low_usd),
    last(close_usd, bucket),
    sum(volume_usd),
    sum(trade_count)::integer
FROM token_ohlc_usd_5m
GROUP BY 1, 2
ON CONFLICT (bucket, base_coin_type) DO NOTHING;

INSERT INTO token_ohlc_usd_30m (
    bucket, base_coin_type, open_usd, high_usd, low_usd, close_usd, volume_usd, trade_count
)
SELECT
    time_bucket('30 minutes', bucket),
    base_coin_type,
    first(open_usd, bucket),
    max(high_usd),
    min(low_usd),
    last(close_usd, bucket),
    sum(volume_usd),
    sum(trade_count)::integer
FROM token_ohlc_usd_5m
GROUP BY 1, 2
ON CONFLICT (bucket, base_coin_type) DO NOTHING;

INSERT INTO token_ohlc_usd_1h (
    bucket, base_coin_type, open_usd, high_usd, low_usd, close_usd, volume_usd, trade_count
)
SELECT
    time_bucket('1 hour', bucket),
    base_coin_type,
    first(open_usd, bucket),
    max(high_usd),
    min(low_usd),
    last(close_usd, bucket),
    sum(volume_usd),
    sum(trade_count)::integer
FROM token_ohlc_usd_1m
GROUP BY 1, 2
ON CONFLICT (bucket, base_coin_type) DO NOTHING;

INSERT INTO token_ohlc_usd_4h (
    bucket, base_coin_type, open_usd, high_usd, low_usd, close_usd, volume_usd, trade_count
)
SELECT
    time_bucket('4 hours', bucket),
    base_coin_type,
    first(open_usd, bucket),
    max(high_usd),
    min(low_usd),
    last(close_usd, bucket),
    sum(volume_usd),
    sum(trade_count)::integer
FROM token_ohlc_usd_1m
GROUP BY 1, 2
ON CONFLICT (bucket, base_coin_type) DO NOTHING;

INSERT INTO token_ohlc_usd_24h (
    bucket, base_coin_type, open_usd, high_usd, low_usd, close_usd, volume_usd, trade_count
)
SELECT
    time_bucket('24 hours', bucket),
    base_coin_type,
    first(open_usd, bucket),
    max(high_usd),
    min(low_usd),
    last(close_usd, bucket),
    sum(volume_usd),
    sum(trade_count)::integer
FROM token_ohlc_usd_1m
GROUP BY 1, 2
ON CONFLICT (bucket, base_coin_type) DO NOTHING;
