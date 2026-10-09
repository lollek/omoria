#[no_mangle]
pub extern "C" fn player_action_toggle_light_source() {
    super::globals::toggle_global();
}
