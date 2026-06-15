use anyhow::{Context, Result};
use clickhouse::Client;

#[derive(Debug, Clone)]
pub struct ClickHouseConfig {
    pub url: String,
    pub database: String,
    pub user: Option<String>,
    pub password: Option<String>,
}

impl ClickHouseConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            url: std::env::var("CLICKHOUSE_URL")
                .context("CLICKHOUSE_URL must be set (e.g. http://localhost:8123)")?,
            database: std::env::var("CLICKHOUSE_DATABASE")
                .unwrap_or_else(|_| "sui_indexer".to_string()),
            user: std::env::var("CLICKHOUSE_USER").ok(),
            password: std::env::var("CLICKHOUSE_PASSWORD").ok(),
        })
    }

    pub fn client(&self) -> Client {
        let mut client = Client::default()
            .with_url(&self.url)
            .with_database(&self.database);

        if let Some(user) = &self.user {
            client = client.with_user(user.clone());
        }
        if let Some(password) = &self.password {
            client = client.with_password(password.clone());
        }

        client
    }
}

pub fn create_client(config: &ClickHouseConfig) -> Client {
    config.client()
}
