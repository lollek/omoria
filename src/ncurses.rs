use std::process;
use std::cell::RefCell;

use pancurses;
pub use pancurses::{
    COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_RED, COLOR_YELLOW,
    COLOR_MAGENTA, COLOR_CYAN,
    A_REVERSE, A_DIM
};

use debug;

thread_local! {
    static STDSCR: RefCell<Option<pancurses::Window>> = RefCell::new(None);
}

fn with_stdscr<S>(fun: S)
    where S: Fn(&pancurses::Window)
{
    STDSCR.with(|stdscr_wrapper| {
        match *stdscr_wrapper.borrow() {
            Some(ref stdscr) => fun(stdscr),
            None => panic!("stdscr not initialized?"),
        }
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

    with_stdscr(|stdscr| {
        if stdscr.refresh() != 0 {
            panic!("refresh returned ERR");
        }
    });

    debug::leave("ncurses::refresh");
}

// Use term::put_buffer instead of this directly
pub fn mvaddstr(row: i32, col: i32, out_str: &str) {
    debug::enter("ncurses::mvaddstr");

    with_stdscr(|stdscr| {
        if stdscr.mvaddstr(row, col, out_str) != 0 {
            panic!("mvaddstr returned ERR");
        }
    });

    debug::leave("ncurses::mvaddstr");
}

pub fn clrtoeol() {
    debug::enter("ncurses::clrtoeol");

    with_stdscr(|stdscr| {
        if stdscr.clrtoeol() != 0 {
            panic!("clrtoeol returned ERR");
        }
    });

    debug::leave("ncurses::clrtoeol");
}

pub fn mov(row: i32, col: i32) {
    debug::enter("ncurses::mov");

    with_stdscr(|stdscr| {
        if stdscr.mv(row, col) != 0 {
            panic!("move returned ERR");
        }
    });

    debug::leave("ncurses::mov");
}

// Use term::clear_screen instead
pub fn clear() {
    debug::enter("ncurses::clear");

    with_stdscr(|stdscr| {
        if stdscr.clear() != 0 {
            panic!("clear returned ERR");
        }
    });

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

pub fn attron(attr: pancurses::chtype) {
    debug::enter("ncurses::attron");

    with_stdscr(|stdscr| {
        if stdscr.attron(attr) != 0 {
            panic!("attron returned ERR");
        }
    });

    debug::leave("ncurses::attron");
}

pub fn attroff(attr: pancurses::chtype) {
    debug::enter("ncurses::attroff");

    with_stdscr(|stdscr| {
        if stdscr.attroff(attr) != 0 {
            panic!("attroff returned ERR");
        }
    });

    debug::leave("ncurses::attroff");
}
