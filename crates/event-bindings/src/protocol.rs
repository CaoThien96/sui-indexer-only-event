//! Protocol enum and event-type classification.

use crate::config::{
    self, POOL_CREATE_EVENT_TYPES, SWAP_EVENT_TYPES, bluefin, cetus, flowx, magma, mmt, turbos,
};

/// Supported DEX protocol slugs (Kafka `protocol` field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    Cetus,
    Turbos,
    Bluefin,
    Mmt,
    Flowx,
    Magma,
}

impl Protocol {
    pub const ALL: [Protocol; 6] = [
        Protocol::Cetus,
        Protocol::Turbos,
        Protocol::Bluefin,
        Protocol::Mmt,
        Protocol::Flowx,
        Protocol::Magma,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cetus => cetus::SLUG,
            Self::Turbos => turbos::SLUG,
            Self::Bluefin => bluefin::SLUG,
            Self::Mmt => mmt::SLUG,
            Self::Flowx => flowx::SLUG,
            Self::Magma => magma::SLUG,
        }
    }

    /// Defining package address embedded in canonical `event_type` strings.
    pub fn type_package_id(self) -> &'static str {
        match self {
            Self::Cetus => cetus::TYPE_PACKAGE,
            Self::Turbos => turbos::TYPE_PACKAGE,
            Self::Bluefin => bluefin::TYPE_PACKAGE,
            Self::Mmt => mmt::TYPE_PACKAGE,
            Self::Flowx => flowx::TYPE_PACKAGE,
            Self::Magma => magma::TYPE_PACKAGE,
        }
    }

    pub fn swap_event_type(self) -> &'static str {
        match self {
            Self::Cetus => cetus::SWAP_EVENT,
            Self::Turbos => turbos::SWAP_EVENT,
            Self::Bluefin => bluefin::SWAP_EVENT,
            Self::Mmt => mmt::SWAP_EVENT,
            Self::Flowx => flowx::SWAP_EVENT,
            Self::Magma => magma::SWAP_EVENT,
        }
    }

    pub fn pool_create_event_type(self) -> &'static str {
        match self {
            Self::Cetus => cetus::POOL_CREATE_EVENT,
            Self::Turbos => turbos::POOL_CREATE_EVENT,
            Self::Bluefin => bluefin::POOL_CREATE_EVENT,
            Self::Mmt => mmt::POOL_CREATE_EVENT,
            Self::Flowx => flowx::POOL_CREATE_EVENT,
            Self::Magma => magma::POOL_CREATE_EVENT,
        }
    }

    pub fn pool_id_field(self) -> &'static str {
        match self {
            Self::Cetus => cetus::POOL_ID_FIELD,
            Self::Turbos => turbos::POOL_ID_FIELD,
            Self::Bluefin => bluefin::POOL_ID_FIELD,
            Self::Mmt => mmt::POOL_ID_FIELD,
            Self::Flowx => flowx::POOL_ID_FIELD,
            Self::Magma => magma::POOL_ID_FIELD,
        }
    }

    /// Resolve protocol from the package prefix in a canonical event type.
    pub fn from_event_type_package(event_type: &str) -> Option<Self> {
        let package = event_type.split("::").next()?;
        Self::from_type_package_id(package)
    }

    pub fn from_type_package_id(package: &str) -> Option<Self> {
        let package = package.to_ascii_lowercase();
        for protocol in Self::ALL {
            if protocol.type_package_id().eq_ignore_ascii_case(&package) {
                return Some(protocol);
            }
        }
        None
    }
}

// Re-export registry constants for callers using `event_bindings::protocol::*`.
pub use config::{
    BLUEFIN_POOL_CREATE_EVENT, BLUEFIN_SWAP_EVENT, BLUEFIN_TYPE_PACKAGE, CETUS_POOL_CREATE_EVENT,
    CETUS_SWAP_EVENT, CETUS_TYPE_PACKAGE, FLOWX_POOL_CREATE_EVENT, FLOWX_SWAP_EVENT,
    FLOWX_TYPE_PACKAGE, MAGMA_POOL_CREATE_EVENT, MAGMA_SWAP_EVENT, MAGMA_TYPE_PACKAGE,
    MMT_POOL_CREATE_EVENT, MMT_SWAP_EVENT, MMT_TYPE_PACKAGE, TURBOS_POOL_CREATE_EVENT,
    TURBOS_SWAP_EVENT, TURBOS_TYPE_PACKAGE,
};

/// Classify swap events by full canonical `event_type` (case-insensitive).
pub fn classify_swap_event(event_type: &str) -> Option<Protocol> {
    if !SWAP_EVENT_TYPES
        .iter()
        .any(|canonical| event_type.eq_ignore_ascii_case(canonical))
    {
        return None;
    }
    Protocol::from_event_type_package(event_type)
}

pub fn classify_pool_create_event(event_type: &str) -> Option<Protocol> {
    if !POOL_CREATE_EVENT_TYPES
        .iter()
        .any(|canonical| event_type.eq_ignore_ascii_case(canonical))
    {
        return None;
    }
    Protocol::from_event_type_package(event_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_swap_by_full_event_type() {
        assert_eq!(
            classify_swap_event(CETUS_SWAP_EVENT),
            Some(Protocol::Cetus)
        );
        assert_eq!(classify_swap_event(MMT_SWAP_EVENT), Some(Protocol::Mmt));
        assert_eq!(classify_swap_event(FLOWX_SWAP_EVENT), Some(Protocol::Flowx));
        assert_eq!(classify_swap_event(MAGMA_SWAP_EVENT), Some(Protocol::Magma));
        assert_eq!(classify_swap_event(CETUS_POOL_CREATE_EVENT), None);
    }

    #[test]
    fn classifies_pool_create_events() {
        assert_eq!(
            classify_pool_create_event(BLUEFIN_POOL_CREATE_EVENT),
            Some(Protocol::Bluefin)
        );
        assert_eq!(
            classify_pool_create_event(FLOWX_POOL_CREATE_EVENT),
            Some(Protocol::Flowx)
        );
        assert_eq!(classify_pool_create_event(BLUEFIN_SWAP_EVENT), None);
    }
}
