use model::{ ItemType };
use types::{ Item };

use misc;

pub fn stiletto() -> Item {
    Item {
        name: misc::rs2item_name("& Stiletto (%P0) (%P2,%P3)"),
        tval: ItemType::Dagger as u8,
        flags: 0x10000000,
        flags2: 0,
        p1: 0,
        cost: 10,
        subval: 3,
        weight: 12,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("1d4"),
        level: 0,
        identified: 1,
    }
}

pub fn quarterstaff() -> Item {
    Item {
        name: misc::rs2item_name("& Iron Shod Quarterstaff^ (%P2,%P3)"),
        tval: ItemType::Maul as u8,
        flags: 0,
        flags2: 0,
        p1: 0,
        cost: 25,
        subval: 13,
        weight: 100,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("1d5"),
        level: 0,
        identified: 1,
    }
}
