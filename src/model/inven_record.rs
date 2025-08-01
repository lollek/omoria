use crate::model::Item;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct InvenRecord {
    pub scost: i64,
    pub sitem: Item,
}
