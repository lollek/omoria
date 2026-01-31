use crate::model::Stat;
use crate::{ncurses, player};
use pancurses::A_DIM;

pub fn print_stats(row: u8, col: u8) {
    let curr = player::curr_stats();

    if player::has_lost_stat(Stat::Strength) {
        print_stat("STR : ", curr.strength, &[A_DIM], row, col);
    } else {
        print_stat("STR : ", curr.strength, &[], row, col);
    }

    if player::has_lost_stat(Stat::Dexterity) {
        print_stat("DEX : ", curr.dexterity, &[A_DIM], row + 1, col);
    } else {
        print_stat("DEX : ", curr.dexterity, &[], row + 1, col);
    }

    if player::has_lost_stat(Stat::Constitution) {
        print_stat("CON : ", curr.constitution, &[A_DIM], row + 2, col);
    } else {
        print_stat("CON : ", curr.constitution, &[], row + 2, col);
    }

    if player::has_lost_stat(Stat::Intelligence) {
        print_stat("INT : ", curr.intelligence, &[A_DIM], row + 3, col);
    } else {
        print_stat("INT : ", curr.intelligence, &[], row + 3, col);
    }

    if player::has_lost_stat(Stat::Wisdom) {
        print_stat("WIS : ", curr.wisdom, &[A_DIM], row + 4, col);
    } else {
        print_stat("WIS : ", curr.wisdom, &[], row + 4, col);
    }

    if player::has_lost_stat(Stat::Charisma) {
        print_stat("CHA : ", curr.charisma, &[A_DIM], row + 5, col);
    } else {
        print_stat("CHA : ", curr.charisma, &[], row + 5, col);
    }
}

fn print_stat<S: AsRef<str>>(
    stat_name: S,
    stat: i16,
    attributes: &[pancurses::chtype],
    row: u8,
    col: u8,
) {
    for attribute in attributes {
        ncurses::attron(*attribute);
    }

    let str = format!("{}{:<6}", stat_name.as_ref(), stat);
    let row1 = row.into();
    let col1 = col.into();
    ncurses::mvaddstr(row1, col1, str);

    for attribute in attributes {
        ncurses::attroff(*attribute);
    }
}
