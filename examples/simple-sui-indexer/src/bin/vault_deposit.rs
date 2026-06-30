//! Move wallet tokens into the on-chain snip vault Bag (migration after agg_swap snips).
//!
//! Requires `USE_SNIP_VAULT=true`, `SNIP_VAULT_PACKAGE`, `SNIP_VAULT_OBJECT_ID`, `VAULT_PATH`.
//!
//! Deposit full wallet balance for one token:
//!   cargo run --release --bin vault-deposit -- \
//!     --token 0x9ba2573e31978148d69aeab42eeb0cf241b84539030dd1dd0fc82216088b4b68::valora::VALORA
//!
//! Partial deposit (mist / base units):
//!   cargo run --release --bin vault-deposit -- --token 0x...::valora::VALORA --amount 1000000000

use anyhow::{Context, Result};
use clap::Parser;
use simple_sui_indexer::bootstrap;
use simple_sui_indexer::bot::config::BotRuntime;
use simple_sui_indexer::dex::SnipVaultClient;

#[derive(Parser, Debug)]
#[command(name = "vault-deposit")]
struct Args {
    /// Coin type to deposit (full package::module::STRUCT path).
    #[arg(long)]
    token: String,

    /// Amount in base units; omit to deposit entire wallet balance for this type.
    #[arg(long)]
    amount: Option<u64>,

    #[arg(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let dotenv = bootstrap::load_dotenv();
    bootstrap::init_tracing();
    bootstrap::log_dotenv_load(&dotenv);

    let args = Args::parse();
    let runtime = BotRuntime::init().await?;
    let vault = runtime
        .snip_vault
        .as_ref()
        .context("USE_SNIP_VAULT must be true with SNIP_VAULT_PACKAGE and SNIP_VAULT_OBJECT_ID")?;

    let digest = vault
        .deposit_token(&runtime, &args.token, args.amount, args.dry_run)
        .await
        .with_context(|| format!("deposit {}", args.token))?;

    println!(
        "{}",
        if args.dry_run {
            format!("deposit dry-run OK: {digest}")
        } else {
            format!("deposit confirmed: {digest}")
        }
    );
    Ok(())
}
