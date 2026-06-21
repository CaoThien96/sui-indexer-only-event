use anyhow::{Context, Result, bail};
use event_bindings::{pool_id, protocol::Protocol};
use serde_json::Value;

/// Protocol-native swap fields extracted from `parsed_json`.
#[derive(Debug, Clone)]
pub struct ExtractedSwapFields {
    pub pool_id: String,
    pub a_to_b: bool,
    pub amount_in_raw: String,
    pub amount_out_raw: String,
    pub fee_amount_raw: Option<String>,
    pub sqrt_price_before: Option<String>,
    pub sqrt_price_after: String,
    pub vault_a_raw: Option<String>,
    pub vault_b_raw: Option<String>,
}

pub fn extract_swap_fields(protocol: Protocol, parsed: &Value) -> Result<ExtractedSwapFields> {
    let pool_id = pool_id::extract_pool_id(protocol, parsed)?;

    match protocol {
        Protocol::Cetus | Protocol::Magma => {
            let a_to_b = parsed
                .get("atob")
                .and_then(Value::as_bool)
                .context("missing atob")?;
            Ok(ExtractedSwapFields {
                pool_id,
                a_to_b,
                amount_in_raw: str_field(parsed, "amount_in")?,
                amount_out_raw: str_field(parsed, "amount_out")?,
                fee_amount_raw: optional_str_field(parsed, "fee_amount"),
                sqrt_price_before: optional_str_field(parsed, "before_sqrt_price"),
                sqrt_price_after: str_field(parsed, "after_sqrt_price")?,
                vault_a_raw: optional_str_field(parsed, "vault_a_amount"),
                vault_b_raw: optional_str_field(parsed, "vault_b_amount"),
            })
        }
        Protocol::Turbos => {
            let a_to_b = parsed
                .get("a_to_b")
                .and_then(Value::as_bool)
                .context("missing a_to_b")?;
            let (amount_in, amount_out) = if a_to_b {
                (str_field(parsed, "amount_a")?, str_field(parsed, "amount_b")?)
            } else {
                (str_field(parsed, "amount_b")?, str_field(parsed, "amount_a")?)
            };
            Ok(ExtractedSwapFields {
                pool_id,
                a_to_b,
                amount_in_raw: amount_in,
                amount_out_raw: amount_out,
                fee_amount_raw: optional_str_field(parsed, "fee_amount"),
                sqrt_price_before: None,
                sqrt_price_after: str_field(parsed, "sqrt_price")?,
                vault_a_raw: None,
                vault_b_raw: None,
            })
        }
        Protocol::Bluefin => {
            let a_to_b = parsed
                .get("a2b")
                .and_then(Value::as_bool)
                .context("missing a2b")?;
            Ok(ExtractedSwapFields {
                pool_id,
                a_to_b,
                amount_in_raw: str_field(parsed, "amount_in")?,
                amount_out_raw: str_field(parsed, "amount_out")?,
                fee_amount_raw: optional_str_field(parsed, "fee_amount"),
                sqrt_price_before: optional_str_field(parsed, "before_sqrt_price"),
                sqrt_price_after: str_field(parsed, "after_sqrt_price")?,
                vault_a_raw: optional_str_field(parsed, "vault_a_amount"),
                vault_b_raw: optional_str_field(parsed, "vault_b_amount"),
            })
        }
        Protocol::Mmt | Protocol::Flowx => {
            let x_for_y = parsed
                .get("x_for_y")
                .and_then(Value::as_bool)
                .context("missing x_for_y")?;
            let a_to_b = x_for_y;
            let (amount_in, amount_out) = if x_for_y {
                (str_field(parsed, "amount_x")?, str_field(parsed, "amount_y")?)
            } else {
                (str_field(parsed, "amount_y")?, str_field(parsed, "amount_x")?)
            };
            Ok(ExtractedSwapFields {
                pool_id,
                a_to_b,
                amount_in_raw: amount_in,
                amount_out_raw: amount_out,
                fee_amount_raw: optional_str_field(parsed, "fee_amount"),
                sqrt_price_before: optional_str_field(parsed, "sqrt_price_before"),
                sqrt_price_after: str_field(parsed, "sqrt_price_after")?,
                vault_a_raw: None,
                vault_b_raw: None,
            })
        }
    }
}

fn str_field(parsed: &Value, key: &str) -> Result<String> {
    parsed
        .get(key)
        .map(json_number_to_string)
        .transpose()?
        .context(format!("missing field `{key}`"))
}

fn optional_str_field(parsed: &Value, key: &str) -> Option<String> {
    parsed.get(key).and_then(|v| json_number_to_string(v).ok())
}

fn json_number_to_string(value: &Value) -> Result<String> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Number(n) => Ok(n.to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        _ => bail!("expected string or number, got {value}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use event_bindings::{
        config::{bluefin, cetus, flowx, magma, mmt, turbos},
        decode_parsed_json,
    };

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    }

    const CETUS_SWAP_BCS: &str = "00440e5e3b13b8220c5c338bb5a4291cab5c58064eaf3654c77f3e9aed5147689c000000000000000000000000000000000000000000000000000000000000000000f9cc2f3c0000006d76b10a00000000000000000000000040cbd01e000000003177cdee010000009ffe799f2475000097577ecc93beb5ea2500000000000000962cd524d50bebec25000000000000000100000000000000";
    const TURBOS_SWAP_BCS: &str = "5eb2dfcdd1b15d2021328258f6d5ec081e9a0cdcfa9e13a0eaeb9b5f7505ca78788a9ada3f7ee01cb93352878d84e68dce92a3ebcdd418f7dde34ccba262db6bf3ab2d9309000000aa4eb301000000008ae2192ed90400000000000000000000f9e3fefffde3feff9fb818799e89c006000000000000000062c034020000000049815a07000000000101";
    const BLUEFIN_SWAP_BCS: &str = "b9ce48f5cf75d7f5744a2cff362f59f8f086b021e31cc9e766755d7c85694dc300412d305205000000b1b85c010000000002eeafef8e000000fa078b15ecad0100ba7c0300000000004505d2d6cb7cc40400000000000000004505d2d6cb7cc40400000000000000008b48f15f4b21609f1f00000000000000e4c90d186922609f1f00000000000000d80d010000069c0300000000000000000000000000";
    const MMT_SWAP_BCS: &str = "99e9a3a2d688324ba7d9b91c5117448247f9ab520f31eb662c5a12b5762d7d9e392745193a7e472a8fd354d9fc38f26f023547566a4cda4864ee29a2c21f6fc800118c000000000000880d3e4007000000d07f5ce07c4edcf6a20300000000000024ed5ee5fe4f5109a30300000000000039e4a75e6400000000000000000000001c1602002051f802000000004814be000000000065b8ea00000000001cda5e8bdc4d0000";
    const FLOWX_SWAP_BCS: &str = "f14f2d50f560fdfa80c87b8c135845e23bc4147595b417176cd3b2df67223fdd8c001681aaf29662b1e58d4bd343b14a3f817219696f77c80817042fe31eab4d0093a2ee1300000000b600d60000000000b078aa0cb5bd5b3400000000000000005324cff2b5667d3400000000000000009e7e6b5b0600000000000000000000003384ffff7b05000000000000";
    const MAGMA_SWAP_BCS: &str = "01e672f3fe0c6c0bee46db41d2fd00916596a2d2384e001d4e1d4a89f98799d94a000000000000000000000000000000000000000000000000000000000000000000c2eb0b0000000008c8d076730000000000000000000000a00f0000000000000000000000000000204e000000000000d1e39cd526010000f82885d963510500288948455c6173cb3100000000000000288948455c6173cb31000000000000000100000000000000";

    #[test]
    fn extracts_cetus_swap_fields() {
        let parsed = decode_parsed_json(cetus::SWAP_EVENT, &hex_to_bytes(CETUS_SWAP_BCS)).unwrap();
        let fields = extract_swap_fields(Protocol::Cetus, &parsed).unwrap();
        assert_eq!(fields.amount_in_raw, "258500000000");
        assert!(!fields.a_to_b);
        assert!(fields.pool_id.starts_with("0x"));
        assert!(!fields.sqrt_price_after.is_empty());
    }

    #[test]
    fn extracts_turbos_swap_fields() {
        let parsed =
            decode_parsed_json(turbos::SWAP_EVENT, &hex_to_bytes(TURBOS_SWAP_BCS)).unwrap();
        let fields = extract_swap_fields(Protocol::Turbos, &parsed).unwrap();
        assert!(fields.a_to_b);
        assert_eq!(fields.amount_in_raw, "41123949555");
        assert_eq!(fields.amount_out_raw, "28528298");
        assert_eq!(fields.fee_amount_raw.as_deref(), Some("123371849"));
        assert_eq!(fields.sqrt_price_after, "486540073485514911");
    }

    #[test]
    fn extracts_bluefin_swap_fields() {
        let parsed =
            decode_parsed_json(bluefin::SWAP_EVENT, &hex_to_bytes(BLUEFIN_SWAP_BCS)).unwrap();
        let fields = extract_swap_fields(Protocol::Bluefin, &parsed).unwrap();
        assert!(!fields.a_to_b);
        assert_eq!(fields.amount_in_raw, "22853725505");
        assert_eq!(
            fields.pool_id,
            "0xb9ce48f5cf75d7f5744a2cff362f59f8f086b021e31cc9e766755d7c85694dc3"
        );
    }

    #[test]
    fn extracts_mmt_swap_fields() {
        let parsed = decode_parsed_json(mmt::SWAP_EVENT, &hex_to_bytes(MMT_SWAP_BCS)).unwrap();
        let fields = extract_swap_fields(Protocol::Mmt, &parsed).unwrap();
        assert!(!fields.a_to_b);
        assert_eq!(fields.amount_in_raw, "31142579592");
        assert_eq!(fields.amount_out_raw, "35857");
    }

    #[test]
    fn extracts_flowx_swap_fields() {
        let parsed =
            decode_parsed_json(flowx::SWAP_EVENT, &hex_to_bytes(FLOWX_SWAP_BCS)).unwrap();
        let fields = extract_swap_fields(Protocol::Flowx, &parsed).unwrap();
        assert!(!fields.a_to_b);
        assert_eq!(fields.amount_in_raw, "14024886");
        assert_eq!(fields.amount_out_raw, "334406291");
        assert_eq!(
            fields.pool_id,
            "0x8c001681aaf29662b1e58d4bd343b14a3f817219696f77c80817042fe31eab4d"
        );
    }

    #[test]
    fn extracts_magma_swap_fields() {
        let parsed =
            decode_parsed_json(magma::SWAP_EVENT, &hex_to_bytes(MAGMA_SWAP_BCS)).unwrap();
        let fields = extract_swap_fields(Protocol::Magma, &parsed).unwrap();
        assert!(fields.a_to_b);
        assert_eq!(fields.amount_in_raw, "200000000");
        assert_eq!(
            fields.pool_id,
            "0xe672f3fe0c6c0bee46db41d2fd00916596a2d2384e001d4e1d4a89f98799d94a"
        );
    }
}
