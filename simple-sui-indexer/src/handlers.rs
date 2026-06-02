use anyhow::Result;
use std::sync::OnceLock;
use std::sync::Arc;
use sui_indexer_alt_framework::pipeline::Processor;
use sui_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;

use crate::models::StoredTransactionDigest;
use crate::schema::transaction_digests::dsl::*;
use diesel_async::RunQueryDsl;
use sui_indexer_alt_framework::{
    pipeline::sequential::Handler,
    postgres::{Connection, Db},
};
use tracing::{debug, info};

pub struct TransactionDigestHandler;

fn log_every_n_checkpoints() -> i64 {
    static LOG_EVERY_N: OnceLock<i64> = OnceLock::new();
    *LOG_EVERY_N.get_or_init(|| {
        std::env::var("LOG_EVERY_N_CHECKPOINTS")
            .ok()
            .and_then(|value| value.parse::<i64>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(100)
    })
}

#[async_trait::async_trait]
impl Processor for TransactionDigestHandler {
    const NAME: &'static str = "transaction_digest_handler";

    type Value = StoredTransactionDigest;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_seq = checkpoint.summary.sequence_number as i64;
        let tx_count = checkpoint.transactions.len();

        let digests = checkpoint
            .transactions
            .iter()
            .map(|tx| StoredTransactionDigest {
                tx_digest: tx.transaction.digest().to_string(),
                checkpoint_sequence_number: checkpoint_seq,
            })
            .collect();

        let log_every_n = log_every_n_checkpoints();
        if checkpoint_seq % log_every_n == 0 {
            info!(
                checkpoint_sequence_number = checkpoint_seq,
                transaction_count = tx_count,
                log_every_n_checkpoints = log_every_n,
                "Indexing progress"
            );
        }

        Ok(digests)
    }
}
#[async_trait::async_trait]
impl Handler for TransactionDigestHandler {
    type Store = Db;
    type Batch = Vec<Self::Value>;

    fn batch(&self, batch: &mut Self::Batch, values: std::vec::IntoIter<Self::Value>) {
        batch.extend(values);
    }

    async fn commit<'a>(&self, batch: &Self::Batch, conn: &mut Connection<'a>) -> Result<usize> {
        let inserted = diesel::insert_into(transaction_digests)
            .values(batch)
            .on_conflict(tx_digest)
            .do_nothing()
            .execute(conn)
            .await?;

        debug!(
            batch_size = batch.len(),
            inserted_rows = inserted,
            "Committed batch to PostgreSQL"
        );

        Ok(inserted)
    }
}