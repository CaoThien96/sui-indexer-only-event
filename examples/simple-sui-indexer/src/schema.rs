// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "bot_dex"))]
    pub struct BotDex;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "bot_token_status"))]
    pub struct BotTokenStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BotDex;

    bot_pools (id) {
        id -> Varchar,
        token_id -> Varchar,
        dex -> BotDex,
        tx_digest -> Varchar,
        initial_shared_version -> Nullable<Int8>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    bot_processed_events (id) {
        id -> Varchar,
        event_type -> Varchar,
        tx_digest -> Varchar,
        event_seq -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    bot_processed_swaps (id) {
        id -> Varchar,
        pool_id -> Varchar,
        tx_digest -> Varchar,
        event_seq -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BotTokenStatus;

    bot_tokens (id) {
        id -> Varchar,
        name -> Varchar,
        symbol -> Varchar,
        decimals -> Int4,
        total_supply -> Int8,
        owner -> Varchar,
        deny_cap_id -> Varchar,
        status -> BotTokenStatus,
        pool_id -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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

diesel::joinable!(bot_pools -> bot_tokens (token_id));

diesel::allow_tables_to_appear_in_same_query!(
    bot_pools,
    bot_processed_events,
    bot_processed_swaps,
    bot_tokens,
    watermarks,
);
