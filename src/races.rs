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
