CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE IF NOT EXISTS swaps_fact (
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

SELECT create_hypertable('swaps_fact', 'time', if_not_exists => TRUE);
SELECT add_retention_policy('swaps_fact', INTERVAL '30 days', if_not_exists => TRUE);

CREATE TABLE IF NOT EXISTS ohlc_1m (
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

SELECT create_hypertable('ohlc_1m', 'bucket', if_not_exists => TRUE);
SELECT add_retention_policy('ohlc_1m', INTERVAL '30 days', if_not_exists => TRUE);

CREATE TABLE IF NOT EXISTS pool_liquidity (
    time            TIMESTAMPTZ NOT NULL,
    pool_id         TEXT NOT NULL,
    vault_a_raw     NUMERIC NOT NULL,
    vault_b_raw     NUMERIC NOT NULL,
    tvl_quote       NUMERIC,
    source          TEXT NOT NULL,
    PRIMARY KEY (time, pool_id, source)
);

SELECT create_hypertable('pool_liquidity', 'time', if_not_exists => TRUE);
SELECT add_retention_policy('pool_liquidity', INTERVAL '30 days', if_not_exists => TRUE);
