use crate::model::Item;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct InvenRecord {
    pub scost: i64,
    pub sitem: Item,
}

