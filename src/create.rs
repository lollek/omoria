use std::ops::Range;
use std::ptr::null;
use std::ffi::CStr;

use libc::sscanf;
use libc::time;
use libc::time_t;

use bank::Currency;
use bank::COIN_VALUE;
use io::inkey_delay;
use io::inkey_flush;
use misc::squish_stat;
use player::*;
use races::RACE_STATS;
use races::SEARCH_MOD;
use races::MELEE_BONUS;
use races::RANGED_BONUS;
use races::SEARCH_FREQ;
use races::STEALTH_MOD;
use races::SAVE_MOD;
use races::HEALTH_BONUS;
use races::EXPFACTOR;
use races::INFRAVISION;
use races::SWIM_SPEED;
use random::randint;
use term::prt_r;
use term::refresh_screen;

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

const PLAYER_EXIT_PAUSE: i32 = 0;

extern "C" {
    fn Get_String(in_str: *const u8, row: i32, col: i32, slen: i32) -> u8;
    fn Clear_From(row: i32);
    fn Pause_Exit(prt_line: i32, delay: i32);
    fn Erase_Line(row: i32, col: i32);
    fn prt_6_stats(p: [u8; 6], l: *const u8, row: u32, col: u32);
    fn get_yes_no(prompt: *const u8) -> u8;
    fn add_money(amount: i64);
    fn inkey() -> u8;

    fn tohit_adj() -> i64;
    fn todam_adj() -> i64;
    fn toac_adj() -> i64;

    fn cc__change_stat(cur_stat: i64, amount: i64) -> i64;
    fn cc__get_name();
    fn cc__choose_race() -> u8;
    fn cc__put_misc3();
    fn cc__get_class() -> u8;
    fn cc__print_history();
    fn cc__get_history();
    fn cc__get_ahw();
    fn cc__get_sex() -> u8;
}

fn get_string(in_str: *const u8, row: i32, col: i32, slen: i32) -> u8 {
    unsafe { Get_String(in_str, row - 1, col - 1, slen) }
}

fn clear_from(row: i32) {
    unsafe { Clear_From(row - 1) }
}

fn pause_exit(prt_line: i32, delay: i32) {
    unsafe { Pause_Exit(prt_line -1, delay) }
}

fn erase_line(row: i32, col: i32) {
    unsafe { Erase_Line(row -1, col -1) }
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
pub extern fn cc__max_stat(mut stat: u8, amount: i8) -> u8 {
    if amount < 0 {
        for _ in amount..0 {
            stat = cc__max_de_statp(stat);
        }
    } else {
        for _ in 1..(amount+1) {
            stat = cc__max_in_statp(stat);
        }
    }

    stat
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

#[no_mangle]
pub extern fn cc__get_min_stat(stat: &str, max: u8) -> u8 {
    let out_str =
        if max == 250 {
            format!("Min {} (racial max 18/00) : ", stat)
        } else if max > 150 {
            format!("Min {} (racial max 18/{}) : ", stat, max - 150)
        } else {
            format!("Min {} (racial max {}) : ", stat, cc__old_stat(max))
        };

    prt_r(&out_str, 1, 1);

    let mut tmp_str: [u8; 134] = [0; 134];
    while tmp_str[0] == b'\0' {
        get_string(tmp_str.as_ptr(), 1, out_str.len() as i32 + 1, 10);
    }

    // 18/xx -> 18 xx. Might be needed for sscanf?
    match tmp_str.iter().position(|&x| x == b'/') {
        Some(pos) => tmp_str[pos] = b' ',
        None => (),
    }

    let abil: u32 = 0;
    let perc: u32 = 0;
    unsafe {
        sscanf(tmp_str.as_ptr() as *const i8, "%u %u\0".as_ptr() as *const i8, &abil, &perc);
    }
    if abil == 18 {
        if perc == 0 {
            250 as u8
        } else {
            150 + perc as u8
        }
    } else {
        cc__new_stat(
            if abil < 3 {
                3
            } else if abil > 18 {
                18
            } else {
                abil as u8
            })
    }
}

#[no_mangle]
pub extern fn cc__get_minimums(mut user: [u8; 6], max_r: [u8; 6]) -> u8 {
    let yn: u8 = unsafe {
        get_yes_no("Do you wish to try for minimum statistics?\0".as_ptr())
        //get_yes_no(format!("{}, {}, {}, {}, {}, {}?\0", max_r[0], max_r[1], max_r[2], max_r[3], max_r[4], max_r[5]).as_ptr())
    };

    if yn != 0 {
        user[0] = cc__get_min_stat("STR", max_r[0]);
        user[1] = cc__get_min_stat("INT", max_r[1]);
        user[2] = cc__get_min_stat("WIS", max_r[2]);
        user[3] = cc__get_min_stat("DEX", max_r[3]);
        user[4] = cc__get_min_stat("CON", max_r[4]);
        user[5] = cc__get_min_stat("CHR", max_r[5]);
        unsafe { prt_6_stats(user, null(), 3, 65); }
    }
    yn
}

#[no_mangle]
pub extern fn cc__get_stat() -> i64 {
    (randint(4) + randint(4) + randint(4) + 2) * 10 // [50, 140]
}

 fn put_character() {
    unsafe {
        clear_from(1);
        prt_r(&format!("Name      : {}", CStr::from_ptr(player_name).to_str().unwrap()), 3, 3);
        prt_r(&format!("Race      : {}", CStr::from_ptr(player_race).to_str().unwrap()), 4, 3);
        prt_r(&format!("Sex       : {}", CStr::from_ptr(player_sex).to_str().unwrap()), 5, 3);
        prt_r(&format!("Class     : {}", CStr::from_ptr(player_tclass).to_str().unwrap()), 6, 3);
    }
}

fn get_money() {
    let mut amount: i64 =
        325 + randint(25)
        // Social Class adj
        + unsafe { player_sc } as i64 * 6
        // Stat adj
        - stats_iter().fold(0, |sum: i64, tstat| unsafe {
            sum + cc__old_stat(player_stats_curr[tstat]) as i64
        })
        // Charisma adj
        + unsafe {
            cc__old_stat(player_stats_curr[Stat::Charisma as usize]) as i64
        };

    // Minimum
    if amount < 80 {
        amount = 80;
    }

    let gold_value = COIN_VALUE[Currency::Gold as usize];
    unsafe { add_money((amount * gold_value) + randint(gold_value)) };
}

fn satisfied(minning: &mut bool,  printed_once: &mut bool, best_min: &mut i64,
                 try_count: &mut i64, best: [u8; 6], user: [u8; 6]) -> bool {
    let mut is_satisfied: bool = false;

    if !*minning {
        /*
         * Figure out what the current bonuses are
         * so the player has a clue
         */
        unsafe {
            player_dis_th = tohit_adj() as i16;
            player_dis_td = todam_adj() as i16;
            player_dis_tac = toac_adj() as i16;
        }

        erase_line(1, 1);
        clear_from(21);
        put_misc1();
        put_stats();

        prt_r("Press 'R' to reroll, <RETURN> to continue:", 21, 3);
        *printed_once = true;

        loop {
            let s: u8 = inkey_flush();
            is_satisfied = s != 'R' as u8;
            if s == 10 || s == 'R' as u8 {
                break;
            }
        }

    } else { /* minning */

        if !*printed_once {
            clear_from(21);
            prt_r("Press any key to give up (50000 rolls max): ", 21, 3);
            refresh_screen();
            *printed_once = true;
        }

        unsafe {
            *best_min = cc__next_best_stats(player_stats_perm, user, best, *best_min);
        }
        *try_count += 1;
        if (*try_count % 250) == 0 {
            prt_r(&format!("Try = {:10}", try_count), 21, 60);
            refresh_screen();
        }

        let s: u8 = inkey_delay(0);
        if s != 0 || *try_count == 50000 {
            *minning = false;
            *printed_once = false;
            for tstat in stats_iter() {
                unsafe {
                    player_stats_perm[tstat] = best[tstat];
                    player_stats_curr[tstat] = best[tstat];
                }
            }
            is_satisfied =
                satisfied(minning, printed_once, best_min,
                              try_count, best, user);
        } else {
            is_satisfied = *best_min == 0i64;
            if *best_min == 0i64 {
                put_misc1();
                put_stats();
            }
        } /* endif s || try_count */
    }	 /* endif minning */

    is_satisfied
}

fn get_stats() {
    let race = unsafe { player_prace as usize };

    for tstat in stats_iter() {
        unsafe {
            let new_stat: u8 = cc__change_stat(cc__get_stat(), RACE_STATS[race][tstat] as i64) as u8;
            player_stats_perm[tstat] = new_stat;
            player_stats_curr[tstat] = new_stat;
        }
    }

    unsafe {
        player_rep = 0;
        player_srh = SEARCH_MOD[race] as i16;
        player_bth = MELEE_BONUS[race] as i16;
        player_bthb = RANGED_BONUS[race] as i16;
        player_fos = SEARCH_FREQ[race] as i16;
        player_stl = STEALTH_MOD[race] as u8; // TODO BUG! overflows for some races
        player_save = SAVE_MOD[race] as i16;
        player_hitdie = HEALTH_BONUS[race];
        player_lev = 1;
        player_ptodam = todam_adj() as i16;
        player_ptohit = tohit_adj() as i16;
        player_ptoac = 0;
        player_pac = toac_adj() as i16;
        player_expfact = EXPFACTOR[race];
        player_flags.see_infra = INFRAVISION[race] as i64;
        player_flags.swim = SWIM_SPEED[race] as i64;
    }
}

fn put_stats() {
    unsafe { prt_6_stats(player_stats_curr, null(), 3, 65) };
    prt_r(&format!("+ To Hit   : {}", unsafe { player_dis_th }), 10, 4);
    prt_r(&format!("+ To Damage: {}", unsafe { player_dis_td }), 11, 4);
    prt_r(&format!("+ To AC    : {}", unsafe { player_dis_tac }), 12, 4);
    prt_r(&format!("  Total AC : {}", unsafe { player_dis_ac }), 13, 4);
}

fn put_misc1() {
    prt_r(&format!("Age          : {}", unsafe { player_age }), 3, 40);
    prt_r(&format!("Height       : {}", unsafe { player_ht }), 4, 40);
    prt_r(&format!("Weight       : {}", unsafe { player_wt }), 5, 40);
    prt_r(&format!("Social Class : {}", unsafe { player_sc }), 6, 40);
    prt_r(&format!("Difficulty   : {}", unsafe { player_diffic }), 7, 40);
}

fn put_misc2() {
    prt_r(&format!("Level      : {}", unsafe { player_lev }), 10, 31);
    prt_r(&format!("Experience : {}", unsafe { player_exp }), 11, 31);
    prt_r(&format!("Gold       : {}", unsafe { player_money[Currency::Total as usize] }), 12, 31);
    prt_r(&format!("Account    : {}", unsafe { player_account }), 13, 31);
    prt_r(&format!("Max Hit Points : {}", unsafe { player_mhp }), 10, 54);
    prt_r(&format!("Cur Hit Points : {}", unsafe { player_chp }), 11, 54);
    prt_r(&format!("Max Mana       : {}", unsafe { player_mana }), 12, 54);
    prt_r(&format!("Cur Mana       : {}", unsafe { player_cmana }), 13, 54);
}

// Display character on screen -RAK-
fn display_char() {
    put_character();
    put_misc1();
    put_stats();
    put_misc2();
    unsafe { cc__put_misc3() };
}

#[no_mangle]
pub extern fn change_name() {
    display_char();
    loop {
        prt_r("<c>hange character name.     <ESCAPE> to continue.", 22, 3);
        match unsafe { inkey() } {
            99 => unsafe { cc__get_name() },
            0 | 3 | 25 | 26 | 27 => break,
            _ => (),
        }

    }
}

#[no_mangle]
pub extern fn create_character() {
        /*
         * This delay may be reduced, but is recomended to keep players
         *
         * from continuously rolling up characters, which can be VERY
         * expensive CPU wise.
         */

        let best: [u8; 6] = [3; 6];
        unsafe {
            loop {
                put_character();
                if cc__choose_race() == 0 {
                    break;
                }
            }

            while cc__get_sex() == 0 {
                put_character();
            }
        }

        let mut is_printed_once: bool = true;
        let mut max_r: [u8; 6] = [0; 6];
        for tstat in stats_iter() {
            unsafe {
                max_r[tstat] = cc__max_stat(140, RACE_STATS[player_prace as usize][tstat]);
            }
        }

        let user: [u8; 6] = [0; 6];
        let mut is_minning: bool = cc__get_minimums(user, max_r) != 0;
        loop {
            get_stats();
            unsafe {
                cc__get_history();
                cc__get_ahw();
            }

            let mut try_count: i64 = 0;
            let mut best_min: i64 = 99999999;
            if satisfied(&mut is_minning, &mut is_printed_once, &mut best_min, &mut try_count, best, user) {
                break;
            }
        }

        unsafe {
            cc__print_history();
            while cc__get_class() == 0 {
                put_character();
                cc__print_history();
                put_misc1();
                put_stats();
            }
        }

        unsafe {
            player_creation_time = time(null::<time_t>() as *mut i64);
            player_save_count = 0;
            player_claim_check = 0;
        }

        get_money();
        unsafe {
            put_stats();
            put_misc2();
            cc__put_misc3();
            cc__get_name();
            pause_exit(24, PLAYER_EXIT_PAUSE);
        }
}
