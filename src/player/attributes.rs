use player;
use player::data::PLAYER;
use player::data::player_flags;

pub fn is_dead() -> bool {
    unsafe { player_flags.dead != 0 }
}

pub fn is_raging() -> bool {
    PLAYER.read().unwrap().is_raging
}

pub fn set_raging(yn: bool) {
    PLAYER.write().unwrap().is_raging = yn;
}

pub fn is_fatigued() -> bool {
    player::rage_exhaustion_rounds_left() > 0
}

pub fn set_infravision(new_value: i64) {
    unsafe { player::player_flags.see_infra = new_value; }
}

pub fn infravision() -> i64 {
    unsafe { player::player_flags.see_infra }
}
