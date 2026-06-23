CREATE TABLE IF NOT EXISTS rolloff_watermarks (
    table_name      TEXT PRIMARY KEY,
    last_rolled_time TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01'::timestamptz
);
