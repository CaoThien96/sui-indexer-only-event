## Tài liệu Custom Indexing Framework trên Sui

Dưới đây là tổng hợp tài liệu đầy đủ về Custom Indexing Framework để bạn có thể dùng cho agent học:

---

### 1. Tổng quan

`sui-indexer-alt-framework` là một Rust framework để xây dựng custom blockchain indexer hiệu suất cao trên Sui. Framework cung cấp các thành phần production-ready cho:
- **Data ingestion** (nhận dữ liệu checkpoint)
- **Processing** (xử lý và transform dữ liệu)
- **Storage** (lưu trữ linh hoạt)

[[Custom Indexers](https://docs.sui.io/develop/accessing-data/custom-indexer/custom-indexers)]

---

### 2. Các API cốt lõi

| API | Mô tả |
|---|---|
| `process()` | Transform raw checkpoint data thành database rows |
| `commit()` | Lưu dữ liệu đã xử lý vào database theo batch |
| `prune()` | Dọn dẹp dữ liệu cũ theo retention policy (tùy chọn) |

[[Custom Indexing Framework](https://docs.sui.io/develop/accessing-data/custom-indexer)]

---

### 3. Kiến trúc dữ liệu

#### Nguồn dữ liệu (Data Sources)

**Polling-based:**
- Remote store HTTPS (`https://checkpoints.mainnet.sui.io`) — chỉ giữ 30 ngày gần nhất
- GCS bucket (`gs://mysten-mainnet-checkpoints-use4`) — full retention, khuyến nghị cho production
- Local files — chỉ dùng cho testing
- RPC endpoints — kết nối trực tiếp full node

**Push-based:**
- gRPC streaming — độ trễ thấp nhất, cần kết hợp với polling làm fallback

[[Integrate Data Sources](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-data-integration)]

#### Ingestion Layer

`Broadcaster` nhận checkpoint từ data source và phân phối đến nhiều pipeline song song. Nó áp dụng **backpressure** dựa trên watermark cao nhất từ các subscriber, đảm bảo không đẩy dữ liệu nhanh hơn pipeline chậm nhất có thể xử lý. [[Custom Indexers](https://docs.sui.io/develop/accessing-data/custom-indexer/custom-indexers#how-custom-indexers-fit-into-the-application-stack)]

---

### 4. Hai loại Pipeline

Framework cung cấp hai kiến trúc pipeline:

- **Sequential Pipeline** — xử lý checkpoint theo đúng thứ tự, đảm bảo tính nhất quán, phù hợp cho hầu hết use case
- **Concurrent Pipeline** — xử lý song song, phù hợp khi cần throughput cao hơn

Chi tiết tại: [[Pipeline Architecture](https://docs.sui.io/develop/accessing-data/custom-indexer/pipeline-architecture)]

---

### 5. Khi nào nên dùng Custom Indexer?

- Cần kiểm soát loại, độ chi tiết và thời gian lưu trữ dữ liệu
- Có query pattern đặc thù không được phục vụ bởi gRPC hoặc GraphQL RPC
- Muốn tối ưu chi phí lưu trữ qua custom pruning
- Managed services từ RPC provider không đáp ứng được nhu cầu

[[Custom Indexers](https://docs.sui.io/develop/accessing-data/custom-indexer/custom-indexers)]

---

### 6. Danh sách tài liệu đầy đủ

| Tài liệu | Nội dung | Link |
|---|---|---|
| What Are Custom Indexers? | Tổng quan, use case, kiến trúc | [Link](https://docs.sui.io/develop/accessing-data/custom-indexer/custom-indexers) |
| Build a Custom Indexer | Hướng dẫn từng bước xây dựng indexer | [Link](https://docs.sui.io/develop/accessing-data/custom-indexer/build) |
| Pipeline Architecture | So sánh sequential vs concurrent pipeline | [Link](https://docs.sui.io/develop/accessing-data/custom-indexer/pipeline-architecture) |
| Integrate Data Sources | Cấu hình checkpoint sources, BYOS, BCS deserialization | [Link](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-data-integration) |
| Bring Your Own Store (BYOS) | Implement custom storage backend | [Link](https://docs.sui.io/develop/accessing-data/custom-indexer/bring-your-own-store) |
| Optimize Runtime & Performance | Tuning, monitoring, Prometheus metrics, pruning | [Link](https://docs.sui.io/develop/accessing-data/custom-indexer/indexer-runtime-perf) |

[[Custom Indexing Framework](https://docs.sui.io/develop/accessing-data/custom-indexer)]

---

### 7. Source code tham khảo

- Framework: [`crates/sui-indexer-alt-framework`](https://github.com/MystenLabs/sui/tree/main/crates/sui-indexer-alt-framework)
- Ví dụ cơ bản: [`examples/rust/basic-sui-indexer`](https://github.com/MystenLabs/sui/tree/main/examples/rust/basic-sui-indexer)