use std::ffi::CStr;
use libc;

use crate::debug;
use crate::term;

#[no_mangle]
pub extern fn C_clear_screen() {
    debug::enter("term_extern::clear_screen");

    term::clear_screen();

    debug::leave("term_extern::clear_screen");
}

#[no_mangle]
pub extern fn prt(msg: *const libc::c_char, row: i32, col: i32) {
    debug::enter("term_extern::prt");

    if msg.is_null() {
        panic!("Null string received");
    }
    prt_(msg, row - 1, col - 1);

    debug::leave("term_extern::prt");
}

#[no_mangle]
pub extern fn prt_(msg: *const libc::c_char, row: i32, col: i32) {
    debug::enter("term_extern::prt_");

    if msg.is_null() {
        panic!("Null string received");
    }
    let rs_cstr = unsafe { CStr::from_ptr(msg) };
    let rs_str = rs_cstr.to_str().unwrap();
    term::prt(rs_str, row, col);

    debug::leave("term_extern::prt_");
}

#[no_mangle]
pub extern fn put_buffer(msg: *const libc::c_char, row: i32, col: i32) {
    debug::enter("term_extern::put_buffer");

    put_buffer_(msg, row - 1, col - 1);

    debug::leave("term_extern::put_buffer");
}

#[no_mangle]
pub extern fn put_buffer_(msg: *const libc::c_char, row: i32, col: i32) {
    debug::enter("term_extern::put_buffer_");

    if msg.is_null() {
        panic!("Null string received");
    }
    let rs_cstr = unsafe { CStr::from_ptr(msg) };
    let rs_str = rs_cstr.to_str().unwrap();
    term::put_buffer(rs_str, row, col);

    debug::leave("term_extern::put_buffer_");
}
