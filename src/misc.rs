use std::ffi::CStr;

use io;
use magic;
use player;
use term;

pub const BTH_LEV_ADJ: i16 = 3; // Adjust BTH per level
pub const BTH_PLUS_ADJ: i16 = 3; // Adjust BTH per plus-to-hit

pub fn max_allowable_weight() -> u16 {
    let player_race = player::race();
    let player_sex = player::sex();

    player_race.weight_base(player_sex) +
        4 * player_race.weight_modifier(player_sex)
}

pub fn min_allowable_weight() -> u16 {
    let player_race = player::race();
    let player_sex = player::sex();

    player_race.weight_base(player_sex) -
        4 * player_race.weight_modifier(player_sex)
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

pub fn print_known_spells() {
    term::clear_from(1);
    term::prt("   Name                         Level  Mana   Known", 1, 1);
    for i in magic::spells_iter() {
        let spell = magic::spell(i);
        let player_knows_spell = if player::knows_spell(i) {
            "true"
        } else {
            "false"
        };
        term::prt(&format!("    {:30}{:2}      {:2}   {}",
        spell.name,
        spell.level,
        spell.mana,
        player_knows_spell),
        (2 + i) as i32,
        0);
    }
    term::prt("[Press any key to continue]", 42, 20);
    io::inkey_flush();
}

