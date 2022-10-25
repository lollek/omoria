use crate::constants::STORE_INVEN_MAX;
use crate::model::{GameTime, InvenRecord};

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Store {
    pub store_open: GameTime,
    pub owner: u8,
    pub insult_cur: i8,
    pub store_ctr: u8,
    pub store_inven: [InvenRecord; STORE_INVEN_MAX + 1],
}
