use libc;

use constants;
use model::Store;

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct TownRecord {
    pub town_seed: libc::c_ulong,
    pub bank: [libc::int64_t; 7],
    pub stores: [Store; constants::MAX_STORES + 1],
}
