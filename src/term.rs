use std::ffi::CString;
use libc::c_char;

use ncurses;

extern "C" {
    #[link_name="screen_change"]
    static mut C_screen_change: i32;
    #[link_name="msg_flag"]
    static C_msg_flag: u8;
    #[link_name="msg_print"]
    fn C_msg_print(str_buff: *const c_char);
}

pub fn put_buffer_r(out_str: &str, row: i32, col: i32) {
    put_buffer(CString::new(out_str).unwrap().as_ptr(), row, col)
}

pub fn prt_r(str_buff: &str, row: i32, col: i32) {
    prt(CString::new(str_buff).unwrap().as_ptr(), row, col)
}

pub fn refresh_screen() {
    unsafe { C_screen_change = <i32>::max_value(); }
    ncurses::refresh_screen();
}

pub fn msg_print(out_str: &str) {
    unsafe { C_msg_print(CString::new(out_str).unwrap().as_ptr()) }
}


#[no_mangle]
pub extern fn put_buffer(out_str: *const c_char, row: i32, col: i32) {
    put_buffer_(out_str, row - 1, col - 1)
}

#[no_mangle]
pub extern fn put_buffer_(out_str: *const c_char, row: i32, col: i32) {
    if out_str.is_null() {
        panic!("Null string received");
    }

    ncurses::move_print(row, col, out_str);
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
