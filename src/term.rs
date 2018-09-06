use std::ffi::CString;
use libc;

use debug;
use misc;
use ncurses;

extern "C" {
    #[link_name="msg_flag"]
    static C_msg_flag: libc::uint8_t;

    #[link_name="msg_print"]
    fn C_msg_print(str_buff: *const libc::c_char);

    #[link_name="Get_String"]
    fn C_Get_String(in_str: *const libc::c_char, row: libc::c_int, col: libc::c_int, slen: libc::c_int) -> libc::uint8_t;

    #[link_name="Clear_From"]
    fn C_Clear_From(row: libc::c_int);
}

pub fn has_msg_flag() -> bool {
    unsafe { C_msg_flag != 0 }
}

pub fn clear_from(row: i32) {
    debug::enter("clear_from");

    unsafe { C_Clear_From(row - 1) };

    debug::leave("clear_from");
}

pub fn refresh_screen() {
    debug::enter("refresh_screen");

    ncurses::refresh();

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
        unsafe { C_Get_String(tmp_str.as_ptr() as *mut libc::c_char, row - 1, col - 1, slen) };
    }
    let cstr = misc::c_array_to_rust_string(tmp_str.to_vec());

    debug::leave("get_string");
    cstr
}


// put_buffer which clears the line first
pub fn prt(msg: &str, row: i32, col: i32) {
    debug::enter("prt");

    if row == -1 && has_msg_flag() {
        msg_print("");
    }
    ncurses::mov(row, col);
    ncurses::clrtoeol();
    put_buffer(msg, row, col);

    debug::leave("prt");
}

// Output text to coordinate. Basically a wrapper around ncurses::move_print
pub fn put_buffer(msg: &str, row: i32, col: i32) {
    debug::enter(&format!("put_buffer_r: '{}'. y: {}. x: {}", msg, row, col));

    let cstr = CString::new(msg).unwrap();
    let cptr = cstr.as_ptr();
    ncurses::mvaddstr(row, col, cptr);

    debug::leave("put_buffer_r");
}

pub fn clear_screen() {
    if has_msg_flag() {
        msg_print("");
    }
    ncurses::clear();
}
