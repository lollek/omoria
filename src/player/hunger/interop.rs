#[no_mangle]
pub extern "C" fn player_hunger_recalculate() {
    super::globals::recalculate_global();
}
