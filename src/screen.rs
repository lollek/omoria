use debug;
use term;

use types::StatBlock;

// Stat -> String
fn cnv_stat(stat: i16) -> String {
    if stat < 0 || stat > 250 {
        panic!(stat)
    }

    if stat > 150 {
        format!("18/{:-2}", stat - 150)
    } else {
        format!("{:2}   ", 3 + stat / 10)
    }
}

fn prt_stat(stat_name: &str, stat: i16, row: u8, column: u8) {
    debug::enter("prt_stat");
    term::put_buffer_r(&format!("{}{}", stat_name, cnv_stat(stat)), row as i32, column as i32);
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
