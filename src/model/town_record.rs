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

impl Default for TownRecord {
    fn default() -> Self {
        TownRecord {
            town_seed: 0,
            bank: [0; 7],
            stores: [Store::default(); constants::MAX_STORES + 1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let town_record = TownRecord::default();
        serde_json::to_string(&town_record).expect("Failed to serialize TownRecord");
    }
}