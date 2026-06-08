use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub rpc_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .context("DATABASE_URL must be set")?;
        let rpc_port = std::env::var("RPC_PORT")
            .unwrap_or_else(|_| "9000".to_string())
            .parse()
            .context("RPC_PORT must be a valid u16")?;

        Ok(Self {
            database_url,
            rpc_port,
        })
    }
}
