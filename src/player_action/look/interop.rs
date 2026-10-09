#[no_mangle]
pub extern "C" fn C_player_action_look_direction(direction: libc::c_long) {
    super::globals::look_global(direction);
}
