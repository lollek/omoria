#[no_mangle]
pub extern "C" fn C_player_action_open_target(y: libc::c_long, x: libc::c_long) {
    super::globals::open_global(y, x);
}
