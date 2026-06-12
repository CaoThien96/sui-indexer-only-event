CREATE TABLE IF NOT EXISTS watermarks
(
    pipeline TEXT PRIMARY KEY,
    epoch_hi_inclusive BIGINT NOT NULL,
    checkpoint_hi_inclusive BIGINT NOT NULL,
    tx_hi BIGINT NOT NULL,
    timestamp_ms_hi_inclusive BIGINT NOT NULL,
    reader_lo BIGINT NOT NULL,
    pruner_timestamp TIMESTAMP NOT NULL,
    pruner_hi BIGINT NOT NULL
);
