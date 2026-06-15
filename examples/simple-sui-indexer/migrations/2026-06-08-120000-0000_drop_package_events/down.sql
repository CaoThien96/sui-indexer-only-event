CREATE TABLE package_events (
    event_id_tx_digest TEXT NOT NULL,
    event_id_seq BIGINT NOT NULL,
    checkpoint_sequence_number BIGINT NOT NULL,
    transaction_sequence_in_checkpoint INTEGER NOT NULL,
    event_sequence_in_transaction INTEGER NOT NULL,
    package_id TEXT NOT NULL,
    transaction_module TEXT,
    event_type TEXT NOT NULL,
    sender TEXT,
    timestamp_ms BIGINT,
    bcs BYTEA NOT NULL,
    json JSONB NOT NULL,
    parsed_json JSONB,
    PRIMARY KEY (event_id_tx_digest, event_id_seq)
);

CREATE INDEX package_events_package_type_cursor_idx
    ON package_events (
        package_id,
        event_type,
        checkpoint_sequence_number,
        transaction_sequence_in_checkpoint,
        event_sequence_in_transaction
    );

CREATE INDEX package_events_package_cursor_idx
    ON package_events (
        package_id,
        checkpoint_sequence_number,
        transaction_sequence_in_checkpoint,
        event_sequence_in_transaction
    );

CREATE INDEX package_events_event_type_cursor_idx
    ON package_events (
        event_type,
        checkpoint_sequence_number,
        transaction_sequence_in_checkpoint,
        event_sequence_in_transaction
    );

CREATE INDEX package_events_sender_cursor_idx
    ON package_events (
        sender,
        checkpoint_sequence_number,
        transaction_sequence_in_checkpoint,
        event_sequence_in_transaction
    );
