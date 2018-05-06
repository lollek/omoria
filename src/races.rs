use random::randint;
use random::randnor;

#[derive(PartialEq)]
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

static HUMAN_STRING: &'static [u8] = b"Human\0";
static HALFELF_STRING: &'static [u8] = b"Half-Elf\0";
static ELF_STRING: &'static [u8] = b"Elf\0";
static HALFLING_STRING: &'static [u8] = b"Halfling\0";
static GNOME_STRING: &'static [u8] = b"Gnome\0";
static DWARF_STRING: &'static [u8] = b"Dwarf\0";
static HALFORC_STRING: &'static [u8] = b"Half-Orc\0";
static HALFTROLL_STRING: &'static [u8] = b"Half-Troll\0";
static PHRAINT_STRING: &'static [u8] = b"Phraint\0";
static DRYAD_STRING: &'static [u8] = b"Dryad\0";

static RACE_STATS: &'static [[i8; 6]] = &[
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

#[no_mangle]
pub extern fn race_name(race: i32) -> *const u8 {
    (match race {
        x if x == Race::Human as i32 => HUMAN_STRING,
        x if x == Race::HalfElf as i32 => HALFELF_STRING,
        x if x == Race::Elf as i32 => ELF_STRING,
        x if x == Race::Halfling as i32 => HALFLING_STRING,
        x if x == Race::Gnome as i32 => GNOME_STRING,
        x if x == Race::Dwarf as i32 => DWARF_STRING,
        x if x == Race::HalfOrc as i32 => HALFORC_STRING,
        x if x == Race::HalfTroll as i32 => HALFTROLL_STRING,
        x if x == Race::Phraint as i32 => PHRAINT_STRING,
        x if x == Race::Dryad as i32 => DRYAD_STRING,
        _ => panic!("Unknown race received"),
    }).as_ptr()
}

#[no_mangle]
pub extern fn race_stats(race: i32) -> *const i8 {
    RACE_STATS[race as usize].as_ptr()
}


#[no_mangle]
pub extern fn race_expfactor(race: i32) -> f32 {
    match race {
        x if x == Race::Human as i32 => 1.00,
        x if x == Race::HalfElf as i32 => 1.10,
        x if x == Race::Elf as i32 => 1.20,
        x if x == Race::Halfling as i32 => 1.10,
        x if x == Race::Gnome as i32 => 1.25,
        x if x == Race::Dwarf as i32 => 1.20,
        x if x == Race::HalfOrc as i32 => 1.10,
        x if x == Race::HalfTroll as i32 => 1.20,
        x if x == Race::Phraint as i32 => 1.20,
        x if x == Race::Dryad as i32 => 1.20,
        _ => panic!("Unknown race received"),
    }
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
pub extern fn race_search_mod(race: i32) -> i8 {
    match race {
        x if x == Race::Human as i32 => 0,
        x if x == Race::HalfElf as i32 => 6,
        x if x == Race::Elf as i32 => 8,
        x if x == Race::Halfling as i32 => 12,
        x if x == Race::Gnome as i32 => 6,
        x if x == Race::Dwarf as i32 => 7,
        x if x == Race::HalfOrc as i32 => 0,
        x if x == Race::HalfTroll as i32 => -1,
        x if x == Race::Phraint as i32 => 10,
        x if x == Race::Dryad as i32 => 6,
        _ => panic!("Unknown race received"),
    }
}

#[no_mangle]
pub extern fn race_stealth_mod(race: i32) -> i8 {
    match race {
        x if x == Race::Human as i32 => 0,
        x if x == Race::HalfElf as i32 => 1,
        x if x == Race::Elf as i32 => 1,
        x if x == Race::Halfling as i32 => 4,
        x if x == Race::Gnome as i32 => 3,
        x if x == Race::Dwarf as i32 => 0,
        x if x == Race::HalfOrc as i32 => -1,
        x if x == Race::HalfTroll as i32 => -2,
        x if x == Race::Phraint as i32 => 5,
        x if x == Race::Dryad as i32 => 1,
        _ => panic!("Unknown race received"),
    }
}

#[no_mangle]
pub extern fn race_search_freq(race: i32) -> i8 {
    match race {
        x if x == Race::Human as i32 => 0,
        x if x == Race::HalfElf as i32 => -1,
        x if x == Race::Elf as i32 => -1,
        x if x == Race::Halfling as i32 => -5,
        x if x == Race::Gnome as i32 => -3,
        x if x == Race::Dwarf as i32 => 0,
        x if x == Race::HalfOrc as i32 => 3,
        x if x == Race::HalfTroll as i32 => 5,
        x if x == Race::Phraint as i32 => 3,
        x if x == Race::Dryad as i32 => -1,
        _ => panic!("Unknown race received"),
    }
}

#[no_mangle]
pub extern fn race_melee_bonus(race: i32) -> i8 {
    match race {
        x if x == Race::Human as i32 => 0,
        x if x == Race::HalfElf as i32 => 0,
        x if x == Race::Elf as i32 => -5,
        x if x == Race::Halfling as i32 => -10,
        x if x == Race::Gnome as i32 => -8,
        x if x == Race::Dwarf as i32 => 15,
        x if x == Race::HalfOrc as i32 => 12,
        x if x == Race::HalfTroll as i32 => 20,
        x if x == Race::Phraint as i32 => 3,
        x if x == Race::Dryad as i32 => -0,
        _ => panic!("Unknown race received"),
    }
}

#[no_mangle]
pub extern fn race_ranged_bonus(race: i32) -> i8 {
    match race {
        x if x == Race::Human as i32 => 0,
        x if x == Race::HalfElf as i32 => 5,
        x if x == Race::Elf as i32 => 15,
        x if x == Race::Halfling as i32 => 20,
        x if x == Race::Gnome as i32 => 12,
        x if x == Race::Dwarf as i32 => 0,
        x if x == Race::HalfOrc as i32 => -5,
        x if x == Race::HalfTroll as i32 => -10,
        x if x == Race::Phraint as i32 => 5,
        x if x == Race::Dryad as i32 => 5,
        _ => panic!("Unknown race received"),
    }
}

#[no_mangle]
pub extern fn race_save_mod(race: i32) -> i8 {
    match race {
        x if x == Race::Human as i32 => 0,
        x if x == Race::HalfElf as i32 => 3,
        x if x == Race::Elf as i32 => 6,
        x if x == Race::Halfling as i32 => 18,
        x if x == Race::Gnome as i32 => 12,
        x if x == Race::Dwarf as i32 => 9,
        x if x == Race::HalfOrc as i32 => -3,
        x if x == Race::HalfTroll as i32 => -9,
        x if x == Race::Phraint as i32 => -3,
        x if x == Race::Dryad as i32 => 3,
        _ => panic!("Unknown race received"),
    }
}

#[no_mangle]
pub extern fn race_health_bonus(race: i32) -> i8 {
    match race {
        x if x == Race::Human as i32 => 10,
        x if x == Race::HalfElf as i32 => 9,
        x if x == Race::Elf as i32 => 8,
        x if x == Race::Halfling as i32 => 6,
        x if x == Race::Gnome as i32 => 7,
        x if x == Race::Dwarf as i32 => 9,
        x if x == Race::HalfOrc as i32 => 10,
        x if x == Race::HalfTroll as i32 => 12,
        x if x == Race::Phraint as i32 => 8,
        x if x == Race::Dryad as i32 => 7,
        _ => panic!("Unknown race received"),
    }
}

#[no_mangle]
pub extern fn race_infravision(race: i32) -> i8 {
    match race {
        x if x == Race::Human as i32 => 0,
        x if x == Race::HalfElf as i32 => 0,
        x if x == Race::Elf as i32 => 0,
        x if x == Race::Halfling as i32 => 4,
        x if x == Race::Gnome as i32 => 3,
        x if x == Race::Dwarf as i32 => 5,
        x if x == Race::HalfOrc as i32 => 3,
        x if x == Race::HalfTroll as i32 => 3,
        x if x == Race::Phraint as i32 => 5,
        x if x == Race::Dryad as i32 => 3,
        _ => panic!("Unknown race received"),
    }
}

#[no_mangle]
pub extern fn race_swim_speed(race: i32) -> i8 {
    match race {
        x if x == Race::Human as i32 => 0,
        x if x == Race::HalfElf as i32 => 1,
        x if x == Race::Elf as i32 => 2,
        x if x == Race::Halfling as i32 => -2,
        x if x == Race::Gnome as i32 => -1,
        x if x == Race::Dwarf as i32 => -2,
        x if x == Race::HalfOrc as i32 => 0,
        x if x == Race::HalfTroll as i32 => 2,
        x if x == Race::Phraint as i32 => -1,
        x if x == Race::Dryad as i32 => -1,
        _ => panic!("Unknown race received"),
    }
}

#[no_mangle]
pub extern fn race_rand_starting_age(race: i32) -> u32 {
    (match race {
        x if x == Race::Human as i32 => 12 + randint(6),
        x if x == Race::HalfElf as i32 => 24 + randint(16),
        x if x == Race::Elf as i32 => 75 + randint(75),
        x if x == Race::Halfling as i32 => 21 + randint(12),
        x if x == Race::Gnome as i32 => 50 + randint(40),
        x if x == Race::Dwarf as i32 => 35 + randint(15),
        x if x == Race::HalfOrc as i32 => 11 + randint(4),
        x if x == Race::HalfTroll as i32 => 20 + randint(10),
        x if x == Race::Phraint as i32 => 15 + randint(10),
        x if x == Race::Dryad as i32 => 75 + randint(75),
        _ => panic!("Unknown race received"),
    }) as u32
}

#[no_mangle]
pub extern fn race_rand_starting_height(race: i32, male: u8) -> u32 {
    (match race {
        x if x == Race::Human as i32 =>
            if male != 0 { randnor(72, 6) } else { randnor(66, 4) },
        x if x == Race::HalfElf as i32 =>
            if male != 0 { randnor(66, 6) } else { randnor(62, 6) },
        x if x == Race::Elf as i32 =>
            if male != 0 { randnor(60, 4) } else { randnor(54, 4) },
        x if x == Race::Halfling as i32 =>
            if male != 0 { randnor(36, 3) } else { randnor(33, 3) },
        x if x == Race::Gnome as i32 =>
            if male != 0 { randnor(42, 3) } else { randnor(39, 3) },
        x if x == Race::Dwarf as i32 =>
            if male != 0 { randnor(48, 3) } else { randnor(46, 3) },
        x if x == Race::HalfOrc as i32 =>
            if male != 0 { randnor(66, 1) } else { randnor(62, 3) },
        x if x == Race::HalfTroll as i32 =>
            if male != 0 { randnor(96, 10) } else { randnor(84, 12) },
        x if x == Race::Phraint as i32 =>
            if male != 0 { randnor(96, 24) } else { randnor(84, 12) },
        x if x == Race::Dryad as i32 =>
            if male != 0 { randnor(60, 4) } else { randnor(40, 4) },
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
    randnor(
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
