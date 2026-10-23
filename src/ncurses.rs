use std::cell::RefCell;
use std::process;

use crate::layout::{compute_layout, set_current_layout, LayoutKind};
use pancurses;
pub use pancurses::{
    A_DIM, A_REVERSE, COLOR_BLACK, COLOR_BLUE, COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED,
    COLOR_YELLOW,
};

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

thread_local! {
    static STDSCR: RefCell<Option<pancurses::Window>> = const { RefCell::new(None) };
}

fn position_in_bounds(row: i32, col: i32, rows: i32, cols: i32) -> bool {
    row >= 0 && col >= 0 && row < rows && col < cols
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

    let layout = compute_layout(window.get_max_y(), window.get_max_x());
    if layout.kind == LayoutKind::TooSmall {
        pancurses::endwin();
        println!("Screen is too small for moria!");
        process::exit(1);
    }

    set_current_layout(layout);
    window.clear();
    window.refresh();
    STDSCR.replace(Some(window));

    pancurses::start_color();
    pancurses::init_pair(COLOR_RED, COLOR_RED, COLOR_BLACK);
    pancurses::init_pair(COLOR_GREEN, COLOR_GREEN, COLOR_BLACK);
    pancurses::init_pair(COLOR_YELLOW, COLOR_YELLOW, COLOR_BLACK);
    pancurses::init_pair(COLOR_BLUE, COLOR_BLUE, COLOR_BLACK);
    pancurses::init_pair(COLOR_MAGENTA, COLOR_MAGENTA, COLOR_BLACK);
    pancurses::init_pair(COLOR_CYAN, COLOR_CYAN, COLOR_BLACK);

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

pub fn mvaddstr<S>(row: i32, col: i32, msg: S)
where
    S: AsRef<str>,
{
    with_stdscr(|stdscr| {
        if !position_in_bounds(row, col, stdscr.get_max_y(), stdscr.get_max_x()) {
            panic!(
                "Attempting to print out of bounds! row: {}, col: {}, msg: {}",
                row,
                col,
                msg.as_ref()
            );
        }
        let msg = msg.as_ref();
        let result = match msg
            .as_bytes()
            .last()
            .copied()
            .filter(|byte| byte.is_ascii_graphic() || *byte == b' ')
        {
            Some(last) => {
                let result = stdscr.mvaddstr(row, col, &msg[..msg.len() - 1]);
                if result != 0 {
                    result
                } else if stdscr.get_cur_yx() == (stdscr.get_max_y() - 1, stdscr.get_max_x() - 1) {
                    // Adding here would draw successfully but return ERR while wrapping.
                    stdscr.insch(last as pancurses::chtype)
                } else {
                    stdscr.addch(last as pancurses::chtype)
                }
            }
            None => stdscr.mvaddstr(row, col, msg),
        };
        if result != 0 {
            panic!(
                "mvaddstr returned ERR, row: {}, col: {}, msg: {}",
                row, col, msg
            );
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
        if !position_in_bounds(row, col, stdscr.get_max_y(), stdscr.get_max_x()) {
            panic!(
                "Attempting to mov out of bounds! row: {}, col: {}",
                row, col
            );
        }
        if stdscr.mv(row, col) != 0 {
            panic!("move returned ERR, row: {}, col: {}", row, col);
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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestScreen {
        screen: pancurses::SCREEN,
        output: *mut libc::FILE,
        input: *mut libc::FILE,
    }

    impl TestScreen {
        fn new(rows: i32, cols: i32) -> Self {
            let output = unsafe { libc::tmpfile() };
            let input = unsafe { libc::tmpfile() };
            assert!(!output.is_null() && !input.is_null(), "tmpfile failed");
            let screen = pancurses::newterm(Some("xterm"), output.cast(), input.cast());
            assert!(!screen.is_null(), "newterm failed");
            let guard = Self {
                screen,
                output,
                input,
            };
            assert_eq!(pancurses::resize_term(rows, cols), 0);
            let window = pancurses::newwin(rows, cols, 0, 0);
            assert_eq!(window.scrollok(false), 0);
            assert!(STDSCR.replace(Some(window)).is_none());
            guard
        }
    }

    impl Drop for TestScreen {
        fn drop(&mut self) {
            drop(STDSCR.replace(None));
            pancurses::endwin();
            pancurses::delscreen(self.screen);
            unsafe {
                libc::fclose(self.output);
                libc::fclose(self.input);
            }
        }
    }

    #[test]
    #[serial_test::serial]
    fn padded_sidebar_line_can_clear_the_bottom_right_cell_without_scrolling() {
        let _screen = TestScreen::new(57, 214);
        mvaddstr(55, 0, "row above");
        mvaddstr(56, 80, "|");
        with_stdscr(|window| {
            assert_eq!(window.mvinsch(56, 213, 'X'), 0);
        });

        mvaddstr(56, 81, " ".repeat(133));

        with_stdscr(|window| {
            assert_eq!(window.mvinch(55, 0), 'r' as pancurses::chtype);
            assert_eq!(window.mvinch(56, 80), '|' as pancurses::chtype);
            for col in 81..214 {
                assert_eq!(window.mvinch(56, col), ' ' as pancurses::chtype);
            }
        });
    }

    #[test]
    #[serial_test::serial]
    fn bottom_right_text_keeps_its_final_character_and_attributes() {
        let _screen = TestScreen::new(35, 150);
        attron(pancurses::A_BOLD);
        mvaddstr(34, 147, "orc");
        attroff(pancurses::A_BOLD);
        with_stdscr(|window| {
            for (col, ch) in [(147, 'o'), (148, 'r'), (149, 'c')] {
                let cell = window.mvinch(34, col);
                assert_eq!(cell, ch as pancurses::chtype | pancurses::A_BOLD);
            }
        });
    }

    #[test]
    #[serial_test::serial]
    fn bottom_right_cell_can_be_written_on_its_own() {
        let _screen = TestScreen::new(35, 150);
        mvaddstr(34, 149, "X");
        with_stdscr(|window| {
            assert_eq!(window.mvinch(34, 149), 'X' as pancurses::chtype);
        });
    }

    #[test]
    #[serial_test::serial]
    fn shorter_last_row_text_does_not_shift_following_cells() {
        let _screen = TestScreen::new(35, 150);
        with_stdscr(|window| {
            assert_eq!(window.mvinsch(34, 149, 'X'), 0);
        });
        mvaddstr(34, 145, "orc");
        with_stdscr(|window| {
            for (col, ch) in [(145, 'o'), (146, 'r'), (147, 'c'), (148, ' '), (149, 'X')] {
                assert_eq!(window.mvinch(34, col), ch as pancurses::chtype);
            }
        });
    }

    #[test]
    #[serial_test::serial]
    fn empty_strings_and_control_characters_keep_curses_behavior() {
        let _screen = TestScreen::new(35, 150);
        mvaddstr(1, 1, "");
        with_stdscr(|window| assert_eq!(window.get_cur_yx(), (1, 1)));
        mvaddstr(1, 1, "X\n");
        with_stdscr(|window| {
            assert_eq!(window.mvinch(1, 1), 'X' as pancurses::chtype);
        });
    }

    #[test]
    #[serial_test::serial]
    #[should_panic(expected = "mvaddstr returned ERR")]
    fn text_overflowing_the_last_cell_still_reports_an_error() {
        let _screen = TestScreen::new(35, 150);
        mvaddstr(34, 149, "XY");
    }

    #[test]
    #[serial_test::serial]
    #[should_panic(expected = "Attempting to print out of bounds!")]
    fn drawing_below_the_window_still_reports_an_error() {
        let _screen = TestScreen::new(35, 150);
        mvaddstr(35, 81, "invalid");
    }

    #[test]
    fn drawing_bounds_follow_terminal_dimensions() {
        assert!(position_in_bounds(24, 149, 25, 150));
        assert!(position_in_bounds(34, 199, 35, 200));
        for (row, col) in [(-1, 0), (0, -1), (35, 0), (0, 200)] {
            assert!(!position_in_bounds(row, col, 35, 200));
        }
        assert!(!position_in_bounds(0, 0, 0, 0));
    }
}
