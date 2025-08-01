use libc;

use crate::constants;
use crate::model::Store;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TownRecord {
    pub town_seed: libc::c_ulong,
    pub bank: [i64; 7],
    pub stores: [Store; constants::MAX_STORES + 1],
}
