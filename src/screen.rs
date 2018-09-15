use debug;
use equipment;
use misc;
use ncurses;
use player;
use term;

const STAT_BLOCK_WIDTH: usize = 14;
const STAT_COL: u8 = 0;
//const WINNER_COL: u8 = 0;

const RACE_ROW: u8 = 1;
const CLASS_ROW: u8 = 2;
const TITLE_ROW: u8 = 3;

const HP_ROW: u8 = 5;
const MANA_ROW: u8 = 6;

const STAT_ROW: u8 = 8; // -> 13

const QUEST_ROW: u8 = 15;
const AC_ROW: u8 = 16;
const GOLD_ROW: u8 = 17;
const CURR_WEIGHT_ROW: u8 = 18;
const MAX_WEIGHT_ROW: u8 = 19;
const TIME_ROW: u8 = 21;

const EQUIP_COL: u8 = 0;
const EQUIP_ROW: u8 = 80;

fn prt_lost_stat(stat_name: &str, stat: i16, row: u8, col: u8) {
    debug::enter("prt_lost_stat");

    ncurses::chattr(ncurses::CursesAttr::Dim, true);

    let str = format!("{}{}", stat_name, misc::stat_to_string(stat));
    term::put_buffer(&str, row.into(), col.into());

    ncurses::chattr(ncurses::CursesAttr::Dim, false);

    debug::leave("prt_lost_stat");
}

pub fn prt_stat(stat_name: &str, stat: i16, row: u8, col: u8) {
    debug::enter("prt_stat");

    let str = format!("{}{}", stat_name, misc::stat_to_string(stat));
    term::put_buffer(&str, row.into(), col.into());

    debug::leave("prt_stat");
}

pub fn print_stats(row: u8, col: u8) {
    debug::enter("print_stats");

    let curr = player::curr_stats();
    let lost = player::lost_stats();

    match lost.strength {
        0 => prt_stat("STR : ", curr.strength, row, col),
        _ => prt_lost_stat("STR : ", curr.strength, row, col),
    }
    match lost.dexterity {
        0 => prt_stat("DEX : ", curr.dexterity, row + 1, col),
        _ => prt_lost_stat("DEX : ", curr.dexterity, row + 1, col),
    }
    match lost.constitution {
        0 => prt_stat("CON : ", curr.constitution, row + 2, col),
        _ => prt_lost_stat("CON : ", curr.constitution, row + 2, col),
    }
    match lost.intelligence {
        0 => prt_stat("INT : ", curr.intelligence, row + 3, col),
        _ => prt_lost_stat("INT : ", curr.intelligence, row + 3, col),
    }
    match lost.wisdom {
        0 => prt_stat("WIS : ", curr.wisdom, row + 4, col),
        _ => prt_lost_stat("WIS : ", curr.wisdom, row + 4, col),
    }
    match lost.charisma {
        0 => prt_stat("CHA : ", curr.charisma, row + 5, col),
        _ => prt_lost_stat("CHA : ", curr.charisma, row + 5, col),
    }

    debug::leave("print_stats");
}

fn print_field(msg: &str, row: u8, col: u8) {
    let msg_formatted = &format!("{:<width$}", msg, width=STAT_BLOCK_WIDTH);
    term::put_buffer(msg_formatted, row.into(), col.into());
}

fn print_hp(row: u8, col: u8) {
    let current = player::current_hp();
    let max = player::max_hp();
    let color =
        if current >= max {
            ncurses::CursesAttr::ColorGreen
        } else if current >= max / 3 {
            ncurses::CursesAttr::ColorYellow
        } else {
            ncurses::CursesAttr::ColorRed
        };

    ncurses::chattr(color.to_owned(), true);
    term::put_buffer(&format!("HP  : {:>6}", current), row.into(), col.into());
    ncurses::chattr(color.to_owned(), false);
}

fn print_mana(row: u8, col: u8) {
    if !player::uses_mana() {
        return;
    }

    let current = player::current_mp();
    let max = player::max_mp();
    let color =
        if current >= max {
            ncurses::CursesAttr::ColorBlue
        } else if current >= max / 3 {
            ncurses::CursesAttr::ColorCyan
        } else {
            ncurses::CursesAttr::ColorMagenta
        };

    ncurses::chattr(color.to_owned(), true);
    term::put_buffer(&format!("MANA: {:>6}", current), row.into(), col.into());
    ncurses::chattr(color.to_owned(), false);
}

fn print_time(row: u8, col: u8) {
    let min = unsafe { player::player_cur_age.secs } / 60;
    let hour = unsafe { player::player_cur_age.hour };
    let dow = match unsafe { player::player_cur_age.day } % 7 {
        0 => "Moonday",
        1 => "Toilday",
        2 => "Wealday",
        3 => "Oathday",
        4 => "Fireday",
        5 => "Starday",
        _ => "Sunday",
    };
    print_field(&format!("{:>7} {:02}:{:02}", dow, hour, min), row, col);
}

fn print_equipment(row: u8, col: u8) {
    for (index, slot_i) in equipment::slots_iter().enumerate() {
        let slot = equipment::Slot::from(slot_i);
        let index_char = ('a' as u8 + index as u8) as char;
        let item_name = equipment::get_name(slot);
        let msg = &format!("{}) {:>10}: {}", index_char, slot.name(), item_name);
        term::prt(msg, (EQUIP_ROW + row + index as u8).into(), (EQUIP_COL + col).into());
    }
}

pub fn print_stat_block() {
    debug::enter("print_stat_block");

    print_field(player::race().name(), RACE_ROW, STAT_COL);
    print_field(player::class().name(), CLASS_ROW, STAT_COL);
    print_field(&player::title(), TITLE_ROW, STAT_COL);

    print_hp(HP_ROW, STAT_COL);
    print_mana(MANA_ROW, STAT_COL);

    print_stats(STAT_ROW, STAT_COL);

    print_field(&format!("QST : {:>6}", player::quests()), QUEST_ROW, STAT_COL);
    print_field(&format!("AC  : {:>6}", unsafe { player::player_dis_ac }), AC_ROW, STAT_COL);
    print_field(&format!("GOLD: {:>6}", player::wallet().total), GOLD_ROW, STAT_COL);
    print_field(&format!("WGHT: {:>6}", player::current_bulk()), CURR_WEIGHT_ROW, STAT_COL);
    print_field(&format!("M_WT: {:>6}", player::max_bulk()), MAX_WEIGHT_ROW, STAT_COL);

    print_time(TIME_ROW, STAT_COL);

    debug::leave("print_stat_block");
}
