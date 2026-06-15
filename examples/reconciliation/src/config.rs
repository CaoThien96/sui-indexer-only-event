use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clickhouse::Client;

#[derive(Debug, Clone)]
pub struct Config {
    pub clickhouse_url: String,
    pub clickhouse_database: String,
    pub clickhouse_user: Option<String>,
    pub clickhouse_password: Option<String>,
    pub fullnode_url: String,
    pub move_event_type: String,
    pub count_tolerance: i64,
    pub key_tolerance: usize,
    pub max_key_samples: usize,
    pub window_start: DateTime<Utc>,
    pub window_end: DateTime<Utc>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let clickhouse_url = std::env::var("CLICKHOUSE_URL")
            .context("CLICKHOUSE_URL must be set (e.g. http://localhost:8123)")?;
        let clickhouse_database = std::env::var("CLICKHOUSE_DATABASE")
            .unwrap_or_else(|_| "sui_indexer".to_string());
        let clickhouse_user = std::env::var("CLICKHOUSE_USER").ok();
        let clickhouse_password = std::env::var("CLICKHOUSE_PASSWORD").ok();

        let fullnode_url = std::env::var("FULLNODE_URL")
            .unwrap_or_else(|_| "https://fullnode.mainnet.sui.io:443".to_string());

        let move_event_type = std::env::var("RECON_MOVE_EVENT_TYPE")
            .context("RECON_MOVE_EVENT_TYPE must be set")?;

        let window_hours: i64 = std::env::var("RECON_WINDOW_HOURS")
            .unwrap_or_else(|_| "1".to_string())
            .parse()
            .context("RECON_WINDOW_HOURS must be a positive integer")?;

        let end_hours_ago: i64 = std::env::var("RECON_WINDOW_END_HOURS_AGO")
            .unwrap_or_else(|_| "2".to_string())
            .parse()
            .context("RECON_WINDOW_END_HOURS_AGO must be a positive integer")?;

        if window_hours <= 0 || end_hours_ago <= 0 {
            anyhow::bail!("window hours and end-hours-ago must be positive");
        }

        let count_tolerance: i64 = std::env::var("RECON_COUNT_TOLERANCE")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .context("RECON_COUNT_TOLERANCE must be a non-negative integer")?;

        if count_tolerance < 0 {
            anyhow::bail!("RECON_COUNT_TOLERANCE must be non-negative");
        }

        let key_tolerance: usize = std::env::var("RECON_KEY_TOLERANCE")
            .unwrap_or_else(|_| "0".to_string())
            .parse()
            .context("RECON_KEY_TOLERANCE must be a non-negative integer")?;

        let max_key_samples: usize = std::env::var("RECON_MAX_KEY_SAMPLES")
            .unwrap_or_else(|_| "20".to_string())
            .parse()
            .context("RECON_MAX_KEY_SAMPLES must be a positive integer")?;

        if max_key_samples == 0 {
            anyhow::bail!("RECON_MAX_KEY_SAMPLES must be greater than 0");
        }

        let now = Utc::now();
        let window_end = now - chrono::Duration::hours(end_hours_ago);
        let window_start = window_end - chrono::Duration::hours(window_hours);

        Ok(Self {
            clickhouse_url,
            clickhouse_database,
            clickhouse_user,
            clickhouse_password,
            fullnode_url,
            move_event_type,
            count_tolerance,
            key_tolerance,
            max_key_samples,
            window_start,
            window_end,
        })
    }

    pub fn clickhouse_client(&self) -> Client {
        let mut client = Client::default()
            .with_url(&self.clickhouse_url)
            .with_database(&self.clickhouse_database);

        if let Some(user) = &self.clickhouse_user {
            client = client.with_user(user.clone());
        }
        if let Some(password) = &self.clickhouse_password {
            client = client.with_password(password.clone());
        }

        client
    }

    pub fn indexer_event_type(&self) -> &str {
        &self.move_event_type
    }

    pub fn start_time_ms(&self) -> i64 {
        self.window_start.timestamp_millis()
    }

    pub fn end_time_ms(&self) -> i64 {
        self.window_end.timestamp_millis()
    }
}
