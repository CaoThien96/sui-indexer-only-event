//! Static event bindings generated via `move_contract!` at compile time.
//!
//! Requires network on first `cargo build` (GraphQL package fetch).
//! Registry: [`config`] — package IDs and event types for all supported DEX protocols.

mod parsed_json;
pub mod coin_metadata;
pub mod config;
mod flowx_manual;
pub mod pool_id;
pub mod protocol;

use move_binding_derive::move_contract;

// Shared framework + dependency packages (not DEX-specific).
move_contract! {
    alias = "sui",
    package = "0x2",
    network = "mainnet",
    register_only = true
}

move_contract! {
    alias = "integer_mate",
    package = "0xdfaadf86be9af246900d1e3f3b996cf549e7948e662a9977bdd7646d8fa3a778",
    network = "mainnet",
    emit_mode = "module_structs",
    modules = "i32,i64,i128,full_math_u128,full_math_u64,math_u128,math_u256,math_u64"
}

move_contract! {
    alias = "magma_integer_mate",
    package = "0x659c0e9c4c8a416f040fa758d4fc4073a5fdd1fed97edadcd5cba5180fb36246",
    network = "mainnet",
    emit_mode = "module_structs",
    modules = "i32,i64,i128,full_math_u128,full_math_u64,math_u128,math_u256,math_u64"
}

/// DEX packages — package ID declared once in [`config::dex_protocol_registry`].
macro_rules! __emit_move_contract {
    (
        $mod_name:ident,
        $move_alias:literal,
        $type_package:literal,
        slug = $slug:literal,
        swap = $swap_path:literal,
        pool_create = $pool_path:literal,
        pool_id_field = $pool_id_field:literal,
        move_contract = { $($move_opts:tt)* }
    ) => {
        move_contract! {
            alias = $move_alias,
            package = $type_package,
            $($move_opts)*
        }
    };
}

crate::dex_protocol_registry!(__emit_move_contract);

use anyhow::{Result, bail};
use serde_json::Value;

macro_rules! decode_arm {
    ($bcs:expr, $ty:ty) => {{
        let decoded: $ty = bcs::from_bytes($bcs)?;
        Ok(parsed_json::normalize(serde_json::to_value(decoded)?))
    }};
}

macro_rules! decode_pkg_events {
    ($event_type:expr, $bcs:expr, $alias:ident, $type_package:expr, {
        $($module:ident :: $event:ident),* $(,)?
    }) => {
        $(
            {
                let canonical = format!(
                    "{}::{}::{}",
                    $type_package,
                    stringify!($module),
                    stringify!($event)
                );
                if $event_type.eq_ignore_ascii_case(&canonical) {
                    return decode_arm!($bcs, $alias::$module::$event);
                }
            }
        )+
    };
}

/// Decode event BCS bytes into JSON matching fullnode `parsedJson` shape.
pub fn decode_parsed_json(event_type: &str, bcs: &[u8]) -> Result<Value> {
    decode_pkg_events!(event_type, bcs, pkg_1eab, config::cetus::TYPE_PACKAGE, {
        config::AddFeeTierEvent,
        config::AddRoleEvent,
        config::DeleteFeeTierEvent,
        config::InitConfigEvent,
        config::RemoveMemberEvent,
        config::RemoveRoleEvent,
        config::SetRolesEvent,
        config::UpdateFeeRateEvent,
        config::UpdateFeeTierEvent,
        factory::CreatePoolEvent,
        factory::InitFactoryEvent,
        partner::ClaimRefFeeEvent,
        partner::CreatePartnerEvent,
        partner::InitPartnerEvent,
        partner::ReceiveRefFeeEvent,
        partner::UpdateRefFeeRateEvent,
        partner::UpdateTimeRangeEvent,
        pool::AddLiquidityEvent,
        pool::AddRewarderEvent,
        pool::ClosePositionEvent,
        pool::CollectFeeEvent,
        pool::CollectProtocolFeeEvent,
        pool::CollectRewardEvent,
        pool::OpenPositionEvent,
        pool::RemoveLiquidityEvent,
        pool::SwapEvent,
        pool::UpdateEmissionEvent,
        pool::UpdateFeeRateEvent,
        rewarder::DepositEvent,
        rewarder::EmergentWithdrawEvent,
        rewarder::RewarderInitEvent,
    });

    decode_pkg_events!(event_type, bcs, pkg_91bf, config::turbos::TYPE_PACKAGE, {
        pool::AddRewardEvent,
        pool::BurnEvent,
        pool::CollectEvent,
        pool::CollectProtocolFeeEvent,
        pool::CollectRewardEvent,
        pool::InitRewardEvent,
        pool::MintEvent,
        pool::RemoveRewardEvent,
        pool::SwapEvent,
        pool::TogglePoolStatusEvent,
        pool::UpdatePoolFeeProtocolEvent,
        pool::UpdateRewardEmissionsEvent,
        pool::UpdateRewardManagerEvent,
        pool::UpgradeEvent,
        pool_factory::FeeAmountEnabledEvent,
        pool_factory::PoolCreatedEvent,
        pool_factory::SetFeeProtocolEvent,
        position_manager::CollectEvent,
        position_manager::CollectRewardEvent,
        position_manager::DecreaseLiquidityEvent,
        position_manager::IncreaseLiquidityEvent,
        position_nft::MintNFTEvent,
    });

    decode_pkg_events!(event_type, bcs, pkg_3492, config::bluefin::TYPE_PACKAGE, {
        events::AdminCapTransferred,
        events::AssetSwap,
        events::FlashSwap,
        events::PoolCreated,
        events::PositionClosed,
        events::PositionOpened,
        events::ProtocolFeeCapTransferred,
    });

    decode_pkg_events!(event_type, bcs, pkg_7028, config::mmt::TYPE_PACKAGE, {
        trade::SwapEvent,
        trade::RepayFlashLoanEvent,
        trade::RepayFlashSwapEvent,
        create_pool::PoolCreatedEvent,
    });

    if event_type.eq_ignore_ascii_case(config::flowx::SWAP_EVENT) {
        return flowx_manual::decode_swap(bcs);
    }
    if event_type.eq_ignore_ascii_case(config::flowx::POOL_CREATE_EVENT) {
        return flowx_manual::decode_pool_created(bcs);
    }

    decode_pkg_events!(event_type, bcs, pkg_4a35, config::magma::TYPE_PACKAGE, {
        pool::SwapEvent,
        factory::CreatePoolEvent,
    });

    bail!("no static binding for event type: {event_type}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::{bluefin, cetus, flowx, magma, mmt, turbos};

    const CETUS_SWAP_BCS: &str = "00440e5e3b13b8220c5c338bb5a4291cab5c58064eaf3654c77f3e9aed5147689c000000000000000000000000000000000000000000000000000000000000000000f9cc2f3c0000006d76b10a00000000000000000000000040cbd01e000000003177cdee010000009ffe799f2475000097577ecc93beb5ea2500000000000000962cd524d50bebec25000000000000000100000000000000";

    const TURBOS_SWAP_BCS: &str = "5eb2dfcdd1b15d2021328258f6d5ec081e9a0cdcfa9e13a0eaeb9b5f7505ca78788a9ada3f7ee01cb93352878d84e68dce92a3ebcdd418f7dde34ccba262db6bf3ab2d9309000000aa4eb301000000008ae2192ed90400000000000000000000f9e3fefffde3feff9fb818799e89c006000000000000000062c034020000000049815a07000000000101";

    const BLUEFIN_SWAP_BCS: &str = "b9ce48f5cf75d7f5744a2cff362f59f8f086b021e31cc9e766755d7c85694dc300412d305205000000b1b85c010000000002eeafef8e000000fa078b15ecad0100ba7c0300000000004505d2d6cb7cc40400000000000000004505d2d6cb7cc40400000000000000008b48f15f4b21609f1f00000000000000e4c90d186922609f1f00000000000000d80d010000069c0300000000000000000000000000";

    const MMT_SWAP_BCS: &str = "99e9a3a2d688324ba7d9b91c5117448247f9ab520f31eb662c5a12b5762d7d9e392745193a7e472a8fd354d9fc38f26f023547566a4cda4864ee29a2c21f6fc800118c000000000000880d3e4007000000d07f5ce07c4edcf6a20300000000000024ed5ee5fe4f5109a30300000000000039e4a75e6400000000000000000000001c1602002051f802000000004814be000000000065b8ea00000000001cda5e8bdc4d0000";

    const MMT_POOL_CREATE_BCS: &str = "506ecadb1d93eb2f9e7e1d32e5146b60d734f6d02bd763e8ec705ba00eaded3057e2a855ab75bffe6095e49b27617666a9840ff6ef09e27e29fe3566c315e8814a303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030323a3a7375693a3a5355494c373237343538323132636130626530353664396363633064343239383162646634313130393737396662643963376661643536633434353665306333643063363a3a626565673a3a42454547c40900000000000032000000";

    const FLOWX_SWAP_BCS: &str = "f14f2d50f560fdfa80c87b8c135845e23bc4147595b417176cd3b2df67223fdd8c001681aaf29662b1e58d4bd343b14a3f817219696f77c80817042fe31eab4d0093a2ee1300000000b600d60000000000b078aa0cb5bd5b3400000000000000005324cff2b5667d3400000000000000009e7e6b5b0600000000000000000000003384ffff7b05000000000000";

    const FLOWX_POOL_CREATE_BCS: &str = "ca0aab96d2b0f7a1d3a26c48e8d3c61c9037585a1e20782cc1a722e7966db041e41cc018b503930d83a912107119f5ce85c377e770ad4e5660be92e998d01c974a346339383166336666373836636462396535313464613839376162386139353336343764616532616365393637396538333538656563316533653838373161633a3a646d633a3a444d434c623435666366636332636330376365303730326363326432323936323165303436633930366566313464396232356538653464323566366538373633666566373a3a73656e643a3a53454e441027000000000000c8000000";

    const MAGMA_SWAP_BCS: &str = "01e672f3fe0c6c0bee46db41d2fd00916596a2d2384e001d4e1d4a89f98799d94a000000000000000000000000000000000000000000000000000000000000000000c2eb0b0000000008c8d076730000000000000000000000a00f0000000000000000000000000000204e000000000000d1e39cd526010000f82885d963510500288948455c6173cb3100000000000000288948455c6173cb31000000000000000100000000000000";

    const MAGMA_POOL_CREATE_BCS: &str = "470898ac48eead3db575d66efac344cc652844e8ebdbf3f7c9cc6a9423486e8d4e643162373239383265343033343864303639626231666637303165363334633131376262356637343166343464666639316534373264336230313436316535353a3a73747375693a3a53545355494a303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030323a3a7375693a3a5355493c000000";

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn decode_cetus_swap_event_from_db_bcs() {
        let parsed = decode_parsed_json(cetus::SWAP_EVENT, &hex_to_bytes(CETUS_SWAP_BCS)).unwrap();
        assert_eq!(parsed["atob"], false);
        assert_eq!(parsed["amount_in"], "258500000000");
        assert_eq!(parsed["steps"], "1");
    }

    #[test]
    fn decode_accepts_legacy_lowercase_event_type() {
        let event_type = format!("{}::pool::swapevent", cetus::TYPE_PACKAGE);
        let parsed = decode_parsed_json(&event_type, &hex_to_bytes(CETUS_SWAP_BCS)).unwrap();
        assert_eq!(parsed["amount_in"], "258500000000");
    }

    #[test]
    fn decode_turbos_swap_event_from_db_bcs() {
        let parsed =
            decode_parsed_json(turbos::SWAP_EVENT, &hex_to_bytes(TURBOS_SWAP_BCS)).unwrap();
        assert_eq!(parsed["a_to_b"], true);
        assert_eq!(parsed["amount_a"], "41123949555");
        assert_eq!(parsed["fee_amount"], "123371849");
    }

    #[test]
    fn decode_bluefin_asset_swap_from_mainnet_bcs() {
        let parsed =
            decode_parsed_json(bluefin::SWAP_EVENT, &hex_to_bytes(BLUEFIN_SWAP_BCS)).unwrap();
        assert_eq!(parsed["a2b"], false);
        assert_eq!(parsed["amount_in"], "22853725505");
        assert_eq!(
            parsed["pool_id"],
            "0xb9ce48f5cf75d7f5744a2cff362f59f8f086b021e31cc9e766755d7c85694dc3"
        );
    }

    #[test]
    fn decode_mmt_swap_event_from_mainnet_bcs() {
        let parsed = decode_parsed_json(mmt::SWAP_EVENT, &hex_to_bytes(MMT_SWAP_BCS)).unwrap();
        assert_eq!(parsed["x_for_y"], false);
        assert_eq!(parsed["amount_x"], "35857");
        assert_eq!(parsed["amount_y"], "31142579592");
    }

    #[test]
    fn decode_mmt_pool_created_from_mainnet_bcs() {
        let parsed =
            decode_parsed_json(mmt::POOL_CREATE_EVENT, &hex_to_bytes(MMT_POOL_CREATE_BCS)).unwrap();
        assert_eq!(
            parsed["pool_id"],
            "0x57e2a855ab75bffe6095e49b27617666a9840ff6ef09e27e29fe3566c315e881"
        );
        assert_eq!(parsed["tick_spacing"], "50");
    }

    #[test]
    fn decode_flowx_swap_from_mainnet_bcs() {
        let parsed =
            decode_parsed_json(flowx::SWAP_EVENT, &hex_to_bytes(FLOWX_SWAP_BCS)).unwrap();
        assert_eq!(parsed["x_for_y"], false);
        assert_eq!(
            parsed["pool_id"],
            "0x8c001681aaf29662b1e58d4bd343b14a3f817219696f77c80817042fe31eab4d"
        );
        assert_eq!(parsed["amount_x"], "334406291");
        assert_eq!(parsed["sqrt_price_before"], "3772817698152151216");
        assert_eq!(parsed["liquidity"], "27303575198");
    }

    #[test]
    fn decode_flowx_pool_created_from_mainnet_bcs() {
        let parsed = decode_parsed_json(
            flowx::POOL_CREATE_EVENT,
            &hex_to_bytes(FLOWX_POOL_CREATE_BCS),
        )
        .unwrap();
        assert_eq!(
            parsed["pool_id"],
            "0xe41cc018b503930d83a912107119f5ce85c377e770ad4e5660be92e998d01c97"
        );
        assert_eq!(parsed["tick_spacing"], "200");
    }

    #[test]
    fn decode_magma_swap_from_mainnet_bcs() {
        let parsed =
            decode_parsed_json(magma::SWAP_EVENT, &hex_to_bytes(MAGMA_SWAP_BCS)).unwrap();
        assert_eq!(parsed["atob"], true);
        assert_eq!(parsed["amount_in"], "200000000");
        assert_eq!(
            parsed["pool"],
            "0xe672f3fe0c6c0bee46db41d2fd00916596a2d2384e001d4e1d4a89f98799d94a"
        );
    }

    #[test]
    fn decode_magma_pool_created_from_mainnet_bcs() {
        let parsed = decode_parsed_json(
            magma::POOL_CREATE_EVENT,
            &hex_to_bytes(MAGMA_POOL_CREATE_BCS),
        )
        .unwrap();
        assert_eq!(
            parsed["pool_id"],
            "0x470898ac48eead3db575d66efac344cc652844e8ebdbf3f7c9cc6a9423486e8d"
        );
        assert_eq!(parsed["tick_spacing"], "60");
    }

    #[test]
    fn pool_id_extraction_per_protocol() {
        let cetus_parsed =
            decode_parsed_json(cetus::SWAP_EVENT, &hex_to_bytes(CETUS_SWAP_BCS)).unwrap();
        let pool = pool_id::extract_pool_id(protocol::Protocol::Cetus, &cetus_parsed).unwrap();
        assert!(pool.starts_with("0x"));

        let bluefin_parsed =
            decode_parsed_json(bluefin::SWAP_EVENT, &hex_to_bytes(BLUEFIN_SWAP_BCS)).unwrap();
        let pool =
            pool_id::extract_pool_id(protocol::Protocol::Bluefin, &bluefin_parsed).unwrap();
        assert_eq!(pool, "0xb9ce48f5cf75d7f5744a2cff362f59f8f086b021e31cc9e766755d7c85694dc3");
    }
}
