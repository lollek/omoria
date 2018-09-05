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

    #[link_name = "chattr"]
    fn C_chattr(attr: libc::c_int, on: libc::uint8_t) -> libc::c_int;
}

pub fn refresh() {
    debug::enter("ncurses::refresh");

    if unsafe { C_refresh() } != 0 {
        panic!("refresh returned ERR");
    }

    debug::leave("ncurses::refresh");
}

// Use term::put_buffer instead of this directly
pub fn mvaddstr(row: libc::c_int, col: libc::c_int, out_str: *const libc::c_char) {
    debug::enter("ncurses::mvaddstr");

    if unsafe { C_mvaddstr(row, col, out_str) } != 0 {
        panic!("mvaddstr returned ERR");
    }

    debug::leave("ncurses::mvaddstr");
}

pub fn clrtoeol() {
    debug::enter("ncurses::clrtoeol");

    if unsafe { C_clrtoeol() } != 0 {
        panic!("clrtoeol returned ERR");
    }

    debug::leave("ncurses::clrtoeol");
}

pub fn mov(row: libc::c_int, col: libc::c_int) {
    debug::enter("ncurses::mov");

    if unsafe { C_move(row, col) } != 0 {
        panic!("move returned ERR");
    }

    debug::leave("ncurses::mov");
}

// Use term::clear_screen instead
pub fn clear() {
    debug::enter("ncurses::clear");

    if unsafe { C_clear() } != 0 {
        panic!("clear returned ERR");
    }

    debug::leave("ncurses::clear");
}

pub enum CursesAttr {
    Dim = 1,
    ColorGreen = 10,
    ColorYellow = 11,
    ColorRed = 12,
    ColorBlue = 13,
    ColorCyan = 14,
    ColorMagenta = 15,
}

pub fn chattr(attr: CursesAttr, on: bool) {
    debug::enter("ncurses::chattr");

    if unsafe { C_chattr(attr as i32, if on {1} else {0}) } != 0 {
        panic!("chattr returned ERR");
    }

    debug::leave("ncurses::chattr");
}
