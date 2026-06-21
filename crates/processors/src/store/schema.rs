diesel::table! {
    protocols (id) {
        id -> Text,
        package_id -> Text,
        name -> Text,
        kind -> Text,
        is_active -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    tokens (coin_type) {
        coin_type -> Text,
        name -> Nullable<Text>,
        symbol -> Nullable<Text>,
        decimals -> Int2,
        description -> Nullable<Text>,
        image_url -> Nullable<Text>,
        creator -> Nullable<Text>,
        created_at_ms -> Nullable<Int8>,
        first_seen_cp -> Nullable<Int8>,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    pools (pool_id) {
        pool_id -> Text,
        protocol -> Text,
        coin_type_a -> Text,
        coin_type_b -> Text,
        tick_spacing -> Nullable<Int4>,
        created_at_ms -> Nullable<Int8>,
        created_tx -> Nullable<Text>,
        created_cp -> Nullable<Int8>,
        is_active -> Bool,
    }
}

diesel::table! {
    token_watchlist (coin_type) {
        coin_type -> Text,
        source -> Text,
        priority -> Int4,
        added_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(protocols, tokens, pools, token_watchlist);
