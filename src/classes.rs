use std::ops::Range;

use types::StatBlock;

#[derive(PartialEq, Clone, Copy)]
pub enum Class {
    Warrior = 0,
    Mage = 1,
    Priest = 2,
    Rogue = 3,
    Ranger = 4,
    Paladin = 5,
    Druid = 6,
    Bard = 7,
    Adventurer = 8,
    Monk = 9,
}

impl Class {
    pub fn name(&self) -> &'static str {
        match self {
            Class::Warrior => "Warrior",
            Class::Mage => "Mage",
            Class::Priest => "Priest",
            Class::Rogue => "Rogue",
            Class::Ranger => "Ranger",
            Class::Paladin => "Paladin",
            Class::Druid => "Druid",
            Class::Bard => "Bard",
            Class::Adventurer => "Adventurer",
            Class::Monk => "Monk",
        }
    }

    pub fn health_bonus(&self) -> u8 {
        match self {
            Class::Warrior => 10,
            Class::Mage => 0,
            Class::Priest => 3,
            Class::Rogue => 6,
            Class::Ranger => 4,
            Class::Paladin => 6,
            Class::Druid => 3,
            Class::Bard => 4,
            Class::Adventurer => 4,
            Class::Monk => 4,
        }
    }

    pub fn melee_bonus(&self) -> i8 {
        match self {
            Class::Warrior => 10,
            Class::Mage => 4,
            Class::Priest => 6,
            Class::Rogue => 6,
            Class::Ranger => 6,
            Class::Paladin => 8,
            Class::Druid => 4,
            Class::Bard => 5,
            Class::Adventurer => 6,
            Class::Monk => 8,
        }
    }

    pub fn ranged_bonus(&self) -> i8 {
        match self {
            Class::Warrior => 8,
            Class::Mage => 4,
            Class::Priest => 5,
            Class::Rogue => 10,
            Class::Ranger => 10,
            Class::Paladin => 6,
            Class::Druid => 7,
            Class::Bard => 6,
            Class::Adventurer => 6,
            Class::Monk => 6,
        }
    }

    pub fn search_mod(&self) -> i8 {
        match self {
            Class::Warrior => 14,
            Class::Mage => 16,
            Class::Priest => 16,
            Class::Rogue => 32,
            Class::Ranger => 24,
            Class::Paladin => 12,
            Class::Druid => 16,
            Class::Bard => 22,
            Class::Adventurer => 24,
            Class::Monk => 24,
        }
    }

    pub fn disarm_mod(&self) -> i8 {
        match self {
            Class::Warrior => 25,
            Class::Mage => 30,
            Class::Priest => 25,
            Class::Rogue => 45,
            Class::Ranger => 30,
            Class::Paladin => 20,
            Class::Druid => 25,
            Class::Bard => 30,
            Class::Adventurer => 30,
            Class::Monk => 45,
        }
    }

    pub fn stealth_mod(&self) -> i8 {
        match self {
            Class::Warrior => 1,
            Class::Mage => 2,
            Class::Priest => 2,
            Class::Rogue => 4,
            Class::Ranger => 3,
            Class::Paladin => 1,
            Class::Druid => 1,
            Class::Bard => 2,
            Class::Adventurer => 3,
            Class::Monk => 3,
        }
    }

    pub fn search_freq(&self) -> i8 {
        match self {
            Class::Warrior => 38,
            Class::Mage => 36,
            Class::Priest => 32,
            Class::Rogue => 16,
            Class::Ranger => 24,
            Class::Paladin => 38,
            Class::Druid => 32,
            Class::Bard => 28,
            Class::Adventurer => 24,
            Class::Monk => 24,
        }
    }

    pub fn save_mod(&self) -> i8 {
        match self {
            Class::Warrior => 10,
            Class::Mage => 25,
            Class::Priest => 20,
            Class::Rogue => 15,
            Class::Ranger => 20,
            Class::Paladin => 15,
            Class::Druid => 20,
            Class::Bard => 20,
            Class::Adventurer => 20,
            Class::Monk => 25,
        }
    }

    pub fn magic_resist(&self) -> i8 {
        match self {
            Class::Warrior => -10,
            Class::Mage => 0,
            Class::Priest => 0,
            Class::Rogue => -5,
            Class::Ranger => -5,
            Class::Paladin => -5,
            Class::Druid => -5,
            Class::Bard => -5,
            Class::Adventurer => -5,
            Class::Monk => -5,
        }
    }

    pub fn expfactor(&self) -> f32 {
        match self {
            Class::Warrior => 0.0,
            Class::Mage => 0.3,
            Class::Priest => 0.2,
            Class::Rogue => 0.1,
            Class::Ranger => 0.3,
            Class::Paladin => 0.35,
            Class::Druid => 0.2,
            Class::Bard => 0.3,
            Class::Adventurer => 0.4,
            Class::Monk => 0.1,
        }
    }

    pub fn title(&self, level: u8) -> &'static str {
        match self {
            Class::Warrior =>
                match level {
                    0 => "Novice",          1 => "Veteran(1st)",
                    2 => "Veteran(2nd)",    3 => "Veteran(3rd)",
                    4 => "Warrior(1st)",    5 => "Warrior(2nd)",
                    6 => "Warrior(3rd)",    7 => "Swordsman-1",
                    8 => "Swordsman-2",     9 => "Swordsman-3",
                   10 => "Hero",           11 => "Swashbuckler",
                   12 => "Myrmidon",       13 => "Champion-1",
                   14 => "Champion-2",     15 => "Champion-3",
                   16 => "Superhero",      17 => "Knight",
                   18 => "Superior Knt",   19 => "Gallant Knt",
                   20 => "Knt Errent",     21 => "Keeper",
                   22 => "Protector",      23 => "Defender",
                   24 => "Warder",         25 => "Guardian Knt",
                   26 => "Chevalier",      27 => "Justiciar",
                   28 => "Lord (1st)",     29 => "Lord (2nd)",
                   30 => "Lord (3rd)",     31 => "Lord (4th)",
                   32 => "Lord (5th)",     33 => "Lord (6th)",
                   34 => "Lord (7th)",     35 => "Lord (8th)",
                   36 => "Lord (9th)",     37 => "Lord Gallant",
                   38 => "Lord Keeper",    _ => "Lord Noble",
                },
            Class::Mage =>
                match level {
                    0 => "Novice",          1 => "Apprentice",
                    2 => "Trickster-1",     3 => "Trickster-2",
                    4 => "Trickster-3",     5 => "Cabalist-1",
                    6 => "Cabalist-2",      7 => "Cabalist-3",
                    8 => "Visionist",       9 => "Phantasmist",
                   10 => "Shadowist",      11 => "Spellbinder",
                   12 => "Illusionist",    13 => "Evoker (1st)",
                   14 => "Evoker (2nd)",   15 => "Evoker (3rd)",
                   16 => "Evoker (4th)",   17 => "Conjurer",
                   18 => "Theurgist",      19 => "Thaumaturer",
                   20 => "Magician",       21 => "Enchanter",
                   22 => "Warlock",        23 => "Sorcerer",
                   24 => "Necromancer",    25 => "Mage (1st)",
                   26 => "Mage (2nd)",     27 => "Mage (3rd)",
                   28 => "Mage (4th)",     29 => "Mage (5th)",
                   30 => "Wizard (1st)",   31 => "Wizard (2nd)",
                   32 => "Wizard (3rd)",   33 => "Wizard (4th)",
                   34 => "Wizard (5th)",   35 => "Wizard (6th)",
                   36 => "Wizard (7th)",   37 => "Wizard (8th)",
                   38 => "Wizard (9th)",    _ => "Wizard Lord",
                },
            Class::Priest =>
                match level {
                    0 => "Believer",        1 => "Acolyte(1st)",
                    2 => "Acolyte(2nd)",    3 => "Acolyte(3rd)",
                    4 => "Adept (1st)",     5 => "Adept (2nd)",
                    6 => "Adept (3rd)",     7 => "Priest (1st)",
                    8 => "Priest (2nd)",    9 => "Priest (3rd)",
                   10 => "Priest (4th)",   11 => "Priest (5th)",
                   12 => "Priest (6th)",   13 => "Priest (7th)",
                   14 => "Priest (8th)",   15 => "Priest (9th)",
                   16 => "Curate (1st)",   17 => "Curate (2nd)",
                   18 => "Curate (3rd)",   19 => "Curate (4th)",
                   20 => "Curate (5th)",   21 => "Curate (6th)",
                   22 => "Curate (7th)",   23 => "Curate (8th)",
                   24 => "Curate (9th)",   25 => "Canon (1st)",
                   26 => "Canon (2nd)",    27 => "Canon (3rd)",
                   28 => "Canon (4th)",    29 => "Canon (5th)",
                   30 => "Low Lama",       31 => "Lama-1",
                   32 => "Lama-2",         33 => "Lama-3",
                   34 => "High Lama",      35 => "Great Lama",
                   36 => "Patriarch",      37 => "High Priest",
                   38 => "Great Priest",    _ => "Noble Priest",
                },
            Class::Rogue =>
                match level {
                    0 => "Apprentice",      1 => "Footpad",
                    2 => "Cutpurse",        3 => "Robber",
                    4 => "Burglar",         5 => "Filcher",
                    6 => "Sharper",         7 => "Magsman",
                    8 => "Common Rogue",    9 => "Rogue (1st)",
                   10 => "Rogue (2nd)",    11 => "Rogue (3rd)",
                   12 => "Rogue (4th)",    13 => "Rogue (5th)",
                   14 => "Rogue (6th)",    15 => "Rogue (7th)",
                   16 => "Rogue (8th)",    17 => "Rogue (9th)",
                   18 => "Master Rogue",   19 => "Expert Rogue",
                   20 => "Senior Rogue",   21 => "Chief Rogue",
                   22 => "Prime Rogue",    23 => "Low Thief",
                   24 => "Thief (1st)",    25 => "Thief (2nd)",
                   26 => "Thief (3rd)",    27 => "Thief (4th)",
                   28 => "Thief (5th)",    29 => "Thief (6th)",
                   30 => "Thief (7th)",    31 => "Thief (8th)",
                   32 => "Thief (9th)",    33 => "High Thief",
                   34 => "Master Thief",   35 => "Executioner",
                   36 => "Low Assassin",   37 => "Assassin",
                   38 => "HighAssassin",    _ => "Guildsmaster",
                },
            Class::Ranger =>
                match level {
                    0 => "Runner (1st)",    1 => "Runner (2nd)",
                    2 => "Runner (3rd)",    3 => "Strider (1st)",
                    4 => "Strider (2nd)",   5 => "Strider (3rd)",
                    6 => "Scout (1st)",     7 => "Scout (2nd)",
                    8 => "Scout (3rd)",     9 => "Scout (4th)",
                   10 => "Scout (5th)",    11 => "Courser (1st)",
                   12 => "Courser (2nd)",  13 => "Courser (3rd)",
                   14 => "Courser (4th)",  15 => "Courser (5th)",
                   16 => "Tracker (1st)",  17 => "Tracker (2nd)",
                   18 => "Tracker (3rd)",  19 => "Tracker (4th)",
                   20 => "Tracker (5th)",  21 => "Tracker (6th)",
                   22 => "Tracker (7th)",  23 => "Tracker (8th)",
                   24 => "Tracker (9th)",  25 => "Guide (1st)",
                   26 => "Guide (2nd)",    27 => "Guide (3rd)",
                   28 => "Guide (4th)",    29 => "Guide (5th)",
                   30 => "Guide (6th)",    31 => "Guide (7th)",
                   32 => "Guide (8th)",    33 => "Guide (9th)",
                   34 => "Pathfinder-1",   35 => "Pathfinder-2",
                   36 => "Pathfinder-3",   37 => "Ranger",
                   38 => "High Ranger",     _ => "Ranger Lord",
                },
            Class::Paladin =>
                match level {
                    0 => "Gallant",         1 => "Keeper (1st)",
                    2 => "Keeper (2nd)",    3 => "Keeper (3rd)",
                    4 => "Keeper (4th)",    5 => "Keeper (5th)",
                    6 => "Keeper (6th)",    7 => "Keeper (7th)",
                    8 => "Keeper (8th)",    9 => "Keeper (9th)",
                   10 => "Protector-1",    11 => "Protector-2",
                   12 => "Protector-3",    13 => "Protector-4",
                   14 => "Protector-5",    15 => "Protector-6",
                   16 => "Protector-7",    17 => "Protector-8",
                   18 => "Defender-1",     19 => "Defender-2",
                   20 => "Defender-3",     21 => "Defender-4",
                   22 => "Defender-5",     23 => "Defender-6",
                   24 => "Defender-7",     25 => "Defender-8",
                   26 => "Warder (1st)",   27 => "Warder (2nd)",
                   28 => "Warder (3rd)",   29 => "Warder (4th)",
                   30 => "Warder (5th)",   31 => "Warder (6th)",
                   32 => "Warder (7th)",   33 => "Warder (8th)",
                   34 => "Warder (9th)",   35 => "Gauardian",
                   36 => "Chevalier",      37 => "Justiciar",
                   38 => "Paladin",         _ => "High Lord",
                },
            Class::Druid =>
                match level {
                    0 => "Aspirant-1",      1 => "Aspirant-2",
                    2 => "Ovate (1st)",     3 => "Ovate (2nd)",
                    4 => "Ovate(3th)",      5 => "Initiate-1",
                    6 => "Initiate-2",      7 => "1st Cabal",
                    8 => "2nd Cabal",       9 => "3rd Cabal",
                   10 => "4th Cabal",      11 => "5th Cabal",
                   12 => "6th Cabal",      13 => "7th Cabal",
                   14 => "8th Cabal",      15 => "9th Cabal",
                   16 => "10th Cabal",     17 => "11th Cabal",
                   18 => "12th Cabal",     19 => "13th Cabal",
                   20 => "Low Druid",      21 => "Druid (1st)",
                   22 => "Druid (2nd)",    23 => "Druid (3rd)",
                   24 => "Druid (4th)",    25 => "Druid (5th)",
                   26 => "Druid (6th)",    27 => "Adept Druid",
                   28 => "Arch Druid",     29 => "Great Druid",
                   30 => "Master Druid",   31 => "Grand Druid",
                   32 => "Lord Druid",     33 => "LowHierophant",
                   34 => "Hierophant-1",   35 => "Hierophant-2",
                   36 => "Hierophant-3",   37 => "Hierophant-4",
                   38 => "Hierophant-5",    _ => "Forest Master",
                },
            Class::Bard =>
                match level {
                    0 => "Singer",          1 => "Rhymer",
                    2 => "Joker",           3 => "Lyrist (1st)",
                    4 => "Lyrist (2nd)",    5 => "Lyrist (3rd)",
                    6 => "Jester (1st)",    7 => "Jester (2nd)",
                    8 => "Sonnateer-1",     9 => "Sonnateer-2",
                   10 => "Sonnateer-3",    11 => "Skald (1st)",
                   12 => "Skald (2nd)",    13 => "Skald (3rd)",
                   14 => "Comic (1st)",    15 => "Comic (2nd)",
                   16 => "Comic (3rd)",    17 => "Racaraide-1",
                   18 => "Racaraide-2",    19 => "Racaraide-3",
                   20 => "Joungleur-1",    21 => "Joungleur-2",
                   22 => "Joungleur-3",    23 => "Minstrel-1",
                   24 => "Minstrel-2",     25 => "Minstrel-3",
                   26 => "Muse (1st)",     27 => "Muse (2nd)",
                   28 => "Muse(3rd)",      29 => "Lorist (1st)",
                   30 => "Lorist (2nd)",   31 => "Lorist (3rd)",
                   32 => "Initiate Bard",  33 => "Bard (1st)",
                   34 => "Bard (2nd)",     35 => "Bard (3rd)",
                   36 => "Master Bard-1",  37 => "Master Bard-2",
                   38 => "Master Bard-3",   _ => "Bardic Sage",
                },
            Class::Adventurer =>
                match level {
                    0 => "Dimwit",          1 => "Beginner",
                    2 => "Amateur",         3 => "Flathead",
                    4 => "PileofLeaves",    5 => "Novice",
                    6 => "Brass Bell",      7 => "Orange Cake",
                    8 => "White Unicorn",   9 => "Charlatan",
                   10 => "CloveofGarlic",  11 => "Nasty Knife",
                   12 => "ParlorMgician",  13 => "Gaudy Crown",
                   14 => "Loaf of Bread",  15 => "Lurking Grue",
                   16 => "Noisome Stew",   17 => "Broken Timber",
                   18 => "Granite Wall",   19 => "FCD #3",
                   20 => "Adventurer-1",   21 => "Adventurer-2",
                   22 => "Adventurer-3",   23 => "AIMFIZ Scroll",
                   24 => "VILSTU Potion",  25 => "Sr Adventurer",
                   26 => "Master Advnt",   27 => "WizofFrobozz",
                   28 => "Enchanter",      29 => "Sorcerer",
                   30 => "Wizard (1st)",   31 => "Wizard (2nd)",
                   32 => "Wizard (3rd)",   33 => "Wizard (4th)",
                   34 => "Wizard (5th)",   35 => "Wizard (6th)",
                   36 => "Wizard (7th)",   37 => "Zorkmid Coin",
                   38 => "DungeonMaster",   _ => "Implementer"
                },
            Class::Monk =>
                match level {
                    0 => "Disciple",        1 => "Acolyte(1st)",
                    2 => "Acolyte(2nd)",    3 => "Acolyte(3rd)",
                    4 => "Neophyte(1st)",   5 => "Neophyte(2nd)",
                    6 => "Neophyte(3rd)",   7 => "Neophyte(4th)",
                    8 => "Adept (1st)",     9 => "Adept (2nd)",
                   10 => "Adept (3rd)",    11 => "Hermit (1st)",
                   12 => "Hermit (2nd)",   13 => "Hermit (3rd)",
                   14 => "Hermit (4th)",   15 => "Hermit (5th)",
                   16 => "Curate (1st)",   17 => "Curate (2nd)",
                   18 => "Curate (3rd)",   19 => "Curate (4th)",
                   20 => "Curate (5th)",   21 => "Curate (6th)",
                   22 => "Curate (7th)",   23 => "Curate (8th)",
                   24 => "Curate (9th)",   25 => "Master (1st)",
                   26 => "Master (2nd)",   27 => "Master (3rd)",
                   28 => "Master (4th)",   29 => "Master (5th)",
                   30 => "SupriorMaster",  31 => "Low Lama",
                   32 => "High Lama",      33 => "Great Lama",
                   34 => "DancingLlama",   35 => "Holy Man",
                   36 => "Hi-YA Man",      37 => "Baby Buddha",
                   38 => "BuddingBuddha",   _ => "Buddha"
                },
        }
    }

    pub fn stat_block(&self) -> StatBlock {
        match self {
            Class::Warrior => StatBlock::from([5, -2, -2, 2, 2, 1]),
            Class::Mage => StatBlock::from([-5, 3, 0, 0, -2, 0]),
            Class::Priest => StatBlock::from([0, -3, 3, -1, 1, 2]),
            Class::Rogue => StatBlock::from([2, 0, -2, 3, 1, 1]),
            Class::Ranger => StatBlock::from([2, 0, 1, 1, 1, 2]),
            Class::Paladin => StatBlock::from([3, -3, 1, 0, 2, 2]),
            Class::Druid => StatBlock::from([-1, -1, 2, 0, 0, 3]),
            Class::Bard => StatBlock::from([2, 1, -1, 2, 0, 3]),
            Class::Adventurer => StatBlock::from([2, 2, -1, 1, 1, 0]),
            Class::Monk => StatBlock::from([2, 0, 2, 3, 1, 1]),
        }
    }

}

pub fn classes_iter() -> Range<usize> {
    (Class::Warrior as usize)..(Class::Monk as usize + 1)
}

impl From<usize> for Class {
    fn from(pos: usize) -> Self {
        match pos {
            0 => Class::Warrior,
            1 => Class::Mage,
            2 => Class::Priest,
            3 => Class::Rogue,
            4 => Class::Ranger,
            5 => Class::Paladin,
            6 => Class::Druid,
            7 => Class::Bard,
            8 => Class::Adventurer,
            9 => Class::Monk,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq)]
pub enum MagicType {
    Arcane = 0,
    Divine = 1,
    Nature = 2,
    Song = 3,
    Chakra = 4,
}

#[no_mangle]
pub extern fn class_uses_magic(class: i32, magic_type: i32) -> u8 {
    match magic_type {
        x if x == MagicType::Arcane as i32 =>
            (class == Class::Mage as i32 || class == Class::Adventurer as i32) as u8,
        x if x == MagicType::Divine as i32 =>
            (class == Class::Priest as i32 || class == Class::Paladin as i32) as u8,
        x if x == MagicType::Nature as i32 =>
            (class == Class::Druid as i32 || class == Class::Ranger as i32) as u8,
        x if x == MagicType::Song as i32 =>
            (class == Class::Bard as i32 || class == Class::Rogue as i32) as u8,
        x if x == MagicType::Chakra as i32 =>
            (class == Class::Monk as i32) as u8,
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_melee_bonus(class: i32) -> i8 {
    Class::from(class as usize).melee_bonus()
}

#[no_mangle]
pub extern fn class_ranged_bonus(class: i32) -> i8 {
    Class::from(class as usize).ranged_bonus()
}
