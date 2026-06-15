use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub clickhouse: crate::db::ClickHouseConfig,
    pub rpc_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let rpc_port = std::env::var("RPC_PORT")
            .unwrap_or_else(|_| "9000".to_string())
            .parse()
            .context("RPC_PORT must be a valid u16")?;

        Ok(Self {
            clickhouse: crate::db::ClickHouseConfig::from_env()?,
            rpc_port,
        })
    }
}
