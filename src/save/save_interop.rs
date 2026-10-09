use libc;
use std::ffi::CStr;

use crate::save;

#[no_mangle]
pub extern "C" fn C_delete_character() {
    save::delete_character();
}

#[no_mangle]
pub extern "C" fn sav__save_character() -> bool {
    save::save_character_with_feedback().is_some()
}

#[no_mangle]
pub extern "C" fn sav__load_character(player_name: *const libc::c_char, player_uid: i64) -> bool {
    if player_name.is_null() {
        panic!("Null string received");
    }
    let rs_cstr = unsafe { CStr::from_ptr(player_name) };
    let rs_str = rs_cstr.to_str().unwrap();
    save::load_character_with_feedback(rs_str, player_uid).is_some()
}

#[cfg(feature = "save-test-support")]
#[no_mangle]
pub extern "C" fn C_save_test_reset() -> bool {
    save::test_reset()
}

#[cfg(feature = "save-test-support")]
#[no_mangle]
pub extern "C" fn C_save_test_reject_mismatched_uid_preserves_state() -> bool {
    save::test_reject_mismatched_uid_preserves_state()
}

#[cfg(feature = "save-test-support")]
#[no_mangle]
pub extern "C" fn C_save_test_apply_fixture_and_verify() -> bool {
    save::test_apply_fixture_and_verify()
}
