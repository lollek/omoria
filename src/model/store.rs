use crate::constants::STORE_INVEN_MAX;
use crate::model::{GameTime, InvenRecord};

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Store {
    pub store_open: GameTime,
    pub owner: u8,
    pub insult_cur: i8,
    pub store_ctr: u8,
    pub store_inven: [InvenRecord; STORE_INVEN_MAX + 1],
}

impl Default for Store {
    fn default() -> Self {
        Store {
            store_open: GameTime::default(),
            owner: 0,
            insult_cur: 0,
            store_ctr: 0,
            store_inven: [InvenRecord::default(); STORE_INVEN_MAX + 1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let store = Store::default();
        serde_json::to_string(&store).expect("Failed to serialize Store");
    }
}