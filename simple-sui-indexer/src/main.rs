mod handlers;
mod models;

use handlers::TransactionDigestHandler;

pub mod schema;

use anyhow::{Result, bail};
use clap::Parser;
use diesel_migrations::{EmbeddedMigrations, embed_migrations};
use sui_indexer_alt_framework::{
    cluster::{Args, IndexerCluster},
    pipeline::sequential::SequentialConfig,
    service::Error,
};
use tracing::info;
use url::Url;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment")
        .parse::<Url>()
        .expect("Invalid database URL");

    let args = Args::parse();
    info!("Parsed CLI arguments and initialized runtime");

    let mut cluster = IndexerCluster::builder()
        .with_args(args)
        .with_database_url(database_url)
        .with_migrations(&MIGRATIONS)
        .build()
        .await?;
    info!("Indexer cluster initialized");

    cluster
        .sequential_pipeline(TransactionDigestHandler, SequentialConfig::default())
        .await?;
    info!("Sequential pipeline registered, indexer is starting");

    match cluster.run().await?.main().await {
        Ok(()) | Err(Error::Terminated) => {
            info!("Indexer terminated normally");
            Ok(())
        }
        Err(Error::Aborted) => bail!("Indexer aborted due to an unexpected error"),
        Err(Error::Task(e)) => bail!(e),
    }
}
