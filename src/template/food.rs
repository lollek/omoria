use misc;
use model::{ ItemType };
use types::{ Item };

pub fn ration_of_food() -> Item {
    Item {
        name: misc::rs2item_name("& Ration~ of Food"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0,
        p1: 5000,
        cost: 3,
        subval: 307,
        weight: 10,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 0,
        identified: 1,
    }
}
