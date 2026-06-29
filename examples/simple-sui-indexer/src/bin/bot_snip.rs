//! Manual snip + LP test (on-chain txs). Does not require the indexer to be running.
//!
//! Full snip (vault if `USE_SNIP_VAULT=true`, else agg_swap + LP):
//!   cargo run --release --bin bot-snip -- \
//!     --pool 0x... \
//!     --token 0x...::coin::COIN \
//!     --dex cetus --symbol MYCOIN
//!
//! Buy only:
//!   cargo run --release --bin bot-snip -- --pool 0x... --token 0x... --buy-only
//!
//! LP only (wallet must hold token):
//!   cargo run --release --bin bot-snip -- --pool 0x... --token 0x... --lp-only --lp-wait-ms 0
//!
//! Dry-run (simulate, no submit):
//!   cargo run --release --bin bot-snip -- --pool 0x... --token 0x... --dry-run

use anyhow::Result;
use clap::Parser;
use simple_sui_indexer::bot::cli::parse_dex;
use simple_sui_indexer::bot::config::BotRuntime;
use simple_sui_indexer::bot::snip::{run_snip, SnipRunOptions};
use simple_sui_indexer::bot::state::{BotStateStore, Dex};
use simple_sui_indexer::bootstrap;

// #region agent log
use simple_sui_indexer::bot::debug_log::agent_log;
// #endregion

#[derive(Parser, Debug)]
#[command(name = "bot-snip")]
struct Args {
    #[arg(long)]
    pool: String,

    #[arg(long)]
    token: String,

    #[arg(long, default_value = "TOKEN")]
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

    /// Simulate PTB via `sui_dryRunTransactionBlock` (no on-chain submit).
    #[arg(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let dotenv = bootstrap::load_dotenv();
    bootstrap::init_tracing();
    bootstrap::log_dotenv_load(&dotenv);

    let args = Args::parse();

    // #region agent log
    agent_log(
        "H4",
        "bot_snip.rs:main",
        "cli args",
        serde_json::json!({
            "pool": args.pool,
            "token": args.token,
            "dex": format!("{:?}", args.dex),
            "dry_run": args.dry_run,
            "buy_only": args.buy_only,
            "lp_only": args.lp_only,
        }),
    );
    // #endregion

    let runtime = BotRuntime::init().await?;

    // #region agent log
    agent_log(
        "H4",
        "bot_snip.rs:main",
        "runtime ready",
        serde_json::json!({
            "vault_address": runtime.vault.address_string(),
            "use_snip_vault": runtime.snip_vault.is_some(),
            "snip_buy_amount": runtime.config.snip_buy_amount,
        }),
    );
    // #endregion

    tracing::info!(
        pool = %args.pool,
        token = %args.token,
        symbol = %args.symbol,
        dex = ?args.dex,
        vault = %runtime.vault.address_string(),
        use_snip_vault = runtime.snip_vault.is_some(),
        buy_only = args.buy_only,
        lp_only = args.lp_only,
        dry_run = args.dry_run,
        "starting manual snip (real on-chain txs)"
    );

    let lp_wait_ms = args.lp_wait_ms.or(if args.lp_only { Some(0) } else { None });

    let store = if let Ok(url) = std::env::var("DATABASE_URL") {
        Some(BotStateStore::connect(&url).await?)
    } else {
        None
    };

    run_snip(
        &runtime,
        store.as_ref(),
        args.dex,
        &args.token,
        &args.pool,
        &args.symbol,
        SnipRunOptions {
            skip_buy: args.lp_only,
            skip_lp: args.buy_only,
            buy_amount: args.buy_amount,
            lp_wait_ms,
            dry_run: args.dry_run,
        },
    )
    .await
    .map_err(|e| {
        // #region agent log
        agent_log(
            "H1",
            "bot_snip.rs:main",
            "run_snip error",
            serde_json::json!({ "error": e.to_string() }),
        );
        // #endregion
        anyhow::anyhow!("{e}")
    })?;

    if args.dry_run {
        println!("snip dry-run finished OK (no tx submitted)");
    } else {
        println!("snip finished OK");
    }
    Ok(())
}
