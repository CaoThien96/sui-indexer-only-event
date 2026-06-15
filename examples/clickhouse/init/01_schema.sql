CREATE DATABASE IF NOT EXISTS sui_indexer;

CREATE TABLE IF NOT EXISTS sui_indexer.package_events
(
    event_id_tx_digest String,
    event_id_seq Int64,
    checkpoint_sequence_number Int64,
    transaction_sequence_in_checkpoint Int32,
    event_sequence_in_transaction Int32,
    package_id String,
    transaction_module Nullable(String),
    event_type String,
    sender Nullable(String),
    timestamp_ms Nullable(Int64),
    bcs String,
    json String,
    parsed_json Nullable(String),
    inserted_at DateTime64(3) DEFAULT now64(3),
    INDEX idx_event_id (event_id_tx_digest, event_id_seq) TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(inserted_at)
PARTITION BY toYYYYMMDD(
    coalesce(
        toDateTime(intDiv(timestamp_ms, 1000)),
        toDateTime(inserted_at)
    )
)
ORDER BY (
    event_type,
    checkpoint_sequence_number,
    transaction_sequence_in_checkpoint,
    event_sequence_in_transaction,
    event_id_tx_digest,
    event_id_seq
)
TTL coalesce(
    toDateTime(intDiv(timestamp_ms, 1000)),
    toDateTime(inserted_at)
) + INTERVAL 3 DAY TO VOLUME 'cold'
SETTINGS storage_policy = 'hot_cold';
