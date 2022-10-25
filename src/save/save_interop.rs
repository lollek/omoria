use libc;
use std::ffi::CStr;

use crate::save;

#[no_mangle]
pub extern "C" fn C_delete_character() {
    save::delete_character();
}

#[no_mangle]
pub extern "C" fn sav__save_character() -> u8 {
    match save::save_character_with_feedback() {
        Some(_) => 255,
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn sav__load_character(player_name: *const libc::c_char, player_uid: i64) -> u8 {
    if player_name.is_null() {
        panic!("Null string received");
    }
    let rs_cstr = unsafe { CStr::from_ptr(player_name) };
    let rs_str = rs_cstr.to_str().unwrap();
    match save::load_character_with_feedback(rs_str, player_uid) {
        Some(_) => 255,
        None => 0,
    }
}
