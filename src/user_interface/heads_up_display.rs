use crate::player_action::attack::{
    calculate_number_of_attacks, calculate_player_tohit2, AttackType, MeleeAttackType,
};
use crate::{data, equipment, ncurses, player, term};
use std::borrow::Cow;

// Stats Column
const STAT_BLOCK_WIDTH: usize = 14;
const STAT_COL: u8 = 0;

const RACE_ROW: u8 = 1;
const CLASS_ROW: u8 = RACE_ROW + 1;
const HP_ROW: u8 = 5;
const MANA_ROW: u8 = HP_ROW + 1;

const QUEST_ROW: u8 = 15;
const AC_ROW: u8 = QUEST_ROW + 1;
const GOLD_ROW: u8 = AC_ROW + 1;
const BULK_ROW: u8 = GOLD_ROW + 1;

const TIME_ROW: u8 = BULK_ROW + 2;

// Status Row
const STATUS_ROW: u8 = 23;
const DEPTH_COL: u8 = 61;

// Equipment Column
const EQUIP_COL: u8 = 81;
const EQUIP_ROW: u8 = 1;

extern "C" {
    static dun_level: libc::c_long;
}

#[no_mangle]
pub extern "C" fn C_print_stat_block() {
    print_stat_block();
}
pub fn print_stat_block() {
    print_stats_column();
    print_status_row();
}

#[no_mangle]
pub extern "C" fn C_print_equipment_block() {
    print_equipment_block();
}
pub fn print_equipment_block() {
    print_equipment(EQUIP_ROW, EQUIP_COL);
}

fn print_field<S>(msg: S, row: u8, col: u8)
where
    S: AsRef<str>,
{
    let msg_formatted = format!("{:<width$}", msg.as_ref(), width = STAT_BLOCK_WIDTH);
    term::put_buffer(msg_formatted, row.into(), col.into());
}

fn print_hp(row: u8, col: u8) {
    let current = player::current_hp();
    let max = player::max_hp();
    let color = if current >= max {
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
    let color = if current >= max {
        ncurses::color_pair(ncurses::COLOR_GREEN)
    } else if current >= max / 3 {
        ncurses::color_pair(ncurses::COLOR_YELLOW)
    } else {
        ncurses::color_pair(ncurses::COLOR_RED)
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

fn print_equipment(row: u8, col: u8) {
    for (index, slot_i) in equipment::slots_iter().enumerate() {
        let slot = equipment::Slot::from(slot_i);
        let index_char = ('a' as u8 + index as u8) as char;
        let item_name = equipment::get_name(slot);
        let msg = format!("{}) {:<13}: {}", index_char, slot.name(), item_name);
        term::prt(msg, (row + index as u8).into(), col.into());
    }
}


fn print_status_row() {
    print_depth(STATUS_ROW, DEPTH_COL);
}

fn print_depth(row: u8, col: u8) {
    let depth = unsafe { dun_level } * 50;
    let string = match depth {
        0 => Cow::from("Town Level      "),
        _ => Cow::from(format!("Depth: {} (feet)", depth)),
    };
    term::put_buffer(string, row.into(), col.into());
}

fn print_stats_column() {
    print_field(data::race::name(&player::race()), RACE_ROW, STAT_COL);
    print_field(data::class::name(&player::class()), CLASS_ROW, STAT_COL);

    print_hp(HP_ROW, STAT_COL);
    print_mana(MANA_ROW, STAT_COL);

    print_field(
        format!("QST : {:>6}", player::quests()),
        QUEST_ROW,
        STAT_COL,
    );
    print_field(
        format!("AC  : {:>6}", unsafe { player::player_dis_ac }),
        AC_ROW,
        STAT_COL,
    );
    print_field(
        format!("Gold: {:>6}", player::wallet().total),
        GOLD_ROW,
        STAT_COL,
    );
    let current_bulk = player::max_bulk() as i64 - player::current_bulk() as i64;
    print_field(format!("Bulk: {:>6}", current_bulk), BULK_ROW, STAT_COL);

    print_time(TIME_ROW, STAT_COL);
}
