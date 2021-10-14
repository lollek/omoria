 use libc;

 use save;

#[no_mangle]
pub extern fn C_delete_character() {
    save::delete_character();
}

#[no_mangle]
pub extern fn C_save_character() -> libc::uint8_t {
    match save::save_character() {
        Some(_) => 255,
        None => 0,
    }
}

#[no_mangle]
pub extern fn C_load_character() -> libc::uint8_t {
    match save::load_character() {
        Some(_) => 255,
        None => 0,
    }
}

