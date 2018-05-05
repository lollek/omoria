use std::ffi::CString;

use ncurses::move_print;
use ncurses::move_cursor;
use ncurses::clear_line;
use ncurses::refresh_screen as ncurses_refresh_screen;
use ncurses::clear_screen as ncurses_clear_screen;

extern "C" {
    static mut screen_change: i32;
    static msg_flag: u8;
    fn msg_print(str_buff: *const u8);
}

pub extern fn put_buffer_r(out_str: &str, row: i32, col: i32) {
    match CString::new(out_str) {
        Ok(cstr) => put_buffer(cstr.as_ptr() as *const u8, row, col),
        Err(_e) => (), //panic!(e),
    }
}

pub extern fn prt_r(str_buff: &str, row: i32, col: i32) {
    match CString::new(str_buff) {
        Ok(cstr) => prt(cstr.as_ptr() as *const u8, row, col),
        Err(_e) => (), //panic!(e),
    }
}

pub extern fn refresh_screen() {
    unsafe {
        screen_change = <i32>::max_value();
    }
    ncurses_refresh_screen();
}

fn msg_print_nothing() {
    unsafe {
        match CString::new("") {
            Ok(cstr) => msg_print(cstr.as_ptr() as *const u8),
            Err(_e) => (), //panic!(e),
        }
    }
}


#[no_mangle]
pub extern fn put_buffer(out_str: *const u8, row: i32, col: i32) {
    put_buffer_(out_str, row - 1, col - 1)
}

#[no_mangle]
pub extern fn put_buffer_(out_str: *const u8, row: i32, col: i32) {
    if out_str.is_null() {
        //panic!("Null string received");
        return;
    }

    move_print(row, col, out_str);
}

#[no_mangle]
pub extern fn prt(str_buff: *const u8, row: i32, col: i32) {
    prt_(str_buff, row - 1, col - 1)
}

#[no_mangle]
pub extern fn prt_(str_buff: *const u8, row: i32, col: i32) {
    unsafe {
        if row == -1 && msg_flag != 0 {
            msg_print_nothing();
        }
    }
    move_cursor(row, col);
    clear_line();
    put_buffer_(str_buff, row, col);
}

#[no_mangle]
pub extern fn clear_screen() {
    unsafe {
        if msg_flag != 0 {
            msg_print_nothing();
        }
        ncurses_clear_screen();
    }
}
