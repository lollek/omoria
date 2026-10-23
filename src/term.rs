use libc;

#[cfg(test)]
use std::sync::Mutex;

use crate::layout::current_layout;
use crate::message::current_turn;
use crate::misc;
use crate::ncurses;
use crate::user_interface::message_log::{LogLine, MessageLog};
use std::ffi::{CStr, CString};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex as LogMutex;

lazy_static! {
    static ref MESSAGE_LOG: LogMutex<(MessageLog, usize)> =
        LogMutex::new((MessageLog::default(), 0));
}
static INPUT_GENERATION: AtomicUsize = AtomicUsize::new(0);
static COMMAND_PENDING: AtomicBool = AtomicBool::new(false);

pub type LogDraw = extern "C" fn(*const libc::c_char, libc::c_int, libc::c_int, bool);
pub type LogMore = extern "C" fn() -> bool;

pub fn message_command_read() {
    COMMAND_PENDING.store(true, Ordering::Relaxed);
}

fn apply_pending_command(log: &mut MessageLog) {
    if COMMAND_PENDING.swap(false, Ordering::Relaxed) {
        log.command_read();
    }
}

pub fn message_input_read() {
    // Input can be read by a -more- callback while the log is being displayed.
    INPUT_GENERATION.fetch_add(1, Ordering::Relaxed);
}

/// # Safety
/// `message` must point to a live NUL-terminated string.
pub unsafe fn print_log_message(
    message: *const libc::c_char,
    draw: LogDraw,
    more: LogMore,
) -> bool {
    let rect = current_layout()
        .log
        .expect("Split log rectangle unavailable");
    let text = unsafe { CStr::from_ptr(message) }
        .to_str()
        .expect("Invalid message UTF-8");
    if text.is_empty() {
        return false;
    }
    let mut state = MESSAGE_LOG.lock().expect("message log lock poisoned");
    let (log, generation) = &mut *state;
    apply_pending_command(log);
    if *generation != INPUT_GENERATION.load(Ordering::Relaxed) {
        log.input_read();
    }
    let aborted = log.print(
        text,
        current_turn(),
        rect,
        |lines| {
            for (offset, line) in lines.iter().enumerate() {
                let text = CString::new(line.text.as_str()).expect("NUL in rendered message");
                draw(text.as_ptr(), rect.row + offset as i32, rect.col, line.dim);
            }
        },
        || more(),
    );
    *generation = INPUT_GENERATION.load(Ordering::Relaxed);
    aborted
}

pub fn redraw_message_log() {
    if let Some(rect) = current_layout().log {
        let lines = {
            let mut state = MESSAGE_LOG.lock().expect("message log lock poisoned");
            apply_pending_command(&mut state.0);
            state.0.render(rect)
        };
        for (offset, LogLine { text, dim }) in lines.iter().enumerate() {
            if *dim {
                ncurses::attron(ncurses::A_DIM);
            }
            ncurses::mvaddstr(rect.row + offset as i32, rect.col, text);
            if *dim {
                ncurses::attroff(ncurses::A_DIM);
            }
        }
    }
}

#[cfg(test)]
use lazy_static::lazy_static;

extern "C" {
    #[link_name = "msg_flag"]
    static C_msg_flag: u8;

    #[link_name = "msg_print"]
    #[cfg(not(test))]
    fn C_msg_print(str_buff: *const libc::c_char);

    #[link_name = "Get_String"]
    fn C_Get_String(
        in_str: *const libc::c_char,
        row: libc::c_int,
        col: libc::c_int,
        slen: libc::c_int,
    ) -> u8;

    #[link_name = "Clear_From"]
    fn C_Clear_From(row: libc::c_int);
}

#[cfg(test)]
lazy_static! {
    static ref TEST_LAST_MSG_PRINT: Mutex<String> = Mutex::new(String::new());
}

pub fn has_msg_flag() -> bool {
    unsafe { C_msg_flag != 0 }
}

pub fn clear_from(row: i32) {
    unsafe { C_Clear_From(row - 1) };
}

#[cfg(test)]
pub fn msg_print<S>(out_str: S)
where
    S: Into<Vec<u8>>,
{
    // In unit tests we avoid calling into ncurses/C and capture the last printed message.
    let s = String::from_utf8_lossy(&out_str.into()).into_owned();
    crate::message::record_message(s.clone());
    let mut guard = TEST_LAST_MSG_PRINT.lock().unwrap();
    *guard = s;
}

#[cfg(not(test))]
pub fn msg_print<S>(out_str: S)
where
    S: Into<Vec<u8>>,
{
    let cstr = CString::new(out_str).unwrap();
    unsafe { C_msg_print(cstr.as_ptr()) }
}

#[cfg(test)]
pub fn test_last_msg_print() -> String {
    TEST_LAST_MSG_PRINT.lock().unwrap().clone()
}

#[cfg(test)]
pub fn test_clear_last_msg_print() {
    *TEST_LAST_MSG_PRINT.lock().unwrap() = String::new();
}

/* Gets a string terminated by <RETURN>		*/
#[allow(clippy::while_immutable_condition)] // C mutates this buffer; its pointer contract still needs an isolated test.
pub fn get_string(row: i32, col: i32, slen: i32) -> String {
    let tmp_str: [u8; 134] = [0; 134];

    while tmp_str[0] == b'\0' {
        unsafe {
            C_Get_String(
                tmp_str.as_ptr() as *mut libc::c_char,
                row - 1,
                col - 1,
                slen,
            )
        };
    }
    misc::c_array_to_rust_string(tmp_str.to_vec())
}

// put_buffer which clears the line first
pub fn prt<S: AsRef<str>>(msg: S, row: i32, col: i32) {
    if row == -1 && has_msg_flag() {
        msg_print("");
    }
    ncurses::mov(row, col);
    ncurses::clrtoeol();
    ncurses::mvaddstr(row, col, msg);
}

pub fn clear_screen() {
    if has_msg_flag() {
        msg_print("");
    }
    ncurses::clear();
    redraw_message_log();
}
