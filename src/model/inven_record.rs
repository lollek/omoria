use libc;

use crate::model::Item;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct InvenRecord {
    pub scost: libc::int64_t,
    pub sitem: Item,
}

