use libc;
use std::borrow::Cow;

use crate::data;
use crate::debug;
use crate::equipment;
use crate::ncurses;
use crate::player;
use crate::term;
use crate::model::Stat;

// Stats Column
const STAT_BLOCK_WIDTH: usize = 14;
const STAT_COL: u8 = 0;

const RACE_ROW: u8 = 1;
const CLASS_ROW: u8 = 2;
const HP_ROW: u8 = 5;
const MANA_ROW: u8 = 6;
const STAT_ROW: u8 = 8; // -> 13

const QUEST_ROW: u8 = 15;
const AC_ROW: u8 = 16;
const GOLD_ROW: u8 = 17;
const BULK_ROW: u8 = 18;
const TIME_ROW: u8 = 20;

// Equipment Column
const EQUIP_COL: u8 = 81;
const EQUIP_ROW: u8 = 1;

// Visible Items Column
const VISIBLE_COL: u8 = 1;
const VISIBLE_ROW: u8 = 30;

// Status Row
const STATUS_ROW: u8 = 23;
const DEPTH_COL: u8 = 61;

extern "C" {
    static dun_level: libc::c_long;
}

fn prt_lost_stat<S>(stat_name: S, stat: i16, row: u8, col: u8)
    where S: AsRef<str>
{
    ncurses::attron(ncurses::A_DIM);

    let str = format!("{}{:<6}", stat_name.as_ref(), stat);
    term::put_buffer(str, row.into(), col.into());

    ncurses::attroff(ncurses::A_DIM);
}

pub fn prt_stat<S>(stat_name: S, stat: i16, row: u8, col: u8)
    where S: AsRef<str>
{
    let str = format!("{}{}", stat_name.as_ref(), stat);
    term::put_buffer(str, row.into(), col.into());
}

pub fn print_stats(row: u8, col: u8) {
    let curr = player::curr_stats();

    if player::has_lost_stat(Stat::Strength) {
        prt_lost_stat("STR : ", curr.strength, row, col);
    } else {
        prt_stat("STR : ", curr.strength, row, col);
    }

    if player::has_lost_stat(Stat::Dexterity) {
        prt_lost_stat("DEX : ", curr.dexterity, row + 1, col);
    } else {
        prt_stat("DEX : ", curr.dexterity, row + 1, col);
    }

    if player::has_lost_stat(Stat::Constitution) {
        prt_lost_stat("CON : ", curr.constitution, row + 2, col);
    } else {
        prt_stat("CON : ", curr.constitution, row + 2, col);
    }

    if player::has_lost_stat(Stat::Intelligence) {
        prt_lost_stat("INT : ", curr.intelligence, row + 3, col);
    } else {
        prt_stat("INT : ", curr.intelligence, row + 3, col);
    }

    if player::has_lost_stat(Stat::Wisdom) {
        prt_lost_stat("WIS : ", curr.wisdom, row + 4, col);
    } else {
        prt_stat("WIS : ", curr.wisdom, row + 4, col);
    }

    if player::has_lost_stat(Stat::Charisma) {
        prt_lost_stat("CHA : ", curr.charisma, row + 5, col);
    } else {
        prt_stat("CHA : ", curr.charisma, row + 5, col);
    }
}

fn print_field<S>(msg: S, row: u8, col: u8)
    where S: AsRef<str>
{
    let msg_formatted = format!("{:<width$}", msg.as_ref(), width=STAT_BLOCK_WIDTH);
    term::put_buffer(msg_formatted, row.into(), col.into());
}

fn print_hp(row: u8, col: u8) {
    let current = player::current_hp();
    let max = player::max_hp();
    let color =
        if current >= max {
            ncurses::color_pair(ncurses::COLOR_GREEN)
        } else if current >= max / 3 {
            ncurses::color_pair(ncurses::COLOR_YELLOW)
        } else {
            ncurses::color_pair(ncurses::COLOR_RED)
        };

    ncurses::attron(color);
    term::put_buffer(format!("HP  : {:>6}", current), row.into(), col.into());
    ncurses::attroff(color);
}

fn print_mana(row: u8, col: u8) {
    if !player::uses_mana() {
        return;
    }

    let current = player::current_mp();
    let max = player::max_mp();
    let color =
        if current >= max {
            ncurses::color_pair(ncurses::COLOR_BLUE)
        } else if current >= max / 3 {
            ncurses::color_pair(ncurses::COLOR_CYAN)
        } else {
            ncurses::color_pair(ncurses::COLOR_MAGENTA)
        };

    ncurses::attron(color);
    term::put_buffer(format!("MANA: {:>6}", current), row.into(), col.into());
    ncurses::attroff(color);
}

fn print_time(row: u8, col: u8) {
    let player_age = player::age();
    let min = player_age.secs / 60;
    let hour = player_age.hour;
    let dow = match player_age.day % 7 {
        0 => "Moonday",
        1 => "Toilday",
        2 => "Wealday",
        3 => "Oathday",
        4 => "Fireday",
        5 => "Starday",
        _ => "Sunday",
    };
    print_field(format!("{:>7} {:02}:{:02}", dow, hour, min), row, col);
}

fn print_depth(row: u8, col: u8) {
    let depth = unsafe { dun_level } * 50;
    let string =
        match depth {
            0 => Cow::from("Town Level      "),
            _ => Cow::from(format!("Depth: {} (feet)", depth)),
        };
    term::put_buffer(string, row.into(), col.into());

}

fn print_equipment(row: u8, col: u8) {
    for (index, slot_i) in equipment::slots_iter().enumerate() {
        let slot = equipment::Slot::from(slot_i);
        let index_char = ('a' as u8 + index as u8) as char;
        let item_name = equipment::get_name(slot);
        let msg = format!("{}) {:<13}: {}", index_char, slot.name(), item_name);
        term::prt(msg, (row + index as u8).into(), col.into());
    }
}

pub fn print_equipment_block() {
    print_equipment(EQUIP_ROW, EQUIP_COL);
}

fn print_stats_column() {
    print_field(data::race::name(&player::race()), RACE_ROW, STAT_COL);
    print_field(data::class::name(&player::class()), CLASS_ROW, STAT_COL);

    print_hp(HP_ROW, STAT_COL);
    print_mana(MANA_ROW, STAT_COL);

    print_stats(STAT_ROW, STAT_COL);

    print_field(format!("QST : {:>6}", player::quests()),
                QUEST_ROW, STAT_COL);
    print_field(format!("AC  : {:>6}", unsafe { player::player_dis_ac }),
                AC_ROW, STAT_COL);
    print_field(format!("Gold: {:>6}", player::wallet().total),
                GOLD_ROW, STAT_COL);
    print_field(format!("Bulk: {:>6}", player::max_bulk() - player::current_bulk()),
                BULK_ROW, STAT_COL);

    print_time(TIME_ROW, STAT_COL);
}

fn print_status_row() {
    print_depth(STATUS_ROW, DEPTH_COL);
}

pub fn print_stat_block() {
    print_stats_column();
    print_status_row();
}

fn print_visible_monsters(_row: &mut u8, _col: u8) {
    // print "VISIBLE MONSTERS"
}

fn print_visible_items(_row: u8, _col: u8) {
    // print "VISIBLE ITEMS"
}

pub fn print_visible_things() {
    let mut row = VISIBLE_ROW;
    let col = VISIBLE_COL;
    print_visible_monsters(&mut row, col);
    print_visible_items(row, col);
}
