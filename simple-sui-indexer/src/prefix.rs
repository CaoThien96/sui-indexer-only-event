/// Returns true when `event_type` matches the given Move event type prefix.
///
/// Prefix forms:
/// - Package-level (ends with `::`): matches all events in the package.
/// - Module/event-level (no trailing `::`): boundary-safe match so `pool`
///   matches `pool::SwapEvent` but not `pool_factory::CreateEvent`.
pub fn matches_prefix(event_type: &str, prefix: &str) -> bool {
    if prefix.ends_with("::") {
        event_type.starts_with(prefix)
    } else {
        event_type == prefix || event_type.starts_with(&format!("{prefix}::"))
    }
}

pub fn matches_any_prefix(event_type: &str, prefixes: &[String]) -> bool {
    prefixes
        .iter()
        .any(|prefix| matches_prefix(event_type, prefix))
}

/// Extracts unique package IDs (the address before the first `::`) from event type prefixes.
pub fn package_ids_from_prefixes(prefixes: &[String]) -> Vec<String> {
    let mut package_ids = std::collections::BTreeSet::new();
    for prefix in prefixes {
        if let Some(package_id) = prefix.split("::").next() {
            if !package_id.is_empty() {
                package_ids.insert(package_id.to_string());
            }
        }
    }
    package_ids.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PKG: &str =
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb";
    const PKG_PREFIX: &str =
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::";

    #[test]
    fn package_prefix_matches_all_modules() {
        assert!(matches_prefix(
            &format!("{PKG}::pool::SwapEvent"),
            PKG_PREFIX
        ));
        assert!(matches_prefix(
            &format!("{PKG}::factory::CreatePoolEvent"),
            PKG_PREFIX
        ));
    }

    #[test]
    fn module_prefix_matches_module_events_only() {
        let module_prefix = format!("{PKG}::pool");

        assert!(matches_prefix(
            &format!("{PKG}::pool::SwapEvent"),
            &module_prefix
        ));
        assert!(matches_prefix(
            &format!("{PKG}::pool::RemoveLiquidityEvent"),
            &module_prefix
        ));
        assert!(!matches_prefix(
            &format!("{PKG}::pool_factory::PoolCreatedEvent"),
            &module_prefix
        ));
        assert!(!matches_prefix(
            &format!("{PKG}::factory::CreatePoolEvent"),
            &module_prefix
        ));
    }

    #[test]
    fn event_prefix_matches_exact_event_only() {
        let event_prefix = format!("{PKG}::pool::SwapEvent");

        assert!(matches_prefix(
            &format!("{PKG}::pool::SwapEvent"),
            &event_prefix
        ));
        assert!(!matches_prefix(
            &format!("{PKG}::pool::BurnEvent"),
            &event_prefix
        ));
    }

    #[test]
    fn package_ids_from_prefixes_deduplicates() {
        let prefixes = vec![
            format!("{PKG}::pool"),
            format!("{PKG}::pool::SwapEvent"),
            PKG_PREFIX.to_string(),
            "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::".to_string(),
        ];

        let ids = package_ids_from_prefixes(&prefixes);
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&PKG.to_string()));
        assert!(ids.contains(
            &"0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1".to_string()
        ));
    }

    #[test]
    fn matches_any_prefix_or_logic() {
        let prefixes = vec![
            format!("{PKG}::pool"),
            "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::".to_string(),
        ];

        assert!(matches_any_prefix(
            &format!("{PKG}::pool::SwapEvent"),
            &prefixes
        ));
        assert!(matches_any_prefix(
            "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::pool_factory::PoolCreatedEvent",
            &prefixes
        ));
        assert!(!matches_any_prefix(
            "0xdeadbeef::pool::SwapEvent",
            &prefixes
        ));
    }
}
