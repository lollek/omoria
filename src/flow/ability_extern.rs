use crate::flow::ability;

#[no_mangle]
pub extern "C" fn C_select_ability() -> bool {
    ability::select_ability()
}

#[no_mangle]
pub extern "C" fn C_check_passive_abilities() {
    ability::check_passive_abilities();
}
