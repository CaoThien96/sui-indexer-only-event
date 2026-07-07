DROP TABLE IF EXISTS token_ohlc_usd_24h CASCADE;
DROP TABLE IF EXISTS token_ohlc_usd_4h CASCADE;
DROP TABLE IF EXISTS token_ohlc_usd_1h CASCADE;
DROP TABLE IF EXISTS token_ohlc_usd_30m CASCADE;
DROP TABLE IF EXISTS token_ohlc_usd_15m CASCADE;
DROP TABLE IF EXISTS token_ohlc_usd_5m CASCADE;
DROP TABLE IF EXISTS token_ohlc_usd_1m CASCADE;

-- Pool ohlc_1m and CAGG views are not recreated on rollback.
