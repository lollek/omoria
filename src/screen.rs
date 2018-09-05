use debug;
use term;
use misc;
use player;
use ncurses;

const STAT_BLOCK_WIDTH: usize = 14;
const STAT_COL: u8 = 0;
//const WINNER_COL: u8 = 0;

const RACE_ROW: u8 = 1;
const CLASS_ROW: u8 = 2;
const TITLE_ROW: u8 = 3;

const HP_ROW: u8 = 5;
const MANA_ROW: u8 = 6;

const STAT_ROW: u8 = 8; // -> 13

//const LEVEL_ROW: u8 = 12;
//const EXP_ROW: u8 = 13;
//const QUEST_ROW: u8 = 16;
//const AC_ROW: u8 = 17;
//const GOLD_ROW: u8 = 18;
//const WEIGHT_ROW: u8 = 19;
//const TIME_ROW: u8 = 21;
//const WINNER_ROW: u8 = 22;

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
    term::put_buffer(&format!("{:<width$}", msg, width=STAT_BLOCK_WIDTH), row.into(), col.into());
}

fn print_hp(row: u8, col: u8) {
    let current = player::current_hp();
    let max = player::max_hp();

    let hpstr = &format!("HP  : {:>6}", current);

    if current >= max {
        ncurses::chattr(ncurses::CursesAttr::ColorGreen, true);
        term::put_buffer(hpstr, row.into(), col.into());
        ncurses::chattr(ncurses::CursesAttr::ColorGreen, false);

    } else if current >= max / 3 {
        ncurses::chattr(ncurses::CursesAttr::ColorYellow, true);
        term::put_buffer(hpstr, row.into(), col.into());
        ncurses::chattr(ncurses::CursesAttr::ColorYellow, false);

    } else {
        ncurses::chattr(ncurses::CursesAttr::ColorRed, true);
        term::put_buffer(hpstr, row.into(), col.into());
        ncurses::chattr(ncurses::CursesAttr::ColorRed, false);
    }
}

fn print_mana(row: u8, col: u8) {
    if !player::uses_mana() {
        return;
    }

    let current = player::current_mp();
    let max = player::max_mp();

    let hpstr = &format!("MANA: {:>6}", current);

    if current >= max {
        ncurses::chattr(ncurses::CursesAttr::ColorBlue, true);
        term::put_buffer(hpstr, row.into(), col.into());
        ncurses::chattr(ncurses::CursesAttr::ColorBlue, false);

    } else if current >= max / 3 {
        ncurses::chattr(ncurses::CursesAttr::ColorCyan, true);
        term::put_buffer(hpstr, row.into(), col.into());
        ncurses::chattr(ncurses::CursesAttr::ColorCyan, false);

    } else {
        ncurses::chattr(ncurses::CursesAttr::ColorMagenta, true);
        term::put_buffer(hpstr, row.into(), col.into());
        ncurses::chattr(ncurses::CursesAttr::ColorMagenta, false);
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

    //prunt_field("EXP : ", player_exp, EXP_ROW, STAT_COL);

    debug::leave("print_stat_block");
}
