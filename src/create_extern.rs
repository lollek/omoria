use create;
use debug;

#[no_mangle]
pub extern fn change_name() {
    debug::enter("create_extern::change_name");
    create::change_name();
    debug::leave("create_extern::change_name");
}

#[no_mangle]
pub extern fn create_character() {
    debug::enter("create_extern::create_character");
    create::create_character();
    debug::leave("create_extern::create_character");
}
