use std::borrow::Cow;
use crate::data;
use crate::data::item_name::helpers::{maybe_armor_bonus, attack_bonus, damage, maybe_number_of};
use crate::model::{Item, ItemType};

pub mod ammo;
pub mod amulet;
pub mod armor;
pub mod bag;
pub mod chest;
pub mod gem;
pub mod jewelry;
pub mod light_source;
pub mod melee_weapon;
pub mod misc_object;
pub mod misc_usable;
pub mod ranged_weapon;
pub mod small_armor;
pub mod spike;
pub mod wand;
pub mod wearable_gem;

fn subtype_name<'a>(item: &Item) -> Cow<'a, str> {
    let plural_s = || if item.number == 1 { "" } else { "s" };

    match item.item_type() {
        ItemType::Food => {
            if 255 < item.subval && item.subval < 300 {
                // Mushrooms
                let attribute = match item.subval {
                    256 => "",
                    257 => " of poison",
                    258 => " of blindness",
                    259 => " of paranoia",
                    260 => " of confusion",
                    261 => " of hallucination",
                    262 => " of cure poison",
                    263 => " of cure blindness",
                    264 => " of cure paranoia",
                    265 => " of cure confusion",
                    266 => " of weakness",
                    267 => " of unhealth",
                    268 => " of restore constitution",
                    269 => " of first aid",
                    270 => " of minor cures",
                    271 => " of light cures",
                    272 => " of restoring",
                    273 => " of poison",
                    274 => " of hallucination",
                    275 => " of cure poison",
                    276 => " of unhealth",
                    277 => " of cure serious wounds",
                    _ => "of ???",
                };
                Cow::from(format!(
                    "%M mushroom{}{}",
                    plural_s(),
                    if item.is_identified() { attribute } else { "" }
                ))
            } else {
                Cow::from(match item.subval {
                    307 => format!("ration{} of food", plural_s()),
                    309 => format!("hard biscuit{}", plural_s()),
                    310 => format!("strip{} of beef jerky", plural_s()),
                    311 => format!("pint{} of fine ale", plural_s()),
                    312 => format!("pint{} of fine wine", plural_s()),
                    313 => format!("piece{} of elvish waybread", plural_s()),
                    314 => format!("stew{}", plural_s()),
                    315 => format!("green jelly{}", plural_s()),
                    316 => format!("handful{} of berries (poisonous)", plural_s()),
                    317 => format!("handful{} of berries (smurfberries)", plural_s()),
                    319 => format!("eyeball{} of Ned", plural_s()),
                    252 => format!("pint{} of fine grade mush", plural_s()),
                    _ => "alien food".to_string(),
                })
            }
        }
        ItemType::GemHelm => {
            let material = match item.subval {
                9 => "Iron helm",
                10 => "Steel helm",
                _ => "Alien helm",
            };
            Cow::from(format!(
                "{}{}",
                material,
                if item.is_identified() { "of gems" } else { "" }
            ))
        }
        ItemType::Shield => Cow::from(match item.subval {
            1 => "Small leather shield",
            2 => "Medium leather shield",
            3 => "Large leather shield",
            4 => "Small metal shield",
            5 => "Medium metal shield",
            6 => "Large metal shield",
            _ => "Alien shield",
        }),
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
        ItemType::Instrument => Cow::from(match item.subval {
            258 => "Pipes of Peace",
            259 => "Lyre of Nature",
            260 => "Lute of the Woods",
            261 => "Harp of the Druids",
            _ => "Alien instrument",
        }),
        ItemType::SongBook => Cow::from(match item.subval {
            262 => "Book of Bard Lyrics [Beginners Handbook]",
            263 => "Songs of Charming [Song Book I]",
            264 => "Ballads of Knowledge [Song Book II]",
            265 => "Epics of the Bards [Greater Song Book]",
            _ => "Alien book",
        }),
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
                281 => "Icky green",
                282 => "Light brown",
                283 => "Clear",
                _ => "%C",
            };
            let attribute = if item.is_identified() {
                match item.subval {
                    257 => " of Gain Strength",
                    258 => " of Poison",
                    259 => " of Restore Strength",
                    260 => " of Gain Intelligence",
                    261 => " of Lose Intelligence",
                    262 => " of Restore Intelligence",
                    263 => " of Gain Wisdom",
                    264 => " of Lose Wisdom",
                    265 => " of Restore Wisdom",
                    266 => " of Charisma",
                    267 => " of Ugliness",
                    268 => " of Restore Charisma",
                    269 => " of Cure Light Wounds",
                    270 => " of Cure Serious Wounds",
                    271 => " of Cure Critical Wounds",
                    272 => " of Healing",
                    273 => " of Gain Constitution",
                    274 => " of Gain Experience",
                    275 => " of Sleep",
                    276 => " of Blindness",
                    277 => " of Confusion",
                    278 => " of Poison",
                    279 => " of Haste item",
                    280 => " of Slowness",
                    281 => " of Slime Mold Juice",
                    282 => " of Apple Juice",
                    283 => " of Water",
                    284 => " of Gain Dexterity",
                    285 => " of Restore Dexterity",
                    286 => " of Restore Constitution",
                    287 => " of Learning",
                    288 => " of Lose Memories",
                    289 => " of Salt Water",
                    290 => " of Invulnerability",
                    291 => " of Heroism",
                    292 => " of Super Heroism",
                    293 => " of Boldliness",
                    294 => " of Restore Life Levels",
                    295 => " of Resist Heat",
                    296 => " of Resist Cold",
                    297 => " of Detect Invisible",
                    298 => " of Slow Poison",
                    299 => " of Neutralize Poison",
                    300 => " of Restore Mana",
                    301 => " of Infra-Vision",
                    302 => " of Flea Bile",
                    _ => " of ???",
                }
            } else {
                ""
            };
            Cow::from(format!("{} potion{}{}", material, plural_s(), attribute))
        }
        ItemType::FlaskOfOil => Cow::from(format!("Flask{} of Oil", plural_s())),
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
        ItemType::MagicBook => {
            let name = if item.is_identified() {
                match item.subval {
                    257 => " of Magic Spells [Beginners-Magik]",
                    258 => " of Magic Spells [Magik I]",
                    259 => " of Magic Spells [Magik II]",
                    261 => " of Magic Spells [The Mages Guide to Power]",
                    _ => " of ???",
                }
            } else {
                ""
            };
            Cow::from(format!("Book{}{}", plural_s(), name))
        }
        ItemType::PrayerBook => {
            let name = if item.is_identified() {
                match item.subval {
                    258 => " of Prayers [Beginners Handbook]",
                    259 => " of Prayers [Words of Wisdom]",
                    260 => " of Prayers [Chants and Blessings]",
                    261 => " of Prayers [Exorcism and Dispelling]",
                    _ => " of ???",
                }
            } else {
                ""
            };
            Cow::from(format!("Holy Book{}{}", plural_s(), name))
        }
        _ => Cow::from("Something alien"),
    }
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
