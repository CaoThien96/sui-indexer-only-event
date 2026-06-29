//! Manual sell test (on-chain txs). Does not require the indexer to be running.
//!
//! Vault sell when `USE_SNIP_VAULT=true` (withdraws from vault Bag), else agg_swap from wallet:
//!   cargo run --release --bin bot-sell -- \
//!     --pool 0x... \
//!     --token 0x...::coin::COIN \
//!     --amount 1000000000 \
//!     --dex cetus --symbol MYCOIN
//!
//! Production-like retries (99%, 98%, …):
//!   cargo run --release --bin bot-sell -- --pool 0x... --token 0x... --amount 1000000000 --retries 5
//!
//! Dry-run:
//!   cargo run --release --bin bot-sell -- --pool 0x... --token 0x... --amount 1000000000 --dry-run

use anyhow::{Context, Result};
use clap::Parser;
use simple_sui_indexer::bot::cli::{parse_dex, parse_swap_mode};
use simple_sui_indexer::bot::config::BotRuntime;
use simple_sui_indexer::bot::sell::run_sell_manual;
use simple_sui_indexer::bot::state::{BotStateStore, Dex};
use simple_sui_indexer::bootstrap;
use simple_sui_indexer::dex::agg_swap::SwapMode;

#[derive(Parser, Debug)]
#[command(name = "bot-sell")]
struct Args {
    #[arg(long)]
    pool: String,

    #[arg(long)]
    token: String,

    #[arg(long, default_value = "TOKEN")]
    symbol: String,

    #[arg(long, value_parser = parse_dex, default_value = "cetus")]
    dex: Dex,

    /// Token amount to sell (base units / mist).
    #[arg(long)]
    amount: u64,

    /// agg_swap mode when not using vault (`safe`, `fast`, `superfast`).
    #[arg(long, value_parser = parse_swap_mode, default_value = "safe")]
    mode: SwapMode,

    /// Retry attempts with decreasing % (default 1 = single try at 99%).
    #[arg(long, default_value = "1")]
    retries: usize,

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
    if args.amount == 0 {
        anyhow::bail!("--amount must be positive");
    }

    let runtime = BotRuntime::init().await?;

    tracing::info!(
        pool = %args.pool,
        token = %args.token,
        symbol = %args.symbol,
        dex = ?args.dex,
        amount = args.amount,
        mode = ?args.mode,
        retries = args.retries,
        vault = %runtime.vault.address_string(),
        use_snip_vault = runtime.snip_vault.is_some(),
        dry_run = args.dry_run,
        "starting manual sell (real on-chain txs)"
    );

    let store = if let Ok(url) = std::env::var("DATABASE_URL") {
        Some(BotStateStore::connect(&url).await?)
    } else {
        None
    };

    let digest = run_sell_manual(
        &runtime,
        store.as_ref(),
        args.dex,
        &args.token,
        &args.pool,
        args.amount,
        args.mode,
        args.retries,
        args.dry_run,
    )
    .await
    .with_context(|| format!("sell {} on {}", args.symbol, args.pool))?;

    println!(
        "{}",
        if args.dry_run {
            format!("sell dry-run OK: {digest}")
        } else {
            format!("sell confirmed: {digest}")
        }
    );
    Ok(())
}
