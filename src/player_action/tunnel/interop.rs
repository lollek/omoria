#[no_mangle]
pub extern "C" fn C_player_action_tunnel_target(y: libc::c_long, x: libc::c_long) {
    super::globals::tunnel_global(y, x);
}
