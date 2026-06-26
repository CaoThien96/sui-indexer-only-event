CREATE TYPE bot_token_status AS ENUM ('created', 'listing', 'done');
CREATE TYPE bot_dex AS ENUM ('CETUS', 'TURBOS');

CREATE TABLE bot_tokens (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    symbol VARCHAR NOT NULL,
    decimals INTEGER NOT NULL,
    total_supply BIGINT NOT NULL,
    owner VARCHAR NOT NULL DEFAULT '',
    deny_cap_id VARCHAR NOT NULL DEFAULT '',
    status bot_token_status NOT NULL,
    pool_id VARCHAR,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE bot_pools (
    id VARCHAR PRIMARY KEY,
    token_id VARCHAR NOT NULL REFERENCES bot_tokens(id),
    dex bot_dex NOT NULL,
    tx_digest VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE bot_processed_swaps (
    id VARCHAR PRIMARY KEY,
    pool_id VARCHAR NOT NULL,
    tx_digest VARCHAR NOT NULL,
    event_seq VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE bot_processed_events (
    id VARCHAR PRIMARY KEY,
    event_type VARCHAR NOT NULL,
    tx_digest VARCHAR NOT NULL,
    event_seq VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX bot_tokens_status_idx ON bot_tokens(status);
CREATE INDEX bot_pools_token_id_idx ON bot_pools(token_id);
CREATE INDEX bot_processed_swaps_pool_id_idx ON bot_processed_swaps(pool_id);
