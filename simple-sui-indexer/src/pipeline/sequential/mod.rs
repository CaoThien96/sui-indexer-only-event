/// Handlers implement the logic for a given indexing pipeline: How to process checkpoint data (by
/// implementing [Processor]) into rows for their table, how to combine multiple rows into a single
/// DB operation, and then how to write those rows atomically to the database.
///
/// The handler is also responsible for tuning the various parameters of the pipeline (provided as
/// associated values).
///
/// Sequential handlers can only be used in sequential pipelines, where checkpoint data is
/// processed out-of-order, but then gathered and written in order. If multiple checkpoints are
/// available, the pipeline will attempt to combine their writes taking advantage of batching to
/// avoid emitting redundant writes.
///
/// Back-pressure is handled by the bounded subscriber channel from the ingestion service, the
/// same as concurrent pipelines: the channel blocks broadcaster sends when full, and the adaptive
/// ingestion controller cuts fetch concurrency as the channel fills up.
#[async_trait]
pub trait Handler: Processor {
    type Store: SequentialStore;

    /// If at least this many rows are pending, the committer will commit them eagerly.
    const MIN_EAGER_ROWS: usize = 50;

    /// Soft cap: once this many rows are pending, the collector stops eagerly draining
    /// its input channel and yields to the flush phase. Receive is never hard-gated — unlike
    /// concurrent pipelines, a missing predecessor may be buried in the input channel, and
    /// blocking receive would risk deadlock. The cap only bounds receive-to-flush latency in
    /// the happy path.
    const MAX_PENDING_ROWS: usize = 5000;

    /// Maximum number of checkpoints to try and write in a single batch. The larger this number
    /// is, the more chances the pipeline has to merge redundant writes, but the longer each write
    /// transaction is likely to be.
    const MAX_BATCH_CHECKPOINTS: usize = 10;

    /// A type to combine multiple `Self::Value`-s into. This can be used to avoid redundant writes
    /// by combining multiple rows into one (e.g. if one row supersedes another, the latter can be
    /// omitted).
    type Batch: Default + Send + Sync + 'static;

    /// Add `values` from processing a checkpoint to the current `batch`. Checkpoints are
    /// guaranteed to be presented to the batch in checkpoint order. The handler takes ownership
    /// of the iterator and consumes all values.
    ///
    /// Returns `BatchStatus::Ready` if the batch is full and should be committed,
    /// or `BatchStatus::Pending` if the batch can accept more values.
    ///
    /// Note: The handler can signal batch readiness via `BatchStatus::Ready`, but the framework
    /// may also decide to commit a batch based on the trait parameters above.
    fn batch(&self, batch: &mut Self::Batch, values: std::vec::IntoIter<Self::Value>);

    /// Take a batch of values and commit them to the database, returning the number of rows
    /// affected.
    async fn commit<'a>(
        &self,
        batch: &Self::Batch,
        conn: &mut <Self::Store as Store>::Connection<'a>,
    ) -> anyhow::Result<usize>;
}