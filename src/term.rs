use libc;
use std::ffi::CString;

use crate::misc;
use crate::ncurses;

extern "C" {
    #[link_name = "msg_flag"]
    static C_msg_flag: u8;

    #[link_name = "msg_print"]
    fn C_msg_print(str_buff: *const libc::c_char);

    #[link_name = "Get_String"]
    fn C_Get_String(
        in_str: *const libc::c_char,
        row: libc::c_int,
        col: libc::c_int,
        slen: libc::c_int,
    ) -> u8;

    #[link_name = "Clear_From"]
    fn C_Clear_From(row: libc::c_int);
}

pub fn has_msg_flag() -> bool {
    unsafe { C_msg_flag != 0 }
}

pub fn clear_from(row: i32) {
    unsafe { C_Clear_From(row - 1) };
}

pub fn refresh_screen() {
    ncurses::refresh();
}

pub fn msg_print<S>(out_str: S)
where
    S: Into<Vec<u8>>,
{
    let cstr = CString::new(out_str).unwrap();
    unsafe { C_msg_print(cstr.as_ptr()) }
}

/* Gets a string terminated by <RETURN>		*/
pub fn get_string(row: i32, col: i32, slen: i32) -> String {
    let tmp_str: [u8; 134] = [0; 134];

    while tmp_str[0] == b'\0' {
        unsafe {
            C_Get_String(
                tmp_str.as_ptr() as *mut libc::c_char,
                row - 1,
                col - 1,
                slen,
            )
        };
    }
    misc::c_array_to_rust_string(tmp_str.to_vec())
}

// put_buffer which clears the line first
pub fn prt<S>(msg: S, row: i32, col: i32)
where
    S: AsRef<str>,
{
    if row == -1 && has_msg_flag() {
        msg_print("");
    }
    ncurses::mov(row, col);
    ncurses::clrtoeol();
    put_buffer(msg, row, col);
}

// Output text to coordinate. Basically a wrapper around ncurses::move_print
pub fn put_buffer<S>(msg: S, row: i32, col: i32)
where
    S: AsRef<str>,
{
    ncurses::mvaddstr(row, col, msg);
}

pub fn clear_screen() {
    if has_msg_flag() {
        msg_print("");
    }
    ncurses::clear();
}
