//! Experimental probe: compile-time `move_contract!` codegen (requires network at build).
//!
//! ```bash
//! cd event-bindings-probe && cargo build
//! cargo test -p event-bindings-probe
//! ```

use move_binding_derive::move_contract;

move_contract! {
    alias = "sui",
    package = "0x2",
    network = "mainnet"
}
move_contract! {
    alias = "integer_mate",
    package = "0xdfaadf86be9af246900d1e3f3b996cf549e7948e662a9977bdd7646d8fa3a778",
    network = "mainnet"
}
move_contract! {
    alias = "pkg_1eab",
    package = "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb",
    network = "mainnet"
}

#[cfg(test)]
mod tests {
    use super::*;

    const CETUS_SWAP_BCS: &str = "00440e5e3b13b8220c5c338bb5a4291cab5c58064eaf3654c77f3e9aed5147689c000000000000000000000000000000000000000000000000000000000000000000f9cc2f3c0000006d76b10a00000000000000000000000040cbd01e000000003177cdee010000009ffe799f2475000097577ecc93beb5ea2500000000000000962cd524d50bebec25000000000000000100000000000000";

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn decode_cetus_swap_event_via_move_contract() {
        let decoded: pkg_1eab::pool::SwapEvent =
            bcs::from_bytes(&hex_to_bytes(CETUS_SWAP_BCS)).unwrap();
        assert!(!decoded.atob);
        assert_eq!(decoded.amount_in, 258500000000);
        assert_eq!(decoded.steps, 1);
    }
}
