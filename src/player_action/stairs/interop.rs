use super::Direction;

#[no_mangle]
pub extern "C" fn player_action_ascend_stairs() {
    super::globals::take_stairs_global(Direction::Up);
}

#[no_mangle]
pub extern "C" fn player_action_descend_stairs() {
    super::globals::take_stairs_global(Direction::Down);
}
