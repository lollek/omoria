#[no_mangle]
pub extern "C" fn C_player_action_disarm_target(
    y: libc::c_long,
    x: libc::c_long,
    direction: libc::c_long,
) {
    super::globals::disarm_global(y, x, direction);
}
