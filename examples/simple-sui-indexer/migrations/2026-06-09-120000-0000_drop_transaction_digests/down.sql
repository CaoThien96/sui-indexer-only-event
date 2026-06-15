CREATE TABLE transaction_digests (
    tx_digest TEXT PRIMARY KEY,
    checkpoint_sequence_number BIGINT NOT NULL
);
CREATE INDEX transaction_digests_checkpoint_sequence_number
    ON transaction_digests (checkpoint_sequence_number);
