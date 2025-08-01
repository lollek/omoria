use crate::logic::menu::draw_help_vec;
use crate::screen::draw_cave;
use std::collections::LinkedList;
use std::ffi::CStr;
use std::sync::RwLock;

lazy_static! {
    static ref MESSAGE_RECORD: RwLock<LinkedList<String>> =
        RwLock::new(LinkedList::default());
}
const MAX_MESSAGES: usize = 50;

#[no_mangle]
extern "C" fn _record_message(message: *const libc::c_char) {
    if message.is_null() {
        panic!("Null string received");
    }
    let message = unsafe { CStr::from_ptr(message) }
        .to_str()
        .expect("Failed to convert C string to rust")
        .to_string();
    record_message(message);
}
pub fn record_message(message: String) {
    let mut guard = MESSAGE_RECORD.write().expect("RwLock poisoned");
    guard.push_back(message);
    if guard.len() > MAX_MESSAGES {
        guard.pop_front();
    }
}

#[no_mangle]
pub extern "C" fn show_recorded_messages() {
    let guard = MESSAGE_RECORD.read().expect("RwLock poisoned");
    let items = guard.iter().rev().map(String::as_ref).collect::<Vec<&str>>();
    draw_help_vec("Messages", &items);
    unsafe {
        draw_cave()
    }
}