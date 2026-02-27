use crate::model::Stat;
use crate::user_interface::helpers;
use crate::{data, helper, misc, ncurses, player, term};
use std::cmp::max;
use crate::player_action::attack::{calculate_number_of_attacks, calculate_player_tohit, AttackType, MeleeAttackType};

pub fn character_screen() {
    put_character(true);
    put_physical_aspects();
    put_combat_abilities();
    put_misc_abilities();
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

fn put_combat_abilities() {
    let left_column = 2;
    let middle_column = 30;
    let right_column = 64;
    let starting_row = 7;

    term::prt("(Combat Abilities)", starting_row, 23);
    let mut row = starting_row;
    for line in [
        format!("Num attacks:  {}", calculate_number_of_attacks()),
        format!("Melee to hit: {} ({} {})", calculate_player_tohit(AttackType::Melee(MeleeAttackType::Standard)), player::base_to_hit(), helper::format_signed(player::player_ptohit())),
        format!("Damage:       {} {}", player::player_main_weapon().damage_string(), helper::format_signed(player::plus_to_damage())),
        format!("AC:           {} ({} {})", player::base_ac() + player::plus_to_ac(), player::base_ac(), helper::format_signed(player::plus_to_ac())),
    ] {
        row += 1;
        term::prt(line, row, left_column);
    }

    let mut row = starting_row;
    for line in [
        format!("Level:  {} ({} to next)", unsafe { player::player_lev } , player::exp_to_next_level()),
        format!("Gold:   {} (Bank: {})", player::wallet().total, unsafe { player::player_account }),
        format!("Health: {}/{}", player::current_hp(), player::max_hp()),
        format!("Mana:   {}/{}", player::current_mp(), player::max_mp()),
    ] {
        row += 1;
        term::prt(line, row, middle_column);
    }

    helpers::print_stats(starting_row as u8 + 1, right_column);
}

fn put_physical_aspects() {
    let column = 39;
    let starting_row = 1;

    let mut row = starting_row;
    term::prt("(Physical Aspects)", row, 23);
    for line in [
        format!("Age:          {}", unsafe { player::player_age }),
        format!("Height:       {}", unsafe { player::player_ht }),
        format!("Height:       {}", unsafe { player::player_wt }),
        format!("Social class: {}", player::social_class()),
    ] {
        row += 1;
        term::prt(line, row, column);
    }
}

fn put_misc_abilities() {
    term::clear_from(14);

    let melee_to_hit: i64 = (player::base_to_hit() + player::plus_to_hit(AttackType::Melee(MeleeAttackType::Standard), player::player_main_weapon())).into();
    let ranged_to_hit: i64 = (player::base_to_hit_bows() + player::plus_to_hit(AttackType::Ranged, player::player_main_weapon())).into();

    let perception: i64 = max(27 - player::search_frequency(), 0).into();
    let searching: i64 = player::curr_search_skill().into();
    let stealth: i64 = player::stealth().into();
    let disarming: i64 = unsafe { player::player_disarm } as i64
        + unsafe { player::player_lev } as i64
        + (2 * player::disarm_from_dex()) as i64
        + player::modifier_from_stat(Stat::Intelligence) as i64;
    let saving_throw: i64 = unsafe { player::player_save } as i64
        + unsafe { player::player_lev } as i64
        + player::modifier_from_stat(Stat::Wisdom) as i64;
    let magic_devices: i64 = unsafe { player::player_save } as i64
        + unsafe { player::player_lev } as i64
        + player::modifier_from_stat(Stat::Intelligence) as i64;
    let swimming: i64 = player::swim_speed() + 4;
    let reputation: i64 = 6 + (unsafe { player::player_rep } / 25);
    let infravision: i64 = player::infravision() * 10;

    term::prt("(Miscellaneous Abilities)", 15, 23);
    ncurses::mvaddstr(16, 1, format!("Fighting    : {}", misc::mod_to_string(melee_to_hit, 12)));
    ncurses::mvaddstr(17, 1, format!("Bows/Throw  : {}", misc::mod_to_string(ranged_to_hit, 12)));
    ncurses::mvaddstr(18, 1, format!("Saving Throw: {}", misc::mod_to_string(saving_throw, 6)));
    ncurses::mvaddstr(19, 1, format!("Reputation  : {}", misc::mod_to_string(reputation, 1)));
    ncurses::mvaddstr(16, 26, format!("Stealth     : {}", misc::mod_to_string(stealth, 1)));
    ncurses::mvaddstr(17, 26, format!("Disarming   : {}", misc::mod_to_string(disarming, 8)));
    ncurses::mvaddstr(18, 26, format!("Magic Device: {}", misc::mod_to_string(magic_devices, 7)));
    ncurses::mvaddstr(16, 51, format!("Perception  : {}", misc::mod_to_string(perception, 3)));
    ncurses::mvaddstr(17, 51, format!("Searching   : {}", misc::mod_to_string(searching, 6)));
    ncurses::mvaddstr(18, 51, format!("Infra-Vision: {} feet", infravision));
    ncurses::mvaddstr(19, 51, format!("Swimming    : {}", misc::mod_to_string(swimming, 1)));
}
