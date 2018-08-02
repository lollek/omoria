use std::ops::Range;

use random;

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

#[no_mangle]
pub extern fn race_disarm_mod(race: i32) -> i8 {
    match race {
        x if x == Race::Human as i32 => 0,
        x if x == Race::HalfElf as i32 => 2,
        x if x == Race::Elf as i32 => 5,
        x if x == Race::Halfling as i32 => 15,
        x if x == Race::Gnome as i32 => 10,
        x if x == Race::Dwarf as i32 => 2,
        x if x == Race::HalfOrc as i32 => -3,
        x if x == Race::HalfTroll as i32 => -5,
        x if x == Race::Phraint as i32 => 15,
        x if x == Race::Dryad as i32 => 2,
        _ => panic!("Unknown race received"),
    }
}

#[no_mangle]
pub extern fn race_rand_starting_age(race: i32) -> u32 {
    (match race {
        x if x == Race::Human as i32 => 12 + random::randint(6),
        x if x == Race::HalfElf as i32 => 24 + random::randint(16),
        x if x == Race::Elf as i32 => 75 + random::randint(75),
        x if x == Race::Halfling as i32 => 21 + random::randint(12),
        x if x == Race::Gnome as i32 => 50 + random::randint(40),
        x if x == Race::Dwarf as i32 => 35 + random::randint(15),
        x if x == Race::HalfOrc as i32 => 11 + random::randint(4),
        x if x == Race::HalfTroll as i32 => 20 + random::randint(10),
        x if x == Race::Phraint as i32 => 15 + random::randint(10),
        x if x == Race::Dryad as i32 => 75 + random::randint(75),
        _ => panic!("Unknown race received"),
    }) as u32
}

#[no_mangle]
pub extern fn race_rand_starting_height(race: i32, male: u8) -> u32 {
    (match race {
        x if x == Race::Human as i32 =>
            if male != 0 { random::randnor(72, 6) } else { random::randnor(66, 4) },
        x if x == Race::HalfElf as i32 =>
            if male != 0 { random::randnor(66, 6) } else { random::randnor(62, 6) },
        x if x == Race::Elf as i32 =>
            if male != 0 { random::randnor(60, 4) } else { random::randnor(54, 4) },
        x if x == Race::Halfling as i32 =>
            if male != 0 { random::randnor(36, 3) } else { random::randnor(33, 3) },
        x if x == Race::Gnome as i32 =>
            if male != 0 { random::randnor(42, 3) } else { random::randnor(39, 3) },
        x if x == Race::Dwarf as i32 =>
            if male != 0 { random::randnor(48, 3) } else { random::randnor(46, 3) },
        x if x == Race::HalfOrc as i32 =>
            if male != 0 { random::randnor(66, 1) } else { random::randnor(62, 3) },
        x if x == Race::HalfTroll as i32 =>
            if male != 0 { random::randnor(96, 10) } else { random::randnor(84, 12) },
        x if x == Race::Phraint as i32 =>
            if male != 0 { random::randnor(96, 24) } else { random::randnor(84, 12) },
        x if x == Race::Dryad as i32 =>
            if male != 0 { random::randnor(60, 4) } else { random::randnor(40, 4) },
        _ => panic!("Unknown race received"),
    }) as u32
}

#[no_mangle]
pub extern fn race_weight_base(race: i32, male: u8) -> u32 {
    (match race {
        x if x == Race::Human as i32 =>
            if male != 0 { 180 } else { 120 },
        x if x == Race::HalfElf as i32 =>
            if male != 0 { 130 } else { 100 },
        x if x == Race::Elf as i32 =>
            if male != 0 { 100 } else { 80 },
        x if x == Race::Halfling as i32 =>
            if male != 0 { 60 } else { 50 },
        x if x == Race::Gnome as i32 =>
            if male != 0 { 90 } else { 75 },
        x if x == Race::Dwarf as i32 =>
            if male != 0 { 150 } else { 120 },
        x if x == Race::HalfOrc as i32 =>
            if male != 0 { 150 } else { 120 },
        x if x == Race::HalfTroll as i32 =>
            if male != 0 { 300 } else { 260 },
        x if x == Race::Phraint as i32 =>
            if male != 0 { 100 } else { 95 },
        x if x == Race::Dryad as i32 =>
            if male != 0 { 85 } else { 70 },
        _ => panic!("Unknown race received"),
    }) as u32
}

#[no_mangle]
pub extern fn race_weight_modifier(race: i32, male: u8) -> u32 {
    (match race {
        x if x == Race::Human as i32 =>
            if male != 0 { 25 } else { 20 },
        x if x == Race::HalfElf as i32 =>
            if male != 0 { 15 } else { 10 },
        x if x == Race::Elf as i32 =>
            6,
        x if x == Race::Halfling as i32 =>
            3,
        x if x == Race::Gnome as i32 =>
            if male != 0 { 6 } else { 3 },
        x if x == Race::Dwarf as i32 =>
            10,
        x if x == Race::HalfOrc as i32 =>
            5,
        x if x == Race::HalfTroll as i32 =>
            if male != 0 { 50 } else { 40 },
        x if x == Race::Phraint as i32 =>
            if male != 0 { 20 } else { 16 },
        x if x == Race::Dryad as i32 =>
            6,
        _ => panic!("Unknown race received"),
    }) as u32
}

#[no_mangle]
pub extern fn race_rand_starting_weight(race: i32, male: u8) -> u32 {
    random::randnor(
        race_weight_base(race, male) as i64,
        race_weight_modifier(race, male) as i64
        ) as u32
}

