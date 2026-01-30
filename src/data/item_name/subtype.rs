use std::borrow::Cow;
use crate::conversion::item_subtype;
use crate::data;
use crate::data::item_name::helpers::{maybe_armor_bonus, attack_bonus, damage, maybe_number_of, plural_s};
use crate::model::{Item, ItemType};
use crate::model::item_subtype::{ItemSubType, LodgingAtInnSubType};

pub mod ammo;
pub mod amulet;
pub mod armor;
pub mod bag;
pub mod book;
pub mod chest;
pub mod chime;
pub mod flask;
pub mod food;
pub mod gem;
pub mod gem_helm;
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
pub mod wand;
pub mod wearable_gem;

fn subtype_name<'a>(item: &Item) -> Cow<'a, str> {
    let plural_s = || if item.number == 1 { "" } else { "s" };

    match item.item_type() {
        ItemType::Horn => {
            let attribute = if item.is_identified() {
                match item.subval {
                    1 => " of bubbles",
                    2 => " of calling",
                    3 => " of soft sounds",
                    4 => " of *Blasting*",
                    5 => " of cold",
                    6 => " of heat",
                    7 => " of gas",
                    8 => " of recall",
                    9 => " of *Chaos*",
                    10 => " of glue",
                    11 => " of valhalla",
                    12 => " of tritons",
                    13 => " of fog",
                    _ => " of ???H",
                }
            } else {
                ""
            };
            Cow::from(format!("%H{}", attribute))
        }
        ItemType::FlaskOfOil => Cow::from(format!("flask{} of oil", plural_s())),
        ItemType::Staff => {
            let attribute = if item.is_identified() {
                match item.subval {
                    1 => " of Light",
                    2 => " of Door/Stair Location",
                    3 => " of Trap Location",
                    4 => " of Treasure Location",
                    5 => " of Object Location",
                    6 => " of Teleportation",
                    7 => " of Earthquakes",
                    8 => " of Summoning",
                    10 => " of *Destruction*",
                    11 => " of Starlite",
                    12 => " of Haste Monsters",
                    13 => " of Slow Monsters",
                    14 => " of Sleep Monsters",
                    15 => " of Cure Light Wounds",
                    16 => " of Detect Invisible",
                    17 => " of Speed",
                    18 => " of Slowness",
                    19 => " of Mass Polymorph",
                    20 => " of Remove Curse",
                    21 => " of Detect Evil",
                    22 => " of Curing",
                    23 => " of Dispel Evil",
                    25 => " of Darkness",
                    26 => " of Identify",
                    _ => "of ???",
                }
            } else {
                ""
            };
            let charges = if item.is_identified() {
                " (%P1 charges)"
            } else {
                ""
            };
            Cow::from(format!("%W wand{}{}", attribute, charges))
        }
        _ => Cow::from(format!("Something alien (tval {})", item.tval)),
    }
}

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

pub fn generic_item(item: &Item) -> String {
    let mut parts = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }
    parts.push(subtype_name(item));
    if item.item_type() == ItemType::LightSource {
        let plural_s = if item.p1 == 0 { "" } else { "s" };
        parts.push(Cow::from(format!(
            " with {} turn{} of light",
            item.p1, plural_s
        )));
    }

    if data::item_type::has_damage(&item.item_type()) {
        parts.push(damage(item));
    }
    if data::item_type::has_attack_enhancement(&item.item_type()) && item.is_identified() {
        parts.push(attack_bonus(item));
    }
    if let Some(armor_string) = maybe_armor_bonus(item) {
        parts.push(armor_string);
    }
    parts.join("")
}
