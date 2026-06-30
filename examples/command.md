# Command Cheatsheet

Quick reference for day-to-day operations in this repo.

**PostgreSQL:** watermarks + pipeline state only (`postgres://postgres:postgres@localhost:5432/sui_indexer`)  
**ClickHouse:** all indexed events — hot SSD (≤3 days) + cold local HDD (>3 days, TTL MOVE)  
**Project root:** `/Users/thiencao/Desktop/sui-indexer`

---

## Environment Setup (first time)

### 1. Rust & Cargo (via rustup)

Install [rustup](https://rustup.rs/) — the official Rust toolchain manager:

```bash
# macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts, then reload shell
source "$HOME/.cargo/env"
```

Verify:

```bash
rustc --version
cargo --version
rustup show
```

Useful rustup commands:

```bash
rustup update              # update toolchain + components
rustup default stable      # ensure stable is default
rustup component add rustfmt clippy   # optional: formatting + linter
```

> This repo uses **Rust 2024 edition** (`edition = "2024"` in `Cargo.toml`). Use a recent stable toolchain (1.85+).

### 2. PostgreSQL client libs (macOS — libpq)

Homebrew `libpq` is required to **compile** Diesel CLI and project crates. Install this **before** `diesel_cli`.

```bash
brew install libpq

# Add to ~/.zshrc (Apple Silicon default path)
export LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:$LIBRARY_PATH"
export CPATH="/opt/homebrew/opt/libpq/include:$CPATH"
export PATH="/opt/homebrew/opt/libpq/bin:$PATH"   # psql, pg_isready, etc.
export PQ_LIB_DIR="$(brew --prefix libpq)/lib"
export PQ_INCLUDE_DIR="$(brew --prefix libpq)/include"
export PKG_CONFIG_PATH="$(brew --prefix libpq)/lib/pkgconfig:$PKG_CONFIG_PATH"
```

Reload shell: `source ~/.zshrc`

Verify:

```bash
pg_config --version
psql --version
ls "$(brew --prefix libpq)/lib/libpq.dylib"
```

### 3. Diesel CLI

Required for running PostgreSQL migrations in `simple-sui-indexer/`. Requires **libpq** (step 2).

```bash
RUSTFLAGS="-L $(brew --prefix libpq)/lib" \
  cargo install diesel_cli --no-default-features --features postgres
```

If a previous install failed halfway, add `--force`:

```bash
RUSTFLAGS="-L $(brew --prefix libpq)/lib" \
  cargo install diesel_cli --no-default-features --features postgres --force
```

Verify:

```bash
diesel --version
# diesel-cli 2.x.x
```

If `diesel` is not found after install:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
# add the line above to ~/.zshrc
```

#### Diesel CLI install failed?

Scroll up in the terminal — the real error is usually **one line above** `could not compile diesel_cli`.

| Error | Fix |
|---|---|
| `ld: library not found for -lpq` | Run step 2 first, then retry with `RUSTFLAGS` |
| `binary diesel already exists` | Add `--force` to overwrite |
| `could not find libpq-fe.h` | `brew install libpq` + set `CPATH` / `PQ_INCLUDE_DIR` |
| Still failing after partial install | Retry with `--force` after libpq is installed |

**Permanent fix (optional)** — add to `~/.cargo/config.toml` so every `cargo build` finds libpq:

```toml
[target.aarch64-apple-darwin]
rustflags = "-L /opt/homebrew/opt/libpq/lib"

# Intel Mac — use /usr/local instead:
# [target.x86_64-apple-darwin]
# rustflags = "-L /usr/local/opt/libpq/lib"
```

Check your Homebrew architecture matches your Mac (`brew config` → should show `arm64` on Apple Silicon).

### 4. PostgreSQL server

**Option A — Docker (recommended for local dev):**

```bash
docker run -d \
  --name sui-indexer-postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=sui_indexer \
  -p 5432:5432 \
  postgres:16

# Check container
docker ps | grep sui-indexer-postgres
```

**Option B — existing Postgres:** create the database manually:

```bash
createdb -h localhost -U postgres sui_indexer
# or
psql postgres://postgres:postgres@localhost:5432/postgres -c "CREATE DATABASE sui_indexer;"
```

Verify connection:

```bash
pg_isready -h localhost -p 5432 -U postgres
psql postgres://postgres:postgres@localhost:5432/sui_indexer -c "SELECT 1;"
```

### 5. Project env files

```bash
cd /Users/thiencao/Desktop/sui-indexer

# Indexer — create simple-sui-indexer/.env (see minimum below)
# RPC service
cp rpc-service/.env.example rpc-service/.env

# Reconciliation
cp reconciliation/.env.example reconciliation/.env
```

Minimum `simple-sui-indexer/.env`:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/sui_indexer
CLICKHOUSE_URL=http://localhost:8123
CLICKHOUSE_DATABASE=sui_indexer
EVENT_TYPE_PREFIXES=0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::,0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::

# Optional — Telegram alert when processor fails (missing binding, decode error, …)
TELEGRAM_BOT_TOKEN=123456789:ABCdefGHIjklMNOpqrsTUVwxyz
TELEGRAM_CHAT_ID=-1001234567890
TELEGRAM_NOTIFY_COOLDOWN_SECS=300

# Integrated snip bot (replaces bot-snip polling)
# BOT_ENABLED=true
# MAIN_RPC=https://fullnode.mainnet.sui.io:443
# VAULT_PATH=./vault.json
# SELL_BUY_THRESHOLD=500000000
```

**Snip bot (integrated):** set `BOT_ENABLED=true` and copy `vault.json` from bot-snip. Events are handled in-process right after decode (no `queryEvents` poll). Migrate existing removed tokens once:

```bash
BOT_SNIP_DATABASE_URL=postgres://...bot_snip_db... \
DATABASE_URL=postgres://...sui_indexer... \
cargo run --release --bin migrate-removed-tokens
```

Stop bot-snip before running the indexer with `BOT_ENABLED=true` to avoid duplicate sell txs.

**Telegram setup:** create a bot via [@BotFather](https://t.me/BotFather), add it to your group/channel, send a message, then resolve chat id (e.g. `https://api.telegram.org/bot<token>/getUpdates`). Cooldown dedupes identical `pipeline + event_type + error` so framework retries do not spam the chat.

### Prometheus metrics (built-in + app)

`sui-indexer-alt-framework` exposes **built-in** ingestion / pipeline / watermark / DB metrics on HTTP (default `http://0.0.0.0:9184/metrics`). This indexer registers **app counters** on the same endpoint.

| Config | Default | Description |
|--------|---------|-------------|
| CLI `--metrics-address` | `0.0.0.0:9184` | Prometheus scrape bind address |
| `METRICS_ADDRESS` | — | Env override for bind address |
| `METRICS_PREFIX` | — | Prefix all framework metric names (e.g. `simple_sui_indexer_…`) |

```bash
# Scrape locally
curl -s http://127.0.0.1:9184/metrics | head -40

# Custom bind address
METRICS_ADDRESS=127.0.0.1:9190 cargo run --release -- --metrics-address 127.0.0.1:9190
```

**Framework metrics** (label `pipeline="event_type_handler"` where applicable):

- `latest_processed_checkpoint`, `latest_processed_checkpoint_timestamp_lag_ms` — lag vs chain tip
- `total_handler_processor_retries` — decode / processor failures (retries)
- `watermark_checkpoint_in_db` — committed watermark
- `total_committer_rows_committed`, `total_committer_batches_failed` — DB write health
- Ingestion: `total_ingested_checkpoints`, `latest_ingested_checkpoint`, …

**App metrics** (this repo):

- `simple_sui_indexer_events_matched_total`
- `simple_sui_indexer_decode_errors_total{event_type="…"}`
- `simple_sui_indexer_package_events_inserted_total`
- `simple_sui_indexer_bot_errors_total`
- `simple_sui_indexer_bot_event_latency_ms`
- `uptime{version="…"}`

Prometheus scrape example:

```yaml
scrape_configs:
  - job_name: simple-sui-indexer
    static_configs:
      - targets: ["127.0.0.1:9184"]
```

Ref: [Sui indexer runtime & metrics](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-runtime-perf#metrics)

### 6. Run migrations & first build

```bash
cd simple-sui-indexer
export LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:$LIBRARY_PATH"
export CPATH="/opt/homebrew/opt/libpq/include:$CPATH"

# Apply migrations (creates watermarks, drops package_events — events live in ClickHouse)
diesel migration run

# Build indexer + backfill binary (event-bindings needs network on first compile)
cargo build --release
```

Event BCS layouts are generated at compile time via `move_contract!` in `event-bindings/` (patched fork in `vendor/move-binding/`). When a package adds new event types:

1. Add or extend `move_contract! { ... }` in `event-bindings/src/lib.rs` (set `event_modules`, `support_modules`, and `linkage` for deps)
2. Add a dispatch arm in `decode_parsed_json`
3. Rebuild with network: `cargo build --release`
4. Restart the indexer

> **Note:** `move_contract!` fetches package modules from GraphQL at compile time. Patches in `vendor/move-binding/`: updated URL (`graphql.mainnet.sui.io`), event-only codegen, explicit `linkage` for cross-package types, numeric fields serialized as strings for fullnode `parsedJson` parity. See `event-bindings-probe/README.md` for background.

Build other crates:

```bash
cd ../rpc-service && cargo build --release
cd ../reconciliation && cargo build --release
```

### 7. Verify full setup

```bash
# Toolchain
rustc --version && cargo --version && diesel --version

# Database + schema
psql postgres://postgres:postgres@localhost:5432/sui_indexer -c "\dt"

# Indexer binary
ls simple-sui-indexer/target/release/simple-sui-indexer
```

---

## Prerequisites (daily use)

After first-time setup, you usually only need these before building:

```bash
export LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:$LIBRARY_PATH"
export CPATH="/opt/homebrew/opt/libpq/include:$CPATH"
```

Add those exports to `~/.zshrc` if you build often.

---

## PostgreSQL

### Check Postgres is up

```bash
pg_isready -h localhost -p 5432 -U postgres
```

### Connect

```bash
psql postgres://postgres:postgres@localhost:5432/sui_indexer
```

### Reset indexer data (keep schema, restart from scratch)

Clears ClickHouse events and PostgreSQL watermark so the next run respects `--first-checkpoint`:

```bash
# ClickHouse events
curl -s 'http://127.0.0.1:8123/' --data 'TRUNCATE TABLE sui_indexer.package_events'

# Watermark only (PostgreSQL)
psql postgres://postgres:postgres@localhost:5432/sui_indexer <<'SQL'
DELETE FROM watermarks WHERE pipeline = 'event_type_handler';
SQL
```

> Legacy `TRUNCATE package_events` on PostgreSQL no longer applies — that table was removed.

### Full reset (drop all tables — re-run migrations after)

```bash
psql postgres://postgres:postgres@localhost:5432/sui_indexer <<'SQL'
DROP SCHEMA public CASCADE;
CREATE SCHEMA public;
SQL

cd simple-sui-indexer
diesel migration run
```

### Useful inspection queries

```bash
# Watermark (last committed checkpoint)
psql postgres://postgres:postgres@localhost:5432/sui_indexer -c \
  "SELECT pipeline, checkpoint_hi_inclusive FROM watermarks;"

# Event counts (ClickHouse)
curl -s 'http://127.0.0.1:8123/?database=sui_indexer' --data \
  "SELECT count() AS total, min(checkpoint_sequence_number), max(checkpoint_sequence_number) FROM package_events FINAL"

# Event types breakdown (ClickHouse)
curl -s 'http://127.0.0.1:8123/?database=sui_indexer' --data \
  "SELECT event_type, count() AS cnt FROM package_events FINAL GROUP BY event_type ORDER BY cnt DESC LIMIT 20"
```

> **Note:** Filter by `event_type`, not `package_id`. After package upgrades, `package_id` is the executing package ID; `event_type` keeps the original type path.

---

## ClickHouse (events store)

Local-only hot/cold tier: SSD hot volume (≤3 days), HDD cold volume (>3 days). TTL `MOVE` — no DELETE, no S3.

### Start ClickHouse (Docker)

```bash
cd examples/clickhouse
cp .env.example .env   # set CLICKHOUSE_PASSWORD
docker compose up -d
chmod +x init-clickhouse.sh
./init-clickhouse.sh
```

Credentials live in `clickhouse/.env` (gitignored) — **not** in `docker-compose.yml`. Copy the same `CLICKHOUSE_USER` / `CLICKHOUSE_PASSWORD` into indexer and rpc-service `.env`.

### Web UI (Tabix — included in Docker)

`docker-compose.yml` includes **Tabix** (browser SQL client).

```bash
cd examples/clickhouse
docker compose up -d    # starts clickhouse + tabix
```

Open **http://127.0.0.1:8124** → **Add server**:

| Field | Value |
|-------|-------|
| Name | `local` (any label) |
| HTTP host | `http://127.0.0.1:8123` |
| Login | `default` (or `CLICKHOUSE_USER` from `.env`) |
| Password | from `clickhouse/.env` |
| Database | `sui_indexer` |

Then browse `package_events`, run SQL, export results.

> Tabix talks to ClickHouse from **your browser** → use `127.0.0.1:8123`, not the Docker service name.

### Desktop UI (DBeaver — optional)

Full GUI on macOS:

```bash
brew install --cask dbeaver-community
```

**New connection** → **ClickHouse** → settings:

| Field | Value |
|-------|-------|
| Host | `localhost` |
| Port | `8123` (HTTP) |
| Database | `sui_indexer` |
| Username / Password | from `clickhouse/.env` |

Useful for ER diagrams, query history, and editing alongside Postgres (`5432`).

### CLI client (terminal)

```bash
brew install clickhouse
# or: brew install --cask clickhouse
```

```bash
source examples/clickhouse/.env
clickhouse client \
  --host localhost --port 9009 \
  --user "$CLICKHOUSE_USER" --password "$CLICKHOUSE_PASSWORD" \
  --database sui_indexer
```

(`9009` = native protocol mapped in `docker-compose.yml`; HTTP port `8123` is for curl/rpc/indexer.)


| Variable | Default | Description |
|---|---|---|
| `CLICKHOUSE_URL` | — | HTTP endpoint, e.g. `http://localhost:8123` |
| `CLICKHOUSE_DATABASE` | `sui_indexer` | Database name |
| `CLICKHOUSE_USER` / `CLICKHOUSE_PASSWORD` | from `clickhouse/.env` | Required for host HTTP access |

### Useful queries

```bash
# Row count
curl -s 'http://127.0.0.1:8123/?database=sui_indexer' --data \
  "SELECT count() FROM package_events FINAL"

# Disk usage by volume (hot vs cold parts)
curl -s 'http://127.0.0.1:8123/' --data \
  "SELECT disk_name, sum(bytes_on_disk) AS bytes FROM system.parts WHERE table='package_events' AND active GROUP BY disk_name"
```

### Hot/cold SLA benchmark

Same `suix_queryEvents` API; latency differs by tier (HDD cold vs SSD hot):

```bash
cd examples
# Local vs fullnode (default)
./scripts/bench-query-events-latency.sh

# Hot vs cold tier on local rpc-service
BENCH_HOT_COLD=1 ./scripts/bench-query-events-latency.sh
```

| Tier | Target latency | Notes |
|------|----------------|-------|
| Hot (≤3 days) | ~ms–100ms | Bot / realtime queries |
| Cold (>3 days) | ~100ms–few seconds | Historical backfill; local HDD |

### Retention (replaces PostgreSQL prune)

**Deprecated:** `scripts/prune-package-events.*` — events are never deleted. Parts older than 3 days move to the cold HDD volume via ClickHouse TTL. Monitor cold disk usage and expand HDD as needed.

---

## simple-sui-indexer

Config lives in `simple-sui-indexer/.env` (`DATABASE_URL`, `CLICKHOUSE_URL`, `EVENT_TYPE_PREFIXES`).

Optional env vars:

| Variable | Default | Description |
|---|---|---|
| `LOG_EVERY_N_CHECKPOINTS` | `100` | Progress log interval |
| `CLICKHOUSE_URL` | — | Required — ClickHouse HTTP endpoint |
| `CLICKHOUSE_DATABASE` | `sui_indexer` | Target database |
| `RUST_LOG` | — | e.g. `info`, `debug` |

Decode is **sync static BCS** via `event-bindings` — zero fullnode RPC at index time. Unknown event types within a matched prefix **fail the checkpoint** (add binding + rebuild).

### Build

```bash
cd simple-sui-indexer
export LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:$LIBRARY_PATH"
export CPATH="/opt/homebrew/opt/libpq/include:$CPATH"

cargo build --release
```

### Start indexing (foreground)

```bash
cd simple-sui-indexer
export LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:$LIBRARY_PATH"
export CPATH="/opt/homebrew/opt/libpq/include:$CPATH"
export RUST_LOG=info

cargo run --release -- \
  --remote-store-url https://checkpoints.mainnet.sui.io \
  --streaming-url https://fullnode.mainnet.sui.io:443 \
  --first-checkpoint 284237164
```

If a watermark already exists, `--first-checkpoint` is **ignored**. Reset the DB first (see above) to use it.

### Start indexing — background + log file

**Build release binary first** (otherwise `./target/release/simple-sui-indexer` → exit 127):

```bash
cd simple-sui-indexer
export LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:$LIBRARY_PATH"
export CPATH="/opt/homebrew/opt/libpq/include:$CPATH"
cargo build --release
```

Uses `stdbuf -eL` so logs flush immediately when not attached to a TTY.  
macOS does **not** ship `stdbuf` by default — install via `brew install coreutils` if needed (`gstdbuf` also works).

```bash
cd simple-sui-indexer
export LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:$LIBRARY_PATH"
export CPATH="/opt/homebrew/opt/libpq/include:$CPATH"
export RUST_LOG=info
export LOG_EVERY_N_CHECKPOINTS=100

# Option A: run release binary (recommended after cargo build --release)
nohup stdbuf -eL ./target/release/simple-sui-indexer \
  --remote-store-url https://checkpoints.mainnet.sui.io \
  --streaming-url https://fullnode.mainnet.sui.io:443 \
  > indexer.log 2>&1 &

echo $! > indexer.pid
echo "Started PID $(cat indexer.pid), logging to indexer.log"
```

```bash
# Option B: cargo run --release (builds + runs, slower startup)
nohup stdbuf -eL cargo run --release -- \
  --remote-store-url https://checkpoints.mainnet.sui.io \
  --streaming-url https://fullnode.mainnet.sui.io:443 \
  > indexer.log 2>&1 &

echo $! > indexer.pid
```

**macOS without `stdbuf`** — drop it (logs may buffer slightly):

```bash
nohup ./target/release/simple-sui-indexer \
  --remote-store-url https://checkpoints.mainnet.sui.io \
  --streaming-url https://fullnode.mainnet.sui.io:443 \
  --first-checkpoint 284237164 \
  > indexer.log 2>&1 &

echo $! > indexer.pid
```

Or use Homebrew's GNU stdbuf:

```bash
nohup gstdbuf -eL ./target/release/simple-sui-indexer \
  --remote-store-url https://checkpoints.mainnet.sui.io \
  --streaming-url https://fullnode.mainnet.sui.io:443 \
  > indexer.log 2>&1 &
```

Quick sanity check before starting:

```bash
test -x ./target/release/simple-sui-indexer && echo "binary OK" || echo "run: cargo build --release"
command -v stdbuf || command -v gstdbuf || echo "install: brew install coreutils"
```

### Backfill from an older checkpoint

Reset watermark + data, then pass `--first-checkpoint`:

```bash
# 1. Reset (see PostgreSQL section)
# 2. Start with --first-checkpoint (example: before recent PoolCreatedEvents)
cd simple-sui-indexer

nohup stdbuf -eL ./target/release/simple-sui-indexer \
  --remote-store-url https://checkpoints.mainnet.sui.io \
  --streaming-url https://fullnode.mainnet.sui.io:443 \
  --first-checkpoint 284115449 \
  > indexer.log 2>&1 &

echo $! > indexer.pid
```

### Bounded smoke test (small checkpoint range)

```bash
cd simple-sui-indexer
export LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:$LIBRARY_PATH"
export CPATH="/opt/homebrew/opt/libpq/include:$CPATH"
export RUST_LOG=info
export LOG_EVERY_N_CHECKPOINTS=1

cargo run --release -- \
  --remote-store-url https://checkpoints.mainnet.sui.io \
  --streaming-url https://fullnode.mainnet.sui.io:443 \
  --first-checkpoint 283469751 \
  --last-checkpoint 283469760
```

### Backfill `parsed_json` for historical rows

Uses the same static decoder as the indexer (no fullnode RPC):

```bash
cd simple-sui-indexer
export LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:$LIBRARY_PATH"
export RUST_LOG=info

# Full backfill (all rows where parsed_json IS NULL)
cargo run --release --bin backfill-parsed-json -- --batch-size 5000

# Smoke test — first 1000 rows only
cargo run --release --bin backfill-parsed-json -- --limit 1000 --batch-size 500
```

Failed rows are logged and skipped (reported in `failures_by_type` summary). Fix bindings, rebuild, and re-run.

### Tail log (realtime)

```bash
cd simple-sui-indexer
tail -f indexer.log
```

Filter progress lines only:

```bash
tail -f indexer.log | grep -E 'progress|checkpoint|ERROR|WARN'
```

### Find PID

```bash
# From pid file (if started with commands above)
cat simple-sui-indexer/indexer.pid

# By process name
pgrep -fl simple-sui-indexer

# Full process list
ps aux | grep simple-sui-indexer | grep -v grep
```

### Stop indexer

```bash
# Graceful stop via pid file
kill "$(cat simple-sui-indexer/indexer.pid)"

# Force stop if needed
kill -9 "$(cat simple-sui-indexer/indexer.pid)"

# Or by name
pkill -f simple-sui-indexer
```

### Check indexer is running

```bash
kill -0 "$(cat simple-sui-indexer/indexer.pid)" 2>/dev/null \
  && echo "running" || echo "not running"
```

---

## rpc-service

Config: `rpc-service/.env` (`CLICKHOUSE_URL`, `CLICKHOUSE_DATABASE`, `RPC_PORT`, `RUST_LOG`).

Reads **ClickHouse only** (`package_events` with `FINAL` for dedup). Same JSON-RPC shape as before.

> **Binary path:** `rpc-service` is a **workspace member** of the repo root [`Cargo.toml`](../Cargo.toml).  
> `cargo build --release` from `examples/rpc-service/` writes the binary to **`../../target/release/rpc-service`** (repo root), **not** `rpc-service/target/`.  
> (`simple-sui-indexer` is different — it has its own `[workspace]` and uses `simple-sui-indexer/target/`.)

### Build & run (foreground)

```bash
cd examples/rpc-service
cargo build --release
cargo run --release
```

Or from repo root:

```bash
cd /Users/thiencao/Desktop/sui-indexer
cargo build --release -p rpc-service
./target/release/rpc-service
```

Default: `http://127.0.0.1:9000` (or `RPC_PORT` from `.env`). Loads `rpc-service/.env` automatically when cwd is `examples/rpc-service`.

### Start in background + log file

**Build release binary first:**

```bash
cd examples/rpc-service
cargo build --release
# binary: ../../target/release/rpc-service
```

```bash
cd examples/rpc-service
export RUST_LOG=info

nohup ../../target/release/rpc-service > rpc.log 2>&1 &

echo $! > rpc.pid
echo "Started PID $(cat rpc.pid), logging to rpc.log"
```

With line-buffered logs (if `stdbuf` available):

```bash
cd examples/rpc-service
export RUST_LOG=info

nohup stdbuf -eL ../../target/release/rpc-service > rpc.log 2>&1 &

echo $! > rpc.pid
```

Quick sanity check:

```bash
cd examples/rpc-service
test -x ../../target/release/rpc-service && echo "binary OK" || echo "run: cargo build --release"
curl -sf http://127.0.0.1:9000/health
```

### Follow logs

```bash
cd rpc-service
tail -f rpc.log
```

### Stop rpc-service

```bash
cd rpc-service
kill "$(cat rpc.pid)"

# Force stop if needed
kill -9 "$(cat rpc.pid)"

# Or by name
pkill -f rpc-service
```

### Check rpc-service is running

```bash
kill -0 "$(cat rpc-service/rpc.pid)" 2>/dev/null \
  && echo "running" || echo "not running"

curl -s http://127.0.0.1:9000/health
```

### Health check

```bash
curl -s http://127.0.0.1:9000/health
```

### Query events (JSON-RPC)

```bash
curl -s -X POST http://127.0.0.1:9000/ \
  -H 'Content-Type: application/json' \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "suix_queryEvents",
    "params": [
      {"MoveEventType": "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent"},
      null,
      5,
      true
    ]
  }' | python3 -m json.tool
```

> `event_type` is stored in **canonical Move casing** (same as fullnode `type`, e.g. `SwapEvent`).

**Page 2+** — pass `nextCursor` from the previous response as param `[1]`:

```bash
curl -s -X POST http://127.0.0.1:9000/ \
  -H 'Content-Type: application/json' \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "suix_queryEvents",
    "params": [
      {"MoveEventType": "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent"},
      {"txDigest": "Zy26oz2EE8nHZicStDjaxwfEYjPYQEz1J7F12ocibsK", "eventSeq": "10"},
      5,
      true
    ]
  }' | python3 -m json.tool
```

Params: `[filter, cursor, limit, descending]`. With `descending: true`, results are **older than** the cursor in on-chain stream order (`checkpoint → tx index → event index`).

### Restore canonical `event_type` on existing DB

```bash
cd simple-sui-indexer
diesel migration run   # applies 2026-06-07-120000-0000_restore_canonical_event_type
```

Backfills from the indexed event JSON (`json->>'type'`). New rows from the indexer already store canonical casing.

**Fullnode parity notes** (intentional differences):

| Topic | Fullnode mainnet | This RPC |
|-------|------------------|----------|
| Pagination order | Stream position | Same (after fix) |
| `type` field casing | Canonical (`SwapEvent`) | Same (canonical Move casing) |
| Filters | All, TimeRange, Transaction, … | Only `MoveEventType`, `MoveModule`, `MoveEventModule`, `Sender` |
| Data scope | All on-chain events | Only indexed prefix packages |
| `parsedJson` | Fullnode decode | Static BCS bindings (`event-bindings/`) |
| Sync lag | Live chain tip | Indexer watermark — may trail fullnode |
| Unknown cursor | Fullnode error | `-32602` if `(txDigest, eventSeq)` not in ClickHouse |
| Hot/cold latency | N/A | Hot SSD ~ms–100ms; cold HDD slower (see bench) |
| `packageId` casing | Lowercase hex | Lowercase hex (unchanged) |

### Test ASC pagination order

Automated check: paginate RPC with `descending=false`, load stream position from ClickHouse for each event.

```bash
# rpc-service + ClickHouse must be running
chmod +x scripts/test-query-events-asc-order.sh
./scripts/test-query-events-asc-order.sh

# Custom filter / fewer pages
MOVE_EVENT_TYPE='0x91bf...::pool::SwapEvent' LIMIT=20 MAX_PAGES=3 ./scripts/test-query-events-asc-order.sh

# Also compare event-id order with fullnode (fails if indexer lags)
COMPARE_FULLNODE=1 ./scripts/test-query-events-asc-order.sh
```

Manual ClickHouse sanity check (oldest first):

```bash
curl -s 'http://127.0.0.1:8123/?database=sui_indexer' --data \
  "SELECT event_id_tx_digest, event_id_seq, checkpoint_sequence_number, \
          transaction_sequence_in_checkpoint, event_sequence_in_transaction \
   FROM package_events FINAL \
   WHERE event_type ILIKE '0x1eab%::pool::SwapEvent' \
   ORDER BY checkpoint_sequence_number, transaction_sequence_in_checkpoint, event_sequence_in_transaction \
   LIMIT 10 FORMAT Pretty"
```

### Bot sandwich statistics (same-checkpoint)

Analyze competitor bot sandwich patterns from indexed Cetus `SwapEvent` rows. Detects per-checkpoint triplets in consensus order (`transaction_sequence_in_checkpoint`, `event_sequence_in_transaction`):

`bot buy (atob=false)` → `victim buy (atob=false, same pool)` → `bot sell (atob=true)`

```bash
cd examples/simple-sui-indexer
python3 scripts/analyze_bot_sandwich.py

# Per-checkpoint tx/event order + amounts (stdout); full report always in markdown file
VERBOSE=1 python3 scripts/analyze_bot_sandwich.py

# Limit checkpoints (quick run)
LIMIT_CHECKPOINTS=20 python3 scripts/analyze_bot_sandwich.py

# Custom report path (default: reports/bot_sandwich_report.md)
REPORT_PATH=reports/my_report.md python3 scripts/analyze_bot_sandwich.py
```

Env vars (defaults in script):

| Var | Default |
|-----|---------|
| `CLICKHOUSE_URL` | `http://127.0.0.1:8123` |
| `BOT_ADDRESS` | `0xf3981a28e88f86255713dada5d7b1ebb23b0b9e499e80fa1406bdd74c3364735` |
| `SWAP_EVENT_TYPE` | `0x1eab...::pool::SwapEvent` |
| `VERBOSE` | `0` (stdout detail only) |
| `LIMIT_CHECKPOINTS` | `0` (all) |
| `REPORT_PATH` | `reports/bot_sandwich_report.md` |

Sanity check — bot swap rows exist:

```bash
psql postgres://postgres:postgres@localhost:5432/sui_indexer -c "
  SELECT COUNT(*) FROM package_events
  WHERE lower(sender) = '0xf3981a28e88f86255713dada5d7b1ebb23b0b9e499e80fa1406bdd74c3364735'
    AND event_type ILIKE '%::pool::SwapEvent';"
```

---

## sandwich-trap

Trap operator: bait BUY + burst DUMP SELL (no indexer wait). User provides pool + wallets.

Location: `examples/sandwich-trap/`

### Setup

```bash
cd examples/sandwich-trap/bot
cp .env.example .env
cp ../config/mainnet.template.json ../config/mainnet.json
# Edit mainnet.json: poolId, coinTypeA, trapMode, baitSuiMist, dumpTokenAmount
npm install
```

### Env (bot/.env)

| Mode | Vars |
|------|------|
| `single-tx` | `SUI_SECRET_KEY` |
| `parallel-tx` | `SUI_SECRET_KEY_BAIT`, `SUI_SECRET_KEY_DUMP` |

### Commands

```bash
cd examples/sandwich-trap/bot

# Pool state (no gas)
npm run read-pool -- --config=../config/mainnet.json

# Debug single legs
npm run bait -- --config=../config/mainnet.json
npm run dump -- --config=../config/mainnet.json

# One trap round (bait + dump together)
npm run trap -- --config=../config/mainnet.json

# Repeat trap rounds
npm run trap-loop -- --config=../config/mainnet.json
npm run trap-loop -- --config=../config/mainnet.json --max-rounds=10
```

### Config (`config/mainnet.json`)

| Field | Description |
|-------|-------------|
| `trapMode` | `single-tx` or `parallel-tx` |
| `poolId` | User-provided Cetus pool |
| `coinTypeA` | Token type (pool coin A) |
| `baitSuiMist` | Bait BUY SUI input |
| `dumpTokenAmount` | Token per burst SELL tx |
| `dumpBurstCount` | Parallel dump txs (`parallel-tx`) |
| `baitGasPrice` / `dumpGasPrice` | Gas price per leg |

### Post-hoc analytics

```bash
cd examples/sandwich-trap
POOL_ID=0x... python3 scripts/analyze_trap_outcome.py
```

| Var | Default |
|-----|---------|
| `DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/sui_indexer` |
| `POOL_ID` | (required) |
| `BOT_ADDRESS` | competitor bot address |
| `REPORT_PATH` | `reports/trap_outcome_report.md` |

---

## reconciliation

Config: `reconciliation/.env` (`CLICKHOUSE_URL`, `RECON_MOVE_EVENT_TYPE`, …).

### Run Phase 2 (count + key diff)

```bash
cd reconciliation
export LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:$LIBRARY_PATH"
export CPATH="/opt/homebrew/opt/libpq/include:$CPATH"

cargo run --release
```

Exit code `0` = match, `1` = mismatch.

### Run tests

```bash
cd reconciliation
cargo test
```

---

## Migrations (Diesel)

```bash
cd simple-sui-indexer
diesel migration run     # apply pending
diesel migration redo    # rollback last + re-apply
diesel print-schema > src/schema.rs
```

---

## Fullnode quick queries (no indexer)

```bash
# Latest checkpoint
curl -s -X POST https://fullnode.mainnet.sui.io:443 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"sui_getLatestCheckpointSequenceNumber","params":[]}' \
  | python3 -m json.tool

# Turbos PoolCreatedEvent (historical)
curl -s -X POST https://fullnode.mainnet.sui.io:443 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"suix_queryEvents","params":[{"MoveEventType":"0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::pool_factory::PoolCreatedEvent"},null,5,true]}' \
  | python3 -m json.tool


curl -s -X POST https://fullnode.mainnet.sui.io:443 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"suix_queryEvents","params":[{"MoveEventType":"0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::pool::SwapEvent"},null,5,true]}' \
  | python3 -m json.tool
```

---

## Typical workflows

### Fresh start (reset + re-index from checkpoint)

```bash
psql postgres://postgres:postgres@localhost:5432/sui_indexer -c \
  "TRUNCATE package_events; DELETE FROM watermarks WHERE pipeline = 'event_type_handler';"

pkill -f simple-sui-indexer 2>/dev/null || true

cd simple-sui-indexer
# then: start indexing — background + log file (with --first-checkpoint if backfilling)
```

### Restart indexer (keep data + watermark)

```bash
kill "$(cat simple-sui-indexer/indexer.pid)"
# wait a few seconds, then start indexing — background + log file again
# indexer resumes from watermark automatically
```
