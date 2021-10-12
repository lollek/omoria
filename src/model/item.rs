use std::borrow::Cow;
use model::ItemType;
use misc;
use libc;
use model::{ Damage, Name };

use thirdparty::serde::BigArray;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
// For more info. Se item_guide.txt
// TODO: tval + subval needs bit for is_identified
pub struct Item { // treasure_type
    // Object name. See below for rules on names.
    #[serde(with = "BigArray")]
    pub name: Name,

    // Object type. Literally, is what kind of object it is.
    pub tval: libc::uint8_t,

    // Flags define an item's properties. the meaning of flag values can
    // depend on the item's tval.  note that the original game designers
    // decided to use bitwise flags (a good design), but came up with more
    // than 32 different properties for some item types (e.g.  potions)
    // and so they had to give items a second flag member to hold them all.
    pub flags2: libc::uint64_t,
    pub flags: libc::uint64_t,

    // A catch-all member for associating some value of interest
    // with a particular item.  e.g. for wands, staves, gems,
    // it's the number of charges, while for certain magical
    // weapons, it can be a damage multiplier
    pub p1: libc::int64_t,

    // The item's nominal cost in iron pieces.
    pub cost: libc::int64_t,

    // A sub-category value.  the semantics of subval also depend
    // in part on the value of tval.
    pub subval: libc::int64_t,

    // Weight in some ill-defined unit, possibly gold pieces, but
    // possibly in fifths of a gold piece.
    pub weight: libc::uint16_t,

    // for countable items (arrows, scrolls, etc), how many of
    // them you have.
    pub number: libc::uint16_t,

    // the next four all apply to items that can be worn or wielded (i.e. weapons
    // and armor)
    pub tohit: libc::int16_t,       // An item's magical + to hit
    pub todam: libc::int16_t,       // An item's magical + to damage
    pub ac: libc::int16_t,          // An item's normal armor class value.
    pub toac: libc::int16_t,        // An item's magical + to AC.

    // the amount of damage an item does to monster.  everything
    // should have a damage value, even if it's just "0d0".
    pub damage: Damage,

    // a vague measurement of how strong an item's magic is.
    pub level: libc::int8_t,

    // is this item identified?
    pub identified: libc::uint8_t,
}

impl Item {
    fn number_of_string<'a>(&self) -> Cow<'a, str> {
        match self.number {
            0 => Cow::from("no more "),
            1 => Cow::from(""),
            _ => Cow::from(self.number.to_string() + " "),
        }
    }

    fn subtype_name<'a>(&self) -> Cow<'a, str> {
        let plural_s = || if self.number == 1 { "s" } else { "" };

        match self.item_type() {
            ItemType::Food =>
                if 255 < self.subval && self.subval < 300 {
                    // Mushrooms
                    let attribute = match self.subval {
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
                    Cow::from(
                        format!("%M mushroom{}{}", plural_s(), if self.is_identified() {
                            attribute } else {""} ))
                } else {
                Cow::from(match self.subval {
                    307 => format!("Ration{} of food", plural_s()),
                    309 => format!("Hard biscuit{}", plural_s()),
                    310 => format!("Strip{} of beef jerky", plural_s()),
                    311 => format!("Pint{} of fine ale", plural_s()),
                    312 => format!("Pint{} of fine wine", plural_s()),
                    313 => format!("Piece{} of elvish waybread", plural_s()),
                    314 => format!("Stew{}", plural_s()),
                    315 => format!("Green jelly{}", plural_s()),
                    316 => format!("Handful{} of berries (poisonous)", plural_s()),
                    317 => format!("Handful{} of berries (smurfberries)", plural_s()),
                    319 => format!("Eyeball{} of Ned", plural_s()),
                    252 => format!("Pint{} of fine grade mush", plural_s()),
                    _ => "Alien food".to_string(),
                })
                },
            ItemType::Dagger =>
                Cow::from(match self.subval {
                    1 => "Main gauche",
                    2 => "Misercorde",
                    3 => "Stiletto",
                    4 => "Bodkin",
                    5 => "Broken dagger",
                    8 => "Bilbo",
                    9 => "Baselard",
                    16 => "Foil",
                    20 => "Rapier",
                    22 => "Small sword",
                    //5 => "Cat-O-Nine Tails",
                    _ => "Alien dagger",
                }),
            ItemType::Sword =>
                Cow::from(match self.subval {
                    6 => "Backsword",
                    7 => "Bastard sword",
                    10 => "Broadsword",
                    11 => "Claymore",
                    12 => "Cutlass",
                    13 => "Espadon",
                    14 => "Executioner's sword",
                    15 => "Flamberge",
                    17 => "Katana",
                    18 => "Longsword",
                    19 => "No-dachi",
                    21 => "Sabre",
                    23 => "Zweihander",
                    24 => "Broken sword",
                    _ => "Alien sword",
                }),
                ItemType::HaftedWeapon =>
                    Cow::from(match self.subval {
                        1 => "Balestarius",
                        3 => "Battle axe",
                        4 => "Broad axe",
                        _ => "Alien hafted weapon",
                    }),
                ItemType::Maul =>
                    Cow::from(match self.subval {
                        2 => "Ball and chain",
                        6 => "Wooden club",
                        7 => "Flail",
                        8 => "Two handed great flail",
                        9 => "Morningstar",
                        10 => "Mace",
                        11 => "War hammer",
                        12 => "Lead filled mace",
                        13 => "Iron-shod quarterstaff",
                        _ => "Alien maul",
                    }),
                ItemType::PoleArm =>
                    Cow::from(match self.subval {
                        1 => "Awl-pike",
                        2 => "Beaked axe",
                        3 => "Fauchard",
                        4 => "Glaive",
                        5 => "Halberd",
                        6 => "Lucerne hammer",
                        7 => "Pike",
                        8 => "Spear",
                        9 => "Lance",
                        10 => "Javelin",
                        _ => "Alien polearm",
                    }),
                ItemType::RangedWeapon =>
                    Cow::from(match self.subval {
                        1 => "Short bow",
                        2 => "Long bow",
                        3 => "Composite bow",
                        10 => "Light crossbow",
                        11 => "Heavy crossbow",
                        20 => "Sling",
                        _ => "Alien ranged weapon",
                    }),
                ItemType::Arrow => Cow::from(format!("Arrow{}", plural_s())),
                ItemType::Bolt => Cow::from(format!("Bolt{}", plural_s())),
                ItemType::SlingAmmo => Cow::from(format!("Rounded pebble{}", plural_s())),
                ItemType::Spike => Cow::from(format!("Iron spike{}", plural_s())),
                ItemType::LightSource =>
                    match self.subval {
                        1 =>  Cow::from("Brass lantern"),
                        2 =>  Cow::from("Brass lantern"),
                        13 => Cow::from("Wooden lantern"),
                        14 => Cow::from("Wooden torch"),
                        15 => Cow::from("Magic torch"),
                        17 => Cow::from("Magic lantern"),
                        _ => Cow::from("Alien lightsource"),
                    },
                ItemType::Pick =>
                    Cow::from(match self.subval {
                        1 => "Pick",
                        2 => "Shovel",
                        //2 => "Orcish Pick",
                        3 => "Dwarven pick",
                        5 => "Gnomish shovel",
                        6 => "Dwarven shovel",
                        7 => "Orcish pick",
                        _ => "Alien pick",
                    }),
                ItemType::Boots =>
                    Cow::from(match self.subval {
                        1 => "Pair of soft leather shoes",
                        2 => "Pair of soft leather boots",
                        3 => "Pair of hard leather boots",
                        4 => "Pair of sandals",
                        _ => "Alien boots",
                    }),
                ItemType::Helm =>
                    Cow::from(match self.subval {
                        1 => "Soft leather cap",
                        2 => "Hard leather cap",
                        3 => "Metal cap",
                        4 => "Iron helm",
                        5 => "Steel helm",
                        6 => "Silver crown",
                        7 => "Golden crown",
                        8 => "Jewel encrusted crown",
                        11 => "Cloth hat",
                        _ => "Alien helm",
                    }),
                ItemType::GemHelm => {
                    let material = match self.subval {
                        9 => "Iron helm",
                        10 => "Steel helm",
                        _ => "Alien helm",
                    };
                    Cow::from(format!("{}{}", material,
                                      if self.is_identified() {
                                          "of gems" } else { "" }))
                },
                ItemType::WearableGem => {
                    let attribute = match self.subval {
                        1 => "of teleportation",
                        2 => "of resist cold",
                        3 => "of resist acid",
                        4 => "of see invisible",
                        5 => "of stealth",
                        6 => "of slow digestation",
                        7 => "of lordly protection (FIRE)",
                        _ => "of ???",
                    };
                    Cow::from(format!("Finely cut %R{}", if self.is_identified() {
                        attribute} else {""}))
                },
                ItemType::SoftArmor =>
                    Cow::from(match self.subval {
                        1 => "Robe",
                        2 => "Soft leather armor",
                        3 => "Soft studded leather",
                        4 => "Hard leather armor",
                        5 => "Hard studded leather",
                        6 => "Woven cord armor",
                        7 => "Soft leather ring mail",
                        8 => "Hard leather ring mail",
                        9 => "Leather scale mail",
                        10 => "Leather bridantine armor",
                        11 => "Cool set of threads",
                        12 => "Filthy naga hide armor",
                        13 => "Elven chain mail",
                        99 => "Some filthy rags",
                        _ => "Alien soft armor",
                    }),
                ItemType::HardArmor =>
                    Cow::from(match self.subval {
                        1 => "Metal scale mail",
                        2 => "Chain mail",
                        3 => "Rusty chain mail",
                        4 => "Double chain mail",
                        5 => "Augmented chain mail",
                        6 => "Bar chain mail",
                        7 => "Metal brindandine armor",
                        8 => "Laminated armor",
                        9 => "Partial plate armor",
                        10 => "Metal lamellar armor",
                        11 => "Full plate armor",
                        12 => "Ribbed plate armor",
                        13 => "Bronze plate mail",
                        14 => "Stone plate armor",
                        15 => "Mithril chain mail",
                        16 => "Mithril plate armor",
                        _ => "Alien hard armor",
                    }),
                ItemType::Cloak => Cow::from("Cloak"),
                ItemType::Gloves =>
                    Cow::from(match self.subval {
                        1 => "Set of leather gloves",
                        2 => "Set of gauntlets",
                        5 => "Set of cloth gloves",
                        _ => "Alien gloves",
                    }),
                ItemType::Bracers =>
                    match self.subval {
                        1 => Cow::from(
                            format!("Set of bracers{}", if self.is_identified() {
                                        " of protection" } else { "" })),
                        2 => Cow::from(
                            format!("Set of bracers{}", if self.is_identified() {
                                        " of defence" } else { "" })),
                        3 => Cow::from(
                            format!("Set of bracers{}", if self.is_identified() {
                                        " of shielding" } else { "" })),
                        4 => Cow::from("Set of mithril bracers"),
                        5 => Cow::from("Set of adamantite bracers"),
                        6 => Cow::from(
                            format!("Set of bracers{}", if self.is_identified() {
                                        " of weapon attraction" } else { "" })),
                        30 => Cow::from("Small silver bracelet"),
                        31 => Cow::from(
                            format!("Small silver bracelet{}", if self.is_identified() {
                                        " of warding" } else { "" })),
                        40 => Cow::from("Small gold bracelet"),
                        50 => Cow::from("Small platinum bracelet"),
                        _ => Cow::from("Alien bracers"),
                    },
                ItemType::Belt =>
                    Cow::from(match self.subval {
                        1 => "Girdle",
                        10 => "Silver belt buckle",
                        11 => "Gold belt buckle",
                        13 => "Leather belt",
                        _ => "Alien belt",
                    }),
                ItemType::Shield =>
                    Cow::from(match self.subval {
                        1 => "Small leather shield",
                        2 => "Medium leather shield",
                        3 => "Large leather shield",
                        4 => "Small metal shield",
                        5 => "Medium metal shield",
                        6 => "Large metal shield",
                        _ => "Alien shield",
                    }),
                ItemType::Ring => {
                    let attribute = if self.is_identified() {
                        match self.subval {
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
                },
                ItemType::Amulet =>
                    match self.subval {
                        5 => Cow::from(
                            format!("%A amulet{}", if self.is_identified() {
                                "of wisdom"} else {""})),
                        6 => Cow::from(
                            format!("%A amulet{}", if self.is_identified() {
                                "of charisma"} else {""})),
                        7 => Cow::from(
                            format!("%A amulet{}", if self.is_identified() {
                                "of searching"} else {""})),
                        8 => Cow::from(
                            format!("%A amulet{}", if self.is_identified() {
                                "of teleportation"} else {""})),
                        9 => Cow::from(
                            format!("%A amulet{}", if self.is_identified() {
                                "of slow digestation"} else {""})),
                        10 => Cow::from(
                            format!("%A amulet{}", if self.is_identified() {
                                "of resist acid"} else {""})),
                        11 => Cow::from(
                            format!("%A amulet{}", if self.is_identified() {
                                "of adornment"} else {""})),
                        12 => Cow::from(
                            format!("%A amulet{}", if self.is_identified() {
                                "of adornment"} else {""})),
                        13 => Cow::from(
                            format!("%A amulet{}", if self.is_identified() {
                                "of the magi"} else {""})),
                        14 => Cow::from(
                            format!("%A amulet{}", if self.is_identified() {
                                "of DOOM"} else {""})),
                        30 => Cow::from("Finely wrought silver necklace"),
                        40 => Cow::from("Finely wrought gold necklace"),
                        60 => Cow::from("Finely wrought mithril necklace"),
                        _ => Cow::from("%A Alien amulet"),
                    },
                ItemType::MiscUsable =>
                    Cow::from(match self.subval {
                        14 => "%A Statue",
                        15 => "Broken set of teeth",
                        16 => "Silver cross",
                        17 => "Gold cross",
                        18 => "Mithril cross",
                        19 => "%M cross",
                        20 => "%M cross",
                        21 => "Corked bottle",
                        22 => "Holy hand grenade of Antioch",
                        _ => "Alien thing",
                    }),
                ItemType::Chime => {
                    let attribute = if self.is_identified() {
                        match self.subval {
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
                },
                ItemType::Horn => {
                    let attribute = if self.is_identified() {
                        match self.subval {
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
                },
                ItemType::Instrument =>
                    Cow::from(match self.subval {
                        258 => "Pipes of Peace",
                        259 => "Lyre of Nature",
                        260 => "Lute of the Woods",
                        261 => "Harp of the Druids",
                        _ => "Alien instrument",
                    }),
                ItemType::SongBook =>
                    Cow::from(match self.subval {
                        262 => "Book of Bard Lyrics [Beginners Handbook]",
                        263 => "Songs of Charming [Song Book I]",
                        264 => "Ballads of Knowledge [Song Book II]",
                        265 => "Epics of the Bards [Greater Song Book]",
                        _ => "Alien book",
                    }),
                ItemType::Scroll1 => {
                    let attribute = if self.is_identified() {
                        match self.subval {
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
                },
                ItemType::Potion1 => {
                    let material = match self.subval {
                        281 => "Icky green",
                        282 => "Light brown",
                        283 => "Clear",
                        _ => "%C",
                    };
                    let attribute = if self.is_identified() {
                        match self.subval {
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
                            279 => " of Haste Self",
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
                },
                ItemType::FlaskOfOil => Cow::from(format!("Flask{} of Oil", plural_s())),
                ItemType::Wand => {
                    let attribute = if self.is_identified() {
                        match self.subval {
                            1 => " of Light",
                            2 => " of Lightning Bolts",
                            3 => " of Frost Bolts",
                            4 => " of Fire Bolts",
                            5 => " of Stone-to-Mud",
                            6 => " of Polymorph",
                            7 => " of Heal Monster",
                            8 => " of Haste Monster",
                            9 => " of Slow Monster",
                            10 => " of Confuse Monster",
                            11 => " of Sleep Monster",
                            12 => " of Drain Life",
                            13 => " of Trap/Door destruction",
                            14 => " of Magic Missile",
                            15 => " of Wall Building",
                            16 => " of Clone Monster",
                            17 => " of Teleport Away",
                            18 => " of Disarming",
                            19 => " of Lightning Balls",
                            20 => " of Cold Balls",
                            21 => " of Fire Balls",
                            22 => " of Stinking Cloud",
                            23 => " of Acid Balls",
                            24 => " of Wonder",
                            25 => " of Probing",
                            _ => "of ???",
                        }
                    } else {
                        ""
                    };
                    let charges = if self.is_identified() {
                        " (%P1 charges)"
                    } else { "" };
                    Cow::from(format!("%M wand{}{}", attribute, charges))
                },
                ItemType::Staff => {
                    let attribute = if self.is_identified() {
                        match self.subval {
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
                    let charges = if self.is_identified() {
                        " (%P1 charges)"
                    } else { "" };
                    Cow::from(format!("%W wand{}{}", attribute, charges))
                },
                ItemType::MagicBook => {
                    let name = if self.is_identified() {
                        match self.subval {
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
                },
                ItemType::PrayerBook => {
                    let name = if self.is_identified() {
                        match self.subval {
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
                },
                ItemType::Chest =>
                    Cow::from(match self.subval {
                        1 => "Small wooden chest",
                        4 => "Large wooden chest",
                        5 => "Dead human body",
                        7 => "Small iron chest",
                        10 => "Large iron chest",
                        13 => "Small steel chest",
                        16 => "Large steel chest",
                        _ => "Alien chest",
                    }),
                ItemType::Bag => {
                    let attribute = if self.is_identified() {
                        match self.subval {
                            1 => " of Holding (250)",
                            2 => " of Holding (500)",
                            3 => " of Holding (1000)",
                            //3 => "%N Bag of Holding (1500)",
                            4 => " of Devouring",
                            _ => " of ???",
                        }
                    } else { "" };
                    Cow::from(format!("%N Bag{}", attribute))
                },
                ItemType::MiscObject =>
                    Cow::from(match self.subval {
                        1 => "Rat skeleton",
                        2 => "Giant centipede skeleton",
                        4 => "Empty bottle",
                        5 => "Broken set of pottery",
                        7 => "Human skeleton",
                        8 => "Dwarf skeleton",
                        9 => "Elf skeleton",
                        10 => "Gnome skeleton",
                        11 => "Broken set of teeth",
                        12 => "Large broken bone",
                        13 => "Broken stick",
                        _ => "Alien thing",
                    }),
            _ => Cow::from("Something alien"),
        }
    }

    fn damage_string<'a>(&self) -> Cow<'a, str> {
        let raw_string = self.damage.iter().map(|&i| i as u8).collect::<Vec<u8>>();
        let damage_string = misc::c_array_to_rust_string(raw_string);
        Cow::from(format!(" ({})", damage_string))
    }

    fn attack_enchantment_string<'a>(&self) -> Cow<'a, str> {
        let tohit_sign = if self.tohit > 0 { "+" } else {""};
        let todam_sign = if self.todam > 0 { "+" } else {""};
        Cow::from(format!(" ({}{},{}{})", tohit_sign, self.tohit, todam_sign, self.todam))
    }

    fn armor_string<'a>(&self) -> Cow<'a, str> {
        if self.ac == 0 && (!self.is_identified() || self.toac == 0) {
            return Cow::from("");
        }

        if !self.is_identified() {
            return Cow::from(format!(" [{}]", self.ac));
        }

        let toac_sign = if self.toac > 0 { "+" } else {""};
        return Cow::from(format!(" [{},{}{}]", self.ac, toac_sign, self.toac))
    }

    pub fn item_type(&self) -> ItemType {
        ItemType::from(self.tval)
    }

    pub fn is_identified(&self) -> bool {
        self.identified != 0
    }

    // In progress..
    pub fn equipment_name(&self) -> String {
        let mut parts = Vec::new();
        parts.push(self.number_of_string());
        parts.push(self.subtype_name());
        if self.item_type() == ItemType::LightSource {
            let plural_s = if self.p1 == 0 { "" } else { "s" };
            parts.push(Cow::from(format!(" with {} turn{} of light", self.p1, plural_s)));
        }

        if self.item_type().has_damage() {
            parts.push(self.damage_string());
        }
        if self.item_type().has_attack_enhancement() && self.is_identified() {
            parts.push(self.attack_enchantment_string());
        }
        parts.push(self.armor_string());
        parts.join("")
    }
}
