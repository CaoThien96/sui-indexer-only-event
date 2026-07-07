CREATE DATABASE IF NOT EXISTS sui_metrics;

CREATE TABLE IF NOT EXISTS sui_metrics.swaps_fact (
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
    amount_usd Nullable(String),
    price_usd_per_base Nullable(String),
    fee_amount Nullable(String),
    sender Nullable(String),
    checkpoint_seq Int64
) ENGINE = ReplacingMergeTree()
ORDER BY (base_coin_type, time, tx_digest, event_seq, protocol);

CREATE TABLE IF NOT EXISTS sui_metrics.token_ohlc_usd_1m (
    bucket DateTime64(3),
    base_coin_type String,
    open_usd String,
    high_usd String,
    low_usd String,
    close_usd String,
    volume_usd String,
    trade_count Int32
) ENGINE = ReplacingMergeTree()
ORDER BY (base_coin_type, bucket);

CREATE TABLE IF NOT EXISTS sui_metrics.token_ohlc_usd_5m (
    bucket DateTime64(3),
    base_coin_type String,
    open_usd String,
    high_usd String,
    low_usd String,
    close_usd String,
    volume_usd String,
    trade_count Int32
) ENGINE = ReplacingMergeTree()
ORDER BY (base_coin_type, bucket);

CREATE TABLE IF NOT EXISTS sui_metrics.token_ohlc_usd_15m (
    bucket DateTime64(3),
    base_coin_type String,
    open_usd String,
    high_usd String,
    low_usd String,
    close_usd String,
    volume_usd String,
    trade_count Int32
) ENGINE = ReplacingMergeTree()
ORDER BY (base_coin_type, bucket);

CREATE TABLE IF NOT EXISTS sui_metrics.token_ohlc_usd_30m (
    bucket DateTime64(3),
    base_coin_type String,
    open_usd String,
    high_usd String,
    low_usd String,
    close_usd String,
    volume_usd String,
    trade_count Int32
) ENGINE = ReplacingMergeTree()
ORDER BY (base_coin_type, bucket);

CREATE TABLE IF NOT EXISTS sui_metrics.token_ohlc_usd_1h (
    bucket DateTime64(3),
    base_coin_type String,
    open_usd String,
    high_usd String,
    low_usd String,
    close_usd String,
    volume_usd String,
    trade_count Int32
) ENGINE = ReplacingMergeTree()
ORDER BY (base_coin_type, bucket);

CREATE TABLE IF NOT EXISTS sui_metrics.token_ohlc_usd_4h (
    bucket DateTime64(3),
    base_coin_type String,
    open_usd String,
    high_usd String,
    low_usd String,
    close_usd String,
    volume_usd String,
    trade_count Int32
) ENGINE = ReplacingMergeTree()
ORDER BY (base_coin_type, bucket);

CREATE TABLE IF NOT EXISTS sui_metrics.token_ohlc_usd_24h (
    bucket DateTime64(3),
    base_coin_type String,
    open_usd String,
    high_usd String,
    low_usd String,
    close_usd String,
    volume_usd String,
    trade_count Int32
) ENGINE = ReplacingMergeTree()
ORDER BY (base_coin_type, bucket);
