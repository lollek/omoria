use std::{
    cmp::{max, min},
    ffi::CString,
};

use libc::strcpy;

use crate::{
    data,
    model::{GameTime, Race, Stat, StatBlock},
    player, random,
};

/**
 * Returns a random starting stat value. i.e. in the range [3, 18]
 */
pub(super) fn random_starting_stat() -> i16 {
    let mut stats = vec![
        random::randint(6),
        random::randint(6),
        random::randint(6),
        random::randint(6),
    ];
    stats.sort();
    stats.iter().skip(1).fold(0, |sum, i| sum + i) as i16
}

pub(super) fn random_starting_stat_block(race_stats: &StatBlock) -> StatBlock {
    StatBlock {
        strength: random_starting_stat() + race_stats.get(Stat::Strength),
        intelligence: random_starting_stat() + race_stats.get(Stat::Intelligence),
        wisdom: random_starting_stat() + race_stats.get(Stat::Wisdom),
        dexterity: random_starting_stat() + race_stats.get(Stat::Dexterity),
        constitution: random_starting_stat() + race_stats.get(Stat::Constitution),
        charisma: random_starting_stat() + race_stats.get(Stat::Charisma),
    }
}

pub(super) fn generate_player_stats() {
    let race = player::race();
    let race_stats = data::race::stat_block(&race);
    let new_stats = random_starting_stat_block(&race_stats);

    player::set_perm_stats(new_stats);
    player::recalc_curr_stats();

    unsafe {
        player::player_rep = 0;
        player::player_bth = data::race::melee_bonus(&race) as i16;
        player::player_bthb = data::race::ranged_bonus(&race) as i16;
        player::player_fos = data::race::search_freq(&race) as i16;
        player::player_stl = data::race::stealth_mod(&race) as i16;
        player::player_save = data::race::save_mod(&race) as i16;
        player::player_lev = 1;
        player::player_ptodam = player::dmg_from_str();
        player::player_ptohit = player::tohit_from_stats();
        player::player_ptoac = 0;
        player::player_pac = player::ac_from_dex();
        player::player_expfact = data::race::expfactor(&race);
    }
    player::set_infravision(data::race::infravision(&race) as i64);
    player::set_swim_speed(data::race::swim_speed(&race) as i64);
}

// Get the racial history, and determines social class
pub(super) fn generate_player_history() {
    let mut history: String = String::new();
    let mut social_class: i16 = 0;

    for list in super::data::history::for_race(player::race()).iter() {
        let roll = random::randint(100) as u8;
        for triple in list {
            if triple.chance >= roll {
                history += triple.history;
                social_class += triple.social_class;
                break;
            }
        }
    }

    // Process block of history text for pretty output
    for i in 0..5 {
        unsafe { player::player_history[i][0] = '\0' as i8 };
    }

    let mut i: usize = 0;
    let mut tmp_str: String = String::new();
    let mut history_words_iter = history.split_whitespace();
    while let Some(word) = history_words_iter.next() {
        let tmp_str_len = tmp_str.len();
        let word_len = word.len();

        if tmp_str_len + word_len > 70 {
            let cstr = CString::new(tmp_str).unwrap();
            unsafe {
                strcpy(player::player_history[i].as_mut_ptr(), cstr.as_ptr());
            }
            i += 1;
            tmp_str = String::new();
        }
        tmp_str += word;
        tmp_str += " ";
    }

    let cstr = CString::new(tmp_str).unwrap();
    unsafe {
        strcpy(player::player_history[i].as_mut_ptr(), cstr.as_ptr());
    }

    social_class = min(max(social_class, 1), 100);

    unsafe {
        player::player_rep = (50 - social_class).into();
        player::player_sc = social_class;
    }
}

fn generate_player_age(player_race: Race) -> u16 {
    match player_race {
        Race::Human => 12 + random::randint(6) as u16,
        Race::HalfElf => 24 + random::randint(16) as u16,
        Race::Elf => 75 + random::randint(75) as u16,
        Race::Halfling => 21 + random::randint(12) as u16,
        Race::Gnome => 50 + random::randint(40) as u16,
        Race::Dwarf => 35 + random::randint(15) as u16,
        Race::HalfOrc => 11 + random::randint(4) as u16,
        Race::HalfTroll => 20 + random::randint(10) as u16,
        Race::Phraint => 15 + random::randint(10) as u16,
        Race::Dryad => 75 + random::randint(75) as u16,
    }
}

// Computes character's age, height, and weight -JWT-
pub(super) fn regenerate_player_ahw() {
    let player_race = player::race();
    let player_sex = player::sex();

    unsafe {
        player::player_age = generate_player_age(player_race);
    }

    let mut player_birthdate: GameTime = GameTime::new();
    player_birthdate.year = 500 + random::randint(50);
    player_birthdate.month = random::randint(13) as u8;
    player_birthdate.day = random::randint(28) as u8;
    player_birthdate.hour = (random::randint(24) - 1) as u8;
    player_birthdate.secs = (random::randint(400) - 1) as u16;
    player::set_birthdate(player_birthdate);

    let mut player_age: GameTime = GameTime::new();
    player_age.year = unsafe { player::player_age } as i64 + player_birthdate.year;
    player_age.month = player_birthdate.month;
    player_age.day = player_birthdate.day + 1;
    player_age.hour = 7u8;
    player_age.secs = (300 + random::randint(99)) as u16;
    player::set_age(player_age);

    unsafe {
        player::player_ht = super::data::height::generate(player_race, player_sex);
        player::player_wt = super::data::weight::generate(player_race, player_sex);

        player::player_disarm =
            (data::race::disarm_mod(&player_race) as i16 + player::disarm_from_dex()).into();
    }
}

pub(super) fn regenerate_player_stats() {
    generate_player_stats();
    generate_player_history();
    regenerate_player_ahw();

    unsafe {
        player::player_dis_th = player::tohit_from_stats().into();
        player::player_dis_td = player::dmg_from_str().into();
        player::player_dis_tac = player::ac_from_dex().into();
    }
}
