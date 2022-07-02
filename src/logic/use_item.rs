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
                ItemType::LightSource(_) => true,
                ItemType::Staff(_) => true,

                // Weapons:
                ItemType::Sling(_) => true, // Sling
                ItemType::Polearm(_) => item.subval == 8,
                ItemType::Dagger(_) => true,
                ItemType::Axe(_) => false,
                ItemType::Sword(_) => false, // No scimitar
                ItemType::Mace(_) => item.subval == 6 || item.subval == 13,

                // Armor
                ItemType::Boots(_) => true,
                ItemType::Gloves(_) => item.subval != 2,
                ItemType::Cloak(_) => true,
                ItemType::Helm(_) => item.subval == 1,
                ItemType::Shield(_) => item.subval <= 3,
                ItemType::SoftArmor(_) => item.subval <= 6,
                ItemType::Bracers(_) => item.subval <= 3,
                ItemType::Belt(_) => true,
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
                ItemType::LightSource(_) => true,
                ItemType::Staff(_) => true,
                ItemType::Pick(_) => true,
                ItemType::Ring(_) => true,
                ItemType::Amulet(_) => true,

                // Weapons:
                ItemType::Bow(_) => true,
                ItemType::Crossbow(_) => true,
                ItemType::Sling(_) => true,
                ItemType::Polearm(_) => true,
                ItemType::Dagger(_) => true,
                ItemType::Axe(_) => true,
                ItemType::Sword(_) => true,
                ItemType::Mace(_) => true,

                // Armor
                ItemType::Boots(_) => true,
                ItemType::Gloves(_) => true,
                ItemType::Cloak(_) => true,
                ItemType::Helm(_) => true,
                ItemType::Shield(_) => true,
                ItemType::SoftArmor(_) => true,
                ItemType::Bracers(_) => true,
                ItemType::Belt(_) => true,
                _ => false,
            }
        }
        _ => true
    }
}
