use term;
use master;


pub fn highscore(max: u8) {
    term::clear_screen();

    let mut master = master::read_master().unwrap();
    master.sort_unstable_by(|a, b| b.points.cmp(&a.points));
    term::put_buffer(
        "Username     Points   Alive    Character name    Level  Race         Class", 0, 0);
    term::put_buffer(
        "____________ ________ _____ ________________________ __ __________ ________________", 1, 0);
    master.iter()
        .take(max as usize)
        .enumerate()
        .for_each(|(i, line)| term::put_buffer(&format!(
                "{:>12} {:>8} {:>5} {:>24} {:>2} {:>10} {:>16}",
                line.user_name,
                line.points,
                line.alive,
                line.character_name,
                line.level,
                line.race,
                line.class), (2 + i) as i32, 0));
    term::refresh_screen();
}
