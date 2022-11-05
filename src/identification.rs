use std::collections::HashMap;
use std::sync::RwLock;

use crate::conversion::item_subtype::from_i64;
use crate::model::item_subtype::ItemSubType;
use crate::model::Item;

pub type IdentifiedSubTypes = HashMap<ItemSubType, bool>;

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
        .insert(subtype, is_identified);
}

pub fn is_identified(subtype: ItemSubType) -> bool {
    *IDENTIFICATION
        .read()
        .unwrap_or_else(|err| panic!("Failed to read identification record: {}", err))
        .get(&subtype)
        .unwrap_or(&false)
}

#[no_mangle]
pub extern "C" fn identification_set_identified(item_ptr: *const Item) {
    let item = unsafe { *item_ptr };
    let subtype = from_i64(item.item_type(), item.subval)
        .unwrap_or_else(|| panic!("Failed to convert {:?} to subval", item));
    set_identified(subtype, true);
}

#[cfg(test)]
mod test {
    use serial_test::serial;

    use crate::model::item_subtype::FoodSubType;

    use super::*;

    #[test]
    #[serial]
    fn test_read_write() {
        let mut ident = IdentifiedSubTypes::default();

        set_record(ident.clone());
        assert_eq!(record(), ident);

        ident.insert(ItemSubType::Food(FoodSubType::RationOfFood), true);
        set_record(ident.clone());
        assert_eq!(record(), ident);
    }

    #[test]
    fn test_get_set_identification() {
        let subtype = ItemSubType::Food(FoodSubType::RationOfFood);
        assert!(!is_identified(subtype));
        set_identified(subtype, true);
        assert!(is_identified(subtype));
    }
}
