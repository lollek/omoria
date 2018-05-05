use libc::c_int;

extern "C" {
    fn mvaddstr(y: c_int, x: c_int, str: *const u8) -> c_int;
    fn refresh() -> c_int;
    fn clrtoeol() -> c_int;
    fn clear() -> c_int;
    #[link_name = "move"]
    fn mov(y: c_int, x: c_int) -> c_int;
}

pub extern fn refresh_screen() {
    unsafe {
        if refresh() != 0 {
            //panic!("refresh returned ERR");
        }
    }
}

pub extern fn move_print(row: i32, col: i32, out_str: *const u8) {
    unsafe {
        if mvaddstr(row, col, out_str) != 0 {
            //panic!("mvaddstr returned ERR");
        }
    }
}

pub extern fn clear_line() {
    unsafe {
        if clrtoeol() != 0 {
            //panic!("clrtoeol returned ERR");
        }
    }
}

pub extern fn move_cursor(row: i32, col: i32) {
    unsafe {
        if mov(row, col) != 0 {
            //panic!("move returned ERR");
        }
    }
}

pub extern fn clear_screen() {
    unsafe {
        if clear() != 0 {
            //panic!("clear returned ERR");
        }
    }
}
