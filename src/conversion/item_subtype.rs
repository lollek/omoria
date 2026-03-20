use std::convert::{TryFrom, TryInto};

use crate::model::{item_subtype::ItemSubType, ItemType};

pub mod amulet;
pub mod arrow;
pub mod bag;
pub mod belt;
pub mod bolt;
pub mod boots;
pub mod bracers;
pub mod chest;
pub mod chime;
pub mod cloak;
pub mod dagger;
pub mod flask_of_oil;
pub mod food;
pub mod gem;
pub mod gem_helm;
pub mod gloves;
pub mod hafted_weapon;
pub mod hard_armor;
pub mod helm;
pub mod horn;
pub mod instrument;
pub mod jewelry;
pub mod junk_food;
pub mod light_source;
pub mod lodging_at_inn;
pub mod magic_book;
pub mod maul;
pub mod misc_item;
pub mod misc_usable;
pub mod pick;
pub mod polearm;
pub mod potion;
pub mod prayer_book;
pub mod ranged_weapon;
pub mod ring;
pub mod scroll;
pub mod shield;
pub mod sling_ammo;
pub mod soft_armor;
pub mod song_book;
pub mod spike;
pub mod staff;
pub mod sword;
pub mod wand;
pub mod wearable_gem;

pub fn to_usize(item_subtype: &ItemSubType) -> usize {
    match item_subtype {
        ItemSubType::MiscObject(subtype) => usize::from(*subtype),
        ItemSubType::Chest(subtype) => usize::from(*subtype),
        ItemSubType::MiscUsable(subtype) => usize::from(*subtype),
        ItemSubType::Jewelry(subtype) => usize::from(*subtype),
        ItemSubType::Gem(subtype) => usize::from(*subtype),
        ItemSubType::Bag(subtype) => usize::from(*subtype),
        ItemSubType::WearableGem(subtype) => usize::from(*subtype),
        ItemSubType::SlingAmmo(subtype) => usize::from(*subtype),
        ItemSubType::Bolt(subtype) => usize::from(*subtype),
        ItemSubType::Arrow(subtype) => usize::from(*subtype),
        ItemSubType::Spike(subtype) => usize::from(*subtype),
        ItemSubType::LightSource(subtype) => usize::from(*subtype),
        ItemSubType::RangedWeapon(subtype) => usize::from(*subtype),
        ItemSubType::HaftedWeapon(subtype) => usize::from(*subtype),
        ItemSubType::PoleArm(subtype) => usize::from(*subtype),
        ItemSubType::Dagger(subtype) => usize::from(*subtype),
        ItemSubType::Sword(subtype) => usize::from(*subtype),
        ItemSubType::Pick(subtype) => usize::from(*subtype),
        ItemSubType::Maul(subtype) => usize::from(*subtype),
        ItemSubType::GemHelm(subtype) => usize::from(*subtype),
        ItemSubType::Boots(subtype) => usize::from(*subtype),
        ItemSubType::Gloves(subtype) => usize::from(*subtype),
        ItemSubType::Cloak(subtype) => usize::from(*subtype),
        ItemSubType::Helm(subtype) => usize::from(*subtype),
        ItemSubType::Shield(subtype) => usize::from(*subtype),
        ItemSubType::HardArmor(subtype) => usize::from(*subtype),
        ItemSubType::SoftArmor(subtype) => usize::from(*subtype),
        ItemSubType::Bracers(subtype) => usize::from(*subtype),
        ItemSubType::Belt(subtype) => usize::from(*subtype),
        ItemSubType::Amulet(subtype) => usize::from(*subtype),
        ItemSubType::Ring(subtype) => usize::from(*subtype),
        ItemSubType::Staff(subtype) => usize::from(*subtype),
        ItemSubType::Rod(_) => panic!("ItemType Rod has been removed"),
        ItemSubType::Wand(subtype) => usize::from(*subtype),
        ItemSubType::Scroll1(subtype) => usize::from(*subtype),
        ItemSubType::Scroll2(_) => panic!("ItemType Scroll2 has been removed"),
        ItemSubType::Potion1(subtype) => usize::from(*subtype),
        ItemSubType::Potion2(_) => panic!("ItemType Potion2 has been removed"),
        ItemSubType::FlaskOfOil(subtype) => usize::from(*subtype),
        ItemSubType::Food(subtype) => usize::from(*subtype),
        ItemSubType::JunkFood(subtype) => usize::from(*subtype),
        ItemSubType::Chime(subtype) => usize::from(*subtype),
        ItemSubType::Horn(subtype) => usize::from(*subtype),
        ItemSubType::MagicBook(subtype) => usize::from(*subtype),
        ItemSubType::PrayerBook(subtype) => usize::from(*subtype),
        ItemSubType::Instrument(subtype) => usize::from(*subtype),
        ItemSubType::SongBook(subtype) => usize::from(*subtype),
        ItemSubType::LodgingAtInn(subtype) => usize::from(*subtype),
    }
}

pub fn from_usize(item_type: ItemType, item_subtype: usize) -> Option<ItemSubType> {
    match item_type {
        ItemType::MiscObject => {
            crate::model::item_subtype::MiscObjectSubType::try_from(item_subtype)
                .ok()
                .map(ItemSubType::MiscObject)
        }
        ItemType::Chest => crate::model::item_subtype::ChestSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Chest),
        ItemType::MiscUsable => {
            crate::model::item_subtype::MiscUsableSubType::try_from(item_subtype)
                .ok()
                .map(ItemSubType::MiscUsable)
        }
        ItemType::Jewelry => crate::model::item_subtype::JewelrySubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Jewelry),
        ItemType::Gem => crate::model::item_subtype::GemSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Gem),
        ItemType::Bag => crate::model::item_subtype::BagSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Bag),
        ItemType::WearableGem => {
            crate::model::item_subtype::WearableGemSubType::try_from(item_subtype)
                .ok()
                .map(ItemSubType::WearableGem)
        }
        ItemType::SlingAmmo => crate::model::item_subtype::SlingAmmoSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::SlingAmmo),
        ItemType::Bolt => crate::model::item_subtype::BoltSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Bolt),
        ItemType::Arrow => crate::model::item_subtype::ArrowSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Arrow),
        ItemType::Spike => crate::model::item_subtype::SpikeSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Spike),
        ItemType::LightSource => {
            crate::model::item_subtype::LightSourceSubType::try_from(item_subtype)
                .ok()
                .map(ItemSubType::LightSource)
        }
        ItemType::RangedWeapon => {
            crate::model::item_subtype::RangedWeaponSubType::try_from(item_subtype)
                .ok()
                .map(ItemSubType::RangedWeapon)
        }
        ItemType::HaftedWeapon => {
            crate::model::item_subtype::HaftedWeaponSubType::try_from(item_subtype)
                .ok()
                .map(ItemSubType::HaftedWeapon)
        }
        ItemType::PoleArm => crate::model::item_subtype::PoleArmSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::PoleArm),
        ItemType::Dagger => crate::model::item_subtype::DaggerSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Dagger),
        ItemType::Sword => crate::model::item_subtype::SwordSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Sword),
        ItemType::Pick => crate::model::item_subtype::PickSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Pick),
        ItemType::Maul => crate::model::item_subtype::MaulSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Maul),
        ItemType::GemHelm => crate::model::item_subtype::GemHelmSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::GemHelm),
        ItemType::Boots => crate::model::item_subtype::BootsSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Boots),
        ItemType::Gloves => crate::model::item_subtype::GlovesSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Gloves),
        ItemType::Cloak => crate::model::item_subtype::CloakSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Cloak),
        ItemType::Helm => crate::model::item_subtype::HelmSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Helm),
        ItemType::Shield => crate::model::item_subtype::ShieldSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Shield),
        ItemType::HardArmor => crate::model::item_subtype::HardArmorSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::HardArmor),
        ItemType::SoftArmor => crate::model::item_subtype::SoftArmorSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::SoftArmor),
        ItemType::Bracers => crate::model::item_subtype::BracersSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Bracers),
        ItemType::Belt => crate::model::item_subtype::BeltSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Belt),
        ItemType::Amulet => crate::model::item_subtype::AmuletSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Amulet),
        ItemType::Ring => crate::model::item_subtype::RingSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Ring),
        ItemType::Staff => crate::model::item_subtype::StaffSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Staff),
        ItemType::Rod => panic!("ItemType Rod has been removed"),
        ItemType::Wand => crate::model::item_subtype::WandSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Wand),
        ItemType::Scroll1 => crate::model::item_subtype::Scroll1SubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Scroll1),
        ItemType::Scroll2 => panic!("ItemType Scroll2 has been removed"),
        ItemType::Potion1 => crate::model::item_subtype::Potion1SubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Potion1),
        ItemType::Potion2 => panic!("ItemType Potion2 has been removed"),
        ItemType::FlaskOfOil => {
            crate::model::item_subtype::FlaskOfOilSubType::try_from(item_subtype)
                .ok()
                .map(ItemSubType::FlaskOfOil)
        }
        ItemType::Food => crate::model::item_subtype::FoodSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Food),
        ItemType::JunkFood => crate::model::item_subtype::JunkFoodSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::JunkFood),
        ItemType::Chime => crate::model::item_subtype::ChimeSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Chime),
        ItemType::Horn => crate::model::item_subtype::HornSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::Horn),
        ItemType::MagicBook => crate::model::item_subtype::MagicBookSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::MagicBook),
        ItemType::PrayerBook => {
            crate::model::item_subtype::PrayerBookSubType::try_from(item_subtype)
                .ok()
                .map(ItemSubType::PrayerBook)
        }
        ItemType::Instrument => {
            crate::model::item_subtype::InstrumentSubType::try_from(item_subtype)
                .ok()
                .map(ItemSubType::Instrument)
        }
        ItemType::SongBook => crate::model::item_subtype::SongBookSubType::try_from(item_subtype)
            .ok()
            .map(ItemSubType::SongBook),
        ItemType::LodgingAtInn => {
            crate::model::item_subtype::LodgingAtInnSubType::try_from(item_subtype)
                .ok()
                .map(ItemSubType::LodgingAtInn)
        }
        _ => panic!("Unhandled item type {:?}", item_type),
    }
}

pub fn from_i64(item_type: ItemType, item_subtype: i64) -> Option<ItemSubType> {
    from_usize(
        item_type,
        item_subtype
            .try_into()
            .unwrap_or_else(|err| panic!("Failed to convert i64 to usize: {}", err)),
    )
}
