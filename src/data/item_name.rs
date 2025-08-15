use crate::data::item_name::subtype::ammo::ammo;
use crate::data::item_name::subtype::amulet::amulet;
use crate::data::item_name::subtype::armor::armor;
use crate::data::item_name::subtype::bag::bag;
use crate::data::item_name::subtype::book::book;
use crate::data::item_name::subtype::chest::chest;
use crate::data::item_name::subtype::food::food;
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

#[no_mangle]
extern "C" fn item_name(out_val: *mut [libc::c_char; 82], item: *const Item) {
    unsafe {
        const OUT_VAL_SIZE: usize = 82;

        let item_name = generate(&*item);
        let mut characters_written = 0;
        for (index, c) in item_name.chars().take(OUT_VAL_SIZE - 1).enumerate() {
            (*out_val)[index] = c as i8;
            characters_written += 1;
        }
        while characters_written < OUT_VAL_SIZE {
            (*out_val)[characters_written] = 0; // Null-terminate the string
            characters_written += 1;
        }
        (*out_val)[OUT_VAL_SIZE - 1] = 0; // Ensure the last character is null
    }
}

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
        | ItemType::GemHelm
        | ItemType::Horn
        | ItemType::Instrument
        | ItemType::LodgingAtInn
        | ItemType::Money
        | ItemType::OpenDoor
        | ItemType::Potion1
        | ItemType::Potion2
        | ItemType::Ring
        | ItemType::Rod
        | ItemType::Rubble
        | ItemType::Scroll1
        | ItemType::Scroll2
        | ItemType::SecretDoor
        | ItemType::SeenTrap
        | ItemType::Shield
        | ItemType::Staff
        | ItemType::UnseenTrap
        | ItemType::UpStaircase
        | ItemType::UpSteepStaircase
        | ItemType::Whirlpool => generic_item(item),
        ItemType::Dagger
        | ItemType::HaftedWeapon
        | ItemType::Maul
        | ItemType::Pick
        | ItemType::PoleArm
        | ItemType::Sword => melee_weapon(item),
        ItemType::Food
        | ItemType::JunkFood => food(item),
        ItemType::Gem => gem(item),
        ItemType::HardArmor | ItemType::SoftArmor => armor(item),
        ItemType::Jewelry => jewelry(item),
        ItemType::LightSource => light_source(item),
        ItemType::MagicBook
        | ItemType::PrayerBook
        | ItemType::SongBook => book(item),
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
    use crate::generate_item::{self, template::FoodTemplate, ItemQuality};

    use super::*;

    #[test]
    fn test_ration_of_food() {
        let mut item =
            generate_item::generate(Box::new(FoodTemplate::RationOfFood), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "ration of food");

        item.number = 0;
        assert_eq!(generate(&item), "no more rations of food");

        item.number = 5;
        assert_eq!(generate(&item), "5 rations of food");
    }
}
