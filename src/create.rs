use std::ops::Range;

use misc::squish_stat;

pub enum Stat {
    Strength = 0,
    Intelligence = 1,
    Wisdom = 2,
    Dexterity = 3,
    Constitution = 4,
    Charisma = 5,
}

pub fn stats_iter() -> Range<usize> {
    (Stat::Strength as usize)..(Stat::Charisma as usize + 1)
}

#[no_mangle]
pub extern fn cc__old_stat(stat: u8) -> u8 {
    if stat < 150 {
        (squish_stat(stat as i32) + 30) / 10
    } else {
        squish_stat(stat as i32) - 132
    }
}

#[no_mangle]
pub extern fn cc__new_stat(stat: u8) -> u8 {
    if stat < 18 {
        squish_stat(((stat * 10) - 30) as i32)
    } else {
        squish_stat((stat + 132) as i32)
    }
}

#[no_mangle]
pub extern fn cc__max_in_statp(stat: u8) -> u8 {
    if stat < 150 {
        stat + 10
    } else if stat < 220 {
        stat + 25
    } else if stat < 240 {
        stat + 10
    } else if stat < 250 {
        stat + 1
    } else {
        stat
    }
}

#[no_mangle]
pub extern fn cc__max_de_statp(stat: u8) -> u8 {
    if stat < 11 {
        0
    } else if stat < 151 {
        stat - 10
    } else if stat < 156 {
        150
    } else if stat < 241 {
        stat - 6
    } else {
        stat - 1
    }
}

#[no_mangle]
pub extern fn cc__max_stat(cur_stat: u8, amount: i8) -> u8 {
    let mut stat: u8 = cur_stat;

    if amount < 0 {
        for _ in -1..amount - 1 {
            stat = cc__max_de_statp(stat)
        }
    } else {
        for _ in 1..amount + 1 {
            stat = cc__max_in_statp(stat)
        }
    }

    return cur_stat
}

#[no_mangle]
pub extern fn cc__next_best_stats(this: [u8; 6], user: [u8; 6], mut best: [u8; 6],
                                  best_min: i64) -> i64 {
    let mut below_sum: i64 = 0;

    for tstat in stats_iter() {
        if this[tstat] < user[tstat] {
            let below = user[tstat] - this[tstat];
            below_sum = below_sum + ((below * (below + 1)) / 2) as i64;
        }
    }

    if below_sum < best_min {
        for tstat in stats_iter() {
            best[tstat] = this[tstat];
        }
        return below_sum
    } else {
        return best_min
    }
}
