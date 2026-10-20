use crate::constants::STORE_INVEN_MAX;
use crate::model::{GameTime, InvenRecord};

/// C `store_t` identifiers, separate from entrance activation subvals.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StoreKind {
    GeneralStore = 0,
    Armory = 1,
    WeaponSmiths = 2,
    Temple = 3,
    AlchemyShop = 4,
    MagicShop = 5,
    Inn = 6,
    Library = 7,
    MusicShop = 8,
    GemStore = 9,
    AllNiteDeli = 10,
    BlackMarket = 11,
    TradingPost = 12,
    InsuranceShop = 13,
    Bank = 14,
    MoneyExchange = 15,
    Casino = 16,
    Fortress = 17,
    Generic1 = 18,
    Generic2 = 19,
    Generic3 = 20,
    Generic4 = 21,
    Generic5 = 22,
}

impl From<StoreKind> for libc::c_int {
    fn from(kind: StoreKind) -> Self {
        kind as libc::c_int
    }
}

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
    fn store_kind_uses_c_store_identifiers_not_entrance_subvals() {
        use super::StoreKind::*;
        let cases = [
            (GeneralStore, 0),
            (Armory, 1),
            (WeaponSmiths, 2),
            (Temple, 3),
            (AlchemyShop, 4),
            (MagicShop, 5),
            (Inn, 6),
            (Library, 7),
            (MusicShop, 8),
            (GemStore, 9),
            (AllNiteDeli, 10),
            (BlackMarket, 11),
            (TradingPost, 12),
            (InsuranceShop, 13),
            (Bank, 14),
            (MoneyExchange, 15),
            (Casino, 16),
            (Fortress, 17),
            (Generic1, 18),
            (Generic2, 19),
            (Generic3, 20),
            (Generic4, 21),
            (Generic5, 22),
        ];
        for (kind, expected) in cases {
            assert_eq!(libc::c_int::from(kind), expected, "{kind:?}");
        }
    }

    #[test]
    fn test_serialize() {
        let store = Store::default();
        serde_json::to_string(&store).expect("Failed to serialize Store");
    }
}
