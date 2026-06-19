//! Frozen mainnet DEX registry — single source for package IDs, event types, and linkage deps.

/// Shared packages referenced via `move_contract!` linkage.
pub mod deps {
    pub const SUI: &str = "0x2";
    pub const INTEGER_MATE: &str =
        "0xdfaadf86be9af246900d1e3f3b996cf549e7948e662a9977bdd7646d8fa3a778";
    pub const INTEGER_MATE_ALT: &str =
        "0x714a63a0dba6da4f017b42d5d0fb78867f18bcde904868e51d951a5a6f5b7f57";
    pub const MAGMA_INTEGER_MATE: &str =
        "0x659c0e9c4c8a416f040fa758d4fc4073a5fdd1fed97edadcd5cba5180fb36246";
}

/// Invoke from `lib.rs` to expand config modules and `move_contract!` blocks.
///
/// Each protocol is declared once — package ID, event types, and codegen options.
#[macro_export]
macro_rules! dex_protocol_registry {
    ($emit:ident) => {
        $emit!(
            cetus,
            "pkg_1eab",
            "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb",
            slug = "cetus",
            swap = "pool::SwapEvent",
            pool_create = "factory::CreatePoolEvent",
            pool_id_field = "pool",
            move_contract = {
                network = "mainnet",
                event_modules = "pool,partner,factory,rewarder,config",
                linkage = "0x2=sui,0xdfaadf86be9af246900d1e3f3b996cf549e7948e662a9977bdd7646d8fa3a778=integer_mate,0x714a63a0dba6da4f017b42d5d0fb78867f18bcde904868e51d951a5a6f5b7f57=integer_mate"
            }
        );

        $emit!(
            turbos,
            "pkg_91bf",
            "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1",
            slug = "turbos",
            swap = "pool::SwapEvent",
            pool_create = "pool_factory::PoolCreatedEvent",
            pool_id_field = "pool",
            move_contract = {
                network = "mainnet",
                event_modules = "pool,position_manager,pool_factory,position_nft",
                support_modules = "i32,i64,i128,full_math_u128,full_math_u64,math_u128,math_u64,math_u256,math_sqrt_price,math_tick,math_liquidity,math_bit,math_swap",
                linkage = "0x2=sui"
            }
        );

        $emit!(
            bluefin,
            "pkg_3492",
            "0x3492c874c1e3b3e2984e8c41b589e642d4d0a5d6459e5a9cfc2d52fd7c89c267",
            slug = "bluefin",
            swap = "events::AssetSwap",
            pool_create = "events::PoolCreated",
            pool_id_field = "pool_id",
            move_contract = {
                network = "mainnet",
                emit_mode = "module_structs",
                modules = "events",
                linkage = "0x2=sui,0xdfaadf86be9af246900d1e3f3b996cf549e7948e662a9977bdd7646d8fa3a778=integer_mate,0x714a63a0dba6da4f017b42d5d0fb78867f18bcde904868e51d951a5a6f5b7f57=integer_mate"
            }
        );

        $emit!(
            mmt,
            "pkg_7028",
            "0x70285592c97965e811e0c6f98dccc3a9c2b4ad854b3594faab9597ada267b860",
            slug = "mmt",
            swap = "trade::SwapEvent",
            pool_create = "create_pool::PoolCreatedEvent",
            pool_id_field = "pool_id",
            move_contract = {
                network = "mainnet",
                event_modules = "trade,create_pool",
                support_modules = "i32,i64,i128,full_math_u128,full_math_u64,math_u128,math_u64,math_u256",
                linkage = "0x2=sui"
            }
        );

        $emit!(
            magma,
            "pkg_4a35",
            "0x4a35d3dfef55ed3631b7158544c6322a23bc434fe4fca1234cb680ce0505f82d",
            slug = "magma",
            swap = "pool::SwapEvent",
            pool_create = "factory::CreatePoolEvent",
            pool_id_field = "pool",
            move_contract = {
                network = "mainnet",
                event_modules = "pool,factory",
                linkage = "0x2=sui,0x659c0e9c4c8a416f040fa758d4fc4073a5fdd1fed97edadcd5cba5180fb36246=magma_integer_mate,0xdfaadf86be9af246900d1e3f3b996cf549e7948e662a9977bdd7646d8fa3a778=integer_mate,0x714a63a0dba6da4f017b42d5d0fb78867f18bcde904868e51d951a5a6f5b7f57=integer_mate"
            }
        );
    };
}

/// Expand protocol config modules (invoked from `config.rs`).
macro_rules! __emit_config_module {
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
        pub mod $mod_name {
            pub const SLUG: &str = $slug;
            pub const TYPE_PACKAGE: &str = $type_package;
            pub const SWAP_EVENT: &str = concat!($type_package, "::", $swap_path);
            pub const POOL_CREATE_EVENT: &str = concat!($type_package, "::", $pool_path);
            pub const POOL_ID_FIELD: &str = $pool_id_field;
        }
    };
}

dex_protocol_registry!(__emit_config_module);

/// FlowX CLMM — manual BCS decode (pool module structs pull in `sui::table` deps).
pub mod flowx {
    pub const SLUG: &str = "flowx";
    pub const TYPE_PACKAGE: &str =
        "0x25929e7f29e0a30eb4e692952ba1b5b65a3a4d65ab5f2a32e1ba3edcb587f26d";
    pub const SWAP_EVENT: &str = "0x25929e7f29e0a30eb4e692952ba1b5b65a3a4d65ab5f2a32e1ba3edcb587f26d::pool::Swap";
    pub const POOL_CREATE_EVENT: &str = "0x25929e7f29e0a30eb4e692952ba1b5b65a3a4d65ab5f2a32e1ba3edcb587f26d::pool_manager::PoolCreated";
    pub const POOL_ID_FIELD: &str = "pool_id";
}

pub const SWAP_EVENT_TYPES: [&str; 6] = [
    cetus::SWAP_EVENT,
    turbos::SWAP_EVENT,
    bluefin::SWAP_EVENT,
    mmt::SWAP_EVENT,
    flowx::SWAP_EVENT,
    magma::SWAP_EVENT,
];

pub const POOL_CREATE_EVENT_TYPES: [&str; 6] = [
    cetus::POOL_CREATE_EVENT,
    turbos::POOL_CREATE_EVENT,
    bluefin::POOL_CREATE_EVENT,
    mmt::POOL_CREATE_EVENT,
    flowx::POOL_CREATE_EVENT,
    magma::POOL_CREATE_EVENT,
];

// Backward-compatible re-exports (prefer `config::cetus::*` in new code).
pub use bluefin::{
    POOL_CREATE_EVENT as BLUEFIN_POOL_CREATE_EVENT, SWAP_EVENT as BLUEFIN_SWAP_EVENT,
    TYPE_PACKAGE as BLUEFIN_TYPE_PACKAGE,
};
pub use cetus::{
    POOL_CREATE_EVENT as CETUS_POOL_CREATE_EVENT, SWAP_EVENT as CETUS_SWAP_EVENT,
    TYPE_PACKAGE as CETUS_TYPE_PACKAGE,
};
pub use mmt::{
    POOL_CREATE_EVENT as MMT_POOL_CREATE_EVENT, SWAP_EVENT as MMT_SWAP_EVENT,
    TYPE_PACKAGE as MMT_TYPE_PACKAGE,
};
pub use flowx::{
    POOL_CREATE_EVENT as FLOWX_POOL_CREATE_EVENT, SWAP_EVENT as FLOWX_SWAP_EVENT,
    TYPE_PACKAGE as FLOWX_TYPE_PACKAGE,
};
pub use magma::{
    POOL_CREATE_EVENT as MAGMA_POOL_CREATE_EVENT, SWAP_EVENT as MAGMA_SWAP_EVENT,
    TYPE_PACKAGE as MAGMA_TYPE_PACKAGE,
};
pub use turbos::{
    POOL_CREATE_EVENT as TURBOS_POOL_CREATE_EVENT, SWAP_EVENT as TURBOS_SWAP_EVENT,
    TYPE_PACKAGE as TURBOS_TYPE_PACKAGE,
};
