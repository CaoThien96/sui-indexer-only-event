pub const CETUS_PKG: &str =
    "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb";
pub const TURBOS_PKG: &str =
    "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1";
pub const SUI_TYPE: &str = "0x2::sui::SUI";

pub fn cetus_swap() -> String {
    format!("{CETUS_PKG}::pool::SwapEvent")
}

pub fn turbos_swap() -> String {
    format!("{TURBOS_PKG}::pool::SwapEvent")
}

pub fn cetus_create_pool() -> String {
    format!("{CETUS_PKG}::factory::CreatePoolEvent")
}

pub fn turbos_create_pool() -> String {
    format!("{TURBOS_PKG}::pool_factory::PoolCreatedEvent")
}

pub fn cetus_remove_liquidity() -> String {
    format!("{CETUS_PKG}::pool::RemoveLiquidityEvent")
}

pub fn turbos_remove_liquidity() -> String {
    format!("{TURBOS_PKG}::pool::BurnEvent")
}
