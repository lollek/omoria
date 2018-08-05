use debug;
use term;
use misc;

use types::StatBlock;

fn prt_stat(stat_name: &str, stat: i16, row: u8, column: u8) {
    debug::enter("prt_stat");
    let str = format!("{}{}", stat_name, misc::stat_to_string(stat));
    term::put_buffer_r(&str, row.into(), column.into());
    debug::leave("prt_stat");
}

// TODO: Not whole function is copied from C
pub fn prt_6_stats(p: &StatBlock, row: u8, col: u8) {
    debug::enter("prt_6_stats");

    prt_stat("STR : ", p.strength, row, col);
    prt_stat("INT : ", p.intelligence, row + 1, col);
    prt_stat("WIS : ", p.wisdom, row + 2, col);
    prt_stat("DEX : ", p.dexterity, row + 3, col);
    prt_stat("CON : ", p.constitution, row + 4, col);
    prt_stat("CHR : ", p.charisma, row + 5, col);

    debug::leave("prt_6_stats");
}
