use std::ops::Range;

use random;

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

pub static RACE_STATS: &'static [[i8; 6]] = &[
    [ 0,  0,  0,  0,  0,  0],   // Human
    [-1,  1,  0,  1, -1,  1],   // Half-Elf
    [-1,  2,  1,  1, -2,  1],   // Elf
    [-2,  2,  1,  3,  1,  1],   // Halfling
    [-1,  2,  0,  2,  1, -2],   // Gnome
    [ 2, -3,  1, -2,  2, -3],   // Dwarf
    [ 2, -1,  0,  0,  1, -4],   // Half-Orc
    [ 4, -4, -3, -4,  4, -6],   // Half-Troll
    [ 0,  0, -4,  5,  0, -3],   // Phraint
    [-1,  0,  3,  0, -2,  3],   // Dryad
];

pub const SEARCH_MOD: [i8; 10] = [0, 6, 8, 12, 6, 7, 0, -1, 10, 6];
pub const MELEE_BONUS: [i8; 10] = [0, 0, -5, -10, -8, 15, 12, 20, 3, 0];
pub const RANGED_BONUS: [i8; 10] = [0, 5, 15, 20, 12, 0, -5, -10, 5, 5];
pub const SEARCH_FREQ: [i8; 10] = [0, -1, -1, -5, -3, 0, 3, 5, 3, -1];
pub const STEALTH_MOD: [i8; 10] = [0, 1, 1, 4, 3, 0, -1, -2, 5, 1];
pub const SAVE_MOD: [i8; 10] = [0, 3, 6, 18, 12, 9, -3, -9, -3, 3];
pub const HEALTH_BONUS: [u8; 10] = [10, 9, 8, 6, 7, 9, 10, 12, 8, 7];
pub const EXPFACTOR: [f32; 10] = [1.00, 1.10, 1.20, 1.10, 1.25, 1.20, 1.10, 1.20, 1.20, 1.20];
pub const INFRAVISION: [i8; 10] = [0, 0, 0, 4, 3, 5, 3, 3, 5, 3];
pub const SWIM_SPEED: [i8; 10] = [0, 1, 2, -2, -1, -2, 0, 2, -1, -1];

#[no_mangle]
pub extern fn race_name(race: i32) -> *const u8 {
    match race {
        x if x == Race::Human as i32 => b"Human\0".as_ptr(),
        x if x == Race::HalfElf as i32 => b"Half-Elf\0".as_ptr(),
        x if x == Race::Elf as i32 => b"Elf\0".as_ptr(),
        x if x == Race::Halfling as i32 => b"Halfling\0".as_ptr(),
        x if x == Race::Gnome as i32 => b"Gnome\0".as_ptr(),
        x if x == Race::Dwarf as i32 => b"Dwarf\0".as_ptr(),
        x if x == Race::HalfOrc as i32 => b"Half-Orc\0".as_ptr(),
        x if x == Race::HalfTroll as i32 => b"Half-Troll\0".as_ptr(),
        x if x == Race::Phraint as i32 => b"Phraint\0".as_ptr(),
        x if x == Race::Dryad as i32 => b"Dryad\0".as_ptr(),
        _ => panic!("Unknown race received"),
    }
}

#[no_mangle]
pub extern fn race_stats(race: i32) -> *const i8 {
    RACE_STATS[race as usize].as_ptr()
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
pub extern fn race_stealth_mod(race: i32) -> i8 {
    STEALTH_MOD[race as usize]
}

#[no_mangle]
pub extern fn race_search_freq(race: i32) -> i8 {
    SEARCH_FREQ[race as usize]
}

#[no_mangle]
pub extern fn race_ranged_bonus(race: i32) -> i8 {
    RANGED_BONUS[race as usize]
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

#[no_mangle]
pub extern fn race_class_field(race: i32) -> u64 {
    (match race {
        x if x == Race::Human as i32 => 0x3FF,
        x if x == Race::HalfElf as i32 => 0x3FF,
        x if x == Race::Elf as i32 => 0x1DF,
        x if x == Race::Halfling as i32 => 0x2BA,
        x if x == Race::Gnome as i32 => 0x04E,
        x if x == Race::Dwarf as i32 => 0x045,
        x if x == Race::HalfOrc as i32 => 0x20D,
        x if x == Race::HalfTroll as i32 => 0x005,
        x if x == Race::Phraint as i32 => 0x39B,
        x if x == Race::Dryad as i32 => 0x2D4,
        _ => panic!("Unknown race received"),
    }) as u64
}
