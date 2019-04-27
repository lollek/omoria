use types::Stat;

use player;
use player::data::PLAYER;

pub fn mod_search_skill(modifier: i16) {
    PLAYER.write().unwrap().search_modifier += modifier;
}

fn search_skill_modifier() -> i16 {
    PLAYER.read().unwrap().search_modifier
}

pub fn get_perm_search_skill() -> i16 {
    (player::race().search_mod() + player::class().search_mod()).into()
}

pub fn get_curr_search_skill() -> i16 {
    get_perm_search_skill()
        + player::modifier_from_stat(Stat::Intelligence)
        + search_skill_modifier()
}
