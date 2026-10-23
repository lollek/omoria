use super::layout::{current_layout, Rect};
use crate::model::GameTime;
use crate::{data, equipment, ncurses, player, term};
use pancurses::{A_BLINK, A_BOLD, A_DIM};
use std::borrow::Cow;

extern "C" {
    static dun_level: libc::c_long;
    static player_cur_quest: u16;
    static mut player_flags: crate::model::PlayerFlags;
}

const IS_HUNGERY: u64 = 0x0000_0001;
const IS_WEAK: u64 = 0x0000_0002;
const IS_BLIND: u64 = 0x0000_0004;
const IS_CONFUSED: u64 = 0x0000_0008;
const IS_AFRAID: u64 = 0x0000_0010;
const IS_POISONED: u64 = 0x0000_0020;
const IS_SEARCHING: u64 = 0x0000_0100;
const IS_RESTING: u64 = 0x0000_0200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct StatBar {
    hp: i16,
    max_hp: i16,
    mana: Option<(i16, i16)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum QuestStatus {
    Active,
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StatusStyle {
    Plain,
    Bold,
    BoldBlink,
    Dim,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StatusField {
    text: String,
    style: StatusStyle,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct StatusState {
    weak: bool,
    hungry: bool,
    blind: bool,
    confused: bool,
    afraid: bool,
    poisoned: bool,
    searching: bool,
    resting: bool,
    quest: Option<QuestStatus>,
    depth: i64,
    light_on: bool,
}

#[no_mangle]
pub extern "C" fn C_print_stat_block() {
    print_stat_block();
}

#[no_mangle]
pub extern "C" fn C_layout_is_split() -> bool {
    current_layout().kind == super::layout::LayoutKind::Split
}

#[no_mangle]
pub extern "C" fn C_print_split_status_row() -> bool {
    let layout = current_layout();
    if layout.kind != super::layout::LayoutKind::Split {
        return false;
    }
    print_split_status_row(layout.status_rows.expect("status rows unavailable"));
    true
}

pub fn print_stat_block() {
    let layout = current_layout();
    let status_rows = layout.status_rows.expect("status rows unavailable");
    if layout.kind == super::layout::LayoutKind::Split {
        print_split_stat_bar(status_rows);
        print_split_status_row(status_rows);
    } else {
        print_stats_column(layout.stat_column.expect("stat column unavailable"));
        print_classic_status_row(status_rows);
    }
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

pub(super) fn format_game_time(player_age: GameTime) -> String {
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
    format!("{:>7} {:02}:{:02}", dow, hour, min)
}

fn print_time(row: i32, rect: Rect) {
    print_field(format_game_time(player::age()), row, rect);
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

fn print_classic_status_row(rect: Rect) {
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

fn print_split_stat_bar(rect: Rect) {
    let stats = StatBar {
        hp: player::current_hp(),
        max_hp: player::max_hp(),
        mana: if player::uses_mana() {
            Some((player::current_mp(), player::max_mp()))
        } else {
            None
        },
    };
    let text = render_stat_bar(stats);
    let row = rect.row;
    let hp_text = format!("HP {}/{}", stats.hp, stats.max_hp);
    let hp_end = draw_colored_stat(row, rect.col, &hp_text, stat_color(stats.hp, stats.max_hp));

    if let Some((mana, max_mana)) = stats.mana {
        ncurses::mvaddstr(row, hp_end, "  ");
        let mana_text = format!("Mana {}/{}", mana, max_mana);
        draw_colored_stat(row, hp_end + 2, &mana_text, stat_color(mana, max_mana));
    }

    let padding = (rect.width as usize).saturating_sub(text.len());
    if padding > 0 {
        ncurses::mvaddstr(row, rect.col + text.len() as i32, " ".repeat(padding));
    }
}

fn draw_colored_stat(row: i32, col: i32, text: &str, color: i16) -> i32 {
    let color = ncurses::color_pair(color);
    ncurses::attron(color);
    ncurses::mvaddstr(row, col, text);
    ncurses::attroff(color);
    col + text.len() as i32
}

fn print_split_status_row(rect: Rect) {
    let flags = unsafe { player_flags };
    let status = StatusState {
        weak: flags.status & IS_WEAK != 0,
        hungry: flags.status & IS_HUNGERY != 0,
        blind: flags.status & IS_BLIND != 0,
        confused: flags.status & IS_CONFUSED != 0,
        afraid: flags.status & IS_AFRAID != 0,
        poisoned: flags.status & IS_POISONED != 0,
        searching: flags.status & IS_SEARCHING != 0,
        resting: flags.status & IS_RESTING != 0,
        quest: if flags.quested != 0 {
            Some(QuestStatus::Active)
        } else if unsafe { player_cur_quest } > 0 {
            Some(QuestStatus::Done)
        } else {
            None
        },
        depth: unsafe { dun_level } * 50,
        light_on: flags.light_on != 0,
    };
    let fields = fitted_status_fields(&status, rect.width as usize);
    let row = rect.row + 1;
    let mut col = rect.col;
    for (index, field) in fields.iter().enumerate() {
        if index > 0 {
            ncurses::mvaddstr(row, col, " ");
            col += 1;
        }
        draw_status_field(row, col, field);
        col += field.text.len() as i32;
    }

    let used_width = (col - rect.col) as usize;
    if used_width < rect.width as usize {
        ncurses::mvaddstr(row, col, " ".repeat(rect.width as usize - used_width));
    }
}

fn draw_status_field(row: i32, col: i32, field: &StatusField) {
    match field.style {
        StatusStyle::Plain => {}
        StatusStyle::Bold => ncurses::attron(A_BOLD),
        StatusStyle::BoldBlink => {
            ncurses::attron(A_BOLD);
            ncurses::attron(A_BLINK);
        }
        StatusStyle::Dim => ncurses::attron(A_DIM),
    }
    ncurses::mvaddstr(row, col, &field.text);
    match field.style {
        StatusStyle::Plain => {}
        StatusStyle::Bold => ncurses::attroff(A_BOLD),
        StatusStyle::BoldBlink => {
            ncurses::attroff(A_BLINK);
            ncurses::attroff(A_BOLD);
        }
        StatusStyle::Dim => ncurses::attroff(A_DIM),
    }
}

fn render_stat_bar(stats: StatBar) -> String {
    let mut text = format!("HP {}/{}", stats.hp, stats.max_hp);
    if let Some((mana, max_mana)) = stats.mana {
        text.push_str(&format!("  Mana {}/{}", mana, max_mana));
    }
    text
}

fn stat_color(current: i16, max: i16) -> i16 {
    if current >= max {
        ncurses::COLOR_GREEN
    } else if current >= max / 3 {
        ncurses::COLOR_YELLOW
    } else {
        ncurses::COLOR_RED
    }
}

#[cfg(test)]
fn render_status_row(status: &StatusState, width: usize) -> String {
    fitted_status_fields(status, width)
        .iter()
        .map(|field| field.text.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

fn fitted_status_fields(status: &StatusState, width: usize) -> Vec<StatusField> {
    let mut fields = Vec::new();
    if status.weak {
        fields.push(StatusField {
            text: "Weak".to_string(),
            style: StatusStyle::BoldBlink,
        });
    } else if status.hungry {
        fields.push(StatusField {
            text: "Hungry".to_string(),
            style: StatusStyle::Bold,
        });
    }
    if status.blind {
        fields.push(StatusField {
            text: "Blind".to_string(),
            style: StatusStyle::Bold,
        });
    }
    if status.confused {
        fields.push(StatusField {
            text: "Confused".to_string(),
            style: StatusStyle::Bold,
        });
    }
    if status.afraid {
        fields.push(StatusField {
            text: "Afraid".to_string(),
            style: StatusStyle::Bold,
        });
    }
    if status.poisoned {
        fields.push(StatusField {
            text: "Poisoned".to_string(),
            style: StatusStyle::Bold,
        });
    }
    if status.searching {
        fields.push(StatusField {
            text: "Searching".to_string(),
            style: StatusStyle::Plain,
        });
    }
    if status.resting {
        fields.push(StatusField {
            text: "Resting".to_string(),
            style: StatusStyle::Plain,
        });
    }
    let quest = match status.quest {
        Some(QuestStatus::Active) => Some("Quest"),
        Some(QuestStatus::Done) => Some("Done"),
        None => None,
    };
    if let Some(quest) = quest {
        fields.push(StatusField {
            text: quest.to_string(),
            style: StatusStyle::Plain,
        });
    }
    fields.push(if status.depth == 0 {
        StatusField {
            text: "Town Level".to_string(),
            style: StatusStyle::Plain,
        }
    } else {
        StatusField {
            text: format!("Depth: {} (feet)", status.depth),
            style: StatusStyle::Plain,
        }
    });
    if !status.light_on {
        fields.push(StatusField {
            text: "Light Off".to_string(),
            style: StatusStyle::Dim,
        });
    }

    let mut fitted = Vec::new();
    let mut used_width = 0;
    for field in fields {
        let separator = usize::from(!fitted.is_empty());
        if used_width + separator + field.text.len() > width {
            break;
        }
        used_width += separator + field.text.len();
        fitted.push(field);
    }
    fitted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_stat_bar_shows_current_and_max_values_for_hp_and_mana() {
        assert_eq!(
            render_stat_bar(StatBar {
                hp: 45,
                max_hp: 60,
                mana: Some((10, 20)),
            }),
            "HP 45/60  Mana 10/20"
        );
    }

    #[test]
    fn split_stat_bar_omits_mana_for_classes_without_mana() {
        assert_eq!(
            render_stat_bar(StatBar {
                hp: 45,
                max_hp: 60,
                mana: None,
            }),
            "HP 45/60"
        );
    }

    #[test]
    fn split_status_row_keeps_fields_whole_when_truncated() {
        let status = StatusState {
            weak: true,
            hungry: false,
            blind: true,
            confused: true,
            afraid: true,
            poisoned: true,
            searching: true,
            resting: false,
            quest: Some(QuestStatus::Active),
            depth: 0,
            light_on: false,
        };

        assert_eq!(render_status_row(&status, 16), "Weak Blind");
    }

    #[test]
    fn split_status_row_includes_flags_depth_and_light_when_they_fit() {
        let status = StatusState {
            weak: false,
            hungry: true,
            blind: true,
            confused: false,
            afraid: false,
            poisoned: false,
            searching: false,
            resting: true,
            quest: Some(QuestStatus::Done),
            depth: 500,
            light_on: false,
        };

        assert_eq!(
            render_status_row(&status, 81),
            "Hungry Blind Resting Done Depth: 500 (feet) Light Off"
        );
    }

    #[test]
    fn split_status_row_renders_each_status_flag() {
        let base = StatusState {
            light_on: true,
            ..StatusState::default()
        };
        for (status, expected) in [
            (StatusState { weak: true, ..base }, "Weak Town Level"),
            (
                StatusState {
                    hungry: true,
                    ..base
                },
                "Hungry Town Level",
            ),
            (
                StatusState {
                    blind: true,
                    ..base
                },
                "Blind Town Level",
            ),
            (
                StatusState {
                    confused: true,
                    ..base
                },
                "Confused Town Level",
            ),
            (
                StatusState {
                    afraid: true,
                    ..base
                },
                "Afraid Town Level",
            ),
            (
                StatusState {
                    poisoned: true,
                    ..base
                },
                "Poisoned Town Level",
            ),
            (
                StatusState {
                    searching: true,
                    ..base
                },
                "Searching Town Level",
            ),
            (
                StatusState {
                    resting: true,
                    ..base
                },
                "Resting Town Level",
            ),
            (
                StatusState {
                    quest: Some(QuestStatus::Active),
                    ..base
                },
                "Quest Town Level",
            ),
        ] {
            assert_eq!(render_status_row(&status, 81), expected);
        }
    }

    #[test]
    fn split_status_fields_keep_legacy_emphasis() {
        let status = StatusState {
            weak: true,
            hungry: true,
            blind: true,
            confused: false,
            afraid: false,
            poisoned: false,
            searching: false,
            resting: false,
            quest: None,
            depth: 0,
            light_on: false,
        };

        let fields = fitted_status_fields(&status, 81);
        assert_eq!(fields[0].text, "Weak");
        assert_eq!(fields[0].style, StatusStyle::BoldBlink);
        assert_eq!(fields[1].style, StatusStyle::Bold);
        assert_eq!(fields.last().unwrap().style, StatusStyle::Dim);
    }

    #[test]
    fn split_stat_bar_colors_full_wounded_and_critical_values() {
        assert_eq!(stat_color(60, 60), ncurses::COLOR_GREEN);
        assert_eq!(stat_color(20, 60), ncurses::COLOR_YELLOW);
        assert_eq!(stat_color(19, 60), ncurses::COLOR_RED);
    }
}
