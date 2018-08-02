use std::ops::Range;

use types::{StatBlock, Class};

#[derive(PartialEq, Clone, Copy)]
pub enum Race {
    Human = 0,
    HalfElf = 1,
    Elf = 2,
    Halfling = 3,
    Gnome = 4,
    Dwarf = 5,
    HalfOrc = 6,
    HalfTroll = 7,
    Phraint = 8,
    Dryad = 9
}

impl Race {
    pub fn name(&self) -> &'static str {
        match self {
            Race::Human => "Human",
            Race::HalfElf => "Half-Elf",
            Race::Elf => "Elf",
            Race::Halfling => "Halfling",
            Race::Gnome => "Gnome",
            Race::Dwarf => "Dwarf",
            Race::HalfOrc => "Half-Orc",
            Race::HalfTroll => "Half-Troll",
            Race::Phraint => "Phraint",
            Race::Dryad => "Dryad"
        }
    }

    pub fn search_mod(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 6,
            Race::Elf => 8,
            Race::Halfling => 12,
            Race::Gnome => 6,
            Race::Dwarf => 7,
            Race::HalfOrc => 0,
            Race::HalfTroll => -1,
            Race::Phraint => 10,
            Race::Dryad => 6,
        }
    }

    pub fn melee_bonus(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 0,
            Race::Elf => -5,
            Race::Halfling => -10,
            Race::Gnome => -8,
            Race::Dwarf => 15,
            Race::HalfOrc => 12,
            Race::HalfTroll => 20,
            Race::Phraint => 3,
            Race::Dryad => 0,
        }
    }

    pub fn ranged_bonus(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 5,
            Race::Elf => 15,
            Race::Halfling => 20,
            Race::Gnome => 12,
            Race::Dwarf => 0,
            Race::HalfOrc => -5,
            Race::HalfTroll => -10,
            Race::Phraint => 5,
            Race::Dryad => 5,
        }
    }

    pub fn search_freq(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => -1,
            Race::Elf => -1,
            Race::Halfling => -5,
            Race::Gnome => -3,
            Race::Dwarf => 0,
            Race::HalfOrc => 3,
            Race::HalfTroll => 5,
            Race::Phraint => 3,
            Race::Dryad => -1,
        }
    }

    pub fn stealth_mod(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 1,
            Race::Elf => 1,
            Race::Halfling => 4,
            Race::Gnome => 3,
            Race::Dwarf => 0,
            Race::HalfOrc => -1,
            Race::HalfTroll => -2,
            Race::Phraint => 5,
            Race::Dryad => 1,
        }
    }

    pub fn save_mod(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 3,
            Race::Elf => 6,
            Race::Halfling => 18,
            Race::Gnome => 12,
            Race::Dwarf => 9,
            Race::HalfOrc => -3,
            Race::HalfTroll => -9,
            Race::Phraint => -3,
            Race::Dryad => 3,
        }
    }

    pub fn health_bonus(&self) -> u8 {
        match self {
            Race::Human => 10,
            Race::HalfElf => 9,
            Race::Elf => 8,
            Race::Halfling => 6,
            Race::Gnome => 7,
            Race::Dwarf => 9,
            Race::HalfOrc => 10,
            Race::HalfTroll => 12,
            Race::Phraint => 8,
            Race::Dryad => 7,
        }
    }

    pub fn expfactor(&self) -> f32 {
        match self {
            Race::Human => 1.00,
            Race::HalfElf => 1.10,
            Race::Elf => 1.20,
            Race::Halfling => 1.10,
            Race::Gnome => 1.25,
            Race::Dwarf => 1.20,
            Race::HalfOrc => 1.10,
            Race::HalfTroll => 1.20,
            Race::Phraint => 1.20,
            Race::Dryad => 1.20,
        }
    }

    pub fn infravision(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 0,
            Race::Elf => 0,
            Race::Halfling => 4,
            Race::Gnome => 3,
            Race::Dwarf => 5,
            Race::HalfOrc => 3,
            Race::HalfTroll => 3,
            Race::Phraint => 5,
            Race::Dryad => 3,
        }
    }

    pub fn swim_speed(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 1,
            Race::Elf => 2,
            Race::Halfling => -2,
            Race::Gnome => -1,
            Race::Dwarf => -2,
            Race::HalfOrc => 0,
            Race::HalfTroll => 2,
            Race::Phraint => -1,
            Race::Dryad => -1,
        }
    }

    pub fn stat_block(&self) -> StatBlock {
        match self {
            Race::Human => StatBlock::from([0, 0, 0, 0, 0, 0]),
            Race::HalfElf => StatBlock::from([-116, 1, 0, 1, -1, 1]),
            Race::Elf => StatBlock::from([-1, 2, 1, 1, -2, 1]),
            Race::Halfling => StatBlock::from([-2, 2, 1, 3, 1, 1]),
            Race::Gnome => StatBlock::from([-1, 2, 0, 2, 1, -2]),
            Race::Dwarf => StatBlock::from([2, -3, 1, -2, 2, -3]),
            Race::HalfOrc => StatBlock::from([2, -1, 0, 0, 1, -4]),
            Race::HalfTroll => StatBlock::from([ 4, -4, -3, -4,  4, -6]),
            Race::Phraint => StatBlock::from([ 0,  0, -4,  5,  0, -3]),
            Race::Dryad => StatBlock::from([-1,  0,  3,  0, -2,  3]),
        }
    }

    pub fn available_classes(&self) -> Vec<Class> {
        match self {
            Race::Human => vec![
                Class::Warrior, Class::Mage, Class::Priest, Class::Rogue,
                Class::Ranger, Class::Paladin, Class::Druid, Class::Bard,
                Class::Adventurer, Class::Monk,
            ],
            Race::HalfElf => vec![
                Class::Warrior, Class::Mage, Class::Priest, Class::Rogue,
                Class::Ranger, Class::Paladin, Class::Druid, Class::Bard,
                Class::Adventurer, Class::Monk,
            ],
            Race::Elf => vec![
                Class::Warrior, Class::Mage, Class::Priest, Class::Rogue,
                Class::Ranger, Class::Druid, Class::Bard, Class::Adventurer,
            ],
            Race::Halfling => vec![
                Class::Rogue, Class::Druid, Class::Bard, Class::Adventurer,
                Class::Monk,
            ],
            Race::Gnome => vec![
                Class::Mage, Class::Priest, Class::Rogue, Class::Druid,
            ],
            Race::Dwarf => vec![
                Class::Warrior, Class::Priest, Class::Druid,
            ],
            Race::HalfOrc => vec![
                Class::Warrior, Class::Priest, Class::Rogue, Class::Monk,
            ],
            Race::HalfTroll => vec![
                Class::Warrior, Class::Priest,
            ],
            Race::Phraint => vec![
                Class::Warrior, Class::Mage, Class::Rogue, Class::Ranger,
                Class::Bard, Class::Adventurer, Class::Monk,
            ],
            Race::Dryad => vec![
                Class::Priest, Class::Ranger, Class::Druid, Class::Bard,
                Class::Monk,
            ],
        }
    }
}

impl From<usize> for Race {
    fn from(pos: usize) -> Self {
        match pos {
            0 => Race::Human,
            1 => Race::HalfElf,
            2 => Race::Elf,
            3 => Race::Halfling,
            4 => Race::Gnome,
            5 => Race::Dwarf,
            6 => Race::HalfOrc,
            7 => Race::HalfTroll,
            8 => Race::Phraint,
            9 => Race::Dryad,
            _ => panic!("pos out of range")
        }
    }
}

pub fn races_iter() -> Range<usize> {
    (Race::Human as usize)..(Race::Dryad as usize + 1)
}

