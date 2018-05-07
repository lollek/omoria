use term::refresh_screen;

extern "C" {
    fn inkey() -> u8;
    fn flush();
    fn check_input(delay_in_ms: i32) -> i32;
}

pub fn inkey_flush() -> u8 {
    refresh_screen();
    unsafe {
        flush();
        inkey()
    }
}

	/* XXXX check_input consumes the input, so we never actually get data */
pub fn inkey_delay(delay_in_ms: i32) -> u8 {
    refresh_screen();
    if unsafe { check_input(delay_in_ms) != 0 } {
        'a' as u8
    } else {
        0
    }
}
