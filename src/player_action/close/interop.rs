#[no_mangle]
pub extern "C" fn C_player_action_close_target(y: libc::c_long, x: libc::c_long) {
    super::globals::close_global(y, x);
}
