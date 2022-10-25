use crate::master;

pub fn highscore(max: u8) {
    let mut master = master::read_master().unwrap();
    master.sort_unstable_by(|a, b| b.points.cmp(&a.points));
    println!("Username     Points   Alive    Character name    Level  Race         Class");
    println!("____________ ________ _____ ________________________ __ __________ ________________");

    for line in master.iter().take(max as usize) {
        println!(
            "{:>12} {:>8} {:>5} {:>24} {:>2} {:>10} {:>16}",
            line.user_name,
            line.points,
            line.alive,
            line.character_name,
            line.level,
            line.race,
            line.class
        );
    }
}
