use model::Class;
use model::{ Item, ItemType };

pub fn class_can_use_item(class: &Class, item: &Item) -> bool {
    match class {
        Class::Druid => {
            /* Weapons:
             * club, dagger, dart, quarterstaff, scimitar, scythe, sickle, shortspear, sling,
             * spear
             *
             * Armor:
             * light / medium. No major metal.
             *
             * Shields:
             * small / medium. No major metal.
             *
             * Need more: shortspear, spear, scimitar
             */
            match item.item_type() {
                // Utility:
                ItemType::LightSource => true,
                ItemType::Staff => true,

                // Weapons:
                ItemType::Sling => true, // Sling
                ItemType::Polearm => item.subval == 8,
                ItemType::Dagger => true,
                ItemType::Axe => false,
                ItemType::Sword => false, // No scimitar
                ItemType::Mace => item.subval == 6 || item.subval == 13,

                // Armor
                ItemType::Boots => true,
                ItemType::Gloves => item.subval != 2,
                ItemType::Cloak => true,
                ItemType::Helm => item.subval == 1,
                ItemType::Shield => item.subval <= 3,
                ItemType::SoftArmor => item.subval <= 6,
                ItemType::Bracers => item.subval <= 3,
                ItemType::Belt => true,
                _ => false,
            }
        },
        Class::Barbarian => {
            /*
             * Armor:
             * light / medium.
             */
            match item.item_type() {
                // Utility:
                ItemType::LightSource => true,
                ItemType::Staff => true,
                ItemType::Pick => true,
                ItemType::Ring => true,
                ItemType::Amulet => true,

                // Weapons:
                ItemType::Bow => true,
                ItemType::Crossbow => true,
                ItemType::Sling => true,
                ItemType::Polearm => true,
                ItemType::Dagger => true,
                ItemType::Axe => true,
                ItemType::Sword => true,
                ItemType::Mace => true,

                // Armor
                ItemType::Boots => true,
                ItemType::Gloves => true,
                ItemType::Cloak => true,
                ItemType::Helm => true,
                ItemType::Shield => true,
                ItemType::SoftArmor => true,
                ItemType::Bracers => true,
                ItemType::Belt => true,
                _ => false,
            }
        }
        _ => true
    }
}
