# Sui Token Analytics — Agent Instructions

Custom indexer + analytics stack for Sui DEX tokens. Planning docs: [docs/README.md](./docs/README.md). `examples/` is reference only; production code is in `crates/` and `apps/`.

## Project structure

| Path | Purpose |
|------|---------|
| `crates/indexer` | Checkpoint ingestion → Kafka (sui-indexer-alt) |
| `crates/indexer-store` | Indexer store traits / adapters |
| `crates/event-bindings` | DEX event type bindings |
| `crates/processors` | Stream processors → TimescaleDB, ClickHouse, Redis |
| `crates/api-service` | REST API for token analytics |
| `apps/web` | React frontend (OHLC charts, token pages) |
| `infra/` | Docker Compose, deployment config |
| `.agents/skills/` | Domain skills (Sui, ClickHouse, Rust) |

## Knowledge sources (MUST use)

### 1. Project skills

Installed via `npx skills` (see [skills-lock.json](./skills-lock.json)). **Read the relevant `SKILL.md` before answering** — do not guess from training data.

**Start here:** [.agents/skills/sui-indexer-router/SKILL.md](./.agents/skills/sui-indexer-router/SKILL.md)

Install or update skills:

```sh
npx skills https://github.com/MystenLabs/skills
npx skills https://github.com/clickhouse/agent-skills
```

### 2. Sui documentation MCP

When skills do not cover the exact API or you need to verify deprecation:

- **Server:** `user-sui-knowledge-docs`
- **Tool:** `search_sui_knowledge_sources`
- Use for: SDK method names, JSON-RPC → gRPC/GraphQL migration, PTB errors, Move API changes

### 3. Official references

- Sui Docs: https://docs.sui.io (llms index: https://docs.sui.io/llms.txt)
- Move Book: https://move-book.com (llms index: https://move-book.com/llms.txt)
- `@mysten/*` SDK docs: `node_modules/@mysten/*/docs/llms-index.md` (in `apps/web`)

## Sui API defaults

- **Backends / indexers:** gRPC (`SuiGrpcClient`, `client.core.*`)
- **Frontends / flexible queries:** GraphQL RPC
- **JSON-RPC is deprecated** — do not use for new code
- **Historical data beyond full-node retention:** GraphQL (archival routing) or Archival Service directly for gRPC

## Workflow for agents

1. Read `sui-indexer-router` skill (or the specific skill from the routing table in `.cursor/rules/`).
2. If writing code, read the skill's reference files (e.g. `accessing-data/grpc.md`, `ptbs/building.md`).
3. If uncertain about an API, call `search_sui_knowledge_sources` before answering.
4. Match existing patterns in the target crate before introducing new abstractions.

## Project rules

- Controllers/handlers thin; logic in services/modules
- Migrations required for schema changes (Timescale: `crates/processors/migrations_timescale/`, ClickHouse: `crates/processors/migrations_clickhouse/`)
- Do not expand scope beyond the requested task
- Do not commit unless explicitly asked
