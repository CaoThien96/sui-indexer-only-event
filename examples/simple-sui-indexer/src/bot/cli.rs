use anyhow::{bail, Result};

use crate::bot::state::Dex;
use crate::dex::agg_swap::SwapMode;

pub fn parse_dex(s: &str) -> Result<Dex> {
    match s.to_ascii_lowercase().as_str() {
        "cetus" => Ok(Dex::Cetus),
        "turbos" => Ok(Dex::Turbos),
        other => bail!("unknown dex '{other}' (use cetus or turbos)"),
    }
}

pub fn parse_swap_mode(s: &str) -> Result<SwapMode> {
    match s.to_ascii_lowercase().as_str() {
        "superfast" => Ok(SwapMode::Superfast),
        "fast" => Ok(SwapMode::Fast),
        "safe" => Ok(SwapMode::Safe),
        other => bail!("unknown swap mode '{other}' (use safe, fast, superfast)"),
    }
}
