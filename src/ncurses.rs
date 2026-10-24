use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(test)]
use crate::layout::{compute_layout, set_current_layout};
use crate::layout::{set_current_layout_for_dimensions, LayoutKind};
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

static XTERM_MOTION_FALLBACK: AtomicBool = AtomicBool::new(false);

extern "C" {
    #[link_name = "tigetstr"]
    fn raw_tigetstr(capname: *const libc::c_char) -> *mut libc::c_char;
}

fn terminfo_capability_is_present(capability: *const libc::c_char) -> bool {
    !capability.is_null() && capability != -1_isize as *const libc::c_char
}

fn needs_xterm_motion_fallback(term_name: &str, has_xm: bool) -> bool {
    !has_xm && term_name.starts_with("xterm")
}

fn set_xterm_motion_reporting(enabled: bool) {
    if !XTERM_MOTION_FALLBACK.load(Ordering::Relaxed) {
        return;
    }
    let sequence = if enabled {
        "\x1b[?1003h"
    } else {
        "\x1b[?1003l"
    };
    if ::ncurses::putp(sequence) != 0 || unsafe { libc::fflush(std::ptr::null_mut()) } != 0 {
        panic!("failed to update xterm mouse motion mode");
    }
}

#[no_mangle]
pub extern "C" fn C_mouse_motion_fallback_start() {
    set_xterm_motion_reporting(true);
}

#[no_mangle]
pub extern "C" fn C_mouse_motion_fallback_stop() {
    set_xterm_motion_reporting(false);
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

pub fn read_row(row: i32) -> Vec<pancurses::chtype> {
    let cells = RefCell::new(Vec::new());
    with_stdscr(|stdscr| {
        cells
            .borrow_mut()
            .extend((0..stdscr.get_max_x()).map(|col| stdscr.mvinch(row, col)));
    });
    cells.into_inner()
}

fn restore_row_cells(
    cells: &[pancurses::chtype],
    width: i32,
    mut restore_cell: impl FnMut(i32, pancurses::chtype) -> i32,
) {
    for (col, cell) in cells
        .iter()
        .copied()
        .take(width.max(0) as usize)
        .enumerate()
    {
        if restore_cell(col as i32, cell) != 0 {
            panic!("mvaddch returned ERR while restoring prompt row");
        }
    }
}

pub fn restore_row(row: i32, cells: &[pancurses::chtype]) {
    with_stdscr(|stdscr| {
        restore_row_cells(cells, stdscr.get_max_x(), |col, cell| {
            stdscr.mvaddch(row, col, cell)
        });
    });
}

#[no_mangle]
pub extern "C" fn C_terminal_layout_update(rows: libc::c_int, cols: libc::c_int) -> bool {
    crate::user_interface::mouse::clear_hover_prompt();
    set_current_layout_for_dimensions(rows, cols).kind != LayoutKind::TooSmall
}

extern "C" {
    fn C_wait_for_usable_terminal();
}

pub fn init_curses() {
    println!("Attempting to start curses...");

    let window = pancurses::initscr();

    if window.keypad(true) != 0 {
        panic!("keypad returned ERR");
    }
    let requested_mouse_events =
        (::ncurses::ALL_MOUSE_EVENTS | ::ncurses::REPORT_MOUSE_POSITION) as u32;
    ::ncurses::mousemask(requested_mouse_events, None);
    let term_name = ::ncurses::termname();
    let xm_name = std::ffi::CString::new("XM").expect("XM has no NUL bytes");
    let xm = unsafe { raw_tigetstr(xm_name.as_ptr()) };
    if needs_xterm_motion_fallback(&term_name, terminfo_capability_is_present(xm)) {
        // Some xterm terminfo entries omit XM, leaving ncurses in click-only mode.
        XTERM_MOTION_FALLBACK.store(true, Ordering::Relaxed);
        C_mouse_motion_fallback_start();
    }
    if ::ncurses::set_escdelay(25) != 0 {
        panic!("set_escdelay returned ERR");
    }

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

    STDSCR.replace(Some(window));
    crate::user_interface::mouse::register_hover_handler();
    crate::user_interface::mouse::register_monster_list_handler();
    crate::user_interface::mouse::register_item_click_handler();
    crate::user_interface::mouse::register_travel_handler();
    unsafe { C_wait_for_usable_terminal() };
    with_stdscr(|stdscr| {
        if stdscr.clear() != 0 || stdscr.refresh() != 0 {
            panic!("screen initialization returned ERR");
        }
    });
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
    fn restoring_hover_overlay_preserves_prompt_text_and_attributes() {
        let _screen = TestScreen::new(35, 150);
        attron(pancurses::A_BOLD);
        mvaddstr(0, 0, "Command:");
        attroff(pancurses::A_BOLD);
        let original = read_row(0);

        mvaddstr(0, 0, "You see an orc.");
        restore_row(0, &original);

        assert_eq!(read_row(0), original);
        with_stdscr(|window| {
            for (col, ch) in "Command:".chars().enumerate() {
                assert_eq!(
                    window.mvinch(0, col as i32),
                    ch as pancurses::chtype | pancurses::A_BOLD
                );
            }
        });
    }

    #[test]
    #[serial_test::serial]
    fn restoring_hover_overlay_after_terminal_narrows_stays_in_bounds() {
        let original = {
            let _wide_screen = TestScreen::new(35, 150);
            mvaddstr(0, 0, "Command:");
            read_row(0)
        };

        let _narrow_screen = TestScreen::new(35, 80);
        restore_row(0, &original);

        with_stdscr(|window| {
            assert_eq!(window.get_max_x(), 80);
            for col in 0..80 {
                assert_eq!(window.mvinch(0, col), original[col as usize]);
            }
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
    fn split_scrollback_preserves_prompt_and_sidebar_and_redraws_on_close() {
        let _screen = TestScreen::new(35, 150);
        let layout = compute_layout(35, 150);
        assert_eq!(layout.kind, LayoutKind::Split);
        set_current_layout(layout);
        let rect = crate::layout::Rect::new(
            1,
            0,
            layout.log.unwrap().width,
            layout.log.unwrap().row + layout.log.unwrap().height - 1,
        );
        mvaddstr(0, 0, "PROMPT");
        mvaddstr(1, rect.width, "SIDEBAR");
        mvaddstr(34, rect.width, "BOTTOM");

        let mut key_reads = 0;
        let mut redrawn = false;
        crate::term::show_message_scrollback_with(
            || {
                key_reads += 1;
                with_stdscr(|window| {
                    assert_eq!(window.mvinch(0, 0), 'P' as pancurses::chtype);
                    assert_eq!(window.mvinch(1, rect.width), 'S' as pancurses::chtype);
                    assert_eq!(window.mvinch(34, rect.width), 'B' as pancurses::chtype);
                    assert_eq!(window.mvinch(34, rect.width - 1), ' ' as pancurses::chtype);
                });
                crate::io::ESCAPE
            },
            || {
                mvaddstr(0, 0, "PROMPT");
                mvaddstr(1, rect.width, "SIDEBAR");
                mvaddstr(34, rect.width, "BOTTOM");
                redrawn = true;
            },
        );

        assert_eq!(key_reads, 1);
        assert!(redrawn, "closing scrollback should redraw the screen");
        with_stdscr(|window| {
            assert_eq!(window.mvinch(0, 0), 'P' as pancurses::chtype);
            assert_eq!(window.mvinch(1, rect.width), 'S' as pancurses::chtype);
            assert_eq!(window.mvinch(34, rect.width), 'B' as pancurses::chtype);
        });
        crate::layout::reset_current_layout();
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

    #[test]
    fn xterm_without_xm_uses_motion_reporting_fallback() {
        assert!(needs_xterm_motion_fallback("xterm-256color", false));
    }

    #[test]
    fn missing_terminfo_capability_sentinels_are_not_dereferenced() {
        assert!(!terminfo_capability_is_present(std::ptr::null()));
        assert!(!terminfo_capability_is_present(
            -1_isize as *const libc::c_char
        ));
        assert!(terminfo_capability_is_present(
            b"capability\0".as_ptr() as *const libc::c_char
        ));
    }

    #[test]
    fn motion_fallback_is_not_used_when_xm_or_non_xterm_is_present() {
        assert!(!needs_xterm_motion_fallback("xterm-256color", true));
        assert!(!needs_xterm_motion_fallback("screen-256color", false));
    }
}
