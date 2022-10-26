use crate::model::{item_subtype::ItemSubType, ItemType};

pub mod light_source;

pub fn to_usize(item_subtype: ItemSubType) -> usize {
    match item_subtype {
        ItemSubType::LightSource(subtype) => light_source::to_usize(subtype),
    }
}

pub fn from_usize(item_type: ItemType, item_subtype: usize) -> Option<ItemSubType> {
    match item_type {
        ItemType::LightSource => {
            light_source::from_usize(item_subtype).map(ItemSubType::LightSource)
        }
        _ => panic!("Unhandled item type {:?}", item_type),
    }
}
