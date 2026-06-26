//! Optional Telegram Bot alerts for indexer processor failures.
//!
//! Set `TELEGRAM_BOT_TOKEN` and `TELEGRAM_CHAT_ID` to enable. Uses cooldown dedup so
//! framework retries do not flood the chat.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use tracing::{debug, warn};

const DEFAULT_COOLDOWN_SECS: u64 = 300;
const TELEGRAM_API_BASE: &str = "https://api.telegram.org";

struct TelegramNotifier {
    client: reqwest::Client,
    bot_token: String,
    chat_id: String,
    cooldown: Duration,
    last_sent: Mutex<HashMap<String, Instant>>,
}

static NOTIFIER: OnceLock<Option<TelegramNotifier>> = OnceLock::new();

impl TelegramNotifier {
    fn from_env() -> Option<Self> {
        let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())?;
        let chat_id = std::env::var("TELEGRAM_CHAT_ID")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())?;

        let cooldown_secs = std::env::var("TELEGRAM_NOTIFY_COOLDOWN_SECS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(DEFAULT_COOLDOWN_SECS);

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .expect("failed to build Telegram HTTP client");

        debug!(
            chat_id = %chat_id,
            cooldown_secs,
            "Telegram processor alerts enabled"
        );

        Some(Self {
            client,
            bot_token,
            chat_id,
            cooldown: Duration::from_secs(cooldown_secs),
            last_sent: Mutex::new(HashMap::new()),
        })
    }

    fn should_notify(&self, dedup_key: &str) -> bool {
        let now = Instant::now();
        let mut last_sent = self
            .last_sent
            .lock()
            .expect("telegram notify mutex poisoned");

        match last_sent.get(dedup_key) {
            Some(previous) if now.duration_since(*previous) < self.cooldown => false,
            _ => {
                last_sent.insert(dedup_key.to_string(), now);
                true
            }
        }
    }

    fn rollback_notify(&self, dedup_key: &str) {
        if let Ok(mut last_sent) = self.last_sent.lock() {
            last_sent.remove(dedup_key);
        }
    }

    async fn send_message(&self, text: &str) -> Result<()> {
        let url = format!("{TELEGRAM_API_BASE}/bot{}/sendMessage", self.bot_token);
        let response = self
            .client
            .post(url)
            .json(&serde_json::json!({
                "chat_id": self.chat_id,
                "text": text,
                "parse_mode": "HTML",
                "disable_web_page_preview": true,
            }))
            .send()
            .await
            .context("telegram sendMessage request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("telegram sendMessage returned {status}: {body}");
        }

        Ok(())
    }
}

fn notifier() -> Option<&'static TelegramNotifier> {
    NOTIFIER
        .get_or_init(TelegramNotifier::from_env)
        .as_ref()
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Notify operators that a checkpoint failed processing and will be retried.
pub async fn notify_processor_error(
    pipeline: &str,
    checkpoint_sequence_number: i64,
    event_type: &str,
    tx_digest: &str,
    event_seq: usize,
    error: &str,
) {
    let Some(notifier) = notifier() else {
        return;
    };

    let dedup_key = format!("{pipeline}:{event_type}:{error}");
    if !notifier.should_notify(&dedup_key) {
        debug!(
            pipeline,
            event_type,
            "Skipping Telegram alert (cooldown)"
        );
        return;
    }

    let message = format!(
        "<b>Indexer processor error</b>\n\n\
         <b>Pipeline:</b> <code>{}</code>\n\
         <b>Checkpoint:</b> <code>{}</code>\n\
         <b>Event type:</b> <code>{}</code>\n\
         <b>Tx:</b> <code>{}</code> (event_seq <code>{}</code>)\n\
         <b>Error:</b> <code>{}</code>\n\n\
         Watermark is blocked until this is fixed.",
        escape_html(pipeline),
        checkpoint_sequence_number,
        escape_html(event_type),
        escape_html(tx_digest),
        event_seq,
        escape_html(error),
    );

    if let Err(send_error) = notifier.send_message(&message).await {
        notifier.rollback_notify(&dedup_key);
        warn!(
            error = %send_error,
            pipeline,
            event_type,
            "Failed to send Telegram processor alert"
        );
    }
}

/// Send a bot alert message (snip/sell/state). No cooldown dedup.
pub async fn send_message(text: &str) {
    let Some(notifier) = notifier() else {
        return;
    };
    if let Err(send_error) = notifier.send_message(text).await {
        warn!(error = %send_error, "Failed to send Telegram bot alert");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_html_escapes_special_chars() {
        assert_eq!(
            escape_html("a & b <c>"),
            "a &amp; b &lt;c&gt;"
        );
    }
}
