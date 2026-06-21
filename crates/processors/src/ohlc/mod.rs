mod merge;

pub use merge::{OhlcBar, merge_bar, swap_to_bar};

use std::collections::HashSet;
use std::sync::Arc;

use anyhow::Result;
use indexer_store::MessageEnvelope;
use tracing::warn;

use crate::metrics::MetricsBundle;
use crate::normalized_swap::parse_normalized_swap;
use crate::timescale::TimescaleStore;

pub struct OhlcAggregator {
    store: TimescaleStore,
    metrics: Arc<MetricsBundle>,
    seen_swaps: HashSet<String>,
}

impl OhlcAggregator {
    pub fn new(store: TimescaleStore, metrics: Arc<MetricsBundle>) -> Self {
        Self {
            store,
            metrics,
            seen_swaps: HashSet::new(),
        }
    }

    pub async fn handle(&mut self, envelope: &MessageEnvelope) -> Result<()> {
        let swap = match parse_normalized_swap(envelope) {
            Ok(s) => s,
            Err(e) => {
                self.metrics
                    .ohlc_skipped
                    .with_label_values(&["parse_error"])
                    .inc();
                warn!(error = %e, "Failed to parse normalized swap for OHLC");
                return Ok(());
            }
        };

        if !self.seen_swaps.insert(swap.swap_key.clone()) {
            self.metrics
                .ohlc_skipped
                .with_label_values(&["duplicate"])
                .inc();
            return Ok(());
        }

        if self.seen_swaps.len() > 100_000 {
            self.seen_swaps.clear();
        }

        let bar = swap_to_bar(&swap);
        self.store.upsert_ohlc_1m(&bar).await?;
        self.metrics
            .ohlc_buckets_updated
            .with_label_values(&[&swap.protocol])
            .inc();

        Ok(())
    }
}
