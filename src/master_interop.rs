use libc;

use crate::debug;
use crate::master;

#[no_mangle]
pub extern fn mst__update_character(uid: libc::int64_t) -> libc::uint8_t {
    match master::update_character(uid) {
        Ok(_) => 255,
        Err(e) => {
            debug::error(format!("Failed to update character: {}", e).as_str());
            return 0;
        },
    }
}

#[no_mangle]
pub extern fn mst__add_character() -> libc::int64_t {
    match master::add_character() {
        Ok(uid) => uid,
        Err(e) => {
            debug::error(format!("Failed to add character: {}", e).as_str());
            return 0;
        },
    }
}
