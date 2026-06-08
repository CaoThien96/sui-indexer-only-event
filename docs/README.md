# Sui Token Analytics — Documentation Index

Official planning docs for the token statistics system on Sui.  
**Production code will be built greenfield** — see [06-reference-examples.md](./06-reference-examples.md).

| # | Document | Purpose |
|---|----------|---------|
| 1 | [01-product-scope.md](./01-product-scope.md) | **Frozen features** — what we build, by phase, in/out of scope |
| 2 | [02-system-architecture.md](./02-system-architecture.md) | **Frozen architecture** — layers, components, data flow, storage |
| 3 | [03-roadmap-timeline.md](./03-roadmap-timeline.md) | **Frozen timeline** — milestones, dependencies, exit criteria |
| 4 | [04-data-contracts.md](./04-data-contracts.md) | Kafka topics, normalized schemas, DB tables |
| 5 | [05-dex-coverage.md](./05-dex-coverage.md) | Supported DEX packages, events, decode status |
| 6 | [06-reference-examples.md](./06-reference-examples.md) | What `examples/` contains — reference only, not production |
| 7 | [07-indexer-optimization-checklist.md](./07-indexer-optimization-checklist.md) | Official Sui runtime/BYOS optimizations — frozen checklist |

## Related repo docs

| File | Purpose |
|------|---------|
| [../requirement.md](../requirement.md) | Product brief (summary) |
| [../events.md](../events.md) | Sample on-chain event payloads |
| [indexing_document.md](./indexing_document.md) | `sui-indexer-alt` framework notes |

## Project status

| Area | Status |
|------|--------|
| Planning docs (`docs/`) | ✅ Frozen |
| Production implementation | ⬜ **Greenfield — not started** |
| `examples/` | Reference / spike code only — **do not deploy** |

## Official Sui docs (must follow for indexer)

- [Custom Indexers](https://docs.sui.io/develop/accessing-data/custom-indexer/custom-indexers)
- [Pipeline Architecture](https://docs.sui.io/develop/accessing-data/custom-indexer/pipeline-architecture)
- [Integrate Data Sources](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-data-integration)
- [Bring Your Own Store (BYOS)](https://docs.sui.io/develop/accessing-data/custom-indexer/bring-your-own-store)
- [Optimize Runtime and Performance](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-runtime-perf)
- [ClickHouse BYOS example](https://github.com/MystenLabs/sui/tree/main/examples/rust/clickhouse-sui-indexer)
