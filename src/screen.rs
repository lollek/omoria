use debug;
use term;

use create::Stat;

// Stat -> String
fn cnv_stat(stat: u8) -> String {
    if stat > 150 {
        format!("18/{:-2}", stat - 150)
    } else {
        format!("{:2}   ", 3 + stat / 10)
    }
}

fn prt_stat(stat_name: &str, stat: u8, row: u8, column: u8) {
    debug::enter("prt_stat");
    term::put_buffer_r(&format!("{}{}", stat_name, cnv_stat(stat)), row as i32, column as i32);
    debug::leave("prt_stat");
}

// TODO: Not whole function is copied from C
pub fn prt_6_stats(p: [u8; 6], row: u8, col: u8) {
    debug::enter("prt_6_stats");
    prt_stat("STR : ", p[Stat::Strength as usize], row, col);
    prt_stat("INT : ", p[Stat::Intelligence as usize], row + 1, col);
    prt_stat("WIS : ", p[Stat::Wisdom as usize], row + 2, col);
    prt_stat("DEX : ", p[Stat::Dexterity as usize], row + 3, col);
    prt_stat("CON : ", p[Stat::Constitution as usize], row + 4, col);
    prt_stat("CHR : ", p[Stat::Charisma as usize], row + 5, col);

    debug::leave("prt_6_stats");
}
