# Snip Vault

On-chain vault for atomic **snip buy + add LP** and **sell** on Cetus / Turbos. Only the deployer or allowlisted bot wallets may call trading functions.

Requires **Sui CLI v1.63+** (framework deps resolve automatically — do not add `Sui` / `MoveStdlib` git lines to root `Move.toml`).

## Modules

| Module | Functions |
|--------|-----------|
| `vault` | `add_bot`, `remove_bot`, `deposit`, `withdraw`, `token_balance`, `sync_package_version`, shared `Vault` |
| `cetus_ops` | `snip_and_lp_cetus`, `sell_cetus` → returns `Coin<SUI>` |
| `turbos_ops` | `snip_and_lp_turbos`, `sell_turbos` → returns `Coin<SUI>` |

## Build & test

```bash
cd examples/snip_vault
sui move build --build-env mainnet
sui move test --build-env mainnet   # 18 unit tests (ACL + vault bag/position ops)
```

Vendored deps under `deps/`:

| Dep | Role |
|-----|------|
| `cetus_clmm` | Cetus CLMM **interface** (`published-at` → latest mainnet upgrade) |
| `turbos_clmm` | Turbos CLMM interface |
| `move-stl`, `integer-mate` | Transitive Cetus deps (local, framework via CLI) |

`Move.lock` pins resolved mainnet framework revisions — commit it.

## Publish (mainnet)

Requires `sui client active-env` = `mainnet` (Sui CLI 1.74+ builds publish for the active env — no `--build-env` on `sui client publish`).

```bash
cd examples/snip_vault
chmod +x scripts/publish.sh

# Estimate gas (no submit)
DRY_RUN=1 ./scripts/publish.sh

# Publish + optional allowlist bot wallet
BOT_ADDRESS=0x... ./scripts/publish.sh
```

After publish, the toolchain writes/updates `Published.toml`. Copy IDs into `examples/simple-sui-indexer/.env`:

```env
USE_SNIP_VAULT=true
SNIP_VAULT_PACKAGE=0x...    # latest package ID from publish (use for moveCall)
SNIP_VAULT_OBJECT_ID=0x...   # shared Vault object
SNIP_VAULT_GAS_BUDGET=500000000
```

Register the bot wallet (`VAULT_PATH` / `vault.json` address):

```bash
sui client ptb \
  --move-call "$SNIP_VAULT_PACKAGE::vault::add_bot" "@$ADMIN_CAP" "@$SNIP_VAULT_OBJECT_ID" "@$BOT_ADDRESS" \
  --gas-budget 50000000
```

Or set `BOT_ADDRESS` when running `publish.sh` to run `add_bot` automatically.

### Migrate wallet tokens into the vault

If snips ran via `agg_swap` (or vault fallback), tokens sit in the bot **wallet**. Use `vault::deposit` to move them into the vault Bag so `sell_*` can withdraw them. Use `vault::withdraw` to move tokens back to the wallet when needed.

Caller must be **deployer** or an **allowlisted bot** (`add_bot`).

**Deposit CLI** (wallet → vault):

```bash
cd examples/simple-sui-indexer
cargo run --release --bin vault-deposit -- \
  --token 0x...::mycoin::MYCOIN

# partial:
cargo run --release --bin vault-deposit -- --token 0x...::mycoin::MYCOIN --amount 1000000000
```

**Withdraw CLI** (vault → wallet):

```bash
cargo run --release --bin vault-withdraw -- \
  --token 0x...::mycoin::MYCOIN

# partial:
cargo run --release --bin vault-withdraw -- --token 0x...::mycoin::MYCOIN --amount 1000000000
```

**PTB**:

```text
MoveCall deposit<T>(vault, coin)
MoveCall withdraw<T>(vault, amount) → TransferObjects to sender
```

After upgrading the package on-chain, republish with `scripts/publish.sh` — existing vault object keeps its Bag; only new bytecode is needed for `deposit`.

## Indexer integration

When `USE_SNIP_VAULT=true`:

- **Snip**: one PTB `snip_and_lp_*` (no `SNIP_LP_WAIT_MS`, no wallet `getCoins` for the buy leg)
- **Sell**: dev-inspect `vault::token_balance`, clamp amount to vault Bag, then `sell_*` + `MergeCoins` into gas coin

### Important: vault sell vs fallback snip

`USE_SNIP_VAULT` must stay enabled for **sell** — sell withdraws from the on-chain **Bag** only.

When vault snip succeeds, tokens sit in the vault Bag. If vault snip fails and `SNIP_VAULT_FALLBACK_AGG=true` (default), the bot retries via `agg_swap` + wallet LP; those tokens stay in the **wallet**, so vault sell will not see them (`EInsufficientBalance` or zero balance clamp).

Set `SNIP_VAULT_FALLBACK_AGG=false` to fail fast on vault snip errors instead of mixing paths.

The indexer clamps sell size to `min(swap_event_amount, vault_balance)` so external buy events cannot over-withdraw.

Falls back to `agg_swap` + separate LP only when `USE_SNIP_VAULT` is unset/false, or when vault snip fails and `SNIP_VAULT_FALLBACK_AGG=true` (default).

## Security

- `assert_authorized`: `ctx.sender()` must be `deployer` or in `bots` table
- Only deployer holding `AdminCap` can `add_bot` / `remove_bot`
- One LP position object per pool id in `ObjectBag` (`EPositionAlreadyStored`)
- Events: `BotAdded`, `BotRemoved`, `TokenDeposited`, `TokenWithdrawn`, `CetusSniped`, `CetusSold`, `TurbosSniped`, `TurbosSold`, etc.

## PTB sketch (sell)

```text
sui_out = MoveCall sell_cetus(vault, config, pool, partner, amount, clock)
MergeCoins(GasCoin, [sui_out])
```

## Package IDs (Cetus)

- **Original / events**: `0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb`
- **Latest upgrade** (`published-at` in `deps/cetus_clmm/Move.toml`): `0x25ebb9a7c50eb17b3fa9c5a30fb8b5ad8f97caaf4928943acbcff7153dfee5e3`

After you upgrade `snip_vault`, update `SNIP_VAULT_PACKAGE` to the new ID; event subscriptions for Cetus pools stay on the original package prefix.

## Package version guard (reject stale package IDs)

Sui keeps **old package bytecode on-chain** after upgrade. Without a guard, a bot could still call `0xOLD_PACKAGE::vault::deposit` on the same shared `Vault`.

The vault stores `package_version` (starts at `1`). Every entrypoint checks it matches `PACKAGE_VERSION` in the **calling** package build:

- **Old package** after you bump vault → `EWrongPackageVersion`
- **New package** before `sync_package_version` → also aborts (forces explicit migration)

**After each upgrade:**

1. Bump `PACKAGE_VERSION` in `sources/vault.move` (e.g. `1` → `2`)
2. `sui client upgrade` (or `scripts/upgrade.sh`)
3. **`sync_package_version`** (admin, same tx block as upgrade if possible):

```bash
sui client ptb \
  --move-call "$SNIP_VAULT_PACKAGE::vault::sync_package_version" "@$ADMIN_CAP" "@$SNIP_VAULT_OBJECT_ID" \
  --gas-budget 50000000
```

4. Update indexer `SNIP_VAULT_PACKAGE` to the new package ID

```bash
chmod +x scripts/upgrade.sh
SNIP_VAULT_OBJECT_ID=0x... ADMIN_CAP=0x... ./scripts/upgrade.sh
```
