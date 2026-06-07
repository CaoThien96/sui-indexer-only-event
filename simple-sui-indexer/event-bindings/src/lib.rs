//! Static event bindings generated via `move_contract!` at compile time.
//!
//! Requires network on first `cargo build` (GraphQL package fetch).
//! Patched move-binding: `../vendor/move-binding` (updated GraphQL URL).

mod parsed_json;

use move_binding_derive::move_contract;

// Sui framework — register type paths for cross-package references (no codegen).
move_contract! {
    alias = "sui",
    package = "0x2",
    network = "mainnet",
    register_only = true
}

// Cetus CLMM dependency: integer types used by pool events (I32 tick fields, etc.)
move_contract! {
    alias = "integer_mate",
    package = "0xdfaadf86be9af246900d1e3f3b996cf549e7948e662a9977bdd7646d8fa3a778",
    network = "mainnet",
    emit_mode = "module_structs",
    modules = "i32,i64,i128,full_math_u128,full_math_u64,math_u128,math_u256,math_u64"
}

// Cetus CLMM — package 0x1eab…
move_contract! {
    alias = "pkg_1eab",
    package = "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb",
    network = "mainnet",
    event_modules = "pool,partner,factory",
    linkage = "0x2=sui,0xdfaadf86be9af246900d1e3f3b996cf549e7948e662a9977bdd7646d8fa3a778=integer_mate,0x714a63a0dba6da4f017b42d5d0fb78867f18bcde904868e51d951a5a6f5b7f57=integer_mate"
}

// Turbos CLMM — package 0x91bf… (i32 lives in the same package)
move_contract! {
    alias = "pkg_91bf",
    package = "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1",
    network = "mainnet",
    event_modules = "pool,position_manager,pool_factory",
    support_modules = "i32,i64,i128,full_math_u128,full_math_u64,math_u128,math_u64,math_u256,math_sqrt_price,math_tick,math_liquidity,math_bit,math_swap",
    linkage = "0x2=sui"
}

use anyhow::{Result, bail};
use serde_json::Value;

macro_rules! decode_arm {
    ($bcs:expr, $ty:ty) => {{
        let decoded: $ty = bcs::from_bytes($bcs)?;
        Ok(parsed_json::normalize(serde_json::to_value(decoded)?))
    }};
}

/// Decode event BCS bytes into JSON matching fullnode `parsedJson` shape.
pub fn decode_parsed_json(event_type: &str, bcs: &[u8]) -> Result<Value> {
    match event_type {
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::swapevent" => {
            decode_arm!(bcs, pkg_1eab::pool::SwapEvent)
        }
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::collectfeeevent" => {
            decode_arm!(bcs, pkg_1eab::pool::CollectFeeEvent)
        }
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::openpositionevent" => {
            decode_arm!(bcs, pkg_1eab::pool::OpenPositionEvent)
        }
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::closepositionevent" => {
            decode_arm!(bcs, pkg_1eab::pool::ClosePositionEvent)
        }
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::addliquidityevent" => {
            decode_arm!(bcs, pkg_1eab::pool::AddLiquidityEvent)
        }
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::removeliquidityevent" => {
            decode_arm!(bcs, pkg_1eab::pool::RemoveLiquidityEvent)
        }
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::collectprotocolfeeevent" => {
            decode_arm!(bcs, pkg_1eab::pool::CollectProtocolFeeEvent)
        }
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::partner::receivereffeeevent" => {
            decode_arm!(bcs, pkg_1eab::partner::ReceiveRefFeeEvent)
        }
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::partner::claimreffeeevent" => {
            decode_arm!(bcs, pkg_1eab::partner::ClaimRefFeeEvent)
        }
        "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::factory::createpoolevent" => {
            decode_arm!(bcs, pkg_1eab::factory::CreatePoolEvent)
        }

        "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::pool::swapevent" => {
            decode_arm!(bcs, pkg_91bf::pool::SwapEvent)
        }
        "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::pool::burnevent" => {
            decode_arm!(bcs, pkg_91bf::pool::BurnEvent)
        }
        "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::pool::mintevent" => {
            decode_arm!(bcs, pkg_91bf::pool::MintEvent)
        }
        "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::pool::collectprotocolfeeevent" => {
            decode_arm!(bcs, pkg_91bf::pool::CollectProtocolFeeEvent)
        }
        "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::position_manager::increaseliquidityevent" => {
            decode_arm!(bcs, pkg_91bf::position_manager::IncreaseLiquidityEvent)
        }
        "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::position_manager::decreaseliquidityevent" => {
            decode_arm!(bcs, pkg_91bf::position_manager::DecreaseLiquidityEvent)
        }
        "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::pool_factory::poolcreatedevent" => {
            decode_arm!(bcs, pkg_91bf::pool_factory::PoolCreatedEvent)
        }

        other => bail!("no static binding for event type: {other}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CETUS_SWAP_BCS: &str = "00440e5e3b13b8220c5c338bb5a4291cab5c58064eaf3654c77f3e9aed5147689c000000000000000000000000000000000000000000000000000000000000000000f9cc2f3c0000006d76b10a00000000000000000000000040cbd01e000000003177cdee010000009ffe799f2475000097577ecc93beb5ea2500000000000000962cd524d50bebec25000000000000000100000000000000";

    const TURBOS_SWAP_BCS: &str = "5eb2dfcdd1b15d2021328258f6d5ec081e9a0cdcfa9e13a0eaeb9b5f7505ca78788a9ada3f7ee01cb93352878d84e68dce92a3ebcdd418f7dde34ccba262db6bf3ab2d9309000000aa4eb301000000008ae2192ed90400000000000000000000f9e3fefffde3feff9fb818799e89c006000000000000000062c034020000000049815a07000000000101";

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn decode_cetus_swap_event_from_db_bcs() {
        let event_type =
            "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::swapevent";
        let parsed = decode_parsed_json(event_type, &hex_to_bytes(CETUS_SWAP_BCS)).unwrap();
        assert_eq!(parsed["atob"], false);
        assert_eq!(parsed["amount_in"], "258500000000");
        assert_eq!(parsed["steps"], "1");
    }

    #[test]
    fn decode_turbos_swap_event_from_db_bcs() {
        let event_type =
            "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::pool::swapevent";
        let parsed = decode_parsed_json(event_type, &hex_to_bytes(TURBOS_SWAP_BCS)).unwrap();
        assert_eq!(parsed["a_to_b"], true);
        assert_eq!(parsed["amount_a"], "41123949555");
        assert_eq!(parsed["fee_amount"], "123371849");
    }
}
