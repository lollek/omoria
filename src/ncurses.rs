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

pub extern fn refresh() {
    debug::enter("ncurses::refresh");

    if unsafe { C_refresh() } != 0 {
        panic!("refresh returned ERR");
    }

    debug::leave("ncurses::refresh");
}

// Use term::put_buffer instead of this directly
pub extern fn mvaddstr(row: libc::c_int, col: libc::c_int, out_str: *const libc::c_char) {
    debug::enter("ncurses::mvaddstr");

    if unsafe { C_mvaddstr(row, col, out_str) } != 0 {
        panic!("mvaddstr returned ERR");
    }

    debug::leave("ncurses::mvaddstr");
}

pub extern fn clrtoeol() {
    debug::enter("ncurses::clrtoeol");

    if unsafe { C_clrtoeol() } != 0 {
        panic!("clrtoeol returned ERR");
    }

    debug::leave("ncurses::clrtoeol");
}

pub extern fn mov(row: libc::c_int, col: libc::c_int) {
    debug::enter("ncurses::mov");

    if unsafe { C_move(row, col) } != 0 {
        panic!("move returned ERR");
    }

    debug::leave("ncurses::mov");
}

// Use term::clear_screen instead
pub extern fn clear() {
    debug::enter("ncurses::clear");

    if unsafe { C_clear() } != 0 {
        panic!("clear returned ERR");
    }

    debug::leave("ncurses::clear");
}
