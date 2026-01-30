use crate::conversion::item_subtype;
use crate::data::item_name::helpers::{maybe_number_of, plural_s};
use crate::model::Item;
use crate::model::ItemType;
use crate::model::item_subtype::{ItemSubType, LodgingAtInnSubType};

pub mod ammo;
pub mod amulet;
pub mod armor;
pub mod bag;
pub mod book;
pub mod chest;
pub mod chime;
pub mod dungeon_feature;
pub mod flask;
pub mod food;
pub mod gem;
pub mod gem_helm;
pub mod horn;
pub mod instrument;
pub mod jewelry;
pub mod light_source;
pub mod melee_weapon;
pub mod misc_object;
pub mod misc_usable;
pub mod potion;
pub mod ranged_weapon;
pub mod scroll;
pub mod ring;
pub mod shield;
pub mod small_armor;
pub mod spike;
pub mod staff;
pub mod wand;
pub mod wearable_gem;

pub fn lodging_at_inn(item: &Item) -> String {
    let ItemSubType::LodgingAtInn(subtype) =
        item_subtype::from_i64(ItemType::LodgingAtInn, item.subval)
            .unwrap_or_else(|| panic!("Invalid item subtype for LodgingAtInn: {:?}", item)) else {
        panic!("Invalid item subtype for LodgingAtInn: {:?}", item)
    };

    match subtype {
        LodgingAtInnSubType::LodgingForOneDay => "one day of lodging",
        LodgingAtInnSubType::LodgingForThreeDays => "three days of lodging",
        LodgingAtInnSubType::LodgingForOneWeek => "one week of lodging",
        LodgingAtInnSubType::RoomAndBoardForOneDay => "room and board for one day",
    }.to_string()
}

pub fn money(item: &Item) -> String {
    let mut parts = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }
    parts.push(format!("copper piece{}", plural_s(item)).into());
    parts.join("")
}