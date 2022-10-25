use crate::flow::ability;

#[no_mangle]
pub extern "C" fn C_select_ability() -> u8 {
    match ability::select_ability() {
        true => 255,
        false => 0,
    }
}

#[no_mangle]
pub extern "C" fn C_check_passive_abilities() {
    ability::check_passive_abilities();
}
