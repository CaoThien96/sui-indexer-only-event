use chrono::{DateTime, Duration, Utc};
use sui_processors::clickhouse::ClickHouseConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageTarget {
    Hot,
    Cold,
    Both,
}

pub fn route_query(from: DateTime<Utc>, to: DateTime<Utc>, hot_days: i64) -> StorageTarget {
    let hot_cutoff = Utc::now() - Duration::days(hot_days);
    if to >= hot_cutoff && from >= hot_cutoff {
        StorageTarget::Hot
    } else if to < hot_cutoff {
        StorageTarget::Cold
    } else {
        StorageTarget::Both
    }
}

pub async fn init_clickhouse() -> anyhow::Result<clickhouse::Client> {
    let config = ClickHouseConfig::from_env()?;
    sui_processors::clickhouse::run_migrations(&config).await?;
    Ok(sui_processors::clickhouse::create_client(&config))
}
