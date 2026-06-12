# Sui Token Analytics — Mục lục tài liệu (Tiếng Việt)

Tài liệu lập kế hoạch chính thức cho hệ thống thống kê token trên Sui.  
**Code production sẽ xây greenfield** — xem [06-reference-examples.md](./06-reference-examples.md).

> **Bản tiếng Anh:** [../README.md](../README.md)

| # | Tài liệu | Nội dung |
|---|----------|----------|
| 1 | [01-product-scope.md](./01-product-scope.md) | **Chốt tính năng** — phạm vi theo phase, in/out of scope |
| 2 | [02-system-architecture.md](./02-system-architecture.md) | **Chốt kiến trúc** — các lớp, component, luồng dữ liệu, storage |
| 3 | [03-roadmap-timeline.md](./03-roadmap-timeline.md) | **Chốt timeline** — milestone, phụ thuộc, tiêu chí hoàn thành |
| 4 | [04-data-contracts.md](./04-data-contracts.md) | Kafka topics, schema chuẩn hóa, bảng DB |
| 5 | [05-dex-coverage.md](./05-dex-coverage.md) | DEX hỗ trợ, package ID, event, trạng thái decode |
| 6 | [06-reference-examples.md](./06-reference-examples.md) | `examples/` là gì — chỉ tham khảo, không production |
| 7 | [07-indexer-optimization-checklist.md](./07-indexer-optimization-checklist.md) | Checklist tối ưu indexer theo official Sui docs |

## Kế hoạch triển khai

| Kế hoạch | Nội dung |
|----------|----------|
| [plans/week-01-02-greenfield-indexer.md](./plans/week-01-02-greenfield-indexer.md) | **Tuần 1–2** — chi tiết từng ngày |

## Tài liệu liên quan trong repo

| File | Mục đích |
|------|----------|
| [../../requirement.md](../../requirement.md) | Tóm tắt sản phẩm |
| [../../events.md](../../events.md) | Mẫu payload event on-chain |
| [../indexing_document.md](../indexing_document.md) | Ghi chú framework `sui-indexer-alt` |

## Trạng thái dự án

| Hạng mục | Trạng thái |
|----------|------------|
| Tài liệu lập kế hoạch (`docs/`) | ✅ Đã chốt |
| Triển khai production | ⬜ **Greenfield — chưa bắt đầu** |
| `examples/` | Code spike / tham khảo — **không deploy** |

## Official Sui docs (bắt buộc tham chiếu khi làm indexer)

- [Custom Indexers](https://docs.sui.io/develop/accessing-data/custom-indexer/custom-indexers)
- [Pipeline Architecture](https://docs.sui.io/develop/accessing-data/custom-indexer/pipeline-architecture)
- [Integrate Data Sources](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-data-integration)
- [Bring Your Own Store (BYOS)](https://docs.sui.io/develop/accessing-data/custom-indexer/bring-your-own-store)
- [Optimize Runtime and Performance](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-runtime-perf)
- [ClickHouse BYOS example](https://github.com/MystenLabs/sui/tree/main/examples/rust/clickhouse-sui-indexer)
