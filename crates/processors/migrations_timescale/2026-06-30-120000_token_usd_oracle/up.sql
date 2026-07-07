CREATE TABLE IF NOT EXISTS token_usd_1m (
    bucket          TIMESTAMPTZ NOT NULL,
    base_coin_type  TEXT NOT NULL,
    price_usd       NUMERIC NOT NULL,
    source_type     TEXT NOT NULL,
    source_pool_id  TEXT,
    quality_score   NUMERIC NOT NULL DEFAULT 0,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (bucket, base_coin_type)
);

SELECT create_hypertable('token_usd_1m', 'bucket', if_not_exists => TRUE);
SELECT add_retention_policy('token_usd_1m', INTERVAL '30 days', if_not_exists => TRUE);

CREATE INDEX IF NOT EXISTS token_usd_1m_coin_bucket_idx
    ON token_usd_1m (base_coin_type, bucket DESC);
