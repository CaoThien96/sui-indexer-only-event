#!/usr/bin/env bash
# Smoke-test parsed_json decode against one row already in package_events.
set -euo pipefail

cd "$(dirname "$0")/simple-sui-indexer"
export LIBRARY_PATH="/opt/homebrew/opt/libpq/lib:${LIBRARY_PATH:-}"
export CPATH="/opt/homebrew/opt/libpq/include:${CPATH:-}"
export FULLNODE_URL="${FULLNODE_URL:-https://fullnode.mainnet.sui.io:443}"

cargo test decode_live_swap_event -- --ignored --nocapture
