use crate::error::Error;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub(crate) fn encode<T: Serialize + ?Sized>(value: &T) -> Result<String, Error> {
    serde_json::to_string(value).map_err(|err| Error::from(err.to_string()))
}

pub(crate) fn decode<T: DeserializeOwned>(json: &str) -> Result<T, Error> {
    serde_json::from_str(json).map_err(|err| Error::from(err.to_string()))
}

fn is_zero_or_empty(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Bool(b) => !b,
        Value::Number(n) => n.as_f64() == Some(0.0),
        Value::String(s) => s.is_empty(),
        Value::Array(a) => a.is_empty(),
        Value::Object(o) => o.is_empty(),
    }
}

/// Removes object fields that are zero/empty and equal to the default record,
/// so save files stay small. Arrays are left untouched because their elements
/// have no default to restore from.
fn prune_defaults(value: &mut Value, default: &Value) {
    if let (Value::Object(map), Value::Object(default_map)) = (value, default) {
        map.retain(|key, field| match default_map.get(key) {
            Some(default_field) => !(is_zero_or_empty(field) && field == default_field),
            None => true,
        });
        for (key, field) in map.iter_mut() {
            if let Some(default_field) = default_map.get(key) {
                prune_defaults(field, default_field);
            }
        }
    }
}

fn restore_defaults(value: &mut Value, default: &Value) {
    if let (Value::Object(map), Value::Object(default_map)) = (value, default) {
        for (key, default_field) in default_map {
            match map.get_mut(key) {
                Some(field) => restore_defaults(field, default_field),
                None => {
                    map.insert(key.clone(), default_field.clone());
                }
            }
        }
    }
}

/// Like `encode`, but omits zero and empty values that match `T::default()`.
pub(crate) fn encode_compact<T: Serialize + Default>(value: &T) -> Result<String, Error> {
    let mut encoded = serde_json::to_value(value).map_err(|err| Error::from(err.to_string()))?;
    let default = serde_json::to_value(T::default()).map_err(|err| Error::from(err.to_string()))?;
    prune_defaults(&mut encoded, &default);
    serde_json::to_string(&encoded).map_err(|err| Error::from(err.to_string()))
}

/// Inverse of `encode_compact`; also accepts fully populated records.
pub(crate) fn decode_compact<T: DeserializeOwned + Serialize + Default>(
    json: &str,
) -> Result<T, Error> {
    let mut value: Value = decode(json)?;
    let default = serde_json::to_value(T::default()).map_err(|err| Error::from(err.to_string()))?;
    restore_defaults(&mut value, &default);
    serde_json::from_value(value).map_err(|err| Error::from(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::master::MasterRecord;

    const MASTER: &str = r#"{"uid":1,"user_name":"-","character_name":"Fixture","points":42,"alive":true,"level":2,"race":"Elf","class":"Wizard"}"#;

    #[test]
    fn master_round_trips_without_semantic_changes() {
        let record: MasterRecord = decode(MASTER).expect("master should decode");
        let encoded = encode(&record).expect("master should encode");
        let decoded: MasterRecord = decode(&encoded).expect("encoded master should decode");
        assert_eq!(decoded.uid, 1);
        assert_eq!(decoded.character_name, "Fixture");
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&encoded).unwrap(),
            serde_json::from_str::<serde_json::Value>(MASTER).unwrap()
        );
    }

    #[test]
    fn malformed_json_returns_an_error() {
        let err = decode::<MasterRecord>("{\"uid\": nope}").err().unwrap();
        assert!(
            err.to_string().contains("expected"),
            "unexpected error: {}",
            err
        );
    }

    #[test]
    fn truncated_json_returns_an_error() {
        let err = decode::<MasterRecord>("{\"uid\":").err().unwrap();
        assert!(err.to_string().contains("EOF"), "unexpected error: {}", err);
    }

    #[test]
    fn missing_field_returns_an_error() {
        let err = decode::<MasterRecord>("{}").err().unwrap();
        assert!(
            err.to_string().contains("missing field `uid`"),
            "unexpected error: {}",
            err
        );
    }

    #[test]
    fn serialization_error_is_returned() {
        struct FailingRecord;

        impl Serialize for FailingRecord {
            fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("injected serialization failure"))
            }
        }

        let err = encode(&FailingRecord).unwrap_err();
        assert_eq!(err.to_string(), "injected serialization failure");
    }

    #[test]
    fn compact_round_trip_omits_default_fields() {
        // SaveRecord::default() is large; run on a bigger stack than the test default.
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(compact_round_trip_body)
            .unwrap()
            .join()
            .unwrap();
    }

    fn compact_round_trip_body() {
        use crate::save::SaveRecord;
        let compact = encode_compact(&SaveRecord::default()).unwrap();
        assert!(compact.len() < encode(&SaveRecord::default()).unwrap().len());
        let decoded: SaveRecord = decode_compact(&compact).unwrap();
        assert_eq!(
            encode(&decoded).unwrap(),
            encode(&SaveRecord::default()).unwrap()
        );
    }
}
