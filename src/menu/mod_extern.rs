use crate::debug;
use crate::menu;

#[no_mangle]
pub extern fn change_name() {
    debug::enter("menu_extern::change_name");

    menu::change_name();

    debug::leave("menu_extern::change_name");
}

#[no_mangle]
pub extern fn create_character() {
    debug::enter("menu_extern::create_character");

    menu::create_character();

    debug::leave("menu_extern::create_character");
}
