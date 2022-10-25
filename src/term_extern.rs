use libc;
use std::ffi::CStr;

use crate::term;

#[no_mangle]
pub extern "C" fn C_clear_screen() {
    term::clear_screen();
}

#[no_mangle]
pub extern "C" fn prt(msg: *const libc::c_char, row: i32, col: i32) {
    if msg.is_null() {
        panic!("Null string received");
    }
    prt_(msg, row - 1, col - 1);
}

#[no_mangle]
pub extern "C" fn prt_(msg: *const libc::c_char, row: i32, col: i32) {
    if msg.is_null() {
        panic!("Null string received");
    }
    let rs_cstr = unsafe { CStr::from_ptr(msg) };
    let rs_str = rs_cstr.to_str().unwrap();
    term::prt(rs_str, row, col);
}

#[no_mangle]
pub extern "C" fn put_buffer(msg: *const libc::c_char, row: i32, col: i32) {
    put_buffer_(msg, row - 1, col - 1);
}

#[no_mangle]
pub extern "C" fn put_buffer_(msg: *const libc::c_char, row: i32, col: i32) {
    if msg.is_null() {
        panic!("Null string received");
    }
    let rs_cstr = unsafe { CStr::from_ptr(msg) };
    let rs_str = rs_cstr.to_str().unwrap();
    term::put_buffer(rs_str, row, col);
}
