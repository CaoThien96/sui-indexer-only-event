//! Mainnet Turbos CLMM contract objects from turbos-clmm-sdk `contract.getConfig()`.

pub const PACKAGE_ID: &str =
    "0xa5a0c25c79e428eba04fb98b3fb2a34db45ab26d4c8faf0d7e39d66a63891e64";
pub const PACKAGE_ID_ORIGINAL: &str =
    "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1";
pub const POSITIONS: &str =
    "0xf5762ae5ae19a2016bb233c72d9a4b2cba5a302237a82724af66292ae43ae52d";
pub const VERSIONED: &str =
    "0xf1cf0e81048df168ebeb1b8030fad24b3e0b53ae827c25053fff0779c1445b6f";

/// Default mint deadline offset (SDK `ONE_MINUTE`).
pub const MINT_DEADLINE_MS: u64 = 60_000;
