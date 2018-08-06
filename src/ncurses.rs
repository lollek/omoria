use libc;

use debug;

extern "C" {
    #[link_name = "mvaddstr"]
    fn C_mvaddstr(y: libc::c_int, x: libc::c_int, str: *const libc::c_char)
        -> libc::c_int;

    #[link_name = "refresh"]
    fn C_refresh() -> libc::c_int;

    #[link_name = "clrtoeol"]
    fn C_clrtoeol() -> libc::c_int;

    #[link_name = "clear"]
    fn C_clear() -> libc::c_int;

    #[link_name = "move"]
    fn C_move(y: libc::c_int, x: libc::c_int) -> libc::c_int;
}

pub extern fn refresh_screen() {
    debug::enter("ncurses::refresh_screen");
    if unsafe { C_refresh() } != 0 {
        panic!("refresh returned ERR");
    }
    debug::leave("ncurses::refresh_screen");
}

pub extern fn move_print(row: i32, col: i32, out_str: *const libc::c_char) {
    debug::enter("ncurses::move_print");
    if unsafe { C_mvaddstr(row as libc::c_int, col as libc::c_int, out_str) } != 0 {
        panic!("mvaddstr returned ERR");
    }
    debug::leave("ncurses::move_print");
}

pub extern fn clear_line() {
    debug::enter("ncurses::clear_line");
    if unsafe { C_clrtoeol() } != 0 {
        panic!("clrtoeol returned ERR");
    }
    debug::leave("ncurses::clear_line");
}

pub extern fn move_cursor(row: i32, col: i32) {
    debug::enter("ncurses::move_curseor");
    if unsafe { C_move(row as libc::c_int, col as libc::c_int) } != 0 {
        panic!("move returned ERR");
    }
    debug::leave("ncurses::move_curseor");
}

pub extern fn clear_screen() {
    debug::enter("ncurses::clear_screen");
    if unsafe { C_clear() } != 0 {
        panic!("clear returned ERR");
    }
    debug::leave("ncurses::clear_screen");
}
