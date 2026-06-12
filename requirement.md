# Sui Token Analytics — Product Brief

> **Full planning docs:** [docs/README.md](./docs/README.md)  
> **Important:** `examples/` is reference only. Production = greenfield in `crates/`.

## Summary

Hệ thống thống kê token trên Sui (multi-DEX). Triển khai lại từ đầu theo official Sui indexer docs (BYOS, GCS+gRPC, multiple pipelines).

## Phases

| Phase | Scope |
|-------|-------|
| **0** ✅ | Planning docs + `examples/` spikes (không deploy) |
| **1** | `crates/indexer` → Kafka (BYOS), multi-DEX, Prometheus |
| **2** | Processors, TimescaleDB, ClickHouse, `api-service` (**MVP**) |
| **3** | Pool snapshots, CLMM depth |
| **4** | Holders |
| **5** | Bubble map |

## Docs

**Mục lục tiếng Việt:** [docs/vi/README.md](./docs/vi/README.md)

| EN | Tiếng Việt | Nội dung |
|----|------------|----------|
| [01-product-scope](./docs/01-product-scope.md) | [vi/01](./docs/vi/01-product-scope.md) | Chốt tính năng |
| [02-system-architecture](./docs/02-system-architecture.md) | [vi/02](./docs/vi/02-system-architecture.md) | Chốt kiến trúc |
| [03-roadmap-timeline](./docs/03-roadmap-timeline.md) | [vi/03](./docs/vi/03-roadmap-timeline.md) | Chốt timeline |
| [04-data-contracts](./docs/04-data-contracts.md) | [vi/04](./docs/vi/04-data-contracts.md) | Kafka + DB schemas |
| [05-dex-coverage](./docs/05-dex-coverage.md) | [vi/05](./docs/vi/05-dex-coverage.md) | DEX events |
| [06-reference-examples](./docs/06-reference-examples.md) | [vi/06](./docs/vi/06-reference-examples.md) | `examples/` ≠ production |
| [07-indexer-optimization](./docs/07-indexer-optimization-checklist.md) | [vi/07](./docs/vi/07-indexer-optimization-checklist.md) | Checklist tối ưu |

## Requirements

- Danh sách pool của token
- Token static: name, symbol, decimal, image, created_at, creator
- Token dynamic: volume, swap history, chart, price, holder, liquidity
- Global: volume 24h, txns 24h
