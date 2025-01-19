use std::cell::RefCell;
use std::process;

use pancurses;
pub use pancurses::{
    A_DIM, A_REVERSE, COLOR_BLACK, COLOR_BLUE, COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED,
    COLOR_YELLOW,
};

thread_local! {
    static STDSCR: RefCell<Option<pancurses::Window>> = RefCell::new(None);
}

fn with_stdscr<S>(fun: S)
where
    S: Fn(&pancurses::Window),
{
    STDSCR.with(|stdscr_wrapper| match *stdscr_wrapper.borrow() {
        Some(ref stdscr) => fun(stdscr),
        None => panic!("stdscr not initialized?"),
    });
}

pub fn color_pair(pair: i16) -> pancurses::chtype {
    pancurses::COLOR_PAIR(pair as pancurses::chtype)
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
    STDSCR.replace(Some(window));

    pancurses::start_color();
    pancurses::init_pair(
        COLOR_RED,
        COLOR_RED,
        COLOR_BLACK,
    );
    pancurses::init_pair(
        COLOR_GREEN,
        COLOR_GREEN,
        COLOR_BLACK,
    );
    pancurses::init_pair(
        COLOR_YELLOW,
        COLOR_YELLOW,
        COLOR_BLACK,
    );
    pancurses::init_pair(
        COLOR_BLUE,
        COLOR_BLUE,
        COLOR_BLACK,
    );
    pancurses::init_pair(
        COLOR_MAGENTA,
        COLOR_MAGENTA,
        COLOR_BLACK,
    );
    pancurses::init_pair(
        COLOR_CYAN,
        COLOR_CYAN,
        COLOR_BLACK,
    );

    pancurses::cbreak();
    pancurses::noecho();
    pancurses::nonl();
}

pub fn refresh() {
    with_stdscr(|stdscr| {
        if stdscr.refresh() != 0 {
            panic!("refresh returned ERR");
        }
    });
}

// Use term::put_buffer instead of this directly
pub fn mvaddstr<'a, S>(row: i32, col: i32, msg: S)
where
    S: AsRef<str>,
{
    with_stdscr(|stdscr| {
        if stdscr.mvaddstr(row, col, msg.as_ref()) != 0 {
            panic!("mvaddstr returned ERR");
        }
    });
}

pub fn clrtoeol() {
    with_stdscr(|stdscr| {
        if stdscr.clrtoeol() != 0 {
            panic!("clrtoeol returned ERR");
        }
    });
}

pub fn mov(row: i32, col: i32) {
    with_stdscr(|stdscr| {
        if stdscr.mv(row, col) != 0 {
            panic!("move returned ERR");
        }
    });
}

// Use term::clear_screen instead
pub fn clear() {
    with_stdscr(|stdscr| {
        if stdscr.clear() != 0 {
            panic!("clear returned ERR");
        }
    });
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

pub fn attron(attr: pancurses::chtype) {
    with_stdscr(|stdscr| {
        if stdscr.attron(attr) != 0 {
            panic!("attron returned ERR");
        }
    });
}

pub fn attroff(attr: pancurses::chtype) {
    with_stdscr(|stdscr| {
        if stdscr.attroff(attr) != 0 {
            panic!("attroff returned ERR");
        }
    });
}
