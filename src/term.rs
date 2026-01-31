use libc;
#[cfg(not(test))]
use std::ffi::CString;

#[cfg(test)]
use std::sync::Mutex;

use crate::misc;
use crate::ncurses;

#[cfg(test)]
use lazy_static::lazy_static;

extern "C" {
    #[link_name = "msg_flag"]
    static C_msg_flag: u8;

    #[link_name = "msg_print"]
    #[cfg(not(test))]
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

#[cfg(test)]
lazy_static! {
    static ref TEST_LAST_MSG_PRINT: Mutex<String> = Mutex::new(String::new());
}

pub fn has_msg_flag() -> bool {
    unsafe { C_msg_flag != 0 }
}

pub fn clear_from(row: i32) {
    unsafe { C_Clear_From(row - 1) };
}

#[cfg(test)]
pub fn msg_print<S>(out_str: S)
where
    S: Into<Vec<u8>>,
{
    // In unit tests we avoid calling into ncurses/C and capture the last printed message.
    let s = String::from_utf8_lossy(&out_str.into()).into_owned();
    let mut guard = TEST_LAST_MSG_PRINT.lock().unwrap();
    *guard = s;
}

#[cfg(not(test))]
pub fn msg_print<S>(out_str: S)
where
    S: Into<Vec<u8>>,
{
    let cstr = CString::new(out_str).unwrap();
    unsafe { C_msg_print(cstr.as_ptr()) }
}

#[cfg(test)]
pub fn test_last_msg_print() -> String {
    TEST_LAST_MSG_PRINT.lock().unwrap().clone()
}

#[cfg(test)]
pub fn test_clear_last_msg_print() {
    *TEST_LAST_MSG_PRINT.lock().unwrap() = String::new();
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
pub fn prt<S: AsRef<str>>(msg: S, row: i32, col: i32) {
    if row == -1 && has_msg_flag() {
        msg_print("");
    }
    ncurses::mov(row, col);
    ncurses::clrtoeol();
    ncurses::mvaddstr(row, col, msg);
}

pub fn clear_screen() {
    if has_msg_flag() {
        msg_print("");
    }
    ncurses::clear();
}
