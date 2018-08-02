use random;

use types::Race;

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

