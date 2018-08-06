use std::ffi::CString;
use libc::{c_char, c_int};

use debug;
use misc;
use ncurses;

extern "C" {
    #[link_name="screen_change"]
    static mut C_screen_change: c_int;
    #[link_name="msg_flag"]
    static C_msg_flag: c_char;
    #[link_name="msg_print"]
    fn C_msg_print(str_buff: *const c_char);
    #[link_name="Get_String"]
    fn C_Get_String(in_str: *const c_char, row: c_int, col: c_int, slen: c_int) -> c_char;
    #[link_name="Clear_From"]
    fn C_Clear_From(row: c_int);
}

pub fn clear_from(row: i32) {
    debug::enter("clear_from");
    unsafe { C_Clear_From(row - 1) };
    debug::leave("clear_from");
}


pub fn put_buffer_r(out_str: &str, row: i32, col: i32) {
    debug::enter(&format!("put_buffer_r: '{}'. y: {}. x: {}", out_str, row, col));
    let cstr = CString::new(out_str).unwrap();
    put_buffer(cstr.as_ptr(), row, col);
    debug::leave("put_buffer_r");
}

pub fn prt_r(str_buff: &str, row: i32, col: i32) {
    debug::enter("prt_r");
    let cstr = CString::new(str_buff).unwrap();
    prt(cstr.as_ptr(), row, col);
    debug::leave("prt_r");
}

pub fn refresh_screen() {
    debug::enter("refresh_screen");
    unsafe { C_screen_change = <i32>::max_value(); }
    ncurses::refresh_screen();
    debug::leave("refresh_screen");
}

pub fn msg_print(out_str: &str) {
    debug::enter("msg_print");
    let cstr = CString::new(out_str).unwrap();
    unsafe { C_msg_print(cstr.as_ptr()) }
    debug::leave("msg_print");
}

/* Gets a string terminated by <RETURN>		*/
pub fn get_string(row: i32, col: i32, slen: i32) -> String {
    debug::enter("get_string");
    let tmp_str: [u8; 134] = [0; 134];

    while tmp_str[0] == b'\0' {
        unsafe { C_Get_String(tmp_str.as_ptr() as *mut c_char, row - 1, col - 1, slen) };
    }
    let cstr = misc::c_array_to_rust_string(tmp_str.to_vec());
    debug::leave("get_string");
    cstr
}

#[no_mangle]
pub extern fn put_buffer(out_str: *const c_char, row: i32, col: i32) {
    debug::enter("put_buffer");
    put_buffer_(out_str, row - 1, col - 1);
    debug::leave("put_buffer");
}

#[no_mangle]
pub extern fn put_buffer_(out_str: *const c_char, row: i32, col: i32) {
    debug::enter("put_buffer_");
    if out_str.is_null() {
        panic!("Null string received");
    }

    ncurses::move_print(row, col, out_str);
    debug::leave("put_buffer_");
}

#[no_mangle]
pub extern fn prt(str_buff: *const c_char, row: i32, col: i32) {
    if str_buff.is_null() {
        panic!("Null string received");
    }
    prt_(str_buff, row - 1, col - 1)
}

#[no_mangle]
pub extern fn prt_(str_buff: *const c_char, row: i32, col: i32) {
    if str_buff.is_null() {
        panic!("Null string received");
    }
    let has_msg_flag = unsafe { C_msg_flag != 0 };
    if row == -1 && has_msg_flag {
        msg_print("");
    }
    ncurses::move_cursor(row, col);
    ncurses::clear_line();
    put_buffer_(str_buff, row, col);
}

#[no_mangle]
pub extern fn clear_screen() {
    let has_msg_flag = unsafe { C_msg_flag != 0 };
    if has_msg_flag {
        msg_print("");
    }
    ncurses::clear_screen();
}
