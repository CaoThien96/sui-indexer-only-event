CREATE TABLE IF NOT EXISTS protocols (
    id            TEXT PRIMARY KEY,
    package_id    TEXT NOT NULL,
    name          TEXT NOT NULL,
    kind          TEXT NOT NULL DEFAULT 'clmm',
    is_active     BOOLEAN NOT NULL DEFAULT true,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS tokens (
    coin_type       TEXT PRIMARY KEY,
    name            TEXT,
    symbol          TEXT,
    decimals        SMALLINT NOT NULL,
    description     TEXT,
    image_url       TEXT,
    creator         TEXT,
    created_at_ms   BIGINT,
    first_seen_cp   BIGINT,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS pools (
    pool_id         TEXT PRIMARY KEY,
    protocol        TEXT NOT NULL REFERENCES protocols(id),
    coin_type_a     TEXT NOT NULL,
    coin_type_b     TEXT NOT NULL,
    tick_spacing    INTEGER,
    created_at_ms   BIGINT,
    created_tx      TEXT,
    created_cp      BIGINT,
    is_active       BOOLEAN NOT NULL DEFAULT true
);

CREATE INDEX IF NOT EXISTS pools_coin_a_idx ON pools(coin_type_a);
CREATE INDEX IF NOT EXISTS pools_coin_b_idx ON pools(coin_type_b);

CREATE TABLE IF NOT EXISTS token_watchlist (
    coin_type   TEXT PRIMARY KEY REFERENCES tokens(coin_type),
    source      TEXT NOT NULL,
    priority    INTEGER NOT NULL DEFAULT 0,
    added_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
