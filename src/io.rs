use std::ffi::CString;
use libc::c_char;

use term;

extern "C" {
    fn inkey() -> u8;
    fn flush();
    fn check_input(delay_in_ms: i32) -> i32;
    fn msg_print(str_buff: *const c_char);
    fn get_com(prompt: *const c_char, command: *mut c_char);
}

pub fn inkey_flush() -> u8 {
    term::refresh_screen();
    unsafe {
        flush();
        inkey()
    }
}

// Gets response to a  Y/N question
pub fn get_yes_no(prompt: &str) -> bool {
    let prompt_yn = CString::new(format!("{} (y/n) ", prompt)).unwrap();
    let empty_string = CString::new(" ").unwrap();
    let ref mut command: c_char = 0;
    loop {
        unsafe {
            msg_print(empty_string.as_ptr());
            get_com(prompt_yn.as_ptr(), command);
        }
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
    if unsafe { check_input(delay_in_ms) != 0 } {
        'a' as u8
    } else {
        0
    }
}
