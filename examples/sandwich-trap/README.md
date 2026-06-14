# Sandwich Trap

Mainnet trap operator: **bait BUY + burst DUMP SELL** fired together (no indexer wait).

User provides:
- Cetus `poolId`, `coinTypeA`, `coinTypeB`
- Wallet(s) with token + SUI inventory

## Trap modes

| `trapMode` | Wallets | Behavior |
|------------|---------|----------|
| `single-tx` | 1 (`SUI_SECRET_KEY`) | One PTB: bait BUY then burst SELL sequential |
| `parallel-tx` | 2 (`SUI_SECRET_KEY_BAIT`, `SUI_SECRET_KEY_DUMP`) | Bait BUY + N dump SELL txs concurrent |

## Quickstart

```bash
# Prerequisite: simple-sui-indexer running (for optional analytics only)

cd examples/sandwich-trap/bot
cp .env.example .env
cp ../config/mainnet.template.json ../config/mainnet.json
# Edit mainnet.json: poolId, coinTypeA, trapMode, baitSuiMist, dumpTokenAmount

npm install
npm run read-pool -- --config=../config/mainnet.json
npm run trap -- --config=../config/mainnet.json
npm run trap-loop -- --config=../config/mainnet.json
```

## Debug legs

```bash
npm run bait -- --config=../config/mainnet.json
npm run dump -- --config=../config/mainnet.json
```

## Config knobs

- `baitSuiMist` — small SUI input for bait BUY (~0.05 SUI default)
- `dumpTokenAmount` — token sold per burst tx
- `dumpBurstCount` — parallel dump txs (`parallel-tx` only; needs N token coins)
- `baitGasPrice` / `dumpGasPrice` — gas price per leg

## Parallel-tx notes

- Bait wallet needs SUI ≥ `baitSuiMist + gasBudget`
- Dump wallet needs N token coins each ≥ `dumpTokenAmount`
- Dump wallet needs SUI coin for gas + receiving swap output

## Post-hoc analytics

```bash
cd examples/sandwich-trap
POOL_ID=0x... python3 scripts/analyze_trap_outcome.py
```

See [examples/command.md](../command.md) § sandwich-trap.
