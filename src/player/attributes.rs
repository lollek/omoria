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
    player::get_rage_exhaustion_rounds_left() > 0
}

