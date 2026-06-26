#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_sui_indexer::run_indexer().await
}
