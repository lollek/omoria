use crate::debug;
use crate::master;

#[no_mangle]
pub extern fn mst__update_character(uid: i64) -> u8 {
    match master::update_character(uid) {
        Ok(_) => 255,
        Err(e) => {
            debug::error(format!("Failed to update character: {}", e).as_str());
            0
        },
    }
}

#[no_mangle]
pub extern fn mst__add_character() -> i64 {
    match master::add_character() {
        Ok(uid) => uid,
        Err(e) => {
            debug::error(format!("Failed to add character: {}", e).as_str());
            0
        },
    }
}
