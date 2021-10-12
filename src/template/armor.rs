use model::{ Item, ItemType };

use misc;

pub fn soft_leather_armor() -> Item {
    Item {
        name: misc::rs2item_name("Soft Leather Armor [%P6,%P4]"),
        tval: ItemType::SoftArmor as u8,
        flags: 0,
        flags2: 0,
        p1: 0,
        cost: 18,
        subval: 2,
        weight: 80,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 4,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 0,
        identified: 1,
    }
}
