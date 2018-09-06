use debug;
use menu;
use player;

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


#[no_mangle]
pub extern fn C_main_menu() {
    if let Some(character) = menu::main_menu() {
        player::set_name(&character.name);
        player::set_uid(character.uid.parse::<i64>().unwrap());
    }
}
