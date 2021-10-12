use model::Class;
use types::{ Item, ItemType };

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
                ItemType::RangedWeapon => item.subval == 20, // Sling
                ItemType::PoleArm => item.subval == 8,
                ItemType::Dagger => true,
                ItemType::HaftedWeapon => false,
                ItemType::Sword => false, // No scimitar
                ItemType::Maul => item.subval == 6 || item.subval == 13,

                // Armor
                ItemType::GemHelm => false, // Only metal atm
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
                ItemType::RangedWeapon => true,
                ItemType::PoleArm => true,
                ItemType::Dagger => true,
                ItemType::HaftedWeapon => true,
                ItemType::Sword => true,
                ItemType::Maul => true,

                // Armor
                ItemType::GemHelm => true,
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
