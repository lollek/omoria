use crate::model::{Item, Name};

const STORE_DOOR_COUNT: usize =
    crate::constants::MAX_STORES + crate::constants::MAX_UNNAMED + 5 + 1;

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

const fn door_record(name: &str, subval: i64) -> Item {
    Item {
        name: name_bytes(name),
        tval: 110,
        flags2: 0,
        flags: 0,
        p1: 0,
        cost: 0,
        subval,
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
    door_record("the entrance to the General Store", 101),
    door_record("the entrance to the Armory", 102),
    door_record("the entrance to the Weapon Smiths", 103),
    door_record("the entrance to the Temple", 104),
    door_record("the entrance to the Alchemy Shop", 105),
    door_record("the entrance to the Magic Shop", 106),
    door_record("the entrance to the Inn", 107),
    door_record("the entrance to the Library", 109),
    door_record("the entrance to the Music Shop", 110),
    door_record("the entrance to the Gem Store", 113),
    door_record("the entrance to the All-Nite Deli", 116),
    door_record("the entrance to a building", 118),
    door_record("the entrance to the Trading Post", 108),
    door_record("the entrance to the Insurance Shop", 111),
    door_record("the entrance to the Bank", 112),
    door_record("the entrance to the Money Exchange", 114),
    door_record("the entrance to the Casino", 115),
    door_record("the entrance to a strange building", 117),
    door_record("the entrance to a building", 120),
    door_record("the entrance to a building", 121),
    door_record("the entrance to a building", 122),
    door_record("the entrance to a building", 123),
    door_record("the entrance to a building", 124),
    EMPTY_ITEM,
];

#[cfg(test)]
mod tests {
    use super::*;

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
                assert_eq!(&item.damage[..3], b"0d0");
                assert_eq!(&item.damage[3..], &[0; 4]);
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
