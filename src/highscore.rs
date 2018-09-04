use libc;

use term;
use master;


pub fn highscore(max: u8) {
    term::clear_screen();

    let mut master = master::read_master().unwrap();
    master.sort_unstable_by(|a, b| b.points.cmp(&a.points));
    term::put_buffer_r(
        "Username     Points   Alive    Character name    Level  Race         Class", 1, 1);
    term::put_buffer_r(
        "____________ ________ _____ ________________________ __ __________ ________________", 2, 1);
    master.iter()
        .take(max as usize)
        .enumerate()
        .for_each(|(i, line)| term::put_buffer_r(&format!(
                "{:>12} {:>8} {:>5} {:>24} {:>2} {:>10} {:>16}",
                line.user_name,
                line.points,
                line.alive,
                line.character_name,
                line.level,
                line.race,
                line.class), (3 + i) as i32, 1));
    term::refresh_screen();
}

#[no_mangle]
pub extern fn C_highscore(max: libc::uint8_t) {
    highscore(max);
}
