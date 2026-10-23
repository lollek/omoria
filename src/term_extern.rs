use libc;
use std::ffi::CStr;

use crate::{ncurses, term};

/// # Safety
/// `message` must point to a live NUL-terminated string; callbacks must remain callable.
#[no_mangle]
pub unsafe extern "C" fn C_message_log_print(
    message: *const libc::c_char,
    draw: term::LogDraw,
    more: term::LogMore,
) -> bool {
    unsafe { term::print_log_message(message, draw, more) }
}

#[no_mangle]
pub extern "C" fn C_message_log_input() {
    term::message_input_read();
}

#[cfg(feature = "terminal-test-support")]
#[no_mangle]
pub extern "C" fn C_terminal_test_layout(rows: libc::c_int, cols: libc::c_int) {
    crate::layout::set_current_layout(crate::layout::compute_layout(rows, cols));
}

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
#[allow(clippy::not_unsafe_ptr_arg_deref)] // Keep the legacy Rust signature until caller contracts are tested.
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
#[allow(clippy::not_unsafe_ptr_arg_deref)] // Keep the legacy Rust signature until caller contracts are tested.
pub extern "C" fn put_buffer_(msg: *const libc::c_char, row: i32, col: i32) {
    if msg.is_null() {
        panic!("Null string received");
    }
    let rs_cstr = unsafe { CStr::from_ptr(msg) };
    let rs_str = rs_cstr.to_str().unwrap();
    ncurses::mvaddstr(row, col, rs_str);
}
