use std::ffi::CString;
use libc::{c_char, c_int};

use crate::term;

extern "C" {
    #[link_name="inkey"]
    fn C_inkey() -> c_char;
    #[link_name="flush"]
    fn C_flush();
    #[link_name="check_input"]
    fn C_check_input(delay_in_ms: c_int) -> c_int;
    #[link_name="get_com"]
    fn C_get_com(prompt: *const c_char, command: *mut c_char);
}

pub fn inkey_flush() -> u8 {
    term::refresh_screen();
    unsafe {
        C_flush();
        C_inkey() as u8
    }
}

// Gets response to a  Y/N question
pub fn get_yes_no<S>(prompt: S) -> bool
    where S: AsRef<str>
{
    let prompt_yn = CString::new(format!("{} (y/n) ", prompt.as_ref())).unwrap();
    let ref mut command: c_char = 0;
    loop {
        term::msg_print(" ");
        unsafe { C_get_com(prompt_yn.as_ptr(), command); }
        if *command == 'y' as c_char || *command == 'Y' as c_char {
            return true;
        }
        else if *command == 'n' as c_char || *command == 'N' as c_char {
            return false;
        }
    }
}

	/* XXXX check_input consumes the input, so we never actually get data */
pub fn inkey_delay(delay_in_ms: i32) -> u8 {
    term::refresh_screen();
    if unsafe { C_check_input(delay_in_ms) != 0 } {
        'a' as u8
    } else {
        0
    }
}
