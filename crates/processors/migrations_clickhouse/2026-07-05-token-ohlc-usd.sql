DROP TABLE IF EXISTS sui_metrics.ohlc_1m;

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
