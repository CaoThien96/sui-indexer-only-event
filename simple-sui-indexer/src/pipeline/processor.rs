/// Implementors of this trait are responsible for transforming checkpoint into rows for their
/// table.
#[async_trait]
pub trait Processor: Send + Sync + 'static {
    /// Used to identify the pipeline in logs and metrics.
    const NAME: &'static str;

    /// The type of value being inserted by the handler.
    type Value: Send + Sync + 'static;

    /// The processing logic for turning a checkpoint into rows of the table.
    ///
    /// All errors returned from this method are treated as transient and will be retried
    /// indefinitely with exponential backoff.
    ///
    /// If you encounter a permanent error that will never succeed on retry (e.g., invalid data
    /// format, unsupported protocol version), you should panic! This stops the indexer and alerts
    /// operators that manual intervention is required. Do not return permanent errors as they will
    /// cause infinite retries and block the pipeline.
    ///
    /// For transient errors (e.g., network issues, rate limiting), simply return the error and
    /// let the framework retry automatically.
    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>>;
}