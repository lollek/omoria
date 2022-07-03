use model::Stat;
use data;
use player;
use player::data::PLAYER;

pub fn mod_search_skill(modifier: i16) {
    PLAYER.try_write().unwrap().search_modifier += modifier;
}

fn search_skill_modifier() -> i16 {
    PLAYER.try_read().unwrap().search_modifier
}

pub fn perm_search_skill() -> i16 {
    (data::race::search_mod(&player::race()) + data::class::search_mod(&player::class())).into()
}

pub fn curr_search_skill() -> i16 {
    perm_search_skill()
        + player::modifier_from_stat(Stat::Intelligence)
        + search_skill_modifier()
}

pub fn set_swim_speed(new_value: i64) {
    unsafe { player::player_flags.swim = new_value; }
}

pub fn swim_speed() -> i64 {
    unsafe { player::player_flags.swim }
}
