use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use chrono::{Duration as ChronoDuration, Utc};
use tracing::{error, info};

use crate::clickhouse::{
    self, ChOhlcRow, ChSwapRow, ClickHouseConfig, create_client,
};
use crate::metrics::RolloffMetrics;
use crate::timescale::{RolloffOhlcRow, RolloffSwapRow, TimescaleStore};

pub struct RolloffJob {
    timescale: TimescaleStore,
    clickhouse: clickhouse::ClickHouseClient,
    metrics: Arc<RolloffMetrics>,
    batch_size: i64,
    hot_storage_days: i64,
}

impl RolloffJob {
    pub async fn new(metrics: Arc<RolloffMetrics>) -> Result<Self> {
        let ts_url = crate::config::timescale_url()?;
        let ch_config = ClickHouseConfig::from_env()?;
        clickhouse::run_migrations(&ch_config).await?;
        let timescale = TimescaleStore::connect(ts_url).await?;
        timescale.run_migrations().await?;
        let clickhouse = create_client(&ch_config);

        let batch_size = std::env::var("ROLLOFF_BATCH_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10_000);
        let hot_storage_days = std::env::var("HOT_STORAGE_DAYS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        Ok(Self {
            timescale,
            clickhouse,
            metrics,
            batch_size,
            hot_storage_days,
        })
    }

    pub async fn run_once(&self) -> Result<()> {
        let cutoff = Utc::now() - ChronoDuration::days(self.hot_storage_days);
        self.roll_table_swaps(cutoff).await?;
        self.roll_table_ohlc(cutoff).await?;
        Ok(())
    }

    async fn roll_table_swaps(&self, cutoff: chrono::DateTime<Utc>) -> Result<()> {
        let table = "swaps_fact";
        let watermark = self.timescale.get_rolloff_watermark(table).await?;
        if watermark >= cutoff {
            return Ok(());
        }

        let rows = self
            .timescale
            .fetch_swaps_for_rolloff(watermark, cutoff, self.batch_size)
            .await?;
        if rows.is_empty() {
            return Ok(());
        }

        let ch_rows: Vec<ChSwapRow> = rows.iter().map(swap_to_ch).collect();
        if let Err(e) = clickhouse::insert_swaps(&self.clickhouse, &ch_rows).await {
            self.metrics.errors.with_label_values(&[table]).inc();
            error!(error = %e, "Failed to insert swaps into ClickHouse");
            return Err(e);
        }

        let last_time = rows.last().map(|r| r.time).unwrap_or(watermark);
        self.timescale.set_rolloff_watermark(table, last_time).await?;
        self.metrics
            .rows
            .with_label_values(&[table])
            .inc_by(rows.len() as u64);
        info!(table, count = rows.len(), ?last_time, "Rolled off swaps");
        Ok(())
    }

    async fn roll_table_ohlc(&self, cutoff: chrono::DateTime<Utc>) -> Result<()> {
        let table = "ohlc_1m";
        let watermark = self.timescale.get_rolloff_watermark(table).await?;
        if watermark >= cutoff {
            return Ok(());
        }

        let rows = self
            .timescale
            .fetch_ohlc_for_rolloff(watermark, cutoff, self.batch_size)
            .await?;
        if rows.is_empty() {
            return Ok(());
        }

        let ch_rows: Vec<ChOhlcRow> = rows.iter().map(ohlc_to_ch).collect();
        if let Err(e) = clickhouse::insert_ohlc(&self.clickhouse, &ch_rows).await {
            self.metrics.errors.with_label_values(&[table]).inc();
            error!(error = %e, "Failed to insert ohlc into ClickHouse");
            return Err(e);
        }

        let last_time = rows.last().map(|r| r.bucket).unwrap_or(watermark);
        self.timescale.set_rolloff_watermark(table, last_time).await?;
        self.metrics
            .rows
            .with_label_values(&[table])
            .inc_by(rows.len() as u64);
        info!(table, count = rows.len(), ?last_time, "Rolled off ohlc");
        Ok(())
    }

    pub async fn run_loop(&self, interval: Duration) -> Result<()> {
        loop {
            if let Err(e) = self.run_once().await {
                error!(error = %e, "Rolloff tick failed");
            }
            tokio::time::sleep(interval).await;
        }
    }
}

fn swap_to_ch(row: &RolloffSwapRow) -> ChSwapRow {
    ChSwapRow {
        time: row.time,
        tx_digest: row.tx_digest.clone(),
        event_seq: row.event_seq,
        protocol: row.protocol.clone(),
        pool_id: row.pool_id.clone(),
        base_coin_type: row.base_coin_type.clone(),
        quote_coin_type: row.quote_coin_type.clone(),
        amount_base: row.amount_base.clone(),
        amount_quote: row.amount_quote.clone(),
        price_quote_per_base: row.price_quote_per_base.clone(),
        fee_amount: row.fee_amount.clone(),
        sender: row.sender.clone(),
        checkpoint_seq: row.checkpoint_seq,
    }
}

fn ohlc_to_ch(row: &RolloffOhlcRow) -> ChOhlcRow {
    ChOhlcRow {
        bucket: row.bucket,
        pool_id: row.pool_id.clone(),
        base_coin_type: row.base_coin_type.clone(),
        quote_coin_type: row.quote_coin_type.clone(),
        open: row.open.clone(),
        high: row.high.clone(),
        low: row.low.clone(),
        close: row.close.clone(),
        volume_quote: row.volume_quote.clone(),
        trade_count: row.trade_count,
    }
}
