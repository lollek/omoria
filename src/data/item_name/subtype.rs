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
pub mod ranged_weapon;
pub mod shield;
pub mod small_armor;
pub mod spike;
pub mod wand;
pub mod wearable_gem;

fn subtype_name<'a>(item: &Item) -> Cow<'a, str> {
    let plural_s = || if item.number == 1 { "" } else { "s" };

    match item.item_type() {
        ItemType::Ring => {
            let attribute = if item.is_identified() {
                match item.subval {
                    1 => " of gain strength",
                    2 => " of gain dexterity",
                    3 => " of gain constitution",
                    4 => " of gain intelligence",
                    7 => " of speed",
                    8 => " of searching",
                    9 => " of teleportation",
                    10 => " of slow digestion",
                    11 => " of resist fire",
                    12 => " of resist cold",
                    13 => " of feather falling",
                    14 => " of adornment",
                    15 => " of adornment",
                    16 => " of weakness",
                    17 => " of lordly protection (FIRE)",
                    18 => " of lordly protection (ACID)",
                    19 => " of lordly protection (COLD)",
                    20 => " of WOE",
                    21 => " of stupidity",
                    22 => " of increase damage",
                    23 => " of increase to-hit",
                    24 => " of protection",
                    25 => " of aggravate monster",
                    26 => " of see invisible",
                    27 => " of sustain strength",
                    28 => " of sustain intelligence",
                    29 => " of sustain wisdom",
                    30 => " of sustain constitution",
                    31 => " of sustain dexterity",
                    32 => " of sustain charisma",
                    33 => " of slaying",
                    34 => " of gnomekind",
                    35 => " of speed",
                    _ => " of ???",
                }
            } else {
                ""
            };
            Cow::from(format!("%R Ring{}", attribute))
        }
        ItemType::Chime => {
            let attribute = if item.is_identified() {
                match item.subval {
                    1 => " of light",
                    2 => " of detect doors/stairs",
                    3 => " of detect traps",
                    4 => " of teleportation",
                    5 => " of thunderblasts",
                    6 => " of summon monster",
                    7 => " of disarming",
                    8 => " of aggravation",
                    9 => " of slow monster",
                    10 => " of soothe monster",
                    11 => " of cure light wound",
                    12 => " of changing",
                    13 => " of remove curse",
                    14 => " of curing",
                    15 => " of dispel evil",
                    16 => " of darkness",
                    _ => " of ???",
                }
            } else {
                ""
            };
            Cow::from(format!("%M chime{}", attribute))
        }
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
        ItemType::Scroll1 => {
            let attribute = if item.is_identified() {
                match item.subval {
                    257 => " of Enchant Weapon To-Hit",
                    258 => " of Enchant Weapon To-Dam",
                    259 => " of Enchant Armor",
                    260 => " of Identify",
                    261 => " of Remove Curse",
                    262 => " of Light",
                    263 => " of Summon Monster",
                    264 => " of Phase Door",
                    265 => " of Teleport",
                    266 => " of Teleport Level",
                    267 => " of Monster Confusion",
                    268 => " of Magic Mapping",
                    269 => " of Sleep Monster",
                    270 => " of Rune of Protection",
                    271 => " of Treasure Detection",
                    272 => " of Object Detection",
                    273 => " of Trap Detection",
                    274 => " of Door/Stair Location",
                    275 => " of Mass Genocide",
                    276 => " of Detect Invisible",
                    277 => " of Aggravate Monster",
                    278 => " of Trap Creation",
                    279 => " of Trap/Door Destruction",
                    280 => " of Door Creation",
                    281 => " of Recharging",
                    282 => " of Genocide",
                    283 => " of Darkness",
                    284 => " of Protection from Evil",
                    285 => " of Create Food",
                    286 => " of Dispel Undead",
                    /* 257 => " of *Enchant Weapon*",
                    258 => " of Curse Weapon",
                    259 => " of *Enchant Armor*",
                    260 => " of Curse Armor",
                    261 => " of Summon Undead",
                    262 => " of Blessing",
                    263 => " of Holy Chant",
                    264 => " of Holy Prayer",
                    265 => " of Word-of-Recall",
                    266 => " of *Destruction*",
                    267 => " of Wishing",
                    268 => " of Feign Death",
                    269 => " of Make Munchies", */
                    _ => "of ??",
                }
            } else {
                ""
            };
            Cow::from(format!("Scroll{}{}", plural_s(), attribute))
        }
        ItemType::Potion1 => {
            let material = match item.subval {
                281 => "icky green ",
                282 => "light brown ",
                283 => "clear ",
                _ => "",
            };
            let attribute = if item.is_identified() {
                match item.subval {
                    257 => " of gain strength",
                    258 => " of poison",
                    259 => " of restore strength",
                    260 => " of gain intelligence",
                    261 => " of lose intelligence",
                    262 => " of restore intelligence",
                    263 => " of gain wisdom",
                    264 => " of lose wisdom",
                    265 => " of restore wisdom",
                    266 => " of charisma",
                    267 => " of ugliness",
                    268 => " of restore charisma",
                    269 => " of cure light wounds",
                    270 => " of cure serious Wounds",
                    271 => " of cure critical Wounds",
                    272 => " of healing",
                    273 => " of gain constitution",
                    274 => " of gain experience",
                    275 => " of sleep",
                    276 => " of blindness",
                    277 => " of confusion",
                    278 => " of poison",
                    279 => " of haste item",
                    280 => " of slowness",
                    281 => " of slime mold juice",
                    282 => " of apple juice",
                    283 => " of water",
                    284 => " of gain dexterity",
                    285 => " of restore dexterity",
                    286 => " of restore constitution",
                    287 => " of learning",
                    288 => " of lose memories",
                    289 => " of salt water",
                    290 => " of invulnerability",
                    291 => " of heroism",
                    292 => " of super heroism",
                    293 => " of boldliness",
                    294 => " of restore life Levels",
                    295 => " of resist heat",
                    296 => " of resist cold",
                    297 => " of detect invisible",
                    298 => " of slow poison",
                    299 => " of neutralize poison",
                    300 => " of restore mana",
                    301 => " of infra-vision",
                    302 => " of flea bile",
                    _ => " of ???",
                }
            } else {
                ""
            };
            Cow::from(format!("{}potion{}{}", material, plural_s(), attribute))
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
