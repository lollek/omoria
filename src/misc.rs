use std::ffi::CStr;

use crate::conversion;
use crate::data;
use crate::io;
use crate::logic;
use crate::model::{Damage, Name};
use crate::player;
use crate::term;

pub const BTH_LEV_ADJ: i16 = 3; // Adjust BTH per level
pub const BTH_PLUS_ADJ: i16 = 3; // Adjust BTH per plus-to-hit

pub fn max_allowable_weight() -> u16 {
    let player_race = player::race();
    let player_sex = player::sex();

    data::race::weight_base(&player_race, &player_sex)
        + 4 * data::race::weight_modifier(&player_race, &player_sex)
}

pub fn min_allowable_weight() -> u16 {
    let player_race = player::race();
    let player_sex = player::sex();

    data::race::weight_base(&player_race, &player_sex)
        - 4 * data::race::weight_modifier(&player_race, &player_sex)
}

// Hack for converting c-array of chars to rust string
pub fn c_i8_array_to_rust_string(array: Vec<i8>) -> String {
    let safe_array = array
        .to_owned()
        .iter_mut()
        .map(|i| i.to_owned() as u8)
        .collect::<Vec<u8>>();
    c_array_to_rust_string(safe_array)
}

// Hack for converting c-array of chars to rust string
pub fn c_array_to_rust_string(array: Vec<u8>) -> String {
    let safe_array = array
        .to_owned()
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
        i if i < 0 => "Very Bad",
        0 | 1 => "Bad",
        2 => "Poor",
        3 | 4 => "Fair",
        5 => "Good",
        6 => "Very Good",
        7 | 8 => "Suberb",
        _ => "Excellent",
    }
}

pub fn print_known_spells() {
    term::clear_from(1);
    term::prt("   Name                         Level  Mana   Known", 1, 1);
    for i in 0..40 {
        let spell = conversion::spell::from_usize_or_blank(player::class(), i);
        let player_knows_spell = if player::knows_spell(i) {
            "true"
        } else {
            "false"
        };
        term::prt(
            &format!(
                "    {:30}{:2}      {:2}   {}",
                spell.name, spell.level, spell.mana, player_knows_spell
            ),
            (2 + i) as i32,
            0,
        );
    }
    term::prt("[Press any key to continue]", 42, 20);
    io::inkey_flush();
}

pub fn rs2item_damage(damage_string: &str) -> Damage {
    const DAMAGE_SIZE: usize = 7;

    let mut damage: [i8; DAMAGE_SIZE] = [0; DAMAGE_SIZE];
    for (index, c) in damage_string.chars().take(DAMAGE_SIZE - 1).enumerate() {
        damage[index] = c as i8;
    }
    damage
}

pub fn rs2item_name(name_string: &str) -> Name {
    const NAME_SIZE: usize = 70;

    let mut name: [i8; NAME_SIZE] = [0; NAME_SIZE];
    for (index, c) in name_string.chars().take(NAME_SIZE - 1).enumerate() {
        name[index] = c as i8;
    }
    name
}

#[no_mangle]
pub extern "C" fn reset_total_cash() {
    let mut player_money = player::wallet();
    player_money.total = logic::wallet::calculate_total(&player_money);
    player::set_wallet(&player_money);

    let mut bank = player::bank_wallet();
    bank.total = logic::wallet::calculate_total(&bank);
    player::set_bank_wallet(&bank);
}
