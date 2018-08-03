use std::ptr::null;
use std::cmp::max;
use std::str;
use std::ffi::CString;

use libc::{c_char, time, time_t};

use debug;
use io;
use misc;
use player;
use ncurses;
use random;
use screen;
use term;

use types::{Class, Stat, StatBlock, stats_iter, Race, races_iter, Currency, Sex};

const PLAYER_EXIT_PAUSE: i32 = 0;

extern "C" {
    fn Pause_Exit(prt_line: i32, delay: i32);
    fn Erase_Line(row: i32, col: i32);
    fn add_money(amount: i64);

    fn cc__get_history();
    fn cc__get_ahw();

    #[link_name = "moria_help"]
    fn C_moria_help(what: *const c_char);
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

fn old_stat(stat: i16) -> i16 {
    if stat < 150 {
        (misc::squish_stat(stat as i32) as i16 + 30) / 10
    } else {
        misc::squish_stat(stat as i32) as i16 - 132
    }
}

fn new_stat(stat: i16) -> i16 {
    if stat < 18 {
        misc::squish_stat(((stat * 10) - 30) as i32) as i16
    } else {
        misc::squish_stat((stat + 132) as i32) as i16
    }
}

fn max_in_statp(stat: i16) -> i16 {
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

fn max_de_statp(stat: i16) -> i16 {
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

fn change_stat(mut cur_stat: i16, amount: i16) -> i16 {
    debug::enter("change_stat");
    if amount < 0 {
        for _ in amount..0 {
            cur_stat -= misc::squish_stat(misc::de_statp(cur_stat as u8) as i32) as i16;
        }
    } else {
        for _ in 1..(amount+1) {
            cur_stat += misc::squish_stat(misc::in_statp(cur_stat as u8) as i32) as i16;
        }
    }
    debug::leave("change_stat");
    cur_stat
}

fn max_stat(mut stat: i16, amount: i16) -> i16 {
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

fn get_min_stat(stat: &str, race_max: i16) -> i16 {
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
                250 as i16
            } else {
                150 + perc as i16
            }
        } else {
            if abil < 3 {
                new_stat(3)
            } else if abil > 18 {
                new_stat(18)
            } else {
                new_stat(abil as i16)
            }
        };
    debug::info(&format!("result: {}", result));
    debug::leave("get_min_stat");
    result
}

// returns [50, 140]
fn random_stat() -> i16 {
    (random::randint(4) + random::randint(4) + random::randint(4) + 2) as i16 * 10
}

fn put_character(show_values: bool) {
    debug::enter("put_character");

    term::clear_from(1);

    term::prt_r("Name      : ", 3, 3);
    term::prt_r("Race      : ", 4, 3);
    term::prt_r("Sex       : ", 5, 3);
    term::prt_r("Class     : ", 6, 3);

    if show_values {
        term::prt_r(&player::name(), 3, 15);
        term::prt_r(&player::race().name(), 4, 15);
        term::prt_r(&player::sex().to_string(), 5, 15);
        term::prt_r(&player::class().name(), 6, 15);
    }

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
            player::player_dis_th = player::tohit_from_stats().into();
            player::player_dis_td = player::dmg_from_str().into();
            player::player_dis_tac = player::ac_from_dex().into();
        }

        erase_line(1, 1);
        term::clear_from(21);
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
            term::clear_from(21);
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
    let race = player::race();
    let race_stats = race.stat_block();

    let new_stats = StatBlock::from(stats_iter()
        .map(|stat| change_stat(random_stat(), race_stats.get_pos(stat)) as i16)
        .collect::<Vec<i16>>());
    player::set_perm_stats(&new_stats);
    player::set_curr_stats(&new_stats);

    unsafe {
        player::player_rep = 0;
        player::player_srh = race.search_mod() as i16;
        player::player_bth = race.melee_bonus() as i16;
        player::player_bthb = race.ranged_bonus() as i16;
        player::player_fos = race.search_freq() as i16;
        player::player_stl = race.stealth_mod() as u8; // TODO BUG! overflows for some races
        player::player_save = race.save_mod() as i16;
        player::player_lev = 1;
        player::player_ptodam = player::dmg_from_str() as i16;
        player::player_ptohit = player::tohit_from_stats() as i16;
        player::player_ptoac = 0;
        player::player_pac = player::ac_from_dex() as i16;
        player::player_expfact = race.expfactor();
        player::player_flags.see_infra = race.infravision() as i64;
        player::player_flags.swim = race.swim_speed() as i64;
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

fn put_misc3() {
    debug::enter("put_misc3");

    term::clear_from(14);

    let xbth: i64 = player::melee_tohit().into();
    let xbthb: i64 = player::ranged_tohit().into();

    let xfos: i64 = max(27 - unsafe { player::player_fos }, 0).into();
    let xsrh: i64 = unsafe { player::player_srh } as i64 +
        misc::mod_from_stat(Stat::Intelligence) as i64;
    let xstl: i64 = unsafe { player::player_stl }.into();
    let xdis: i64 = unsafe { player::player_disarm } as i64 +
        unsafe { player::player_lev } as i64 +
            (2 * player::disarm_from_dex()) as i64 +
                misc::mod_from_stat(Stat::Intelligence) as i64;
    let xsave: i64 = unsafe { player::player_save } as i64 +
        unsafe { player::player_lev } as i64 +
        misc::mod_from_stat(Stat::Wisdom) as i64;
    let xdev: i64 = unsafe { player::player_save } as i64 +
        unsafe { player::player_lev } as i64 +
        misc::mod_from_stat(Stat::Intelligence) as i64;
    let xswm: i64 = unsafe { player::player_flags.swim } + 4;
    let xrep: i64 = 6 + (unsafe { player::player_rep } / 25);
    let xinf: i64 = unsafe { player::player_flags.see_infra } * 10;

    term::prt_r("(Miscellaneous Abilities)", 16, 24);
    term::put_buffer_r(&format!("Fighting    : {}", misc::mod_to_string(xbth, 12)), 17, 2);
    term::put_buffer_r(&format!("Bows/Throw  : {}", misc::mod_to_string(xbthb, 12)), 18, 2);
    term::put_buffer_r(&format!("Saving Throw: {}", misc::mod_to_string(xsave, 6)), 19, 2);
    term::put_buffer_r(&format!("Stealth     : {}", misc::mod_to_string(xstl, 1)), 17, 27);
    term::put_buffer_r(&format!("Disarming   : {}", misc::mod_to_string(xdis, 8)), 18, 27);
    term::put_buffer_r(&format!("Magic Device: {}", misc::mod_to_string(xdev, 7)), 19, 27);
    term::put_buffer_r(&format!("Perception  : {}", misc::mod_to_string(xfos, 3)), 17, 52);
    term::put_buffer_r(&format!("Searching   : {}", misc::mod_to_string(xsrh, 6)), 18, 52);
    term::put_buffer_r(&format!("Infra-Vision: {} feet", xinf), 19, 52);
    term::put_buffer_r(&format!("Swimming    : {}", misc::mod_to_string(xswm, 1)), 20, 52);
    term::put_buffer_r(&format!("Reputation  : {}", misc::mod_to_string(xrep, 1)), 20, 2);

    debug::leave("put_misc3");
}

fn print_history() {
    term::clear_from(14);
    term::put_buffer_r("Character Background", 14, 28);
    for i in 0..5 {
        let c_hist = unsafe { player::player_history[i].to_vec() };
        let hist = misc::c_array_to_rust_string(c_hist);
        term::put_buffer_r(&hist, i as i32 + 15, 5);
    }
}



fn apply_stats_from_class() {
    unsafe {
        player::player_mhp = (player::hitdie() as i8 + player::hp_from_con()).into();
        player::player_chp = player::player_mhp.into();
        player::player_bth += ((player::class().melee_bonus() * 5) + 20) as i16;
        player::player_bthb += ((player::class().ranged_bonus() * 5) + 20) as i16;
        player::player_srh += player::class().search_mod() as i16;
        player::player_disarm += player::class().disarm_mod() as i16;
        player::player_fos += player::class().search_freq() as i16;
        player::player_stl += player::class().stealth_mod() as u8;
        player::player_save += player::class().save_mod() as i16;
        player::player_expfact += player::class().expfactor();
        player::refresh_title();
        player::player_mr = player::class().magic_resist().into();
    }

    let mut player_stat_block = player::perm_stats();
    let class_stat_block = player::class().stat_block();
    for stat in stats_iter() {
        let new_stat = change_stat(
            player_stat_block.get_pos(stat),
            class_stat_block.get_pos(stat));
        player_stat_block.set_pos(stat, new_stat as i16);
    }
    player::set_perm_stats(&player_stat_block);
    player::set_curr_stats(&player_stat_block);

    unsafe {
        // Real values
        player::player_ptodam = player::dmg_from_str() as i16;
        player::player_ptohit = player::tohit_from_stats() as i16;
        player::player_ptoac = player::ac_from_dex() as i16;
        player::player_pac = 0;

        // Displayed values
        player::player_dis_td = player::player_ptodam;
        player::player_dis_th = player::player_ptohit;
        player::player_dis_tac = player::player_ptoac;
        player::player_dis_ac = player::player_pac;
    }
}

// Display character on screen -RAK-
fn display_char() {
    debug::enter("display_char");
    put_character(true);
    put_misc1();
    put_stats();
    put_misc2();
    put_misc3();
    debug::leave("display_char");
}

#[no_mangle]
pub extern fn change_name() {
    debug::enter("change_name");
    display_char();
    loop {
        term::prt_r("<c>hange character name.     <ESCAPE> to continue.", 22, 3);
        match io::inkey_flush() {
            99 => choose_name(),
            0 | 3 | 25 | 26 | 27 => break,
            _ => (),
        }

    }
    debug::leave("change_name");
}

/* Gets a name for the character    -JWT- */
fn choose_name() {
    debug::enter("choose_name");

    term::prt_r("Enter your player's name  [press <RETURN> when finished]", 22, 3);
    loop {
        let new_name = term::get_string(3, 15, 24);
        if !new_name.is_empty() {
            player::set_name(&new_name);
            break;
        }
    }
    term::clear_from(21);

    debug::leave("choose_name");
}

/*	{ Gets a character class				-JWT-	}*/
fn choose_class() -> bool {
    debug::enter("choose_class");

    term::clear_from(21);
    term::prt_r("Choose a class (? for Help):", 21, 3);

    let start_x = 3;

    let mut x = start_x;
    let mut y = 22;
    let available_classes = player::race().available_classes();
    let available_classes_i: Vec<usize> = available_classes.iter()
        .map(|&i| i as usize)
        .collect();

    for class in available_classes {
        let i = class.to_owned() as usize;
        let char_visual = ('a' as u8 + i as u8) as char;
        let class_string = format!("{}) {}", char_visual, class.name());

        term::put_buffer_r(&class_string, y, x);
        x += 15;
        if x > 70 {
            x = start_x;
            y += 1;
        }
    }

    term::put_buffer_r("", 21, 30);

    loop {
        ncurses::move_cursor(5, 14);
        let key = io::inkey_flush();
        let selection = (key as u8 - 'a' as u8) as usize;

        if available_classes_i.contains(&selection) {
            player::set_class(Class::from(selection));
            debug::leave("choose_class");
            return true;
        }

        if selection as u8 as char == '?' {
            unsafe { C_moria_help(CString::new("Character Classes").unwrap().as_ptr()) };
            debug::leave("choose_class");
            return false;
        }
    }
}

/*	{ Allows player to select a race			-JWT- }*/
fn choose_race() -> bool {
    debug::enter("choose_race");

    term::clear_from(21);
    term::prt_r("Choose a race (? for Help):", 21, 3);

    let start_x = 3;

    let mut x = start_x;
    let mut y = 22;
    for i in races_iter() {
        let race = Race::from(i);
        let char_visual = ('a' as u8 + i as u8) as char;
        let race_string = format!("{}) {}", char_visual, race.name());

        term::put_buffer_r(&race_string, y, x);
        x += 15;
        if x > 70 {
            x = start_x;
            y += 1;
        }
    }

    term::put_buffer_r("", 21, 30);

    loop {
        ncurses::move_cursor(3, 14);
        let key = io::inkey_flush();
        let selection = (key as u8 - 'a' as u8) as usize;

        if let Some(_) = races_iter().find(|&i| i == selection) {
            player::set_race(Race::from(selection));
            debug::leave("choose_race");
            return true;
        }

        if selection as u8 as char == '?' {
            unsafe { C_moria_help(CString::new("Character Races").unwrap().as_ptr()) };
            debug::leave("choose_race");
            return false;
        }
    }
}

/*	{ Gets the character's sex				-JWT- }*/
fn choose_sex() -> bool {
    debug::enter("choose_sex");

    if player::race() == Race::Dryad {
        player::set_sex(Sex::Female);
        debug::leave("choose_sex");
        return true;
    }

    term::clear_from(21);
    term::prt_r("Choose a sex (? for Help):", 21, 3);
    term::prt_r("m) Male       f) Female", 22, 3);
    term::prt_r("", 21, 29);

    loop {
        ncurses::move_cursor(4, 14);
        let key = io::inkey_flush() as char;

        if key == 'f' {
            player::set_sex(Sex::Female);
            debug::leave("choose_sex");
            return true;
        }

        if key == 'm' {
            player::set_sex(Sex::Male);
            debug::leave("choose_sex");
            return true;
        }

        if key == '?' {
            unsafe { C_moria_help(CString::new("Character Sex").unwrap().as_ptr()) };
            debug::leave("choose_sex");
            return false;
        }
    }
}

fn choose_stats() {
    debug::enter("choose_stats");
    let player_race = player::race();
    let race_stats = player_race.stat_block();

    let max_stats: Vec<i16> = stats_iter()
        .map(|stat| max_stat(140, race_stats.get_pos(stat)))
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

    put_character(false);
    loop {
        if choose_race() {
            break;
        }
    }
    term::put_buffer_r(&player::race().name(), 4, 15);

    loop {
        if choose_sex() {
            break;
        }
    }
    term::put_buffer_r(&player::sex().to_string(), 5, 15);

    choose_stats();
    print_history();

    loop {
        if choose_class() {
            break;
        }
    }

    term::clear_from(21);
    term::put_buffer_r(&player::class().name(), 6, 15);
    apply_stats_from_class();

    print_history();
    put_misc1();
    put_stats();

    unsafe {
        player::player_creation_time = time(null::<time_t>() as *mut i64);
        player::player_save_count = 0;
        player::player_claim_check = 0;
    }

    get_money();
    put_stats();
    put_misc2();
    put_misc3();
    choose_name();
    pause_exit(24, PLAYER_EXIT_PAUSE);
    debug::leave("create_character");
}
