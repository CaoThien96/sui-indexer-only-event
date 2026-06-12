//! CompositeStore: Kafka is the primary fact output; Postgres holds watermarks only.
//!
//! Handler `commit()` publishes facts via [`CompositeConnection::publish_facts`].
//! The framework advances watermarks through the [`Connection`] trait on the same
//! connection after a successful commit. Kafka produce failures prevent watermark
//! advancement (at-least-once on partial failure — see runbook).

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use scoped_futures::ScopedBoxFuture;
use sui_indexer_alt_framework_store_traits as store;
use sui_indexer_alt_framework_store_traits::Store;

use crate::kafka::{FactTopic, KafkaFactWriter, MessageEnvelope};
use crate::postgres::{PostgresConnection, PostgresStore};

#[derive(Clone)]
pub struct CompositeStore {
    pg: PostgresStore,
    kafka: Arc<KafkaFactWriter>,
}

pub struct CompositeConnection<'a> {
    pg: PostgresConnection<'a>,
    kafka: Arc<KafkaFactWriter>,
}

impl CompositeStore {
    pub fn new(pg: PostgresStore, kafka: KafkaFactWriter) -> Self {
        Self {
            pg,
            kafka: Arc::new(kafka),
        }
    }

    pub fn pg(&self) -> &PostgresStore {
        &self.pg
    }

    pub fn kafka(&self) -> &KafkaFactWriter {
        &self.kafka
    }
}

impl CompositeConnection<'_> {
    /// Publish fact envelopes to Kafka (BYOS primary output).
    pub async fn publish_facts(
        &self,
        topic: FactTopic,
        records: &[MessageEnvelope],
        partition_key_fn: impl Fn(&MessageEnvelope) -> String,
    ) -> Result<usize> {
        self.kafka.publish(topic, records, partition_key_fn).await
    }
}

#[async_trait]
impl store::Connection for CompositeConnection<'_> {
    async fn init_watermark(
        &mut self,
        pipeline_task: &str,
        checkpoint_hi_inclusive: Option<u64>,
    ) -> anyhow::Result<Option<store::InitWatermark>> {
        self.pg
            .init_watermark(pipeline_task, checkpoint_hi_inclusive)
            .await
    }

    async fn accepts_chain_id(
        &mut self,
        pipeline_task: &str,
        chain_id: [u8; 32],
    ) -> anyhow::Result<bool> {
        self.pg.accepts_chain_id(pipeline_task, chain_id).await
    }

    async fn committer_watermark(
        &mut self,
        pipeline_task: &str,
    ) -> anyhow::Result<Option<store::CommitterWatermark>> {
        self.pg.committer_watermark(pipeline_task).await
    }

    async fn set_committer_watermark(
        &mut self,
        pipeline_task: &str,
        watermark: store::CommitterWatermark,
    ) -> anyhow::Result<bool> {
        self.pg
            .set_committer_watermark(pipeline_task, watermark)
            .await
    }
}

#[async_trait]
impl store::SequentialConnection for CompositeConnection<'_> {}

#[async_trait]
impl store::Store for CompositeStore {
    type Connection<'c> = CompositeConnection<'c>;

    async fn connect<'c>(&'c self) -> anyhow::Result<Self::Connection<'c>> {
        Ok(CompositeConnection {
            pg: self.pg.connect().await?,
            kafka: Arc::clone(&self.kafka),
        })
    }
}

#[async_trait]
impl store::SequentialStore for CompositeStore {
    type SequentialConnection<'c> = CompositeConnection<'c>;

    async fn transaction<'a, R, F>(&self, f: F) -> anyhow::Result<R>
    where
        R: Send + 'a,
        F: Send + 'a,
        F: for<'r> FnOnce(
            &'r mut Self::Connection<'_>,
        ) -> ScopedBoxFuture<'a, 'r, anyhow::Result<R>>,
    {
        // Watermark ops are single-statement; Kafka is outside PG txn.
        // Framework calls commit then set_committer_watermark on the same connection.
        let mut conn = self.connect().await?;
        f(&mut conn).await
    }
}
