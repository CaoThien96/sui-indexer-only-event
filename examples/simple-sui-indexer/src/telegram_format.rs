//! Markdown Telegram message templates for the snip bot.

use crate::bot::state::Dex;

const EMOJI_CAP: u128 = 30;
const ERROR_MAX_CHARS: usize = 120;

pub fn dexscreener_pool_url(pool: &str) -> String {
    format!("https://dexscreener.com/sui/{pool}")
}

pub fn suivision_tx_url(tx_digest: &str) -> String {
    format!("https://suivision.xyz/txblock/{tx_digest}")
}

pub fn suivision_account_url(account: &str) -> String {
    format!("https://suivision.xyz/account/{account}")
}

pub fn short_address(addr: &str) -> String {
    if addr.len() > 10 {
        format!("{}...{}", &addr[..5], &addr[addr.len() - 5..])
    } else {
        addr.to_string()
    }
}

pub fn format_sui(amount_mist: u128) -> String {
    let sui = amount_mist as f64 / 1e9;
    format!("{sui:.2}")
}

pub fn format_token_amount(amount: u128) -> String {
    let value = amount as f64 / 1e9;
    let formatted = format!("{value:.9}");
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

pub fn volume_emojis(sui_mist: u128, is_buy: bool) -> String {
    let emoji = if is_buy { "🐳" } else { "🛑" };
    let count = (sui_mist / 1_000_000_000).max(1).min(EMOJI_CAP);
    emoji.repeat(count as usize)
}

pub fn truncate_error(err: &str) -> String {
    let one_line = err.replace('\n', " ");
    if one_line.len() <= ERROR_MAX_CHARS {
        one_line
    } else {
        format!("{}...", &one_line[..ERROR_MAX_CHARS])
    }
}

pub fn escape_markdown(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('_', "\\_")
        .replace('*', "\\*")
        .replace('`', "\\`")
        .replace('[', "\\[")
}

fn dex_label(dex: Dex) -> &'static str {
    dex.as_str()
}

fn dex_link_markdown(pool: &str) -> String {
    let url = dexscreener_pool_url(pool);
    format!("[DEX]({url})")
}

pub fn format_detect_pool(symbol: &str, dex: Dex, pool: &str, tx_digest: &str) -> String {
    let symbol = escape_markdown(symbol);
    let dex = dex_label(dex);
    let tx_url = suivision_tx_url(tx_digest);
    format!(
        "🚀 Detect {symbol} on {dex} added {} [Link]({tx_url})",
        dex_link_markdown(pool)
    )
}

pub fn format_snip_success(symbol: &str, dex: Dex, tx_digest: &str) -> String {
    let symbol = escape_markdown(symbol);
    let dex = dex_label(dex);
    let tx_url = suivision_tx_url(tx_digest);
    format!("⚡️ Snip {symbol} on {dex} success [Link]({tx_url})")
}

pub fn format_snip_fail_buy(symbol: &str, dex: Dex, pool: &str, err: &str) -> String {
    let symbol = escape_markdown(symbol);
    let dex = dex_label(dex);
    let err = escape_markdown(&truncate_error(err));
    format!(
        "⭕️ Snip {symbol} on {dex} failed\n💼: {}\n❌ Buy failed: {err}",
        dex_link_markdown(pool)
    )
}

pub fn format_snip_fail_lp_after_buy(
    symbol: &str,
    dex: Dex,
    pool: &str,
    buy_digest: &str,
    err: &str,
) -> String {
    let symbol = escape_markdown(symbol);
    let dex = dex_label(dex);
    let buy_url = suivision_tx_url(buy_digest);
    let err = escape_markdown(&truncate_error(err));
    format!(
        "⭕️ Snip {symbol} on {dex} failed\n⚡️ Buy may have succeeded [Link]({buy_url})\n⭕️ Add Liquidity failed\n💼: {}\n❌ {err}",
        dex_link_markdown(pool)
    )
}

pub fn format_add_liquidity_success(symbol: &str, tx_digest: &str) -> String {
    let symbol = escape_markdown(symbol);
    let tx_url = suivision_tx_url(tx_digest);
    format!("✅ Add Liquidity {symbol} success [Link]({tx_url})")
}

pub fn format_add_liquidity_fail(symbol: &str, pool: &str, err: &str) -> String {
    let symbol = escape_markdown(symbol);
    let err = escape_markdown(&truncate_error(err));
    format!(
        "⭕️ Add Liquidity {symbol} failed\n💼: {}\n❌ {err}",
        dex_link_markdown(pool)
    )
}

pub fn format_old_token_swap(
    sui_mist: u128,
    token_amount: u128,
    is_buy: bool,
    maker: &str,
    pool: &str,
) -> String {
    let emojis = volume_emojis(sui_mist, is_buy);
    let sui = format_sui(sui_mist);
    let token = format_token_amount(token_amount);
    let short = short_address(maker);
    let account_url = suivision_account_url(maker);
    format!(
        "{emojis}\n🔀 {sui} Sui ~ {token} Token 👥: [{short}]({account_url})\n💼: {}",
        dex_link_markdown(pool)
    )
}

pub fn format_sell_success(symbol: &str, pool: &str, tx_digest: &str, trigger_sui_mist: u128) -> String {
    let symbol = escape_markdown(symbol);
    let sui = format_sui(trigger_sui_mist);
    let tx_url = suivision_tx_url(tx_digest);
    format!(
        "✅ Sold {symbol} success [Link]({tx_url})\n🔀 triggered by {sui} Sui buy\n💼: {}",
        dex_link_markdown(pool)
    )
}

pub fn format_sell_fail(
    symbol: &str,
    pool: &str,
    trigger_sui_mist: u128,
    attempts: usize,
    err: &str,
) -> String {
    let symbol = escape_markdown(symbol);
    let sui = format_sui(trigger_sui_mist);
    let err = escape_markdown(&truncate_error(err));
    format!(
        "⭕️ Sell {symbol} failed\n🔀 triggered by {sui} Sui buy\n💼: {}\n❌ after {attempts} attempts: {err}",
        dex_link_markdown(pool)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_address_truncates() {
        assert_eq!(
            short_address("0xa6c4f8e499b2abcdef1234567890"),
            "0xa6c...67890"
        );
    }

    #[test]
    fn volume_emojis_capped() {
        assert_eq!(volume_emojis(50_000_000_000, true).chars().count(), 30);
        assert_eq!(volume_emojis(500_000_000, true), "🐳");
        assert_eq!(volume_emojis(12_870_000_000, false).chars().count(), 12);
    }

    #[test]
    fn format_sui_two_decimals() {
        assert_eq!(format_sui(12_870_000_000), "12.87");
    }

    #[test]
    fn detect_pool_message_contains_links() {
        let msg = format_detect_pool(
            "SATO",
            Dex::Turbos,
            "0xpool",
            "8t5XmojBUchJXWJT6ygC1aKyAH1hMEyY79ybFf7Wp9yF",
        );
        assert!(msg.contains("dexscreener.com/sui/0xpool"));
        assert!(msg.contains("suivision.xyz/txblock/"));
        assert!(msg.contains("SATO on TURBOS"));
    }

    #[test]
    fn snip_fail_buy_message() {
        let msg = format_snip_fail_buy("SATO", Dex::Turbos, "0xpool", "no coins");
        assert!(msg.contains("Buy failed"));
        assert!(msg.contains("dexscreener"));
    }

    #[test]
    fn old_token_swap_buy_emojis() {
        let msg = format_old_token_swap(
            12_870_000_000,
            240_642_534,
            true,
            "0xa6c4f8e499b2",
            "0xpool",
        );
        assert!(msg.starts_with("🐳"));
        assert!(msg.contains("12.87 Sui"));
    }
}
