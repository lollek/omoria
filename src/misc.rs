use std::ffi::CStr;

use player;
use random;

use types::Stat;

pub const BTH_LEV_ADJ: i16 = 3; // Adjust BTH per level
pub const BTH_PLUS_ADJ: i16 = 3; // Adjust BTH per plus-to-hit

#[no_mangle]
pub extern fn max_allowable_weight() -> u16 {
    let player_race = player::race();
    let player_sex = player::sex();

    player_race.weight_base(player_sex) +
        4 * player_race.weight_modifier(player_sex)
}

#[no_mangle]
pub extern fn min_allowable_weight() -> u16 {
    let player_race = player::race();
    let player_sex = player::sex();

    player_race.weight_base(player_sex) -
        4 * player_race.weight_modifier(player_sex)
}

// Squish the stat into allowed limits
#[no_mangle]
pub extern fn squish_stat(stat: i32) -> u8 {
    if stat > 250 {
        250
    } else if stat < 0 {
        0
    } else {
        stat as u8
    }
}

// Decreases a stat by one randomized level
#[no_mangle]
pub extern fn de_statp(stat: u8) -> u8 {
    if stat < 11 {
        stat
    } else if stat < 151 {
        10
    } else if stat < 241 {
        let mut duh = random::randint(10) as u8 + 5;
        if stat - duh < 150 {
            duh = stat - 150;
        }
        duh
    } else {
        random::randint(3) as u8
    }
}

// Increases a stat by one randomized level
#[no_mangle]
pub extern fn in_statp(stat: u8) -> u8 {
    if stat < 150 {
        10
    } else if stat < 220 {
        random::randint(25) as u8
    } else if stat < 240 {
        random::randint(10) as u8
    } else if stat < 250 {
        1
    } else {
        0
    }
}

// Hack for converting c-array of chars to rust string
pub fn c_i8_array_to_rust_string(array: Vec<i8>) -> String {
    let safe_array = array.to_owned()
        .iter_mut()
        .map(|i| i.to_owned() as u8)
        .collect::<Vec<u8>>();
    c_array_to_rust_string(safe_array)
}

// Hack for converting c-array of chars to rust string
pub fn c_array_to_rust_string(array: Vec<u8>) -> String {
    let safe_array = array.to_owned()
        .iter_mut()
        .take_while(|i| i != &&0u8)
        .chain([0u8].iter_mut())
        .map(|i| i.to_owned())
        .collect::<Vec<u8>>();

    CStr::from_bytes_with_nul(&safe_array)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn mod_from_stat(stat: Stat) -> u8 {
    match player::curr_stats().get(stat) {
        statval if statval > 249 => 7,
        statval if statval > 239 => 6,
        statval if statval > 219 => 5,
        statval if statval > 199 => 4,
        statval if statval > 149 => 3,
        statval if statval > 109 => 2,
        statval if statval > 39 => 1,
        _ => 0,
    }
}

// Returns a rating of stat depending on base
pub fn mod_to_string(stat: i64, base: i64) -> &'static str {
    match stat / base {
        i if i < 0  => "Very Bad",
        0|1         => "Bad",
        2           => "Poor",
        3|4         => "Fair",
        5           => "Good",
        6           => "Very Good",
        7|8         => "Suberb",
        _           => "Excellent",
    }
}

pub fn stat_to_string(stat: i16) -> String {
    if stat < 0 || stat > 250 {
        panic!(stat)
    }

    if stat > 150 {
        format!("18/{:-2}", stat - 150)
    } else {
        format!("{:2}   ", 3 + stat / 10)
    }
}

