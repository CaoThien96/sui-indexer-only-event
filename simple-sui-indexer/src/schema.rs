// @generated automatically by Diesel CLI.

diesel::table! {
    package_events (event_id_tx_digest, event_id_seq) {
        event_id_tx_digest -> Text,
        event_id_seq -> Int8,
        checkpoint_sequence_number -> Int8,
        transaction_sequence_in_checkpoint -> Int4,
        event_sequence_in_transaction -> Int4,
        package_id -> Text,
        transaction_module -> Nullable<Text>,
        event_type -> Text,
        sender -> Nullable<Text>,
        timestamp_ms -> Nullable<Int8>,
        bcs -> Bytea,
        json -> Jsonb,
    }
}

diesel::table! {
    transaction_digests (tx_digest) {
        tx_digest -> Text,
        checkpoint_sequence_number -> Int8,
    }
}

diesel::table! {
    watermarks (pipeline) {
        pipeline -> Text,
        epoch_hi_inclusive -> Int8,
        checkpoint_hi_inclusive -> Int8,
        tx_hi -> Int8,
        timestamp_ms_hi_inclusive -> Int8,
        reader_lo -> Int8,
        pruner_timestamp -> Timestamp,
        pruner_hi -> Int8,
        chain_id -> Nullable<Bytea>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(package_events, transaction_digests, watermarks,);
