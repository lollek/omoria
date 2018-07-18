use std::ptr::null;
use std::cmp::max;
use std::str;

use libc::{time, time_t};

use debug;
use io;
use misc;
use player;
use random;
use screen;
use term;

use types::{Stat, StatBlock, stats_iter, Currency};

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

const PLAYER_EXIT_PAUSE: i32 = 0;

extern "C" {
    fn Clear_From(row: i32);
    fn Pause_Exit(prt_line: i32, delay: i32);
    fn Erase_Line(row: i32, col: i32);
    fn add_money(amount: i64);
    fn inkey() -> u8;

    fn tohit_adj() -> i64;
    fn todam_adj() -> i64;
    fn toac_adj() -> i64;

    fn cc__get_name();
    fn cc__choose_race() -> u8;
    fn cc__put_misc3();
    fn cc__get_class() -> u8;
    fn cc__print_history();
    fn cc__get_history();
    fn cc__get_ahw();
    fn cc__get_sex() -> u8;
}

fn clear_from(row: i32) {
    debug::enter("clear_from");
    let result = unsafe { Clear_From(row - 1) };
    debug::leave("clear_from");
    result
}

fn pause_exit(prt_line: i32, delay: i32) {
    debug::enter("pause_exit");
    let result = unsafe { Pause_Exit(prt_line -1, delay) };
    debug::leave("pause_exit");
    result
}

fn erase_line(row: i32, col: i32) {
    debug::enter("erase_line");
    let result = unsafe { Erase_Line(row -1, col -1) };
    debug::leave("erase_line");
    result
}

fn old_stat(stat: u8) -> u8 {
    if stat < 150 {
        (misc::squish_stat(stat as i32) + 30) / 10
    } else {
        misc::squish_stat(stat as i32) - 132
    }
}

fn new_stat(stat: u8) -> u8 {
    if stat < 18 {
        misc::squish_stat(((stat * 10) - 30) as i32)
    } else {
        misc::squish_stat((stat + 132) as i32)
    }
}

fn max_in_statp(stat: u8) -> u8 {
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

fn max_de_statp(stat: u8) -> u8 {
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

fn change_stat(mut cur_stat: u8, amount: i64) -> u8 {
    debug::enter("change_stat");
    if amount < 0 {
        for _ in amount..0 {
            cur_stat -= misc::squish_stat(misc::de_statp(cur_stat) as i32);
        }
    } else {
        for _ in 1..(amount+1) {
            cur_stat += misc::squish_stat(misc::in_statp(cur_stat) as i32);
        }
    }
    debug::leave("change_stat");
    cur_stat
}

fn max_stat(mut stat: u8, amount: i8) -> u8 {
    debug::enter("max_stat");
    if amount < 0 {
        for _ in amount..0 {
            stat = max_de_statp(stat);
        }
    } else {
        for _ in 1..(amount+1) {
            stat = max_in_statp(stat);
        }
    }
    debug::leave("max_stat");
    stat
}

fn next_best_stats(curr: &StatBlock, user: &StatBlock, best: &mut StatBlock,
                   best_min: i64) -> i64 {
    debug::enter("next_best_stats");
    let mut below_sum: i64 = 0;

    for tstat in stats_iter() {
        if curr.get_pos(tstat) < user.get_pos(tstat) {
            let below: i64 = (user.get_pos(tstat) - curr.get_pos(tstat)) as i64;
            below_sum += (below * (below + 1)) / 2;
            debug::info(&format!("curr: {}, user: {}, below: {}, below_sum: {}",
                                 curr.get_pos(tstat), user.get_pos(tstat),
                                 below, below_sum));
        }
    }

    let result =
        if below_sum < best_min {
            for tstat in stats_iter() {
                best.set_pos(tstat, curr.get_pos(tstat));
            }
            below_sum
        } else {
            best_min
        };
    debug::leave("next_best_stats");
    result
}

fn get_min_stat(stat: &str, race_max: u8) -> u8 {
    debug::enter("get_min_stat");
    let out_str =
        if race_max >= 250 {
            format!("Min {} (racial max 18/00) : ", stat)
        } else if race_max > 150 {
            format!("Min {} (racial max 18/{}) : ", stat, race_max - 150)
        } else {
            format!("Min {} (racial max {}) : ", stat, old_stat(race_max))
        };

    term::prt_r(&out_str, 1, 1);

    let abil: u32;
    let mut perc: u32;
    loop {
        let tmp_str = term::get_string(1, out_str.len() as i32 + 1, 10);
        if tmp_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = tmp_str.split("/").collect();

        perc = 0;
        if parts.len() > 1 {
            if let Ok(new_perc) = parts[1].parse::<u32>() {
                perc = max(new_perc, 100);
            }
        }

        if parts.len() > 0 {
            if let Ok(new_abil) = parts[0].parse::<u32>() {
                abil = new_abil;
                break;
            }
        }
    }

    debug::info(&format!("abil: {}, perc: {}", abil, perc));

    let result =
        if abil == 18 {
            if perc == 0 {
                250 as u8
            } else {
                150 + perc as u8
            }
        } else {
            if abil < 3 {
                new_stat(3)
            } else if abil > 18 {
                new_stat(18)
            } else {
                new_stat(abil as u8)
            }
        };
    debug::info(&format!("result: {}", result));
    debug::leave("get_min_stat");
    result
}

// returns [50, 140]
fn random_stat() -> u8 {
    (random::randint(4) + random::randint(4) + random::randint(4) + 2) as u8 * 10
}

fn put_character() {
    debug::enter("put_character");
    clear_from(1);
    term::prt_r(&format!("Name      : {}", player::name()), 3, 3);
    term::prt_r(&format!("Race      : {}", player::race_string()), 4, 3);
    term::prt_r(&format!("Sex       : {}", player::sex_string()), 5, 3);
    term::prt_r(&format!("Class     : {}", player::class_string()), 6, 3);
    debug::leave("put_character");
}

fn get_money() {
    debug::enter("get_money");
    let player_stats = player::curr_stats();
    let mut amount: i64 =
        325 + random::randint(25)
        // Social Class adj
        + unsafe { player::player_sc } as i64 * 6
        // Stat adj
        - stats_iter().fold(0, |sum: i64, tstat|
            sum + old_stat(player_stats.get_pos(tstat)) as i64)
        // Charisma adj
        + old_stat(player_stats.get(Stat::Charisma)) as i64;

    // Minimum
    amount = max(amount, 80);

    let gold_value = Currency::Gold.value();
    unsafe { add_money((amount * gold_value) + random::randint(gold_value)) };
    debug::leave("get_money");
}

fn satisfied(minning: &mut bool,  printed_once: &mut bool, best_min: &mut i64,
                 try_count: &mut i64, mut best: &mut StatBlock, user: &StatBlock) -> bool {
    debug::enter("satisfied");
    let mut is_satisfied: bool;

    if !*minning {
        /*
         * Figure out what the current bonuses are
         * so the player has a clue
         */
        unsafe {
            player::player_dis_th = tohit_adj() as i16;
            player::player_dis_td = todam_adj() as i16;
            player::player_dis_tac = toac_adj() as i16;
        }

        erase_line(1, 1);
        clear_from(21);
        put_misc1();
        put_stats();

        term::prt_r("Press 'R' to reroll, <RETURN> to continue:", 21, 3);
        *printed_once = true;

        loop {
            let s: u8 = io::inkey_flush();
            is_satisfied = s != 'R' as u8;
            if s == 10 || s == 'R' as u8 {
                break;
            }
        }

    } else { /* minning */

        if !*printed_once {
            clear_from(21);
            term::prt_r("Press any key to give up (50000 rolls max): ", 21, 3);
            term::refresh_screen();
            *printed_once = true;
        }

        *best_min = next_best_stats(&player::perm_stats(), &user, &mut best, *best_min);

        *try_count += 1;
        if (*try_count % 250) == 0 {
            term::prt_r(&format!("Try = {:10}", try_count), 21, 60);
            term::refresh_screen();
        }

        let s: u8 = io::inkey_delay(0);
        if s != 0 || *try_count == 50000 {
            *minning = false;
            *printed_once = false;
            player::set_perm_stats(&best);
            player::set_curr_stats(&best);
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

    debug::leave("satisfied");
    is_satisfied
}

fn get_stats() {
    debug::enter("get_stats");
    let race = unsafe { player::player_prace } as usize;

    let new_stats = StatBlock::from(stats_iter()
        .map(|stat| change_stat(random_stat(), RACE_STATS[race][stat] as i64) as u8)
        .collect::<Vec<u8>>());
    player::set_perm_stats(&new_stats);
    player::set_curr_stats(&new_stats);

    unsafe {
        player::player_rep = 0;
        player::player_srh = SEARCH_MOD[race] as i16;
        player::player_bth = MELEE_BONUS[race] as i16;
        player::player_bthb = RANGED_BONUS[race] as i16;
        player::player_fos = SEARCH_FREQ[race] as i16;
        player::player_stl = STEALTH_MOD[race] as u8; // TODO BUG! overflows for some races
        player::player_save = SAVE_MOD[race] as i16;
        player::player_hitdie = HEALTH_BONUS[race];
        player::player_lev = 1;
        player::player_ptodam = todam_adj() as i16;
        player::player_ptohit = tohit_adj() as i16;
        player::player_ptoac = 0;
        player::player_pac = toac_adj() as i16;
        player::player_expfact = EXPFACTOR[race];
        player::player_flags.see_infra = INFRAVISION[race] as i64;
        player::player_flags.swim = SWIM_SPEED[race] as i64;
    }
    debug::leave("get_stats");
}

fn put_stats() {
    debug::enter("put_stats");
    screen::prt_6_stats(&player::curr_stats(), 3, 65);
    term::prt_r(&format!("+ To Hit   : {}", unsafe { player::player_dis_th }), 10, 4);
    term::prt_r(&format!("+ To Damage: {}", unsafe { player::player_dis_td }), 11, 4);
    term::prt_r(&format!("+ To AC    : {}", unsafe { player::player_dis_tac }), 12, 4);
    term::prt_r(&format!("  Total AC : {}", unsafe { player::player_dis_ac }), 13, 4);
    debug::leave("put_stats");
}

fn put_misc1() {
    debug::enter("put_misc1");
    term::prt_r(&format!("Age          : {}", unsafe { player::player_age }), 3, 40);
    term::prt_r(&format!("Height       : {}", unsafe { player::player_ht }), 4, 40);
    term::prt_r(&format!("Weight       : {}", unsafe { player::player_wt }), 5, 40);
    term::prt_r(&format!("Social Class : {}", unsafe { player::player_sc }), 6, 40);
    term::prt_r(&format!("Difficulty   : {}", unsafe { player::player_diffic }), 7, 40);
    debug::leave("put_misc1");
}

fn put_misc2() {
    debug::enter("put_misc2");
    term::prt_r(&format!("Level      : {}", unsafe { player::player_lev }), 10, 31);
    term::prt_r(&format!("Experience : {}", unsafe { player::player_exp }), 11, 31);
    term::prt_r(&format!("Gold       : {}", player::wallet().total), 12, 31);
    term::prt_r(&format!("Account    : {}", unsafe { player::player_account }), 13, 31);
    term::prt_r(&format!("Max Hit Points : {}", unsafe { player::player_mhp }), 10, 54);
    term::prt_r(&format!("Cur Hit Points : {}", unsafe { player::player_chp }), 11, 54);
    term::prt_r(&format!("Max Mana       : {}", unsafe { player::player_mana }), 12, 54);
    term::prt_r(&format!("Cur Mana       : {}", unsafe { player::player_cmana }), 13, 54);
    debug::leave("put_misc2");
}

// Display character on screen -RAK-
fn display_char() {
    debug::enter("display_char");
    put_character();
    put_misc1();
    put_stats();
    put_misc2();
    unsafe { cc__put_misc3() };
    debug::leave("display_char");
}

#[no_mangle]
pub extern fn change_name() {
    debug::enter("change_name");
    display_char();
    loop {
        term::prt_r("<c>hange character name.     <ESCAPE> to continue.", 22, 3);
        match unsafe { inkey() } {
            99 => unsafe { cc__get_name() },
            0 | 3 | 25 | 26 | 27 => break,
            _ => (),
        }

    }
    debug::leave("change_name");
}

fn choose_stats() {
    debug::enter("choose_stats");
    let player_race = unsafe { player::player_prace } as usize;

    let max_stats: Vec<u8> = stats_iter()
        .map(|stat| max_stat(140, RACE_STATS[player_race][stat]))
        .collect();


    let mut is_minning = io::get_yes_no("Do you wish to try for minimum statistics?");
    let mut user: StatBlock = StatBlock::new(0);
    if is_minning {
        user.strength = get_min_stat("STR", max_stats[0]);
        user.intelligence = get_min_stat("INT", max_stats[1]);
        user.wisdom = get_min_stat("WIS", max_stats[2]);
        user.dexterity = get_min_stat("DEX", max_stats[3]);
        user.constitution = get_min_stat("CON", max_stats[4]);
        user.charisma = get_min_stat("CHR", max_stats[5]);
        screen::prt_6_stats(&user, 3, 65);
    }

    let mut best: StatBlock = StatBlock::new(3);
    let mut is_printed_once: bool = true;

    loop {
        get_stats();
        unsafe {
            cc__get_history();
            cc__get_ahw();
        }

        /*
         * This delay may be reduced, but is recomended to keep players
         *
         * from continuously rolling up characters, which can be VERY
         * expensive CPU wise.
         */

        let mut try_count: i64 = 0;
        let mut best_min: i64 = 99999999;
        if satisfied(&mut is_minning, &mut is_printed_once, &mut best_min, &mut try_count, &mut best, &user) {
            break;
        }
    }

    debug::leave("choose_stats");
}

#[no_mangle]
pub extern fn create_character() {
    debug::enter("create_character");

    loop {
        put_character();
        if unsafe { cc__choose_race() } != 0 {
            break;
        }
    }

    while unsafe { cc__get_sex() } == 0 {
        put_character();
    }

    choose_stats();

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
        player::player_creation_time = time(null::<time_t>() as *mut i64);
        player::player_save_count = 0;
        player::player_claim_check = 0;
    }

    get_money();
    put_stats();
    put_misc2();
    unsafe {
        cc__put_misc3();
        cc__get_name();
        pause_exit(24, PLAYER_EXIT_PAUSE);
    }
    debug::leave("create_character");
}
