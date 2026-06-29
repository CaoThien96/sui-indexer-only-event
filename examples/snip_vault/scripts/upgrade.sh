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

if [[ ! -f Published.toml ]]; then
  echo "Published.toml not found. Run scripts/publish.sh first." >&2
  exit 1
fi

UPGRADE_CAP="${UPGRADE_CAP:-}"
if [[ -z "${UPGRADE_CAP}" ]]; then
  UPGRADE_CAP="$(rg 'upgrade-capability' Published.toml -A1 | rg -o '0x[a-f0-9]{64}' | head -1 || true)"
fi
if [[ -z "${UPGRADE_CAP}" ]]; then
  echo "Set UPGRADE_CAP=0x... or ensure Published.toml has upgrade-capability." >&2
  exit 1
fi

VAULT_ID="${SNIP_VAULT_OBJECT_ID:-}"
ADMIN_CAP="${ADMIN_CAP:-}"

echo "==> Building snip_vault (build-env=${BUILD_ENV})..."
sui move build --build-env "${BUILD_ENV}"

echo "==> Upgrading package (cap=${UPGRADE_CAP})..."
UPGRADE_OUT="$(sui client upgrade --upgrade-capability "${UPGRADE_CAP}" --gas-budget "${GAS_BUDGET}" 2>&1)"
echo "$UPGRADE_OUT"

PACKAGE_ID="$(echo "$UPGRADE_OUT" | rg -o 'PackageID: (0x[a-f0-9]+)' -m1 | awk '{print $2}' || true)"
echo ""
echo "SNIP_VAULT_PACKAGE=${PACKAGE_ID:-<check Published.toml>}"

if [[ -n "${VAULT_ID}" && -n "${ADMIN_CAP}" && -n "${PACKAGE_ID}" ]]; then
  echo "==> Syncing vault package_version (required — old package IDs abort after this)..."
  sui client ptb \
    --move-call "${PACKAGE_ID}::vault::sync_package_version" "@${ADMIN_CAP}" "@${VAULT_ID}" \
    --gas-budget 50000000
else
  cat <<EOF

Next steps (required after every upgrade):
1. Update SNIP_VAULT_PACKAGE in indexer .env to the new package ID
2. Call sync_package_version so old package moveCalls abort:

   sui client ptb \\
     --move-call "\$SNIP_VAULT_PACKAGE::vault::sync_package_version" "@\$ADMIN_CAP" "@\$SNIP_VAULT_OBJECT_ID" \\
     --gas-budget 50000000

Or re-run with SNIP_VAULT_OBJECT_ID and ADMIN_CAP set.

EOF
fi
