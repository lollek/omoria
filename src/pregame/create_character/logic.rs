use libc::{strcpy, time, time_t};
use std::ptr::null;
use std::{
    cmp::{max, min},
    ffi::CString,
};

use crate::generate_item::template::{CloakTemplate, FoodTemplate, LightSourceTemplate};
use crate::logic::stat_modifiers;
use crate::model::{Class, Currency, InventoryItem, Item, Sex};
use crate::pregame::create_character::data::{height, weight};
use crate::pregame::create_character::model::StatsFromRace;
use crate::{
    data, generate_item,
    model::{GameTime, Race, Stat, StatBlock},
    player, random,
};

extern "C" {
    fn add_inven_item(item: Item) -> *mut InventoryItem;
    fn add_money(amount: i64);
}

/**
 * Returns a random starting stat value. i.e. in the range [3, 18]
 */
fn random_starting_stat() -> i16 {
    let mut stats = vec![
        random::randint(6),
        random::randint(6),
        random::randint(6),
        random::randint(6),
    ];
    stats.sort();
    stats.iter().skip(1).fold(0, |sum, i| sum + i) as i16
}

fn generate_player_age(player_race: &Race) -> u16 {
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

fn unsafe_apply_history(race_stats: StatsFromRace) {
    // Process block of history text for pretty output
    for i in 0..5 {
        unsafe { player::player_history[i][0] = '\0' as i8 };
    }

    let mut i: usize = 0;
    let mut tmp_str: String = String::new();
    let mut history_words_iter = race_stats.history.split_whitespace();
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
}

pub(crate) fn apply_stats_from_class(player_class: &Class) {
    player::modify_max_hp(player::hitdie() as i16);
    player::reset_current_hp();
    unsafe {
        player::player_bth += ((data::class::melee_bonus(&player_class) * 5) + 20) as i16;
        player::player_bthb += ((data::class::ranged_bonus(&player_class) * 5) + 20) as i16;
        player::player_disarm += data::class::disarm_mod(&player_class) as i16;
        player::player_fos += data::class::search_freq(&player_class) as i16;
        player::player_stl += data::class::stealth_mod(&player_class) as i16;
        player::player_save += data::class::save_mod(&player_class) as i16;
        player::player_expfact += data::class::expfactor(&player_class);
        player::player_mr = data::class::magic_resist(&player_class).into();
        player::player_creation_time = time(null::<time_t>() as *mut i64);
        player::player_claim_check = 0;

        // Real values
        player::player_ptodam = player::dmg_from_str();
        player::player_ptohit = player::tohit_from_stats();
        player::player_ptoac = player::ac_from_dex();
        player::player_pac = 0;

        // Displayed values
        player::player_dis_td = player::player_ptodam;
        player::player_dis_th = player::player_ptohit;
        player::player_dis_tac = player::player_ptoac;
        player::player_dis_ac = player::player_pac;
    }
}

pub(crate) fn generate_stats_from_race(race: &Race, sex: &Sex) -> StatsFromRace {
    let race_stats = data::race::stat_block(&race);
    let stat_block = StatBlock {
        strength: random_starting_stat() + race_stats.get(Stat::Strength),
        intelligence: random_starting_stat() + race_stats.get(Stat::Intelligence),
        wisdom: random_starting_stat() + race_stats.get(Stat::Wisdom),
        dexterity: random_starting_stat() + race_stats.get(Stat::Dexterity),
        constitution: random_starting_stat() + race_stats.get(Stat::Constitution),
        charisma: random_starting_stat() + race_stats.get(Stat::Charisma),
    };

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
    social_class = min(max(social_class, 1), 100);

    let birthdate = GameTime {
        year: 500 + random::randint(50),
        month: random::randint(13) as u8,
        day: random::randint(28) as u8,
        hour: (random::randint(24) - 1) as u8,
        secs: (random::randint(400) - 1) as u16,
    };

    let age_plain = generate_player_age(race);
    StatsFromRace {
        age_plain,
        age_game_time: GameTime {
            year: birthdate.year + age_plain as i64,
            month: birthdate.month,
            day: birthdate.day + 1,
            hour: 7,
            secs: (300 + random::randint(99)) as u16,
        },
        birthdate,
        disarm_modifier: data::race::disarm_mod(&race) as i16 + stat_modifiers::disarm(&stat_block),
        experience_factor: data::race::expfactor(&race),
        height: height::generate(&race, &sex),
        history,
        infravision: data::race::infravision(&race) as i64,
        melee_bonus: data::race::melee_bonus(&race) as i16,
        ranged_bonus: data::race::ranged_bonus(&race) as i16,
        save_modifier: data::race::save_mod(&race) as i16,
        search_frequency: data::race::search_freq(&race) as i16,
        social_class,
        stat_block,
        stealth_modifier: data::race::stealth_mod(&race) as i16,
        swim_speed: data::race::swim_speed(&race) as i64,
        weight: weight::generate(&race, &sex),
    }
}

pub(crate) fn apply_stats_from_race(race_stats: StatsFromRace) {
    player::set_perm_stats(race_stats.stat_block);
    player::recalc_curr_stats();

    unsafe {
        player::player_rep = 0;
        player::player_bth = race_stats.melee_bonus;
        player::player_bthb = race_stats.ranged_bonus;
        player::player_fos = race_stats.search_frequency;
        player::player_stl = race_stats.stealth_modifier;
        player::player_save = race_stats.save_modifier;
        player::player_lev = 1;
        player::player_ptodam = stat_modifiers::damage(&race_stats.stat_block);
        player::player_ptohit = stat_modifiers::to_hit_bonus(&race_stats.stat_block);
        player::player_ptoac = 0;
        player::player_pac = stat_modifiers::ac(&race_stats.stat_block);
        player::player_expfact = race_stats.experience_factor;
        player::player_rep = (50 - race_stats.social_class).into();
        player::player_sc = race_stats.social_class;
        player::player_age = race_stats.age_plain;
        player::player_ht = race_stats.height;
        player::player_wt = race_stats.weight;
        player::player_disarm = race_stats.disarm_modifier;
        player::player_dis_th = stat_modifiers::to_hit_bonus(&race_stats.stat_block);
        player::player_dis_td = stat_modifiers::damage(&race_stats.stat_block);
        player::player_dis_tac = stat_modifiers::ac(&race_stats.stat_block);
    }
    player::set_birthdate(race_stats.birthdate);
    player::set_age(race_stats.age_game_time);
    player::set_infravision(race_stats.infravision);
    player::set_swim_speed(race_stats.swim_speed);

    unsafe_apply_history(race_stats);
}

pub(crate) fn generate_and_apply_money() {
    let player_stats = player::curr_stats();
    let mut amount: i64 = 325 + random::randint(25)
        // Social Class adj
        + unsafe { player::player_sc } as i64 * 6
        // Stat adj
        - Stat::iter().fold(0, |sum: i64, tstat|
        sum + player_stats.get(tstat) as i64)
        // Charisma adj
        + player_stats.get(Stat::Charisma) as i64;

    // Minimum
    amount = max(amount, 80);

    let gold_value = data::currency::value(&Currency::Gold);
    unsafe { add_money((amount * gold_value) + random::randint(gold_value)) };
}

pub(crate) fn generate_and_apply_equipment() {
    // General starting items

    let mut ration_of_food = generate_item::generate(Box::new(FoodTemplate::RationOfFood), 0);
    ration_of_food.number = 5;

    for item in [
        ration_of_food,
        generate_item::generate(Box::new(LightSourceTemplate::WoodenTorch), 0),
        generate_item::generate(Box::new(CloakTemplate::LightCloak), 0),
        generate_item::generate_boots(10),
        generate_item::generate_belt(10),
    ] {
        unsafe {
            add_inven_item(item);
        }
    }

    // Class specific starting items
    for item in data::class::starting_items(&player::class()) {
        unsafe {
            add_inven_item(item);
        }
    }
}
