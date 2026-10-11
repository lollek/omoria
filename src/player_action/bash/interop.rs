#[no_mangle]
pub extern "C" fn C_player_action_bash_target(y: libc::c_long, x: libc::c_long) {
    super::globals::bash_global(y, x);
}
