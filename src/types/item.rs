use std::borrow::Cow;
use types::ItemType;
use misc;
use libc;

use thirdparty::serde::BigArray;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
// For more info. Se item_guide.txt
// TODO: Add bit for is_identified
// TODO: tval + subval needs bit for is_identified
pub struct Item { // treasure_type
    // Object name. See below for rules on names.
    #[serde(with = "BigArray")]
    pub name: [libc::c_char; 70],

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
    pub damage: [libc::c_char; 7],

    // a vague measurement of how strong an item's magic is.
    pub level: libc::int8_t,
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

        match ItemType::from(self.tval) {
            ItemType::Food =>
                Cow::from(match self.subval {
                    257 => format!("&M Mushroom{} of Poison", plural_s()),
                    258 => format!("%M Mushroom{} of Blindness", plural_s()),
                    259 => format!("%M Mushroom{} of Paranoia", plural_s()),
                    260 => format!("%M Mushroom{} of Confusion", plural_s()),
                    261 => format!("%M Mushroom{} of Hallucination", plural_s()),
                    262 => format!("%M Mushroom{} of Cure Poison", plural_s()),
                    263 => format!("%M Mushroom{} of Cure Blindness", plural_s()),
                    264 => format!("%M Mushroom{} of Cure Paranoia", plural_s()),
                    265 => format!("%M Mushroom{} of Cure Confusion", plural_s()),
                    266 => format!("%M Mushroom{} of Weakness", plural_s()),
                    267 => format!("%M Mushroom{} of Unhealth", plural_s()),
                    268 => format!("%M Mushroom{} of Restore Constitution", plural_s()),
                    269 => format!("%M Mushroom{} of First Aid", plural_s()),
                    270 => format!("%M Mushroom{} of Minor Cures", plural_s()),
                    271 => format!("%M Mushroom{} of Light Cures", plural_s()),
                    272 => format!("%M Mushroom{} of Restoring", plural_s()),
                    273 => format!("Hairy %M Mold{} of Poison", plural_s()),
                    274 => format!("Hairy %M Mold{} of Hallucination", plural_s()),
                    275 => format!("Hairy %M Mold{} of Cure Poison", plural_s()),
                    276 => format!("Hairy %M Mold{} of Unhealth", plural_s()),
                    277 => format!("Hairy %M Mold{} of Cure Serious Wounds", plural_s()),
                    307 => format!("Ration{} of Food", plural_s()),
                    308 => format!("Slime Mold{}", plural_s()),
                    309 => format!("Hard Biscuit{}", plural_s()),
                    310 => format!("Strip{} of Beef Jerky", plural_s()),
                    311 => format!("Pint{} of Fine Ale", plural_s()),
                    312 => format!("Pint{} of Fine Wine", plural_s()),
                    313 => format!("Piece{} of Elvish Waybread", plural_s()),
                    314 => format!("Rice-a-Roni{}", plural_s()),
                    315 => format!("Jolly Green Jelly{}", plural_s()),
                    316 => format!("Handful{} of Berries (Poisonous)", plural_s()),
                    317 => format!("Handful{} of Berries (Smurfberries)", plural_s()),
                    319 => format!("Eyeball{} of Ned", plural_s()),
                    252 => format!("Pint{} of Fine Grade Mush", plural_s()),
                    _ => "Alien Food".to_string(),
                }),
            ItemType::Dagger =>
                Cow::from(match self.subval {
                    1 => "Main Gauche",
                    2 => "Misercorde",
                    3 => "Stiletto",
                    4 => "Bodkin",
                    5 => "Broken dagger",
                    8 => "Bilbo",
                    9 => "Baselard",
                    16 => "Foil",
                    20 => "Rapier",
                    22 => "Small Sword",
                    //5 => "Cat-O-Nine Tails",
                    _ => "Alien Dagger",
                }),
            ItemType::Sword =>
                Cow::from(match self.subval {
                    6 => "Backsword",
                    7 => "Bastard Sword",
                    10 => "Broadsword",
                    11 => "Claymore",
                    12 => "Cutlass",
                    13 => "Espadon",
                    14 => "Executioner's Sword",
                    15 => "Flamberge",
                    17 => "Katana",
                    18 => "Longsword",
                    19 => "No-Dachi",
                    21 => "Sabre",
                    23 => "Zweihander",
                    24 => "Broken Sword",
                    _ => "Alien Sword",
                }),
                ItemType::HaftedWeapon =>
                    Cow::from(match self.subval {
                        1 => "Balestarius",
                        3 => "Battle Axe",
                        4 => "Broad Axe",
                        _ => "Alien Hafted Weapon",
                    }),
                ItemType::Maul =>
                    Cow::from(match self.subval {
                        2 => "Ball and Chain",
                        6 => "Wooden Club",
                        7 => "Flail",
                        8 => "Two Handed Great Flail",
                        9 => "Morningstar",
                        10 => "Mace",
                        11 => "War Hammer",
                        12 => "Lead Filled Mace",
                        13 => "Iron Shod Quarterstaff",
                        _ => "Alien Maul",
                    }),
                ItemType::PoleArm =>
                    Cow::from(match self.subval {
                        1 => "Awl-Pike",
                        2 => "Beaked Axe",
                        3 => "Fauchard",
                        4 => "Glaive",
                        5 => "Halberd",
                        6 => "Lucerne Hammer",
                        7 => "Pike",
                        8 => "Spear",
                        9 => "Lance",
                        10 => "Javelin",
                        _ => "Alien Polearm",
                    }),
                ItemType::RangedWeapon =>
                    Cow::from(match self.subval {
                        1 => "Short Bow",
                        2 => "Long Bow",
                        3 => "Composite Bow",
                        10 => "Light Crossbow",
                        11 => "Heavy Crossbow",
                        20 => "Sling",
                        _ => "Alien Ranged Weapon",
                    }),
                ItemType::Arrow => Cow::from(format!("Arrow{}", plural_s())),
                ItemType::Bolt => Cow::from(format!("Bolt{}", plural_s())),
                ItemType::SlingAmmo => Cow::from(format!("Rounded Pebble{}", plural_s())),
                ItemType::Spike => Cow::from(format!("Iron Spike{}", plural_s())),
                ItemType::LightSource =>
                    match self.subval {
                        1 =>  Cow::from(format!("Brass Lantern{}", plural_s())),
                        13 => Cow::from(format!("Wooden Lantern{}", plural_s())),
                        17 => Cow::from("Magic Lantern"),
                        15 => Cow::from("Magic Torch"),
                        _ => Cow::from("Alien Lightsource"),
                    },
                ItemType::Pick =>
                    Cow::from(match self.subval {
                        1 => "Pick",
                        2 => "Shovel",
                        //2 => "Orcish Pick",
                        3 => "Dwarven Pick",
                        5 => "Gnomish Shovel",
                        6 => "Dwarven Shovel",
                        7 => "Orcish Pick",
                        _ => "Alien Pick",
                    }),
                ItemType::Boots =>
                    Cow::from(match self.subval {
                        1 => "Pair of Soft Leather Shoes",
                        2 => "Pair of Soft Leather Boots",
                        3 => "Pair of Hard Leather Boots",
                        4 => "Pair of Sandals",
                        _ => "Alien Boots",
                    }),
                ItemType::Helm =>
                    Cow::from(match self.subval {
                        1 => "Soft Leather Cap",
                        2 => "Hard Leather Cap",
                        3 => "Metal Cap",
                        4 => "Iron Helm",
                        5 => "Steel helm",
                        6 => "Silver Crown",
                        7 => "Golden Crown",
                        8 => "Jewel Encrusted Crown",
                        11 => "Cloth Hat",
                        _ => "Alien Helm",
                    }),
                ItemType::GemHelm =>
                    Cow::from(match self.subval {
                        9 => "Iron Helm of Gems",
                        10 => "Steel Helm of Gems",
                        _ => "Alien Helm of Gems",
                    }),
                ItemType::WearableGem =>
                    // if !is_identified "Finely cut %R"
                    Cow::from(match self.subval {
                        1 => "Finely cut %R of Teleportation",
                        2 => "Finely cut %R of Resist Cold",
                        3 => "Finely cut %R of Resist Acid",
                        4 => "Finely cut %R of See Invisible",
                        5 => "Finely cut %R of Stealth",
                        6 => "Finely cut %R of Slow Digestation",
                        7 => "Finely cut %R of Lordly Protection (FIRE)",
                        _ => "Alient Gem",
                    }),
                ItemType::SoftArmor =>
                    Cow::from(match self.subval {
                        1 => "Robe",
                        2 => "Soft Leather Armor",
                        3 => "Soft Studded Leather",
                        4 => "Hard Leather Armor",
                        5 => "Hard Studded Leather",
                        6 => "Woven Cord Armor",
                        7 => "Soft Leather Ring Mail",
                        8 => "Hard Leather Ring Mail",
                        9 => "Leather Scale Mail",
                        10 => "Leather Bridantine Armor",
                        11 => "Cool Set of Threads",
                        12 => "Filthy Naga Hide Armor",
                        13 => "Elven Chain Mail",
                        99 => "Some filthy rags",
                        _ => "Alien Soft Armor",
                    }),
                ItemType::HardArmor =>
                    Cow::from(match self.subval {
                        1 => "Metal Scale Mail",
                        2 => "Chain Mail",
                        3 => "Rusty Chain Mail",
                        4 => "Double Chain Mail",
                        5 => "Augmented Chain Mail",
                        6 => "Bar Chain Mail",
                        7 => "Metal Brindandine Armor",
                        8 => "Laminated Armor",
                        9 => "Partial Plate Armor",
                        10 => "Metal Lamellar Armor",
                        11 => "Full Plate Armor",
                        12 => "Ribbed Plate Armor",
                        13 => "Bronze Plate Mail",
                        14 => "Stone Plate Armor",
                        15 => "Mithril Chain Mail",
                        16 => "Mithril Plate Armor",
                        _ => "Alien Hard Armor",
                    }),
                ItemType::Cloak => Cow::from("Cloak"),
                ItemType::Gloves =>
                    Cow::from(match self.subval {
                        1 => "Set of Leather Gloves",
                        2 => "Set of Gauntlets",
                        5 => "Set of Cloth Gloves",
                        _ => "Alien Gloves",
                    }),
                ItemType::Bracers =>
                    match self.subval {
                        // if !type_is_identified -> "Set of Bracers"
                        1 => Cow::from("Set of Bracers of Protection"),
                        2 => Cow::from("Set of Bracers of Defence"),
                        3 => Cow::from("Set of Bracers of Shielding"),
                        4 => Cow::from("Set of Mithril Bracers"),
                        5 => Cow::from("Set of Adamantite Bracers"),
                        6 => Cow::from("Set of Bracers of Weapon Attraction"),
                        7 => Cow::from("Small Silver Bracelet of Warding"),
                        264 => Cow::from(format!("Small Bronze Bracelet{}", plural_s())),
                        271 => Cow::from(format!("Small Silver Bracelet{}", plural_s())),
                        272 => Cow::from(format!("Small Gold Bracelet{}", plural_s())),
                        273 => Cow::from(format!("Small Platinum Bracelet{}", plural_s())),
                        _ => Cow::from("Alien Bracers"),
                    },
                ItemType::Belt =>
                    Cow::from(match self.subval {
                        1 => "Girdle",
                        10 => "Silver Belt Buckle",
                        11 => "Gold Belt Buckle",
                        13 => "Leather Belt",
                        _ => "Alien Belt",
                    }),
                ItemType::Shield =>
                    Cow::from(match self.subval {
                        1 => "Small Leather Shield",
                        2 => "Medium Leather Shield",
                        3 => "Large Leather Shield",
                        4 => "Small Metal Shield",
                        5 => "Medium Metal Shield",
                        6 => "Large Metal Shield",
                        _ => "Alien Shield",
                    }),
                ItemType::Ring =>
                    Cow::from(match self.subval {
                        // !is_identified -> %R Ring
                        1 => "%R Ring of Gain Strength",
                        2 => "%R Ring of Gain Dexterity",
                        3 => "%R Ring of Gain Constitution",
                        4 => "%R Ring of Gain Intelligence",
                        7 => "%R Ring of Speed",
                        8 => "%R Ring of Searching",
                        9 => "%R Ring of Teleportation",
                        10 => "%R Ring of Slow Digestion",
                        11 => "%R Ring of Resist Fire",
                        12 => "%R Ring of Resist Cold",
                        13 => "%R Ring of Feather Falling",
                        14 => "%R Ring of Adornment",
                        15 => "%R Ring of Adornment",
                        16 => "%R Ring of Weakness",
                        17 => "%R Ring of Lordly Protection (FIRE)",
                        18 => "%R Ring of Lordly Protection (ACID)",
                        19 => "%R Ring of Lordly Protection (COLD)",
                        20 => "%R Ring of WOE",
                        21 => "%R Ring of Stupidity",
                        22 => "%R Ring of Increase Damage",
                        23 => "%R Ring of Increase To-Hit",
                        24 => "%R Ring of Protection",
                        25 => "%R Ring of Aggravate Monster",
                        26 => "%R Ring of See Invisible",
                        27 => "%R Ring of Sustain Strength",
                        28 => "%R Ring of Sustain Intelligence",
                        29 => "%R Ring of Sustain Wisdom",
                        30 => "%R Ring of Sustain Constitution",
                        31 => "%R Ring of Sustain Dexterity",
                        32 => "%R Ring of Sustain Charisma",
                        33 => "%R Ring of Slaying",
                        34 => "%R Ring of Gnomekind",
                        35 => "%R Ring of Speed",
                        _ => "Alien Ring",
                    }),
                ItemType::Amulet =>
                    match self.subval {
                        // !is_identified -> "%A Amulet"
                        5 => Cow::from("Amulet of Wisdom"),
                        6 => Cow::from("Amulet of Charisma"),
                        7 => Cow::from("Amulet of Searching"),
                        8 => Cow::from("Amulet of Teleportation"),
                        9 => Cow::from("Amulet of Slow Digestation"),
                        10 => Cow::from("Amulet of Resist Acid"),
                        11 => Cow::from("Amulet of Adornment"),
                        12 => Cow::from("Amulet of Adornment"),
                        13 => Cow::from("Amulet of the Magi"),
                        14 => Cow::from("Amulet of DOOM"),
                        268 => Cow::from(format!("Finely Wrought Silver Necklace{}", plural_s())),
                        269 => Cow::from(format!("Finely Wrought Gold Necklace{}", plural_s())),
                        270 => Cow::from(format!("Finely Wrought Mithril Necklace{}", plural_s())),
                        _ => Cow::from("Alien Amulet"),
                    },
                ItemType::MiscUsable =>
                    Cow::from(match self.subval {
                        14 => "%A Statue",
                        15 => "Broken Set of Teeth",
                        16 => "Silver Cross",
                        17 => "Gold Cross",
                        18 => "Mithril Cross",
                        19 => "%M Cross",
                        20 => "%M Cross",
                        21 => "Corked Bottle",
                        22 => "Holy Hand Grenade of Antioch",
                        _ => "Alien Thing",
                    }),
                ItemType::Chime =>
                    Cow::from(match self.subval {
                        1 => "%M Chime of Light",
                        2 => "%M Chime of Detect Doors/Stairs",
                        3 => "%M Chime of Detect Traps",
                        4 => "%M Chime of Teleportation",
                        5 => "%M Chime of Thunderblasts",
                        6 => "%M Chime of Summon Monster",
                        7 => "%M Chime of Disarming",
                        8 => "%M Chime of Aggravation",
                        9 => "%M Chime of Slow Monster",
                        10 => "%M Chime of Soothe Monster",
                        11 => "%M Chime of Cure Light Wound",
                        12 => "%M Chime of Changing",
                        13 => "%M Chime of Remove Curse",
                        14 => "%M Chime of Curing",
                        15 => "%M Chime of Dispel Evil",
                        16 => "%M Chime of Darkness",
                        _ => "Alien %M Chime",
                    }),
                ItemType::Horn =>
                    Cow::from(match self.subval {
                        1 => "%H of Bubbles",
                        2 => "%H of Calling",
                        3 => "%H of Soft Sounds",
                        4 => "%H of *Blasting*",
                        5 => "%H of Cold",
                        6 => "%H of Heat",
                        7 => "%H of Gas",
                        8 => "%H of Recall",
                        9 => "%H of *Chaos*",
                        10 => "%H of Glue",
                        11 => "%H of Valhalla",
                        12 => "%H of Tritons",
                        13 => "%H of Fog",
                        _ => "Alien %H",
                    }),
                ItemType::Instrument =>
                    Cow::from(match self.subval {
                        258 => "Pipes of Peace [Beginners Instrument]",
                        259 => "Lyre of Nature [Instrument I]",
                        260 => "Lute of the Woods [Instrument II]",
                        261 => "Harp of the Druids [Greater Instrument]",
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
                ItemType::Scroll1 =>
                    Cow::from(match self.subval {
                        257 => format!("Scroll{} of Enchant Weapon To-Hit", plural_s()),
                        258 => format!("Scroll{} of Enchant Weapon To-Dam", plural_s()),
                        259 => format!("Scroll{} of Enchant Armor", plural_s()),
                        260 => format!("Scroll{} of Identify", plural_s()),
                        261 => format!("Scroll{} of Remove Curse", plural_s()),
                        262 => format!("Scroll{} of Light", plural_s()),
                        263 => format!("Scroll{} of Summon Monster", plural_s()),
                        264 => format!("Scroll{} of Phase Door", plural_s()),
                        265 => format!("Scroll{} of Teleport", plural_s()),
                        266 => format!("Scroll{} of Teleport Level", plural_s()),
                        267 => format!("Scroll{} of Monster Confusion", plural_s()),
                        268 => format!("Scroll{} of Magic Mapping", plural_s()),
                        269 => format!("Scroll{} of Sleep Monster", plural_s()),
                        270 => format!("Scroll{} of Rune of Protection", plural_s()),
                        271 => format!("Scroll{} of Treasure Detection", plural_s()),
                        272 => format!("Scroll{} of Object Detection", plural_s()),
                        273 => format!("Scroll{} of Trap Detection", plural_s()),
                        274 => format!("Scroll{} of Door/Stair Location", plural_s()),
                        275 => format!("Scroll{} of Mass Genocide", plural_s()),
                        276 => format!("Scroll{} of Detect Invisible", plural_s()),
                        277 => format!("Scroll{} of Aggravate Monster", plural_s()),
                        278 => format!("Scroll{} of Trap Creation", plural_s()),
                        279 => format!("Scroll{} of Trap/Door Destruction", plural_s()),
                        280 => format!("Scroll{} of Door Creation", plural_s()),
                        281 => format!("Scroll{} of Recharging", plural_s()),
                        282 => format!("Scroll{} of Genocide", plural_s()),
                        283 => format!("Scroll{} of Darkness", plural_s()),
                        284 => format!("Scroll{} of Protection from Evil", plural_s()),
                        285 => format!("Scroll{} of Create Food", plural_s()),
                        286 => format!("Scroll{} of Dispel Undead", plural_s()),
                        /*257 => format!("Scroll{} of *Enchant Weapon*", plural_s()),
                        258 => format!("Scroll{} of Curse Weapon", plural_s()),
                        259 => format!("Scroll{} of *Enchant Armor*", plural_s()),
                        260 => format!("Scroll{} of Curse Armor", plural_s()),
                        261 => format!("Scroll{} of Summon Undead", plural_s()),
                        262 => format!("Scroll{} of Blessing", plural_s()),
                        263 => format!("Scroll{} of Holy Chant", plural_s()),
                        264 => format!("Scroll{} of Holy Prayer", plural_s()),
                        265 => format!("Scroll{} of Word-of-Recall", plural_s()),
                        266 => format!("Scroll{} of *Destruction*", plural_s()),
                        267 => format!("Scroll{} of Wishing", plural_s()),
                        268 => format!("Scroll{} of Feign Death", plural_s()),
                        269 => format!("Scroll{} of Make Munchies", plural_s()),*/
                        _ => format!("Alien potion{}", plural_s()),
                    }),
                ItemType::Potion1 =>
                    Cow::from(match self.subval {
                        257 => format!("%C Potion{} of Gain Strength", plural_s()),
                        258 => format!("%C Potion{} of Poison", plural_s()),
                        259 => format!("%C Potion{} of Restore Strength", plural_s()),
                        260 => format!("%C Potion{} of Gain Intelligence", plural_s()),
                        261 => format!("%C Potion{} of Lose Intelligence", plural_s()),
                        262 => format!("%C Potion{} of Restore Intelligence", plural_s()),
                        263 => format!("%C Potion{} of Gain Wisdom", plural_s()),
                        264 => format!("%C Potion{} of Lose Wisdom", plural_s()),
                        265 => format!("%C Potion{} of Restore Wisdom", plural_s()),
                        266 => format!("%C Potion{} of Charisma", plural_s()),
                        267 => format!("%C Potion{} of Ugliness", plural_s()),
                        268 => format!("%C Potion{} of Restore Charisma", plural_s()),
                        269 => format!("%C Potion{} of Cure Light Wounds", plural_s()),
                        270 => format!("%C Potion{} of Cure Serious Wounds", plural_s()),
                        271 => format!("%C Potion{} of Cure Critical Wounds", plural_s()),
                        272 => format!("%C Potion{} of Healing", plural_s()),
                        273 => format!("%C Potion{} of Gain Constitution", plural_s()),
                        274 => format!("%C Potion{} of Gain Experience", plural_s()),
                        275 => format!("%C Potion{} of Sleep", plural_s()),
                        276 => format!("%C Potion{} of Blindness", plural_s()),
                        277 => format!("%C Potion{} of Confusion", plural_s()),
                        278 => format!("%C Potion{} of Poison", plural_s()),
                        279 => format!("%C Potion{} of Haste Self", plural_s()),
                        280 => format!("%C Potion{} of Slowness", plural_s()),
                        281 => format!("Icky Green Potion{} of Slime Mold Juice", plural_s()),
                        282 => format!("Light Brown Potion{} of Apple Juice", plural_s()),
                        283 => format!("Clear Potion{} of Water", plural_s()),
                        284 => format!("%C Potion{} of Gain Dexterity", plural_s()),
                        285 => format!("%C Potion{} of Restore Dexterity", plural_s()),
                        286 => format!("%C Potion{} of Restore Constitution", plural_s()),
                        287 => format!("%C Potion{} of Learning", plural_s()),
                        288 => format!("%C Potion{} of Lose Memories", plural_s()),
                        289 => format!("%C Potion{} of Salt Water", plural_s()),
                        290 => format!("%C Potion{} of Invulnerability", plural_s()),
                        291 => format!("%C Potion{} of Heroism", plural_s()),
                        292 => format!("%C Potion{} of Super Heroism", plural_s()),
                        293 => format!("%C Potion{} of Boldliness", plural_s()),
                        294 => format!("%C Potion{} of Restore Life Levels", plural_s()),
                        295 => format!("%C Potion{} of Resist Heat", plural_s()),
                        296 => format!("%C Potion{} of Resist Cold", plural_s()),
                        297 => format!("%C Potion{} of Detect Invisible", plural_s()),
                        298 => format!("%C Potion{} of Slow Poison", plural_s()),
                        299 => format!("%C Potion{} of Neutralize Poison", plural_s()),
                        300 => format!("%C Potion{} of Restore Mana", plural_s()),
                        301 => format!("%C Potion{} of Infra-Vision", plural_s()),
                        302 => format!("%C Potion{} of Flea Bile", plural_s()),
                        _ => format!("Alien Potion{}", plural_s()),
                    }),
                ItemType::FlaskOfOil => Cow::from(format!("Flask{} of Oil", plural_s())),
                ItemType::Wand =>
                    Cow::from(match self.subval {
                        1 => format!("%M Wand{} of Light (%P1 charges)", plural_s()),
                        2 => format!("%M Wand{} of Lightning Bolts (%P1 charges)", plural_s()),
                        3 => format!("%M Wand{} of Frost Bolts (%P1 charges)", plural_s()),
                        4 => format!("%M Wand{} of Fire Bolts (%P1 charges)", plural_s()),
                        5 => format!("%M Wand{} of Stone-to-Mud (%P1 charges)", plural_s()),
                        6 => format!("%M Wand{} of Polymorph (%P1 charges)", plural_s()),
                        7 => format!("%M Wand{} of Heal Monster (%P1 charges)", plural_s()),
                        8 => format!("%M Wand{} of Haste Monster (%P1 charges)", plural_s()),
                        9 => format!("%M Wand{} of Slow Monster (%P1 charges)", plural_s()),
                        10 => format!("%M Wand{} of Confuse Monster (%P1 charges)", plural_s()),
                        11 => format!("%M Wand{} of Sleep Monster (%P1 charges)", plural_s()),
                        12 => format!("%M Wand{} of Drain Life (%P1 charges)", plural_s()),
                        13 => format!("%M Wand{} of Trap/Door destruction (%P1 charges)", plural_s()),
                        14 => format!("%M Wand{} of Magic Missile (%P1 charges)", plural_s()),
                        15 => format!("%M Wand{} of Wall Building (%P1 charges)", plural_s()),
                        16 => format!("%M Wand{} of Clone Monster (%P1 charges)", plural_s()),
                        17 => format!("%M Wand{} of Teleport Away (%P1 charges)", plural_s()),
                        18 => format!("%M Wand{} of Disarming (%P1 charges)", plural_s()),
                        19 => format!("%M Wand{} of Lightning Balls (%P1 charges)", plural_s()),
                        20 => format!("%M Wand{} of Cold Balls (%P1 charges)", plural_s()),
                        21 => format!("%M Wand{} of Fire Balls (%P1 charges)", plural_s()),
                        22 => format!("%M Wand{} of Stinking Cloud (%P1 charges)", plural_s()),
                        23 => format!("%M Wand{} of Acid Balls (%P1 charges)", plural_s()),
                        24 => format!("%M Wand{} of Wonder (%P1 charges)", plural_s()),
                        25 => format!("%M Wand{} of Probing (%P1 charges)", plural_s()),
                        _ => format!("Alien wand{}", plural_s()),
                    }),
                ItemType::Staff =>
                    Cow::from(match self.subval {
                        1 => "%W Staff of Light (%P1 charges)",
                        2 => "%W Staff of Door/Stair Location (%P1 charges)",
                        3 => "%W Staff of Trap Location (%P1 charges)",
                        4 => "%W Staff of Treasure Location (%P1 charges)",
                        5 => "%W Staff of Object Location (%P1 charges)",
                        6 => "%W Staff of Teleportation (%P1 charges)",
                        7 => "%W Staff of Earthquakes (%P1 charges)",
                        8 => "%W Staff of Summoning (%P1 charges)",
                        10 => "%W Staff of *Destruction* (%P1 charges)",
                        11 => "%W Staff of Starlite (%P1 charges)",
                        12 => "%W Staff of Haste Monsters (%P1 charges)",
                        13 => "%W Staff of Slow Monsters (%P1 charges)",
                        14 => "%W Staff of Sleep Monsters (%P1 charges)",
                        15 => "%W Staff of Cure Light Wounds (%P1 charges)",
                        16 => "%W Staff of Detect Invisible (%P1 charges)",
                        17 => "%W Staff of Speed (%P1 charges)",
                        18 => "%W Staff of Slowness (%P1 charges)",
                        19 => "%W Staff of Mass Polymorph (%P1 charges)",
                        20 => "%W Staff of Remove Curse (%P1 charges)",
                        21 => "%W Staff of Detect Evil (%P1 charges)",
                        22 => "%W Staff of Curing (%P1 charges)",
                        23 => "%W Staff of Dispel Evil (%P1 charges)",
                        25 => "%W Staff of Darkness (%P1 charges)",
                        26 => "%W Staff of Identify (%P1 charges)",
                        _ => "Alien staff{}",
                    }),
                ItemType::MagicBook =>
                    Cow::from(match self.subval {
                        257 => format!("Book{} of Magic Spells [Beginners-Magik]", plural_s()),
                        258 => format!("Book{} of Magic Spells [Magik I]", plural_s()),
                        259 => format!("Book{} of Magic Spells [Magik II]", plural_s()),
                        261 => format!("Book{} of Magic Spells [The Mages Guide to Power]", plural_s()),
                        _ => format!("Alien magic book{}", plural_s()),
                    }),
                ItemType::PrayerBook =>
                    Cow::from(match self.subval {
                        258 => format!("Holy Book{} of Prayers [Beginners Handbook]", plural_s()),
                        259 => format!("Holy Book{} of Prayers [Words of Wisdom]", plural_s()),
                        260 => format!("Holy Book{} of Prayers [Chants and Blessings]", plural_s()),
                        261 => format!("Holy Book{} of Prayers [Exorcism and Dispelling]", plural_s()),
                        _ => format!("Alien holy book{}", plural_s()),
                    }),
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
                ItemType::Bag =>
                    Cow::from(match self.subval {
                        1 => "%N Bag of Holding (250)",
                        2 => "%N Bag of Holding (500)",
                        3 => "%N Bag of Holding (1000)",
                        //3 => "%N Bag of Holding (1500)",
                        4 => "%N Bag of Devouring",
                        _ => "Alien bag",
                }),
                ItemType::MiscObject =>
                    Cow::from(match self.subval {
                        1 => "Rat Skeleton",
                        2 => "Giant Centipede Skeleton",
                        4 => "Empty bottle",
                        5 => "Broken set of pottery",
                        7 => "Human Skeleton",
                        8 => "Dwarf Skeleton",
                        9 => "Elf Skeleton",
                        10 => "Gnome Skeleton",
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
        if damage_string != "0d0" {
            Cow::from(format!(" ({})", damage_string))
        } else {
            Cow::from("")
        }
    }

    fn attack_enchantment_string<'a>(&self) -> Cow<'a, str> {
        let tohit_sign = if self.tohit > 0 { "+" } else if self.tohit < 0 { "-" } else {""};
        let todam_sign = if self.todam > 0 { "+" } else if self.todam < 0 { "-" } else {""};
        Cow::from(format!(" ({}{},{}{})", tohit_sign, self.tohit, todam_sign, self.todam))
    }

    pub fn item_type(&self) -> ItemType {
        ItemType::from(self.tval)
    }

    pub fn is_identified(&self) -> bool {
        true // TODO: Implement this
    }

    // In progress..
    pub fn equipment_name(&self) -> String {
        let mut parts = Vec::new();
        parts.push(self.number_of_string());
        parts.push(self.subtype_name());
        if self.item_type().is_weapon() {
            parts.push(self.damage_string());
            if self.is_identified() {
                parts.push(self.attack_enchantment_string());
            }
        }
        parts.join("")
    }
}
