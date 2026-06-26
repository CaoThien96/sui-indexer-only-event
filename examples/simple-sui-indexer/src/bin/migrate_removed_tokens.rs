use anyhow::{Context, Result};
use simple_sui_indexer::bootstrap;
use std::env;
use tokio_postgres::{Client, NoTls};

async fn connect_db(url: &str, label: &'static str) -> Result<Client> {
    let (client, connection) = tokio_postgres::connect(url, NoTls)
        .await
        .with_context(|| format!("connect {label} db"))?;
    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("{label} postgres connection error: {err}");
        }
    });
    Ok(client)
}

#[tokio::main]
async fn main() -> Result<()> {
    let dotenv = bootstrap::load_dotenv();

    let source_url = env::var("BOT_SNIP_DATABASE_URL")
        .context("BOT_SNIP_DATABASE_URL must be set (bot-snip Postgres)")?;
    let target_url =
        env::var("DATABASE_URL").context("DATABASE_URL must be set (indexer Postgres)")?;

    let source = connect_db(&source_url, "source").await?;
    let target = connect_db(&target_url, "target").await?;

    let rows = source
        .query(
            r#"
            SELECT t.id, t.name, t.symbol, t.decimals, t.total_supply, t.owner,
                   t.deny_cap_id, t.pool_id, t.created_at, t.updated_at,
                   p.id AS pool_object_id, p.dex::text AS dex, p.tx_digest
            FROM tokens t
            JOIN pools p ON t.pool_id = p.id
            WHERE t.status = '3' AND p.dex IN ('CETUS', 'TURBOS')
            "#,
            &[],
        )
        .await
        .context("query removed tokens from bot-snip")?;

    let mut tokens = 0usize;
    let mut pools = 0usize;
    let mut swaps = 0usize;

    for row in &rows {
        let token_id: String = row.get(0);
        let name: String = row.get(1);
        let symbol: String = row.get(2);
        let decimals: i32 = row.get(3);
        let total_supply: i64 = row.get(4);
        let owner: String = row.get(5);
        let deny_cap_id: String = row.get(6);
        let pool_id: Option<String> = row.get(7);
        let created_at: chrono::DateTime<chrono::Utc> = row.get(8);
        let updated_at: chrono::DateTime<chrono::Utc> = row.get(9);
        let pool_object_id: String = row.get(10);
        let dex: String = row.get(11);
        let tx_digest: String = row.get(12);

        let inserted = target
            .execute(
                r#"
                INSERT INTO bot_tokens (
                    id, name, symbol, decimals, total_supply, owner, deny_cap_id,
                    status, pool_id, created_at, updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, 'done', $8, $9, $10)
                ON CONFLICT (id) DO NOTHING
                "#,
                &[
                    &token_id,
                    &name,
                    &symbol,
                    &decimals,
                    &total_supply,
                    &owner,
                    &deny_cap_id,
                    &pool_id,
                    &created_at,
                    &updated_at,
                ],
            )
            .await?;
        if inserted > 0 {
            tokens += 1;
        }

        let inserted = target
            .execute(
                r#"
                INSERT INTO bot_pools (id, token_id, dex, tx_digest, created_at, updated_at)
                VALUES ($1, $2, $3::text::bot_dex, $4, NOW(), NOW())
                ON CONFLICT (id) DO NOTHING
                "#,
                &[&pool_object_id, &token_id, &dex, &tx_digest],
            )
            .await?;
        if inserted > 0 {
            pools += 1;
        }
    }

    let pool_ids: Vec<String> = rows.iter().map(|r| r.get::<_, String>(10)).collect();
    if !pool_ids.is_empty() {
        let swap_rows = source
            .query(
                "SELECT id, pool_id, tx_digest, event_seq FROM swaps WHERE pool_id = ANY($1)",
                &[&pool_ids],
            )
            .await
            .context("query swaps from bot-snip")?;

        for row in swap_rows {
            let id: String = row.get(0);
            let pool_id: String = row.get(1);
            let tx_digest: String = row.get(2);
            let event_seq: String = row.get(3);
            let inserted = target
                .execute(
                    r#"
                    INSERT INTO bot_processed_swaps (id, pool_id, tx_digest, event_seq)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT (id) DO NOTHING
                    "#,
                    &[&id, &pool_id, &tx_digest, &event_seq],
                )
                .await?;
            if inserted > 0 {
                swaps += 1;
            }
        }
    }

    println!(
        "Migration complete: {tokens} tokens, {pools} pools, {swaps} processed swaps copied (from {} source rows)",
        rows.len()
    );

    Ok(())
}
