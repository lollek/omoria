use crate::model::Stat;
use crate::user_interface::helpers;
use crate::{data, misc, ncurses, player, term};
use std::cmp::max;
use crate::player_action::attack::{calculate_number_of_attacks, calculate_player_tohit2, AttackType, MeleeAttackType};

pub fn character_screen() {
    put_character(true);
    put_misc1();
    put_stats();
    put_misc2();
    put_misc3();
}

fn put_character(show_values: bool) {
    term::prt("Name      : ", 2, 2);
    term::prt("Race      : ", 3, 2);
    term::prt("Sex       : ", 4, 2);
    term::prt("Class     : ", 5, 2);

    if show_values {
        term::prt(player::name(), 2, 14);
        term::prt(data::race::name(&player::race()), 3, 14);
        term::prt(data::sex::name(&player::sex()), 4, 14);
        term::prt(data::class::name(&player::class()), 5, 14);
    }
}

fn put_stats() {
    helpers::print_stats(2, 64);

    term::prt(
        format!("  Attacks  : {}", calculate_number_of_attacks()),
        8,
        3,
    );
    term::prt(
        format!("  Melee atk: {}", calculate_player_tohit2(&AttackType::Melee(MeleeAttackType::Standard))),
        9,
        3,
    );
    term::prt(
        format!("+ To Damage: {}", unsafe { player::player_dis_td }),
        10,
        3,
    );
    term::prt(
        format!("+ To AC    : {}", unsafe { player::player_dis_tac }),
        11,
        3,
    );
    term::prt(
        format!("  Total AC : {}", unsafe { player::player_dis_ac }),
        12,
        3,
    );
}

fn put_misc1() {
    term::prt(
        format!("Age          : {}", unsafe { player::player_age }),
        2,
        39,
    );
    term::prt(
        format!("Height       : {}", unsafe { player::player_ht }),
        3,
        39,
    );
    term::prt(
        format!("Weight       : {}", unsafe { player::player_wt }),
        4,
        39,
    );
    term::prt(
        format!("Social Class : {}", unsafe { player::player_sc }),
        5,
        39,
    );
}

fn put_misc2() {
    term::prt(
        format!("Level      : {}", unsafe { player::player_lev }),
        9,
        30,
    );
    term::prt(
        format!("Experience : {}", unsafe { player::player_exp }),
        10,
        30,
    );
    term::prt(format!("Gold       : {}", player::wallet().total), 11, 30);
    term::prt(
        format!("Account    : {}", unsafe { player::player_account }),
        12,
        30,
    );
    term::prt(format!("Max Hit Points : {}", player::max_hp()), 9, 53);
    term::prt(format!("Cur Hit Points : {}", player::current_hp()), 10, 53);
    term::prt(
        format!("Max Mana       : {}", unsafe { player::player_mana }),
        11,
        53,
    );
    term::prt(
        format!("Cur Mana       : {}", unsafe { player::player_cmana }),
        12,
        53,
    );
}

fn put_misc3() {
    term::clear_from(14);

    let xbth: i64 = player::melee_tohit().into();
    let xbthb: i64 = player::ranged_tohit().into();

    let xfos: i64 = max(27 - unsafe { player::player_fos }, 0).into();
    let xsrh: i64 = player::curr_search_skill().into();
    let xstl: i64 = unsafe { player::player_stl }.into();
    let xdis: i64 = unsafe { player::player_disarm } as i64
        + unsafe { player::player_lev } as i64
        + (2 * player::disarm_from_dex()) as i64
        + player::modifier_from_stat(Stat::Intelligence) as i64;
    let xsave: i64 = unsafe { player::player_save } as i64
        + unsafe { player::player_lev } as i64
        + player::modifier_from_stat(Stat::Wisdom) as i64;
    let xdev: i64 = unsafe { player::player_save } as i64
        + unsafe { player::player_lev } as i64
        + player::modifier_from_stat(Stat::Intelligence) as i64;
    let xswm: i64 = player::swim_speed() + 4;
    let xrep: i64 = 6 + (unsafe { player::player_rep } / 25);
    let xinf: i64 = player::infravision() * 10;

    term::prt("(Miscellaneous Abilities)", 15, 23);
    let msg = format!("Fighting    : {}", misc::mod_to_string(xbth, 12));
    ncurses::mvaddstr(16, 1, msg);
    let msg = format!("Bows/Throw  : {}", misc::mod_to_string(xbthb, 12));
    ncurses::mvaddstr(17, 1, msg);
    let msg = format!("Saving Throw: {}", misc::mod_to_string(xsave, 6));
    ncurses::mvaddstr(18, 1, msg);
    let msg = format!("Stealth     : {}", misc::mod_to_string(xstl, 1));
    ncurses::mvaddstr(16, 26, msg);
    let msg = format!("Disarming   : {}", misc::mod_to_string(xdis, 8));
    ncurses::mvaddstr(17, 26, msg);
    let msg = format!("Magic Device: {}", misc::mod_to_string(xdev, 7));
    ncurses::mvaddstr(18, 26, msg);
    let msg = format!("Perception  : {}", misc::mod_to_string(xfos, 3));
    ncurses::mvaddstr(16, 51, msg);
    let msg = format!("Searching   : {}", misc::mod_to_string(xsrh, 6));
    ncurses::mvaddstr(17, 51, msg);
    let msg = format!("Infra-Vision: {} feet", xinf);
    ncurses::mvaddstr(18, 51, msg);
    let msg = format!("Swimming    : {}", misc::mod_to_string(xswm, 1));
    ncurses::mvaddstr(19, 51, msg);
    let msg = format!("Reputation  : {}", misc::mod_to_string(xrep, 1));
    ncurses::mvaddstr(19, 1, msg);
}
