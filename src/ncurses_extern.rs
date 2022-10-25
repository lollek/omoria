use crate::ncurses;

#[no_mangle]
pub extern "C" fn C_init_curses() {
    ncurses::init_curses();
}
