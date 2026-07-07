use chrono::{DateTime, TimeZone, Timelike, Utc};

pub const OHLC_INTERVALS: &[&str] = &["1m", "5m", "15m", "30m", "1h", "4h", "24h"];

pub fn token_ohlc_usd_table(interval: &str) -> Option<&'static str> {
    match interval {
        "1m" => Some("token_ohlc_usd_1m"),
        "5m" => Some("token_ohlc_usd_5m"),
        "15m" => Some("token_ohlc_usd_15m"),
        "30m" => Some("token_ohlc_usd_30m"),
        "1h" => Some("token_ohlc_usd_1h"),
        "4h" => Some("token_ohlc_usd_4h"),
        "24h" => Some("token_ohlc_usd_24h"),
        _ => None,
    }
}

/// Floor `time` to the start of the OHLC bucket for `interval`.
pub fn bucket_for_interval(time: DateTime<Utc>, interval: &str) -> DateTime<Utc> {
    match interval {
        "1m" => {
            let floored = (time.timestamp_millis() / 60_000) * 60_000;
            Utc.timestamp_millis_opt(floored)
                .single()
                .unwrap_or(time)
        }
        "5m" => {
            let floored = (time.timestamp_millis() / 300_000) * 300_000;
            Utc.timestamp_millis_opt(floored)
                .single()
                .unwrap_or(time)
        }
        "15m" => {
            let floored = (time.timestamp_millis() / 900_000) * 900_000;
            Utc.timestamp_millis_opt(floored)
                .single()
                .unwrap_or(time)
        }
        "30m" => {
            let floored = (time.timestamp_millis() / 1_800_000) * 1_800_000;
            Utc.timestamp_millis_opt(floored)
                .single()
                .unwrap_or(time)
        }
        "1h" => time
            .with_minute(0)
            .and_then(|t| t.with_second(0))
            .and_then(|t| t.with_nanosecond(0))
            .unwrap_or(time),
        "4h" => {
            let hour = (time.hour() / 4) * 4;
            time.with_hour(hour)
                .and_then(|t| t.with_minute(0))
                .and_then(|t| t.with_second(0))
                .and_then(|t| t.with_nanosecond(0))
                .unwrap_or(time)
        }
        "24h" => time
            .with_hour(0)
            .and_then(|t| t.with_minute(0))
            .and_then(|t| t.with_second(0))
            .and_then(|t| t.with_nanosecond(0))
            .unwrap_or(time),
        _ => time,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn bucket_5m_aligns() {
        let t = Utc.with_ymd_and_hms(2026, 7, 5, 12, 7, 30).unwrap();
        let b = bucket_for_interval(t, "5m");
        assert_eq!(b, Utc.with_ymd_and_hms(2026, 7, 5, 12, 5, 0).unwrap());
    }

    #[test]
    fn bucket_1h_aligns() {
        let t = Utc.with_ymd_and_hms(2026, 7, 5, 12, 45, 0).unwrap();
        let b = bucket_for_interval(t, "1h");
        assert_eq!(b, Utc.with_ymd_and_hms(2026, 7, 5, 12, 0, 0).unwrap());
    }
}
