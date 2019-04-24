use ability;

#[no_mangle]
pub extern fn C_select_ability() -> libc::uint8_t {
    match ability::select_ability() {
        true => 255,
        false => 0,
    }
}

#[no_mangle]
pub extern fn C_check_passive_abilities() {
    ability::check_passive_abilities();
}
