use main_menu;
use player;

#[no_mangle]
pub extern fn C_main_menu() {
    if let Some(character) = main_menu::main_menu() {
        player::set_name(&character.name);
        player::set_uid(character.uid.parse::<i64>().unwrap());
    }
}
