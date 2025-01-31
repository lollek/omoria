use crate::debug;
use crate::master;

#[no_mangle]
pub extern "C" fn mst__init_masters() -> u8 {
    match master::init_master() {
        Ok(_) => 255,
        Err(e) => {
            debug::error(format!("Failed to init masters database: {}", e).as_str());
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn mst__update_character(uid: i64) -> u8 {
    match master::update_character(uid) {
        Ok(_) => 255,
        Err(e) => {
            debug::error(format!("Failed to update character: {}", e).as_str());
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn mst__add_character() -> i64 {
    master::add_character().unwrap_or_else(|e| {
        debug::error(format!("Failed to add character: {}", e).as_str());
        0
    })
}
