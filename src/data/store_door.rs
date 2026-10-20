use crate::model::{Item, Name};

const STORE_DOOR_COUNT: usize =
    crate::constants::MAX_STORES + crate::constants::MAX_UNNAMED + 5 + 1;

enum StoreDoorKind {
    GeneralStore = 101,
    Armory = 102,
    WeaponSmiths = 103,
    Temple = 104,
    AlchemyShop = 105,
    MagicShop = 106,
    Inn = 107,
    TradingPost = 108,
    Library = 109,
    MusicShop = 110,
    InsuranceShop = 111,
    Bank = 112,
    GemStore = 113,
    MoneyExchange = 114,
    Casino = 115,
    AllNiteDeli = 116,
    Fortress = 117,
    BlackMarket = 118,
    // Preserve the distinct legacy codes; traps.c handles 120-122 as houses.
    House120 = 120,
    House121 = 121,
    House122 = 122,
    // Despite their building names, 123 triggers a whirlpool and 124 is unhandled.
    Whirlpool = 123,
    Unknown124 = 124,
}

const fn name_bytes(value: &str) -> Name {
    let mut name = [0; 70];
    let bytes = value.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        name[index] = bytes[index] as libc::c_char;
        index += 1;
    }

    name
}

const fn door_record(name: &str, kind: StoreDoorKind) -> Item {
    Item {
        name: name_bytes(name),
        tval: 110,
        flags2: 0,
        flags: 0,
        p1: 0,
        cost: 0,
        subval: kind as i64,
        weight: 0,
        number: 0,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: [
            b'0' as libc::c_char,
            b'd' as libc::c_char,
            b'0' as libc::c_char,
            0,
            0,
            0,
            0,
        ],
        level: 0,
        identified: 0,
    }
}

const EMPTY_ITEM: Item = Item {
    name: [0; 70],
    tval: 0,
    flags2: 0,
    flags: 0,
    p1: 0,
    cost: 0,
    subval: 0,
    weight: 0,
    number: 0,
    tohit: 0,
    todam: 0,
    ac: 0,
    toac: 0,
    damage: [0; 7],
    level: 0,
    identified: 0,
};

#[no_mangle]
pub static mut store_door: [Item; STORE_DOOR_COUNT] = [
    door_record(
        "the entrance to the General Store",
        StoreDoorKind::GeneralStore,
    ),
    door_record("the entrance to the Armory", StoreDoorKind::Armory),
    door_record(
        "the entrance to the Weapon Smiths",
        StoreDoorKind::WeaponSmiths,
    ),
    door_record("the entrance to the Temple", StoreDoorKind::Temple),
    door_record(
        "the entrance to the Alchemy Shop",
        StoreDoorKind::AlchemyShop,
    ),
    door_record("the entrance to the Magic Shop", StoreDoorKind::MagicShop),
    door_record("the entrance to the Inn", StoreDoorKind::Inn),
    door_record("the entrance to the Library", StoreDoorKind::Library),
    door_record("the entrance to the Music Shop", StoreDoorKind::MusicShop),
    door_record("the entrance to the Gem Store", StoreDoorKind::GemStore),
    door_record(
        "the entrance to the All-Nite Deli",
        StoreDoorKind::AllNiteDeli,
    ),
    door_record("the entrance to a building", StoreDoorKind::BlackMarket),
    door_record(
        "the entrance to the Trading Post",
        StoreDoorKind::TradingPost,
    ),
    door_record(
        "the entrance to the Insurance Shop",
        StoreDoorKind::InsuranceShop,
    ),
    door_record("the entrance to the Bank", StoreDoorKind::Bank),
    door_record(
        "the entrance to the Money Exchange",
        StoreDoorKind::MoneyExchange,
    ),
    door_record("the entrance to the Casino", StoreDoorKind::Casino),
    door_record(
        "the entrance to a strange building",
        StoreDoorKind::Fortress,
    ),
    door_record("the entrance to a building", StoreDoorKind::House120),
    door_record("the entrance to a building", StoreDoorKind::House121),
    door_record("the entrance to a building", StoreDoorKind::House122),
    door_record("the entrance to a building", StoreDoorKind::Whirlpool),
    door_record("the entrance to a building", StoreDoorKind::Unknown124),
    EMPTY_ITEM,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn door_record_converts_typed_kind_to_legacy_subval() {
        let cases = [
            (StoreDoorKind::GeneralStore, 101),
            (StoreDoorKind::Armory, 102),
            (StoreDoorKind::WeaponSmiths, 103),
            (StoreDoorKind::Temple, 104),
            (StoreDoorKind::AlchemyShop, 105),
            (StoreDoorKind::MagicShop, 106),
            (StoreDoorKind::Inn, 107),
            (StoreDoorKind::TradingPost, 108),
            (StoreDoorKind::Library, 109),
            (StoreDoorKind::MusicShop, 110),
            (StoreDoorKind::InsuranceShop, 111),
            (StoreDoorKind::Bank, 112),
            (StoreDoorKind::GemStore, 113),
            (StoreDoorKind::MoneyExchange, 114),
            (StoreDoorKind::Casino, 115),
            (StoreDoorKind::AllNiteDeli, 116),
            (StoreDoorKind::Fortress, 117),
            (StoreDoorKind::BlackMarket, 118),
            (StoreDoorKind::House120, 120),
            (StoreDoorKind::House121, 121),
            (StoreDoorKind::House122, 122),
            (StoreDoorKind::Whirlpool, 123),
            (StoreDoorKind::Unknown124, 124),
        ];

        for (kind, expected_subval) in cases {
            let record = door_record("test entrance", kind);
            assert_eq!(record.subval, expected_subval);
        }
    }

    const STORE_DOORS: &[(&str, i64)] = &[
        ("the entrance to the General Store", 101),
        ("the entrance to the Armory", 102),
        ("the entrance to the Weapon Smiths", 103),
        ("the entrance to the Temple", 104),
        ("the entrance to the Alchemy Shop", 105),
        ("the entrance to the Magic Shop", 106),
        ("the entrance to the Inn", 107),
        ("the entrance to the Library", 109),
        ("the entrance to the Music Shop", 110),
        ("the entrance to the Gem Store", 113),
        ("the entrance to the All-Nite Deli", 116),
        ("the entrance to a building", 118),
        ("the entrance to the Trading Post", 108),
        ("the entrance to the Insurance Shop", 111),
        ("the entrance to the Bank", 112),
        ("the entrance to the Money Exchange", 114),
        ("the entrance to the Casino", 115),
        ("the entrance to a strange building", 117),
        ("the entrance to a building", 120),
        ("the entrance to a building", 121),
        ("the entrance to a building", 122),
        ("the entrance to a building", 123),
        ("the entrance to a building", 124),
    ];

    #[test]
    #[serial_test::serial]
    fn store_door_records_match_legacy_data() {
        let store_door_ptr = std::ptr::addr_of!(store_door).cast::<Item>();

        unsafe {
            for (index, (name, subval)) in STORE_DOORS.iter().enumerate() {
                let item = store_door_ptr.add(index).read();
                assert_eq!(item.name_bytes(), name.as_bytes());
                assert_eq!(item.tval, 110);
                assert_eq!(item.subval, *subval);
                assert_eq!(item.damage.map(i16::from), b"0d0\0\0\0\0".map(i16::from));
                assert_eq!(item.flags2, 0);
                assert_eq!(item.flags, 0);
                assert_eq!(item.p1, 0);
                assert_eq!(item.cost, 0);
                assert_eq!(item.weight, 0);
                assert_eq!(item.number, 0);
                assert_eq!(item.tohit, 0);
                assert_eq!(item.todam, 0);
                assert_eq!(item.ac, 0);
                assert_eq!(item.toac, 0);
                assert_eq!(item.level, 0);
                assert_eq!(item.identified, 0);
            }

            let empty_item = store_door_ptr.add(STORE_DOORS.len()).read();
            assert_eq!(empty_item.name, [0; 70]);
            assert_eq!(empty_item.tval, 0);
            assert_eq!(empty_item.flags2, 0);
            assert_eq!(empty_item.flags, 0);
            assert_eq!(empty_item.p1, 0);
            assert_eq!(empty_item.cost, 0);
            assert_eq!(empty_item.subval, 0);
            assert_eq!(empty_item.weight, 0);
            assert_eq!(empty_item.number, 0);
            assert_eq!(empty_item.tohit, 0);
            assert_eq!(empty_item.todam, 0);
            assert_eq!(empty_item.ac, 0);
            assert_eq!(empty_item.toac, 0);
            assert_eq!(empty_item.damage, [0; 7]);
            assert_eq!(empty_item.level, 0);
            assert_eq!(empty_item.identified, 0);
        }
    }
}
