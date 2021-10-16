 use libc;
 use std::ffi::CStr;

 use save;

#[no_mangle]
pub extern fn C_delete_character() {
    save::delete_character();
}

#[no_mangle]
pub extern fn sav__save_character() -> libc::uint8_t {
    match save::save_character_with_feedback() {
        Some(_) => 255,
        None => 0,
    }
}

#[no_mangle]
pub extern fn sav__load_character(player_name: *const libc::c_char, player_uid: libc::int64_t) -> libc::uint8_t {
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

