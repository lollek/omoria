use libc::{c_int, c_char};

extern "C" {
    #[link_name = "mvaddstr"]
    fn C_mvaddstr(y: c_int, x: c_int, str: *const c_char) -> c_int;
    #[link_name = "refresh"]
    fn C_refresh() -> c_int;
    #[link_name = "clrtoeol"]
    fn C_clrtoeol() -> c_int;
    #[link_name = "clear"]
    fn C_clear() -> c_int;
    #[link_name = "move"]
    fn C_move(y: c_int, x: c_int) -> c_int;
}

pub extern fn refresh_screen() {
    unsafe {
        if C_refresh() != 0 {
            panic!("refresh returned ERR");
        }
    }
}

pub extern fn move_print(row: i32, col: i32, out_str: *const c_char) {
    unsafe {
        if C_mvaddstr(row as c_int, col as c_int, out_str) != 0 {
            panic!("mvaddstr returned ERR");
        }
    }
}

pub extern fn clear_line() {
    unsafe {
        if C_clrtoeol() != 0 {
            panic!("clrtoeol returned ERR");
        }
    }
}

pub extern fn move_cursor(row: i32, col: i32) {
    unsafe {
        if C_move(row as c_int, col as c_int) != 0 {
            panic!("move returned ERR");
        }
    }
}

pub extern fn clear_screen() {
    unsafe {
        if C_clear() != 0 {
            panic!("clear returned ERR");
        }
    }
}
