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
    fee_amount Nullable(String),
    sender Nullable(String),
    checkpoint_seq Int64
) ENGINE = ReplacingMergeTree()
ORDER BY (base_coin_type, time, tx_digest, event_seq, protocol);

CREATE TABLE IF NOT EXISTS sui_metrics.ohlc_1m (
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
