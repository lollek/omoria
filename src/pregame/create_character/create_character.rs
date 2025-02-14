use std::cmp::{max, min};
use std::str;

use crate::conversion;
use crate::data;
use crate::io;
use crate::logic::menu;
use crate::misc;
use crate::player;
use crate::screen;
use crate::term;

use super::logic::*;
use crate::model::{Class, Race, Sex, Stat};
use crate::pregame::create_character::model::StatsFromRace;

fn put_character(show_values: bool) {
    term::prt("Name      : ", 2, 2);
    term::prt("Race      : ", 3, 2);
    term::prt("Sex       : ", 4, 2);
    term::prt("Class     : ", 5, 2);

    if show_values {
        term::prt(player::name(), 2, 14);
        term::prt(data::race::name(&player::race()), 3, 14);
        term::prt(data::sex::name(&player::sex()), 4, 14);
        term::prt(data::class::name(&player::class()), 5, 14);
    }
}

fn put_stats() {
    screen::print_stats(2, 64);

    term::prt(
        format!("+ To Hit   : {}", unsafe { player::player_dis_th }),
        9,
        3,
    );
    term::prt(
        format!("+ To Damage: {}", unsafe { player::player_dis_td }),
        10,
        3,
    );
    term::prt(
        format!("+ To AC    : {}", unsafe { player::player_dis_tac }),
        11,
        3,
    );
    term::prt(
        format!("  Total AC : {}", unsafe { player::player_dis_ac }),
        12,
        3,
    );
}

fn put_misc1() {
    term::prt(
        format!("Age          : {}", unsafe { player::player_age }),
        2,
        39,
    );
    term::prt(
        format!("Height       : {}", unsafe { player::player_ht }),
        3,
        39,
    );
    term::prt(
        format!("Weight       : {}", unsafe { player::player_wt }),
        4,
        39,
    );
    term::prt(
        format!("Social Class : {}", unsafe { player::player_sc }),
        5,
        39,
    );
}

fn put_misc2() {
    term::prt(
        format!("Level      : {}", unsafe { player::player_lev }),
        9,
        30,
    );
    term::prt(
        format!("Experience : {}", unsafe { player::player_exp }),
        10,
        30,
    );
    term::prt(format!("Gold       : {}", player::wallet().total), 11, 30);
    term::prt(
        format!("Account    : {}", unsafe { player::player_account }),
        12,
        30,
    );
    term::prt(format!("Max Hit Points : {}", player::max_hp()), 9, 53);
    term::prt(format!("Cur Hit Points : {}", player::current_hp()), 10, 53);
    term::prt(
        format!("Max Mana       : {}", unsafe { player::player_mana }),
        11,
        53,
    );
    term::prt(
        format!("Cur Mana       : {}", unsafe { player::player_cmana }),
        12,
        53,
    );
}

fn put_misc3() {
    term::clear_from(14);

    let xbth: i64 = player::melee_tohit().into();
    let xbthb: i64 = player::ranged_tohit().into();

    let xfos: i64 = max(27 - unsafe { player::player_fos }, 0).into();
    let xsrh: i64 = player::curr_search_skill().into();
    let xstl: i64 = unsafe { player::player_stl }.into();
    let xdis: i64 = unsafe { player::player_disarm } as i64
        + unsafe { player::player_lev } as i64
        + (2 * player::disarm_from_dex()) as i64
        + player::modifier_from_stat(Stat::Intelligence) as i64;
    let xsave: i64 = unsafe { player::player_save } as i64
        + unsafe { player::player_lev } as i64
        + player::modifier_from_stat(Stat::Wisdom) as i64;
    let xdev: i64 = unsafe { player::player_save } as i64
        + unsafe { player::player_lev } as i64
        + player::modifier_from_stat(Stat::Intelligence) as i64;
    let xswm: i64 = player::swim_speed() + 4;
    let xrep: i64 = 6 + (unsafe { player::player_rep } / 25);
    let xinf: i64 = player::infravision() * 10;

    term::prt("(Miscellaneous Abilities)", 15, 23);
    term::put_buffer(
        format!("Fighting    : {}", misc::mod_to_string(xbth, 12)),
        16,
        1,
    );
    term::put_buffer(
        format!("Bows/Throw  : {}", misc::mod_to_string(xbthb, 12)),
        17,
        1,
    );
    term::put_buffer(
        format!("Saving Throw: {}", misc::mod_to_string(xsave, 6)),
        18,
        1,
    );
    term::put_buffer(
        format!("Stealth     : {}", misc::mod_to_string(xstl, 1)),
        16,
        26,
    );
    term::put_buffer(
        format!("Disarming   : {}", misc::mod_to_string(xdis, 8)),
        17,
        26,
    );
    term::put_buffer(
        format!("Magic Device: {}", misc::mod_to_string(xdev, 7)),
        18,
        26,
    );
    term::put_buffer(
        format!("Perception  : {}", misc::mod_to_string(xfos, 3)),
        16,
        51,
    );
    term::put_buffer(
        format!("Searching   : {}", misc::mod_to_string(xsrh, 6)),
        17,
        51,
    );
    term::put_buffer(format!("Infra-Vision: {} feet", xinf), 18, 51);
    term::put_buffer(
        format!("Swimming    : {}", misc::mod_to_string(xswm, 1)),
        19,
        51,
    );
    term::put_buffer(
        format!("Reputation  : {}", misc::mod_to_string(xrep, 1)),
        19,
        1,
    );
}

fn print_history() {
    let history = (0..5)
        .map(|i| {
            let c_hist: Vec<u8> = unsafe { player::player_history[i] }
                .iter()
                .map(|&i| i as u8)
                .collect();
            misc::c_array_to_rust_string(c_hist)
        })
        .collect::<Vec<String>>();

    menu::draw_menu(
        "Your background",
        &history.iter().map(String::as_str).collect::<Vec<&str>>(),
        "enter=confirm",
        255,
    );

    loop {
        match io::inkey_flush() as char {
            '\r' => return,
            _ => {}
        }
    }
}

// Display character on screen -RAK-
fn display_char() {
    put_character(true);
    put_misc1();
    put_stats();
    put_misc2();
    put_misc3();
}

pub fn change_name() {
    term::clear_screen();
    display_char();
    loop {
        term::prt("<c>hange character name.     <ESCAPE> to continue.", 21, 2);
        match io::inkey_flush() {
            99 => choose_name(),
            0 | 3 | 25 | 26 | 27 => break,
            _ => (),
        }
    }
}

/* Gets a name for the character    -JWT- */
fn choose_name() {
    term::prt(
        "Enter your player's name  [press <RETURN> when finished]",
        21,
        2,
    );
    loop {
        let new_name = term::get_string(3, 15, 24);
        if !new_name.is_empty() {
            player::set_name(&new_name);
            break;
        }
    }
    term::clear_from(21);
}

fn choose_class() -> Class {
    let classes = data::race::available_classes(&player::race());
    let class_strings = classes.iter().map(data::class::name).collect::<Vec<&str>>();
    let mut index = 0;

    loop {
        menu::draw_menu(
            "Choose your class",
            &class_strings,
            "j=up, k=down, enter=select, ?=info, r=restrictions",
            index,
        );
        match io::inkey_flush() as char {
            'k' => index = if index == 0 { 0 } else { index - 1 },
            'j' => index = min(classes.len() as u8 - 1, index + 1),
            '\r' => {
                return classes[index as usize];
            }
            '?' => menu::draw_help(
                data::class::name(&classes[index as usize]),
                data::class::info(&classes[index as usize]),
            ),
            'r' => menu::draw_help(
                data::class::name(&classes[index as usize]),
                data::class::restriction_info(&classes[index as usize]),
            ),
            _ => {}
        }
    }
}

pub fn stats_info(race: &Race) -> Vec<String> {
    let stats = data::race::stat_block(race);
    vec![
        format!("Melee bonus:       {}", data::race::melee_bonus(race)),
        format!("Ranged bonus:      {}", data::race::ranged_bonus(race)),
        format!("Experience factor: {}", data::race::expfactor(race)),
        format!("Search frequence:  {}", data::race::search_freq(race)),
        format!("Search modifier:   {}", data::race::search_mod(race)),
        format!("Stealth modifier:  {}", data::race::stealth_mod(race)),
        format!("Save modifier:     {}", data::race::save_mod(race)),
        format!("Disarm modifier:   {}", data::race::disarm_mod(race)),
        format!("Infravision:       {}", data::race::infravision(race)),
        format!("Swim speed:        {}", data::race::swim_speed(race)),
        "ATTRIBUTES:".to_owned(),
        format!("Strength:          {}", stats.strength),
        format!("Dexterity:         {}", stats.dexterity),
        format!("Constituton:       {}", stats.constitution),
        format!("Intelligence:      {}", stats.intelligence),
        format!("Wisdom:            {}", stats.wisdom),
        format!("Charisma:          {}", stats.charisma),
    ]
}

fn choose_race() -> Race {
    let races = Race::iter()
        .map(|i| data::race::name(&Race::from(i)))
        .collect::<Vec<&str>>();
    let mut index = 0;

    loop {
        menu::draw_menu(
            "Choose your race",
            &races,
            "j=up, k=down, enter=select, ?=info, s=stats, c=classes",
            index,
        );

        match io::inkey_flush() as char {
            'k' => index = if index == 0 { 0 } else { index - 1 },
            'j' => index = min(races.len() as u8 - 1, index + 1),
            '\r' => {
                return conversion::race::from_usize(index as usize).unwrap();
            }
            '?' => menu::draw_help(
                races[index as usize],
                data::race::info(&conversion::race::from_usize(index as usize).unwrap()),
            ),
            's' => menu::draw_help_vec(
                races[index as usize],
                &stats_info(&conversion::race::from_usize(index as usize).unwrap())
                    .iter()
                    .map(|it| it.as_ref())
                    .collect::<Vec<&str>>(),
            ),
            'c' => menu::draw_help_vec(
                races[index as usize],
                &data::race::available_classes(
                    &conversion::race::from_usize(index as usize).unwrap(),
                )
                .iter()
                .map(data::class::name)
                .collect::<Vec<&str>>(),
            ),
            _ => {}
        }
    }
}

fn choose_sex() -> Sex {
    if player::race() == Race::Dryad {
        return Sex::Female;
    }

    let mut index = 0;
    loop {
        menu::draw_menu(
            "Choose your sex",
            &vec!["Male", "Female"],
            "j=up, k=down, enter=select",
            index,
        );

        match io::inkey_flush() as char {
            'k' => index = 0,
            'j' => index = 1,
            '\r' => {
                return if index == 0 { Sex::Male } else { Sex::Female };
            }
            _ => {}
        }
    }
}

fn choose_stats(race: &Race, sex: &Sex) -> StatsFromRace {
    loop {
        let stats = generate_stats_from_race(race, sex);

        menu::draw_menu(
            "Roll up your stats",
            &vec![
                format!("Age:           {}", stats.age_plain),
                format!("Height:        {}", stats.height),
                format!("Weight:        {}", stats.weight),
                format!("Social Class:  {}", stats.social_class),
                "".to_string(),
                "(Attributes):".to_owned(),
                format!("Strength:      {}", stats.stat_block.strength),
                format!("Dexterity:     {}", stats.stat_block.dexterity),
                format!("Constitution:  {}", stats.stat_block.constitution),
                format!("Intelligence:  {}", stats.stat_block.intelligence),
                format!("Wisdom:        {}", stats.stat_block.wisdom),
                format!("Charisma:      {}", stats.stat_block.charisma),
            ]
            .iter()
            .map(|it| it.as_ref())
            .collect::<Vec<&str>>(),
            "r=reroll stats, enter=confirm",
            255,
        );
        match io::inkey_flush() as char {
            'r' => continue,
            '\r' => return stats,
            _ => {}
        }
    }
}

fn confirm_character() {
    let stats = player::curr_stats();

    menu::draw_menu(
        "Confirm your character",
        &vec![
            "Name: ".to_string(),
            format!("Race:          {}", data::race::name(&player::race())),
            format!("Sex:           {}", data::sex::name(&player::sex())),
            format!("Class:         {}", data::class::name(&player::class())),
            "".to_string(),
            format!("Hit Points     {}", player::max_hp()),
            format!("Mana           {}", player::max_mp()),
            "".to_string(),
            "(Attributes):".to_string(),
            format!("Strength:      {}", stats.strength),
            format!("Dexterity:     {}", stats.dexterity),
            format!("Constitution:  {}", stats.constitution),
            format!("Intelligence:  {}", stats.intelligence),
            format!("Wisdom:        {}", stats.wisdom),
            format!("Charisma:      {}", stats.charisma),
        ]
        .iter()
        .map(|it| it.as_ref())
        .collect::<Vec<&str>>(),
        "Enter your name, finish with enter",
        255,
    );

    loop {
        let new_name = term::get_string(4, 18, 24);
        if !new_name.is_empty() {
            player::set_name(&new_name);
            break;
        }
    }
}

pub fn create_character() {
    let race = choose_race();
    player::set_race(race);

    let sex = choose_sex();
    player::set_sex(sex);

    let stats_from_race = choose_stats(&race, &sex);
    apply_stats_from_race(stats_from_race);

    print_history();

    let class = choose_class();
    player::set_class(class);
    apply_stats_from_class(&player::class());

    generate_and_apply_money();

    confirm_character(); // And choose name
    generate_and_apply_equipment();
}
