use crate::conversion;
use crate::model::item_subtype::ItemSubType;
use crate::model::ItemType;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::RwLock;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct IdentifiedSubTypes {
    inner: HashMap<ItemSubType, bool>,
}

#[cfg(test)]
impl IdentifiedSubTypes {
    pub(crate) fn get(&self, subtype: ItemSubType) -> Option<bool> {
        self.inner.get(&subtype).copied()
    }
}

impl Serialize for IdentifiedSubTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner
            .iter()
            .map(|(k, v)| {
                let item_type: u8 = k.get_type().into();
                let item_subtype: usize = conversion::item_subtype::to_usize(k);
                (item_type, item_subtype, *v)
            })
            .collect::<Vec<_>>()
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IdentifiedSubTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec: Vec<(u8, usize, bool)> = Deserialize::deserialize(deserializer)?;
        let mut inner = HashMap::new();
        for (item_type_as_usize, item_subtype_as_usize, is_identified) in vec {
            let item_type = ItemType::try_from(item_type_as_usize).map_err(|_| {
                D::Error::custom(format!("Invalid item type: {}", item_type_as_usize))
            })?;
            let item_subtype =
                conversion::item_subtype::from_usize(item_type, item_subtype_as_usize).ok_or_else(
                    || {
                        D::Error::custom(format!(
                            "Invalid item subtype: {} for type {}",
                            item_subtype_as_usize, item_type_as_usize
                        ))
                    },
                )?;
            inner.insert(item_subtype, is_identified);
        }
        Ok(IdentifiedSubTypes { inner })
    }
}

lazy_static! {
    pub(super) static ref IDENTIFICATION: RwLock<IdentifiedSubTypes> =
        RwLock::new(IdentifiedSubTypes::default());
}

pub fn record() -> IdentifiedSubTypes {
    IDENTIFICATION
        .read()
        .unwrap_or_else(|err| panic!("Failed to read identification record: {}", err))
        .clone()
}

pub fn set_record(record: IdentifiedSubTypes) {
    let mut data = IDENTIFICATION
        .write()
        .unwrap_or_else(|err| panic!("Failed to write identification record: {}", err));
    *data = record;
}

pub fn set_identified(subtype: ItemSubType, is_identified: bool) {
    IDENTIFICATION
        .write()
        .unwrap_or_else(|err| panic!("Failed to read identification record: {}", err))
        .inner
        .insert(subtype, is_identified);
}

pub fn is_identified(subtype: ItemSubType) -> bool {
    *IDENTIFICATION
        .read()
        .unwrap_or_else(|err| panic!("Failed to read identification record: {}", err))
        .inner
        .get(&subtype)
        .unwrap_or(&false)
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::model::item_subtype::{FoodSubType, MagicBookSubType};

    use super::*;

    #[test]
    #[serial]
    fn test_read_write() {
        let mut ident = IdentifiedSubTypes::default();

        set_record(ident.clone());
        assert_eq!(record(), ident);

        ident
            .inner
            .insert(ItemSubType::Food(FoodSubType::RationOfFood), true);
        set_record(ident.clone());
        assert_eq!(record(), ident);
    }

    #[test]
    #[serial]
    fn test_get_set_identification() {
        let subtype = ItemSubType::Food(FoodSubType::RationOfFood);
        set_identified(subtype, false);
        assert!(!is_identified(subtype));
        set_identified(subtype, true);
        assert!(is_identified(subtype));
        set_identified(subtype, false);
    }

    #[test]
    fn test_serialize_deserialize() {
        let mut identification = IdentifiedSubTypes::default();
        let subtype_true = ItemSubType::Food(FoodSubType::RationOfFood);
        let subtype_false = ItemSubType::MagicBook(MagicBookSubType::Magic1);
        assert!(identification.inner.is_empty());

        identification.inner.insert(subtype_true, true);
        identification.inner.insert(subtype_false, false);
        let serialized =
            serde_json::to_string(&identification).expect("Failed to serialize Identification");
        let deserialized: IdentifiedSubTypes =
            serde_json::from_str(&serialized).expect("Failed to deserialize Identification");

        assert_eq!(deserialized, identification);
        assert!(deserialized
            .inner
            .get(&subtype_true)
            .expect("subtype_true not found"));
        assert!(!deserialized
            .inner
            .get(&subtype_false)
            .expect("subtype_false not found"));
    }

    #[test]
    fn deserialize_unknown_item_type_returns_an_error() {
        let err = serde_json::from_str::<IdentifiedSubTypes>("[[255,0,true]]")
            .expect_err("unknown item type should fail deserialization");

        assert!(err.to_string().contains("255"));
        assert!(err.to_string().contains("item type"));
    }

    #[test]
    fn deserialize_unknown_item_subtype_returns_an_error() {
        let food_type = u8::from(ItemType::Food);
        let encoded = format!("[[{food_type},9999,true]]");
        let err = serde_json::from_str::<IdentifiedSubTypes>(&encoded)
            .expect_err("unknown item subtype should fail deserialization");

        assert!(err.to_string().contains("9999"));
        assert!(err.to_string().contains(&food_type.to_string()));
        assert!(err.to_string().contains("item subtype"));
    }

    #[test]
    fn deserialize_removed_item_type_returns_an_error() {
        let err = serde_json::from_str::<IdentifiedSubTypes>("[[60,0,true]]")
            .expect_err("removed item type should fail deserialization");

        assert!(err.to_string().contains("60"));
    }
}
