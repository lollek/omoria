use crate::data;
use crate::io;
use crate::logic::menu;
use crate::model;
use crate::player;
use crate::term;

// Let the player select an action
// Returns true if the turn was free (i.e. don't take a turn)
pub fn select_ability() -> bool {
    let abilities = player::abilities();
    let abilities_str = abilities
        .iter()
        .map(data::ability::name)
        .collect::<Vec<&str>>();

    menu::draw_quick_menu("Use which ability?", &abilities_str);

    loop {
        let selection = io::inkey_flush() as char;
        let max_selection = (('a' as usize) + abilities.len()) as u8 as char;
        if selection < 'a' || selection >= max_selection {
            return true;
        }
        let index = (selection as u8 - 'a' as u8) as usize;
        return use_ability(abilities[index as usize]);
    }
}

// Activate an ability
// Returns true if the action was a free action (i.e. don't take a turn)
pub fn use_ability(ability: model::Ability) -> bool {
    match ability {
        model::Ability::Rage => rage(),
    }
}

pub fn rage() -> bool {
    if player::rage_rounds_spent() > 0 {
        term::msg_print("You need to rest before you can rage again");
        return true;
    }

    if player::rage_exhaustion_rounds_left() > 0 {
        term::msg_print("You are too exhausted to rage");
        return true;
    }

    player::set_raging(true);
    term::msg_print("You enter a rage");
    player::recalc_curr_stats();
    return true;
}

pub fn check_passive_abilities() {
    let mut exhaust_rounds_left = player::rage_exhaustion_rounds_left();
    if exhaust_rounds_left > 0 {
        exhaust_rounds_left -= 1;
        player::set_rage_exhaustion_rounds_left(exhaust_rounds_left);
        if exhaust_rounds_left == 0 {
            term::msg_print("You are no longer exhausted");
            player::recalc_curr_stats();
        }
    }

    if player::is_raging() {
        let rage_rounds = player::rage_rounds_spent() + 1;
        player::set_rage_rounds_spent(rage_rounds);
        if rage_rounds > player::max_rage_rounds() {
            player::set_raging(false);
            player::set_rage_rounds_spent(0);
            player::set_rage_exhaustion_rounds_left(rage_rounds * 2);
            term::msg_print("You leave your rage feeling exhausted");
            player::recalc_curr_stats();
        }
    }
}
