use serde::Serializer;
use serde_json::Value;

/// Serialize integer fields as decimal strings (fullnode `parsedJson` convention).
pub fn serialize_num<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

/// Normalize move-binding JSON to fullnode `parsedJson` conventions:
/// - u64/u128 numbers → decimal strings
/// - leave bool/string/object structure intact
pub fn normalize(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, val)| (key, normalize(val)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.into_iter().map(normalize).collect()),
        Value::Number(number) => {
            if number.is_u64() || number.is_i64() {
                Value::String(number.to_string())
            } else if number.as_f64().is_some_and(|f| f.fract() == 0.0) {
                Value::String(number.to_string())
            } else {
                Value::Number(number)
            }
        }
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn stringifies_u64_fields() {
        let raw = json!({ "amount_in": 42u64, "atob": false });
        let out = normalize(raw);
        assert_eq!(out["amount_in"], "42");
        assert_eq!(out["atob"], false);
    }
}
