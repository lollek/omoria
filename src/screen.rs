use debug;
use term;
use misc;
use player;

const STAT_COL: u8 = 1;
//const WINNER_COL: u8 = 1;

//const RACE_ROW: u8 = 2;
//const CLASS_ROW: u8 = 3;
//const TITLE_ROW: u8 = 4;

const STR_ROW: u8 = 6;
const DEX_ROW: u8 = 7;
const CON_ROW: u8 = 8;
const INT_ROW: u8 = 9;
const WIS_ROW: u8 = 10;
const CHA_ROW: u8 = 11;

//const LEVEL_ROW: u8 = 13;
//const EXP_ROW: u8 = 14;
//const MANA_ROW: u8 = 15;
//const HP_ROW: u8 = 16;
//const QUEST_ROW: u8 = 17;
//const AC_ROW: u8 = 18;
//const GOLD_ROW: u8 = 19;
//const WEIGHT_ROW: u8 = 20;
//const TIME_ROW: u8 = 22;
//const WINNER_ROW: u8 = 23;

fn prt_lost_stat(stat_name: &str, stat: i16, row: u8, col: u8) {
    debug::enter("prt_lost_stat");

    let str = format!("{}{}", stat_name, misc::stat_to_string(stat));
    term::put_buffer(&str, row.into(), col.into());

    debug::leave("prt_lost_stat");
}

pub fn prt_stat(stat_name: &str, stat: i16, row: u8, col: u8) {
    debug::enter("prt_stat");

    let str = format!("{}{}", stat_name, misc::stat_to_string(stat));
    term::put_buffer(&str, row.into(), col.into());

    debug::leave("prt_stat");
}

pub fn prt_6_stats() {
    debug::enter("prt_6_stats");

    let curr = player::curr_stats();
    let lost = player::lost_stats();

    match lost.strength {
        0 => prt_stat("STR : ", curr.strength, STR_ROW, STAT_COL),
        _ => prt_lost_stat("STR : ", curr.strength, STR_ROW, STAT_COL),
    }
    match lost.dexterity {
        0 => prt_stat("STR : ", curr.dexterity, DEX_ROW, STAT_COL),
        _ => prt_lost_stat("STR : ", curr.dexterity, DEX_ROW, STAT_COL),
    }
    match lost.constitution {
        0 => prt_stat("STR : ", curr.constitution, CON_ROW, STAT_COL),
        _ => prt_lost_stat("STR : ", curr.constitution, CON_ROW, STAT_COL),
    }
    match lost.intelligence {
        0 => prt_stat("STR : ", curr.intelligence, INT_ROW, STAT_COL),
        _ => prt_lost_stat("STR : ", curr.intelligence, INT_ROW, STAT_COL),
    }
    match lost.wisdom {
        0 => prt_stat("STR : ", curr.wisdom, WIS_ROW, STAT_COL),
        _ => prt_lost_stat("STR : ", curr.wisdom, WIS_ROW, STAT_COL),
    }
    match lost.charisma {
        0 => prt_stat("STR : ", curr.charisma, CHA_ROW, STAT_COL),
        _ => prt_lost_stat("STR : ", curr.charisma, CHA_ROW, STAT_COL),
    }

    debug::leave("prt_6_stats");
}

