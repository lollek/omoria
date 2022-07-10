use crate::debug;

#[no_mangle]
pub extern fn change_name() {
    debug::enter("menu_extern::change_name");

    super::create_character::change_name();

    debug::leave("menu_extern::change_name");
}

#[no_mangle]
pub extern fn create_character() {
    debug::enter("menu_extern::create_character");

    super::create_character::create_character();

    debug::leave("menu_extern::create_character");
}
