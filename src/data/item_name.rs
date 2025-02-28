use crate::data::item_name::subtype::ammo::ammo;
use crate::data::item_name::subtype::amulet::amulet;
use crate::data::item_name::subtype::bag::bag;
use crate::data::item_name::subtype::belt::belt;
use crate::data::item_name::subtype::boots::boots;
use crate::data::item_name::subtype::bracers::bracers;
use crate::data::item_name::subtype::chest::chest;
use crate::data::item_name::subtype::gem::gem;
use crate::data::item_name::subtype::generic_item;
use crate::data::item_name::subtype::jewelry::jewelry;
use crate::data::item_name::subtype::light_source::light_source;
use crate::data::item_name::subtype::melee_weapon::melee_weapon;
use crate::data::item_name::subtype::misc_object::misc_object;
use crate::data::item_name::subtype::misc_usable::misc_usable;
use crate::data::item_name::subtype::ranged_weapon::ranged_weapon;
use crate::data::item_name::subtype::spike::spike;
use crate::data::item_name::subtype::wand::wand;
use crate::data::item_name::subtype::wearable_gem::wearable_gem;
use crate::model::{Item, ItemType};

mod helpers;
mod subtype;

pub fn generate(item: &Item) -> String {
    match item.item_type() {
        ItemType::Amulet => amulet(item),
        ItemType::Arrow => ammo(item),
        ItemType::Bag => bag(item),
        ItemType::Belt => belt(item),
        ItemType::Bolt => ammo(item),
        ItemType::Boots => boots(item),
        ItemType::Bracers => bracers(item),
        ItemType::Chest => chest(item),
        ItemType::Chime => generic_item(item),
        ItemType::Cloak => generic_item(item),
        ItemType::ClosedDoor => generic_item(item),
        ItemType::DownStaircase => generic_item(item),
        ItemType::DownSteepStaircase => generic_item(item),
        ItemType::EntranceToStore => generic_item(item),
        ItemType::FlaskOfOil => generic_item(item),
        ItemType::Food => generic_item(item),
        ItemType::Gem => gem(item),
        ItemType::GemHelm => generic_item(item),
        ItemType::Gloves => generic_item(item),
        ItemType::HaftedWeapon | ItemType::Dagger => melee_weapon(item),
        ItemType::HardArmor => generic_item(item),
        ItemType::Helm => generic_item(item),
        ItemType::Horn => generic_item(item),
        ItemType::Instrument => generic_item(item),
        ItemType::Jewelry => jewelry(item),
        ItemType::JunkFood => generic_item(item),
        ItemType::LightSource => light_source(item),
        ItemType::LodgingAtInn => generic_item(item),
        ItemType::MagicBook => generic_item(item),
        ItemType::Maul => generic_item(item),
        ItemType::MiscObject => misc_object(item),
        ItemType::MiscUsable => misc_usable(item),
        ItemType::Money => generic_item(item),
        ItemType::OpenDoor => generic_item(item),
        ItemType::Pick => generic_item(item),
        ItemType::PoleArm => generic_item(item),
        ItemType::Potion1 => generic_item(item),
        ItemType::Potion2 => generic_item(item),
        ItemType::PrayerBook => generic_item(item),
        ItemType::RangedWeapon => ranged_weapon(item),
        ItemType::Ring => generic_item(item),
        ItemType::Rod => generic_item(item),
        ItemType::Rubble => generic_item(item),
        ItemType::Scroll1 => generic_item(item),
        ItemType::Scroll2 => generic_item(item),
        ItemType::SecretDoor => generic_item(item),
        ItemType::SeenTrap => generic_item(item),
        ItemType::Shield => generic_item(item),
        ItemType::SlingAmmo => ammo(item),
        ItemType::SoftArmor => generic_item(item),
        ItemType::SongBook => generic_item(item),
        ItemType::Spike => spike(item),
        ItemType::Staff => generic_item(item),
        ItemType::Sword => generic_item(item),
        ItemType::UnseenTrap => generic_item(item),
        ItemType::UpStaircase => generic_item(item),
        ItemType::UpSteepStaircase => generic_item(item),
        ItemType::Wand => wand(item),
        ItemType::WearableGem => wearable_gem(item),
        ItemType::Whirlpool => generic_item(item),
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
