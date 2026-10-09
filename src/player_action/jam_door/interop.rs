#[no_mangle]
pub extern "C" fn C_player_action_jam_door_target(y: libc::c_long, x: libc::c_long) {
    super::globals::jam_global(y, x);
}
