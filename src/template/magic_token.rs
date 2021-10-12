use model::{ ItemType };
use types::{ Item };

use misc;

pub fn magic_book() -> Item {
    Item {
        name: misc::rs2item_name("& Book of Magic Spells [Beginners-Magik]"),
        tval: ItemType::MagicBook as u8,
        flags: 0x00000000,
        flags2: 0x0000007F,
        p1: 0,
        cost: 25,
        subval: 257,
        weight: 60,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("1d1"),
        level: 0,
        identified: 1,
    }
}

pub fn prayer_book() -> Item {
    Item {
        name: misc::rs2item_name("& Holy Book of Prayers [Beginners Handbook]"),
        tval: ItemType::PrayerBook as u8,
        flags: 0x00000000,
        flags2: 0x000000FF,
        p1: 0,
        cost: 25,
        subval: 258,
        weight: 60,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("1d1"),
        level: 0,
        identified: 1,
    }
}

pub fn instrument() -> Item {
    Item {
        name: misc::rs2item_name("& Pipes of Peace"),
        tval: ItemType::Instrument as u8,
        flags: 0x00000000,
        flags2: 0x000003FF,
        p1: 0,
        cost: 30,
        subval: 258,
        weight: 40,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("1d1"),
        level: 0,
        identified: 1,
    }
}

pub fn song_book() -> Item {
    Item {
        name: misc::rs2item_name("& Book of Bard Lyrics [Beginners Song book]"),
        tval: ItemType::SongBook as u8,
        flags: 0x00000000,
        flags2: 0x000007FF,
        p1: 0,
        cost: 30,
        subval: 262,
        weight: 50,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("1d1"),
        level: 0,
        identified: 1,
    }
}
