use types::{ Item, ItemType };

use misc;

pub fn wooden_torch() -> Item {
    Item {
        name: misc::rs2item_name("& Wooden Torch~ with %P5 turns of light"),
        tval: ItemType::LightSource as u8,
        flags: 0,
        flags2: 0,
        p1: 4000,
        cost: 2,
        subval: 14,
        weight: 30,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("1d2"),
        level: 1,
        identified: 1,
    }
}

pub fn cloak() -> Item {
    Item {
        name: misc::rs2item_name("& Cloak [%P6,%P4]"),
        tval: ItemType::Cloak as u8,
        flags: 0,
        flags2: 0,
        p1: 0,
        cost: 3,
        subval: 1,
        weight: 10,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 1,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 0,
        identified: 1,
    }
}
