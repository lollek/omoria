use crate::error::Error;
use serde::{de::DeserializeOwned, Serialize};

pub(crate) fn encode<T: Serialize + ?Sized>(value: &T) -> Result<String, Error> {
    serde_json::to_string(value).map_err(|err| Error::from(err.to_string()))
}

pub(crate) fn decode<T: DeserializeOwned>(json: &str) -> Result<T, Error> {
    serde_json::from_str(json).map_err(|err| Error::from(err.to_string()))
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
}
