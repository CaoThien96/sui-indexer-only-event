use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::coin_type;
use crate::timescale::TimescaleStore;

#[derive(Debug, Clone)]
pub struct EnrichedUsd {
    pub price_usd_per_base: Decimal,
    pub amount_usd: Decimal,
}

#[derive(Debug, Clone)]
pub enum UsdEnrichmentOutcome {
    Enriched(EnrichedUsd),
    NotApplicable,
    OracleMissing,
}

pub async fn enrich_swap_usd(
    store: &TimescaleStore,
    quote_coin_type: &str,
    price_quote_per_base: Decimal,
    amount_quote: Decimal,
    time: DateTime<Utc>,
) -> Result<UsdEnrichmentOutcome> {
    let quote = coin_type::normalize(quote_coin_type);
    let maybe_quote_usd = if quote == coin_type::USDC_COIN_TYPE {
        Some(Decimal::ONE)
    } else if quote == coin_type::SUI_COIN_TYPE {
        store.latest_sui_usd_at_or_before(time).await?
    } else {
        return Ok(UsdEnrichmentOutcome::NotApplicable);
    };

    let Some(quote_usd) = maybe_quote_usd else {
        return Ok(UsdEnrichmentOutcome::OracleMissing);
    };

    Ok(UsdEnrichmentOutcome::Enriched(EnrichedUsd {
        price_usd_per_base: (price_quote_per_base * quote_usd).normalize(),
        amount_usd: (amount_quote * quote_usd).normalize(),
    }))
}
