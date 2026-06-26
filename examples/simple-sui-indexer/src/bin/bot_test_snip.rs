//! Manual snip + LP test (on-chain txs). Does not require the indexer to be running.
//!
//! Example — full snip flow for VALORA:
//!   cargo run --release --bin bot-test-snip -- \
//!     --pool 0x0fe41e5fbefdbbe4ed9a31bfca2ca376b82363efac8facd7736482b040af234b \
//!     --token 0x9ba2573e31978148d69aeab42eeb0cf241b84539030dd1dd0fc82216088b4b68::valora::VALORA \
//!     --dex cetus --symbol VALORA
//!
//! Buy only (no LP):
//!   cargo run --release --bin bot-test-snip -- --valora --buy-only
//!
//! LP only (wallet must already hold the token):
//!   cargo run --release --bin bot-test-snip -- --valora --lp-only --lp-wait-ms 0

use anyhow::{Context, Result, bail};
use clap::Parser;
use simple_sui_indexer::bot::config::BotRuntime;
use simple_sui_indexer::bot::snip::{run_snip, SnipRunOptions};
use simple_sui_indexer::bot::state::Dex;
use simple_sui_indexer::bootstrap;

const VALORA_POOL: &str =
    "0x0fe41e5fbefdbbe4ed9a31bfca2ca376b82363efac8facd7736482b040af234b";
const VALORA_TOKEN: &str =
    "0x9ba2573e31978148d69aeab42eeb0cf241b84539030dd1dd0fc82216088b4b68::valora::VALORA";

#[derive(Parser, Debug)]
#[command(name = "bot-test-snip")]
struct Args {
    /// Shorthand: use the VALORA pool/token from the recent create-pool log.
    #[arg(long)]
    valora: bool,

    #[arg(long)]
    pool: Option<String>,

    #[arg(long)]
    token: Option<String>,

    #[arg(long, default_value = "VALORA")]
    symbol: String,

    #[arg(long, value_parser = parse_dex, default_value = "cetus")]
    dex: Dex,

    /// Run snip buy only (SUI → token).
    #[arg(long, conflicts_with = "lp_only")]
    buy_only: bool,

    /// Run add-liquidity only (skip buy).
    #[arg(long, conflicts_with = "buy_only")]
    lp_only: bool,

    /// Override `SNIP_BUY_AMOUNT` (mist).
    #[arg(long)]
    buy_amount: Option<u64>,

    /// Wait before LP step (default: `SNIP_LP_WAIT_MS` or 0 with `--lp-only`).
    #[arg(long)]
    lp_wait_ms: Option<u64>,
}

fn parse_dex(s: &str) -> Result<Dex> {
    match s.to_ascii_lowercase().as_str() {
        "cetus" => Ok(Dex::Cetus),
        "turbos" => Ok(Dex::Turbos),
        other => bail!("unknown dex '{other}' (use cetus or turbos)"),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let dotenv = bootstrap::load_dotenv();
    bootstrap::init_tracing();
    bootstrap::log_dotenv_load(&dotenv);

    let args = Args::parse();

    let (pool, token) = if args.valora {
        (
            args.pool.unwrap_or_else(|| VALORA_POOL.to_string()),
            args.token.unwrap_or_else(|| VALORA_TOKEN.to_string()),
        )
    } else {
        (
            args.pool.context("--pool is required (or use --valora)")?,
            args.token.context("--token is required (or use --valora)")?,
        )
    };

    let runtime = BotRuntime::init().await?;
    let vault = runtime.vault.address_string();
    tracing::info!(
        pool = %pool,
        token = %token,
        symbol = %args.symbol,
        dex = ?args.dex,
        vault = %vault,
        buy_only = args.buy_only,
        lp_only = args.lp_only,
        "starting manual snip test (real on-chain txs)"
    );

    let lp_wait_ms = args.lp_wait_ms.or(if args.lp_only { Some(0) } else { None });

    run_snip(
        &runtime,
        args.dex,
        &token,
        &pool,
        &args.symbol,
        SnipRunOptions {
            skip_buy: args.lp_only,
            skip_lp: args.buy_only,
            buy_amount: args.buy_amount,
            lp_wait_ms,
        },
    )
    .await?;

    tracing::info!("manual snip test finished");
    Ok(())
}
