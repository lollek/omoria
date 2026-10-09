#[no_mangle]
pub extern "C" fn player_action_search(y: libc::c_long, x: libc::c_long, chance: libc::c_long) {
    super::globals::search_global(y, x, chance);
}
