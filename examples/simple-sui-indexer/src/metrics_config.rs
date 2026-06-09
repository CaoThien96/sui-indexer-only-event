//! Metrics configuration helpers (framework built-ins + env overrides).

use std::net::SocketAddr;

use sui_indexer_alt_framework::cluster::Args;
use tracing::info;

/// Override framework metrics CLI args from environment when set.
pub fn apply_metrics_env(args: &mut Args) {
    if let Ok(raw) = std::env::var("METRICS_ADDRESS") {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            match trimmed.parse::<SocketAddr>() {
                Ok(addr) => args.metrics_args.metrics_address = addr,
                Err(error) => {
                    tracing::warn!(
                        metrics_address = %trimmed,
                        error = %error,
                        "Ignoring invalid METRICS_ADDRESS"
                    );
                }
            }
        }
    }
}

pub fn metrics_prefix_from_env() -> Option<String> {
    std::env::var("METRICS_PREFIX")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn log_metrics_endpoint(args: &Args) {
    let addr = args.metrics_args.metrics_address;
    info!(
        metrics_address = %addr,
        metrics_url = format!("http://{addr}/metrics"),
        "Prometheus metrics (framework built-ins + app counters)"
    );
}
