use std::ptr;

use crate::thirdparty::serde::nullptr::UnitVisitor;
use crate::thirdparty::serde::NullPtr;
use serde::{Deserializer, Serializer};

use crate::model::Item;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct InventoryItem {
    //treas_rec
    pub data: Item,   // Real item
    pub ok: u8,       // Transient for sorting usable items
    pub insides: u16, // Something with bags?
    pub is_in: u8,    // Something with bags?
    #[serde(with = "NullPtr")]
    pub next: *mut InventoryItem, // Linked list next
}

impl<'de> NullPtr<'de> for *mut InventoryItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit()
    }

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_unit(UnitVisitor)?;
        Ok(ptr::null_mut())
    }
}
