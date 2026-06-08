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

| Document | Content |
|----------|---------|
| [docs/01-product-scope.md](./docs/01-product-scope.md) | Chốt tính năng |
| [docs/02-system-architecture.md](./docs/02-system-architecture.md) | Chốt kiến trúc (greenfield) |
| [docs/03-roadmap-timeline.md](./docs/03-roadmap-timeline.md) | Chốt timeline |
| [docs/06-reference-examples.md](./docs/06-reference-examples.md) | `examples/` ≠ production |

## Requirements

- Danh sách pool của token
- Token static: name, symbol, decimal, image, created_at, creator
- Token dynamic: volume, swap history, chart, price, holder, liquidity
- Global: volume 24h, txns 24h
