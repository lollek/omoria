use libc;

use master;

#[no_mangle]
pub extern fn C_master_update_character(uid: libc::int64_t) -> libc::uint8_t {
    match master::master_update_character(uid) {
        Some(_) => 255,
        None => 0,
    }
}

#[no_mangle]
pub extern fn C_master_add_character() -> libc::int64_t {
    match master::master_add_character() {
        Some(uid) => uid,
        None => 0,
    }
}

#[no_mangle]
pub extern fn C_master_character_exists(uid: libc::int64_t) -> libc::uint8_t {
    match master::master_character_exists(uid) {
        Some(_) => 255,
        None => 0,
    }
}
