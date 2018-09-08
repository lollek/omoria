use std::process;
use std::cell::RefCell;

use libc;
use pancurses;

use debug;

thread_local! {
    static STDSCR: RefCell<Option<pancurses::Window>> = RefCell::new(None);
}

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

    fn C_chattr(attr: libc::c_int, on: libc::uint8_t) -> libc::c_int;
}

pub fn init_curses() {
    println!("Attempting to start curses...");

    let window = pancurses::initscr();

    if window.get_max_y() < 24 || window.get_max_x() < 80 {
        pancurses::endwin();
        println!("Screen is too small for moria!");
        process::exit(1);
    }

    window.clear();
    window.refresh();
    STDSCR.with(|stdscr| stdscr.replace(Some(window)));

    pancurses::start_color();
    pancurses::init_pair(pancurses::COLOR_RED,
                         pancurses::COLOR_RED, pancurses::COLOR_BLACK);
    pancurses::init_pair(pancurses::COLOR_GREEN,
                         pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
    pancurses::init_pair(pancurses::COLOR_YELLOW,
                         pancurses::COLOR_YELLOW, pancurses::COLOR_BLACK);
    pancurses::init_pair(pancurses::COLOR_BLUE,
                         pancurses::COLOR_BLUE, pancurses::COLOR_BLACK);
    pancurses::init_pair(pancurses::COLOR_MAGENTA,
                         pancurses::COLOR_MAGENTA, pancurses::COLOR_BLACK);
    pancurses::init_pair(pancurses::COLOR_CYAN,
                         pancurses::COLOR_CYAN, pancurses::COLOR_BLACK);

    pancurses::cbreak();
    pancurses::noecho();
    pancurses::nonl();
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

#[derive(Copy, Clone)]
pub enum CursesAttr {
    Standout = 1,
    Underline = 2,
    Reverse = 3,
    Blink = 4,
    Dim = 5,
    Bold = 6,
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
