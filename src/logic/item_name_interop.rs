use logic::item_name;
use debug;

use pancurses;
use libc;

#[no_mangle]
pub extern fn C_item_name_generate_name(item_ptr: *const model::Item, result: [libc::c_char; 70]) {
    debug::enter(&format!("C_item_name_generate_name"));

    let item = unsafe { *item_ptr };
    let item_name = item_name::generate_name(item);
    result = [0; 70];
    for (index, c) in item_name.chars().take(70 - 1).enumerate() {
        result[index] = c as libc::c_char;
    }

    debug::leave("C_item_name_generate_name");
}

