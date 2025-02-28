use crate::data::item_name::subtype::ammo::ammo;
use crate::data::item_name::subtype::amulet::amulet;
use crate::data::item_name::subtype::bag::bag;
use crate::data::item_name::subtype::chest::chest;
use crate::data::item_name::subtype::gem::gem;
use crate::data::item_name::subtype::generic_item;
use crate::data::item_name::subtype::jewelry::jewelry;
use crate::data::item_name::subtype::light_source::light_source;
use crate::data::item_name::subtype::melee_weapon::melee_weapon;
use crate::data::item_name::subtype::misc_object::misc_object;
use crate::data::item_name::subtype::misc_usable::misc_usable;
use crate::data::item_name::subtype::ranged_weapon::ranged_weapon;
use crate::data::item_name::subtype::small_armor::small_armor;
use crate::data::item_name::subtype::spike::spike;
use crate::data::item_name::subtype::wand::wand;
use crate::data::item_name::subtype::wearable_gem::wearable_gem;
use crate::model::{Item, ItemType};

mod helpers;
mod subtype;

pub fn generate(item: &Item) -> String {
    match item.item_type() {
        ItemType::Amulet => amulet(item),
        ItemType::Arrow | ItemType::Bolt | ItemType::SlingAmmo => ammo(item),
        ItemType::Bag => bag(item),
        ItemType::Belt
        | ItemType::Boots
        | ItemType::Bracers
        | ItemType::Cloak
        | ItemType::Gloves
        | ItemType::Helm => small_armor(item),
        ItemType::Chest => chest(item),
        ItemType::Chime
        | ItemType::ClosedDoor
        | ItemType::DownStaircase
        | ItemType::DownSteepStaircase
        | ItemType::EntranceToStore
        | ItemType::FlaskOfOil
        | ItemType::Food
        | ItemType::GemHelm
        | ItemType::HardArmor
        | ItemType::Horn
        | ItemType::Instrument
        | ItemType::JunkFood
        | ItemType::LodgingAtInn
        | ItemType::MagicBook
        | ItemType::Maul
        | ItemType::Money
        | ItemType::OpenDoor
        | ItemType::PoleArm
        | ItemType::Potion1
        | ItemType::Potion2
        | ItemType::PrayerBook
        | ItemType::Ring
        | ItemType::Rod
        | ItemType::Rubble
        | ItemType::Scroll1
        | ItemType::Scroll2
        | ItemType::SecretDoor
        | ItemType::SeenTrap
        | ItemType::Shield
        | ItemType::SoftArmor
        | ItemType::SongBook
        | ItemType::Staff
        | ItemType::UnseenTrap
        | ItemType::UpStaircase
        | ItemType::UpSteepStaircase
        | ItemType::Whirlpool => generic_item(item),
        ItemType::Dagger | ItemType::HaftedWeapon | ItemType::Pick | ItemType::Sword => {
            melee_weapon(item)
        }
        ItemType::Gem => gem(item),
        ItemType::Jewelry => jewelry(item),
        ItemType::LightSource => light_source(item),
        ItemType::MiscObject => misc_object(item),
        ItemType::MiscUsable => misc_usable(item),
        ItemType::RangedWeapon => ranged_weapon(item),
        ItemType::Spike => spike(item),
        ItemType::Wand => wand(item),
        ItemType::WearableGem => wearable_gem(item),
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_item::{self, template::FoodTemplate};

    use super::*;

    #[test]
    fn test_ration_of_food() {
        let mut item = generate_item::generate(Box::new(FoodTemplate::RationOfFood), 0);
        assert_eq!(generate(&item), "ration of food");

        item.number = 0;
        assert_eq!(generate(&item), "no more rations of food");

        item.number = 5;
        assert_eq!(generate(&item), "5 rations of food");
    }
}
