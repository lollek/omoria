use std::cmp::min;
use std::str;

use crate::{conversion, user_interface};
use crate::data;
use crate::io;
use crate::logic::menu;
use crate::misc;
use crate::player;
use crate::term;

use super::logic::*;
use crate::model::{Class, Race, Sex};
use crate::pregame::create_character::model::StatsFromRace;

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

pub fn change_name() {
    term::clear_screen();
    user_interface::character_screen();
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
