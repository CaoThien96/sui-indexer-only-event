# Sui Token Analytics — Product Brief

> **Full planning docs:** [docs/README.md](./docs/README.md)

## Summary

Hệ thống thống kê token trên Sui (multi-DEX): metadata, pools, volume, price/OHLC, liquidity. Holders và bubble map ở phase sau.

## Core features (by phase)

| Phase | Scope |
|-------|-------|
| **0** ✅ | Event indexer, BCS decode (Cetus/Turbos), `suix_queryEvents`, reconciliation |
| **1** | Kafka ingestion, multi-DEX, token metadata |
| **2** | OHLC, volume 24h, pools API, TimescaleDB hot + ClickHouse cold |
| **3** | Pool snapshots, CLMM depth (batch) |
| **4** | Holders (coin balance pipeline) |
| **5** | Bubble map (transfer graph) |

## Docs

| Document | Content |
|----------|---------|
| [docs/01-product-scope.md](./docs/01-product-scope.md) | Chốt tính năng |
| [docs/02-system-architecture.md](./docs/02-system-architecture.md) | Chốt kiến trúc |
| [docs/03-roadmap-timeline.md](./docs/03-roadmap-timeline.md) | Chốt timeline |
| [docs/04-data-contracts.md](./docs/04-data-contracts.md) | Kafka + DB schemas |
| [docs/05-dex-coverage.md](./docs/05-dex-coverage.md) | DEX event checklist |

## Original requirements

- Danh sách pool của token và thông tin pool
- Token static: name, symbol, decimal, description, image, created_at, creator
- Token dynamic: volume (1h/6h/24h), swap history, chart, price, holder, liquidity
- Global stats: volume 24h, txns 24h
