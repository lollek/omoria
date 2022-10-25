use crate::debug;

#[no_mangle]
pub extern fn change_name() {
    super::create_character::change_name();
}

#[no_mangle]
pub extern fn create_character() {
    super::create_character::create_character();
}
