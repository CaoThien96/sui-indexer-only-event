#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

BUILD_ENV="${BUILD_ENV:-mainnet}"
GAS_BUDGET="${GAS_BUDGET:-500000000}"

ACTIVE_ENV="$(sui client active-env 2>/dev/null || echo "")"
if [[ "${ACTIVE_ENV}" != "${BUILD_ENV}" ]]; then
  echo "error: sui client active-env is '${ACTIVE_ENV:-unknown}' but BUILD_ENV=${BUILD_ENV}." >&2
  echo "Run: sui client switch --env ${BUILD_ENV}" >&2
  exit 1
fi

echo "==> Sui CLI: $(sui --version)"
echo "==> Active env: ${ACTIVE_ENV}"
echo "==> Building snip_vault (build-env=${BUILD_ENV})..."
sui move build --build-env "${BUILD_ENV}"

if [[ "${DRY_RUN:-}" == "1" ]]; then
  echo "==> Dry-run publish (no submit)..."
  # sui 1.74+: publish builds for active-env only — no --build-env flag
  sui client publish --gas-budget "${GAS_BUDGET}" --dry-run
  echo ""
  echo "Dry-run complete. Unset DRY_RUN or run without it to publish for real."
  exit 0
fi

echo "==> Publishing to active network (${BUILD_ENV})..."
PUBLISH_OUT="$(sui client publish --gas-budget "${GAS_BUDGET}" 2>&1)"
echo "$PUBLISH_OUT"

PACKAGE_ID="$(echo "$PUBLISH_OUT" | rg -o 'PackageID: (0x[a-f0-9]+)' -m1 | awk '{print $2}' || true)"
VAULT_ID="$(echo "$PUBLISH_OUT" | rg 'Shared Objects' -A20 | rg -o '0x[a-f0-9]{64}' | head -1 || true)"
ADMIN_CAP="$(echo "$PUBLISH_OUT" | rg 'Created Objects' -A40 | rg 'AdminCap' -B1 | rg -o '0x[a-f0-9]{64}' | head -1 || true)"

echo ""
echo "SNIP_VAULT_PACKAGE=${PACKAGE_ID:-<fill-me>}"
echo "SNIP_VAULT_OBJECT_ID=${VAULT_ID:-<fill-me>}"
echo "ADMIN_CAP=${ADMIN_CAP:-<fill-me>}"
echo ""
echo "Published.toml updated in this directory (if publish succeeded)."

if [[ -n "${BOT_ADDRESS:-}" && -n "${ADMIN_CAP:-}" && "${ADMIN_CAP}" != "<fill-me>" && -n "${PACKAGE_ID:-}" ]]; then
  echo "==> Adding bot ${BOT_ADDRESS} to vault allowlist..."
  sui client ptb \
    --move-call "${PACKAGE_ID}::vault::add_bot" "@${ADMIN_CAP}" "@${VAULT_ID}" "@${BOT_ADDRESS}" \
    --gas-budget 50000000
fi

cat <<EOF

Next steps:
1. Copy SNIP_VAULT_* into examples/simple-sui-indexer/.env
2. Set USE_SNIP_VAULT=true (required for both snip AND sell)
3. Ensure VAULT_PATH wallet matches BOT_ADDRESS registered via add_bot
4. cargo build --release -p simple-sui-indexer --manifest-path examples/simple-sui-indexer/Cargo.toml

EOF
