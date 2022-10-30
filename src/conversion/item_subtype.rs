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

pub fn to_usize(item_subtype: ItemSubType) -> usize {
    match item_subtype {
        ItemSubType::MiscObject(subtype) => misc_item::to_usize(subtype),
        ItemSubType::Chest(subtype) => chest::to_usize(subtype),
        ItemSubType::MiscUsable(subtype) => misc_usable::to_usize(subtype),
        ItemSubType::Jewelry(subtype) => jewelry::to_usize(subtype),
        ItemSubType::Gem(subtype) => gem::to_usize(subtype),
        ItemSubType::Bag(subtype) => bag::to_usize(subtype),
        ItemSubType::WearableGem(subtype) => wearable_gem::to_usize(subtype),
        ItemSubType::SlingAmmo(subtype) => sling_ammo::to_usize(subtype),
        ItemSubType::Bolt(subtype) => bolt::to_usize(subtype),
        ItemSubType::Arrow(subtype) => arrow::to_usize(subtype),
        ItemSubType::Spike(subtype) => spike::to_usize(subtype),
        ItemSubType::LightSource(subtype) => light_source::to_usize(subtype),
        ItemSubType::RangedWeapon(subtype) => ranged_weapon::to_usize(subtype),
        ItemSubType::HaftedWeapon(subtype) => hafted_weapon::to_usize(subtype),
        ItemSubType::PoleArm(subtype) => polearm::to_usize(subtype),
        ItemSubType::Dagger(subtype) => dagger::to_usize(subtype),
        ItemSubType::Sword(subtype) => sword::to_usize(subtype),
        ItemSubType::Pick(subtype) => pick::to_usize(subtype),
        ItemSubType::Maul(subtype) => maul::to_usize(subtype),
        ItemSubType::GemHelm(_) => panic!("ItemType Gem Helm has been removed"),
        ItemSubType::Boots(subtype) => boots::to_usize(subtype),
        ItemSubType::Gloves(subtype) => gloves::to_usize(subtype),
        ItemSubType::Cloak(subtype) => cloak::to_usize(subtype),
        ItemSubType::Helm(subtype) => helm::to_usize(subtype),
        ItemSubType::Shield(subtype) => shield::to_usize(subtype),
        ItemSubType::HardArmor(subtype) => hard_armor::to_usize(subtype),
        ItemSubType::SoftArmor(subtype) => soft_armor::to_usize(subtype),
        ItemSubType::Bracers(subtype) => bracers::to_usize(subtype),
        ItemSubType::Belt(subtype) => belt::to_usize(subtype),
        ItemSubType::Amulet(subtype) => amulet::to_usize(subtype),
        ItemSubType::Ring(subtype) => ring::to_usize(subtype),
        ItemSubType::Staff(subtype) => staff::to_usize(subtype),
        ItemSubType::Rod(_) => panic!("ItemType Rod has been removed"),
        ItemSubType::Wand(subtype) => wand::to_usize(subtype),
        ItemSubType::Scroll1(subtype) => scroll::to_usize(subtype),
        ItemSubType::Scroll2(_) => panic!("ItemType Scroll2 has been removed"),
        ItemSubType::Potion1(subtype) => potion::to_usize(subtype),
        ItemSubType::Potion2(_) => panic!("ItemType Potion2 has been removed"),
        ItemSubType::FlaskOfOil(subtype) => flask_of_oil::to_usize(subtype),
        ItemSubType::Food(subtype) => food::to_usize(subtype),
        ItemSubType::JunkFood(subtype) => junk_food::to_usize(subtype),
        ItemSubType::Chime(subtype) => chime::to_usize(subtype),
        ItemSubType::Horn(subtype) => horn::to_usize(subtype),
        ItemSubType::MagicBook(subtype) => magic_book::to_usize(subtype),
        ItemSubType::PrayerBook(subtype) => prayer_book::to_usize(subtype),
        ItemSubType::Instrument(subtype) => instrument::to_usize(subtype),
        ItemSubType::SongBook(subtype) => song_book::to_usize(subtype),
        ItemSubType::LodgingAtInn(subtype) => lodging_at_inn::to_usize(subtype),
    }
}

pub fn from_usize(item_type: ItemType, item_subtype: usize) -> Option<ItemSubType> {
    match item_type {
        ItemType::MiscObject => misc_item::from_usize(item_subtype).map(ItemSubType::MiscObject),
        ItemType::Chest => chest::from_usize(item_subtype).map(ItemSubType::Chest),
        ItemType::MiscUsable => misc_usable::from_usize(item_subtype).map(ItemSubType::MiscUsable),
        ItemType::Jewelry => jewelry::from_usize(item_subtype).map(ItemSubType::Jewelry),
        ItemType::Gem => gem::from_usize(item_subtype).map(ItemSubType::Gem),
        ItemType::Bag => bag::from_usize(item_subtype).map(ItemSubType::Bag),
        ItemType::WearableGem => {
            wearable_gem::from_usize(item_subtype).map(ItemSubType::WearableGem)
        }
        ItemType::SlingAmmo => sling_ammo::from_usize(item_subtype).map(ItemSubType::SlingAmmo),
        ItemType::Bolt => bolt::from_usize(item_subtype).map(ItemSubType::Bolt),
        ItemType::Arrow => arrow::from_usize(item_subtype).map(ItemSubType::Arrow),
        ItemType::Spike => spike::from_usize(item_subtype).map(ItemSubType::Spike),
        ItemType::LightSource => {
            light_source::from_usize(item_subtype).map(ItemSubType::LightSource)
        }
        ItemType::RangedWeapon => {
            ranged_weapon::from_usize(item_subtype).map(ItemSubType::RangedWeapon)
        }
        ItemType::HaftedWeapon => {
            hafted_weapon::from_usize(item_subtype).map(ItemSubType::HaftedWeapon)
        }
        ItemType::PoleArm => polearm::from_usize(item_subtype).map(ItemSubType::PoleArm),
        ItemType::Dagger => dagger::from_usize(item_subtype).map(ItemSubType::Dagger),
        ItemType::Sword => sword::from_usize(item_subtype).map(ItemSubType::Sword),
        ItemType::Pick => pick::from_usize(item_subtype).map(ItemSubType::Pick),
        ItemType::Maul => maul::from_usize(item_subtype).map(ItemSubType::Maul),
        ItemType::GemHelm => panic!("ItemType Gem Helm has been removed"),
        ItemType::Boots => boots::from_usize(item_subtype).map(ItemSubType::Boots),
        ItemType::Gloves => gloves::from_usize(item_subtype).map(ItemSubType::Gloves),
        ItemType::Cloak => cloak::from_usize(item_subtype).map(ItemSubType::Cloak),
        ItemType::Helm => helm::from_usize(item_subtype).map(ItemSubType::Helm),
        ItemType::Shield => shield::from_usize(item_subtype).map(ItemSubType::Shield),
        ItemType::HardArmor => hard_armor::from_usize(item_subtype).map(ItemSubType::HardArmor),
        ItemType::SoftArmor => soft_armor::from_usize(item_subtype).map(ItemSubType::SoftArmor),
        ItemType::Bracers => bracers::from_usize(item_subtype).map(ItemSubType::Bracers),
        ItemType::Belt => belt::from_usize(item_subtype).map(ItemSubType::Belt),
        ItemType::Amulet => amulet::from_usize(item_subtype).map(ItemSubType::Amulet),
        ItemType::Ring => ring::from_usize(item_subtype).map(ItemSubType::Ring),
        ItemType::Staff => staff::from_usize(item_subtype).map(ItemSubType::Staff),
        ItemType::Rod => panic!("ItemType Rod has been removed"),
        ItemType::Wand => wand::from_usize(item_subtype).map(ItemSubType::Wand),
        ItemType::Scroll1 => scroll::from_usize(item_subtype).map(ItemSubType::Scroll1),
        ItemType::Scroll2 => panic!("ItemType Scroll2 has been removed"),
        ItemType::Potion1 => potion::from_usize(item_subtype).map(ItemSubType::Potion1),
        ItemType::Potion2 => panic!("ItemType Potion2 has been removed"),
        ItemType::FlaskOfOil => flask_of_oil::from_usize(item_subtype).map(ItemSubType::FlaskOfOil),
        ItemType::Food => food::from_usize(item_subtype).map(ItemSubType::Food),
        ItemType::JunkFood => junk_food::from_usize(item_subtype).map(ItemSubType::JunkFood),
        ItemType::Chime => chime::from_usize(item_subtype).map(ItemSubType::Chime),
        ItemType::Horn => horn::from_usize(item_subtype).map(ItemSubType::Horn),
        ItemType::MagicBook => magic_book::from_usize(item_subtype).map(ItemSubType::MagicBook),
        ItemType::PrayerBook => prayer_book::from_usize(item_subtype).map(ItemSubType::PrayerBook),
        ItemType::Instrument => instrument::from_usize(item_subtype).map(ItemSubType::Instrument),
        ItemType::SongBook => song_book::from_usize(item_subtype).map(ItemSubType::SongBook),
        ItemType::LodgingAtInn => {
            lodging_at_inn::from_usize(item_subtype).map(ItemSubType::LodgingAtInn)
        }
        _ => panic!("Unhandled item type {:?}", item_type),
    }
}
