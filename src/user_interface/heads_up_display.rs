use super::layout::{current_layout, Rect};
use crate::{data, equipment, ncurses, player, term};
use std::borrow::Cow;

extern "C" {
    static dun_level: libc::c_long;
}

#[no_mangle]
pub extern "C" fn C_print_stat_block() {
    print_stat_block();
}
pub fn print_stat_block() {
    let layout = current_layout();
    print_stats_column(layout.stat_column.expect("stat column unavailable"));
    print_status_row(layout.status_rows.expect("status rows unavailable"));
}

#[no_mangle]
pub extern "C" fn C_print_equipment_block() {
    print_equipment_block();
}
pub fn print_equipment_block() {
    let rect = current_layout()
        .equipment
        .expect("equipment panel unavailable");
    print_equipment(rect.row, rect.col);
}

fn print_field<S>(msg: S, row: i32, rect: Rect)
where
    S: AsRef<str>,
{
    let msg_formatted = format!("{:<width$}", msg.as_ref(), width = rect.width as usize);
    ncurses::mvaddstr(row, rect.col, msg_formatted);
}

fn print_hp(row: i32, col: i32) {
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
    let msg = format!("HP  : {:>6}", current);
    ncurses::mvaddstr(row, col, msg);
    ncurses::attroff(color);
}

fn print_mana(row: i32, col: i32) {
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
    let msg = format!("MANA: {:>6}", current);
    ncurses::mvaddstr(row, col, msg);
    ncurses::attroff(color);
}

fn print_time(row: i32, rect: Rect) {
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
    print_field(format!("{:>7} {:02}:{:02}", dow, hour, min), row, rect);
}

fn print_equipment(row: i32, col: i32) {
    for (index, slot_i) in equipment::slots_iter().enumerate() {
        let slot = equipment::Slot::from(slot_i);
        let index_char = (b'a' + index as u8) as char;
        let item_name = equipment::get_name(slot);
        let msg = format!("{}) {:<13}: {}", index_char, slot.name(), item_name);
        term::prt(msg, row + index as i32, col);
    }
}

fn print_status_row(rect: Rect) {
    print_depth(rect.row, rect.col + 61);
}

fn print_depth(row: i32, col: i32) {
    let depth = unsafe { dun_level } * 50;
    let string = match depth {
        0 => Cow::from("Town Level      "),
        _ => Cow::from(format!("Depth: {} (feet)", depth)),
    };
    ncurses::mvaddstr(row, col, string);
}

fn print_stats_column(rect: Rect) {
    print_field(data::race::name(&player::race()), rect.row, rect);
    print_field(data::class::name(&player::class()), rect.row + 1, rect);

    print_hp(rect.row + 4, rect.col);
    print_mana(rect.row + 5, rect.col);

    print_field(
        format!("QST : {:>6}", player::quests()),
        rect.row + 14,
        rect,
    );
    print_field(
        format!("Gold: {:>6}", player::wallet().total),
        rect.row + 16,
        rect,
    );
    let current_bulk = player::max_bulk() as i64 - player::current_bulk() as i64;
    print_field(format!("Bulk: {:>6}", current_bulk), rect.row + 17, rect);

    print_time(rect.row + 19, rect);
}
