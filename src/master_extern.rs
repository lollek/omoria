use libc;

use master;

#[no_mangle]
pub extern fn mst__update_character(uid: libc::int64_t) -> libc::uint8_t {
    match master::update_character(uid) {
        Some(_) => 255,
        None => 0,
    }
}

#[no_mangle]
pub extern fn mst__add_character() -> libc::int64_t {
    match master::add_character() {
        Some(uid) => uid,
        None => 0,
    }
}
