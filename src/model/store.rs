use libc;

use model::{ InvenRecord, GameTime };
use constants::STORE_INVEN_MAX;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Store {
    pub store_open: GameTime,
    pub owner: libc::uint8_t,
    pub insult_cur: libc::int8_t,
    pub store_ctr: libc::uint8_t,
    pub store_inven: [InvenRecord; STORE_INVEN_MAX + 1],
}

