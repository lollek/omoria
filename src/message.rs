use crate::logic::menu::draw_help_vec;
use std::collections::LinkedList;
use std::ffi::CStr;
use std::sync::{Arc, Mutex, RwLock};

type MessageStream = Arc<Mutex<Vec<String>>>;

lazy_static! {
    static ref MESSAGE_RECORD: RwLock<LinkedList<String>> = RwLock::new(LinkedList::default());
    static ref MESSAGE_CAPTURES: Mutex<Vec<MessageStream>> = Mutex::new(Vec::new());
    static ref C_MESSAGE_CAPTURES: Mutex<Vec<MessageCapture>> = Mutex::new(Vec::new());
}
const MAX_MESSAGES: usize = 50;

#[no_mangle]
pub extern "C" fn message_capture_active() -> bool {
    !MESSAGE_CAPTURES.lock().expect("Mutex poisoned").is_empty()
}

#[no_mangle]
pub extern "C" fn C_message_capture_begin() {
    C_MESSAGE_CAPTURES
        .lock()
        .expect("Mutex poisoned")
        .push(capture_messages());
}

#[no_mangle]
pub extern "C" fn C_message_capture_end() {
    C_MESSAGE_CAPTURES.lock().expect("Mutex poisoned").pop();
}

#[no_mangle]
pub extern "C" fn C_message_capture_count() -> libc::size_t {
    C_MESSAGE_CAPTURES
        .lock()
        .expect("Mutex poisoned")
        .last()
        .map_or(0, |capture| {
            capture.messages.lock().expect("Mutex poisoned").len()
        })
}

/// # Safety
/// A non-null buffer must be writable for length bytes.
#[no_mangle]
pub unsafe extern "C" fn C_message_capture_get(
    index: libc::size_t,
    buffer: *mut libc::c_char,
    length: libc::size_t,
) -> bool {
    if buffer.is_null() || length == 0 {
        return false;
    }
    let captures = C_MESSAGE_CAPTURES.lock().expect("Mutex poisoned");
    let Some(capture) = captures.last() else {
        return false;
    };
    let messages = capture.messages.lock().expect("Mutex poisoned");
    let Some(message) = messages.get(index) else {
        return false;
    };
    let count = message.len().min(length - 1);
    unsafe {
        std::ptr::copy_nonoverlapping(message.as_ptr(), buffer.cast(), count);
        *buffer.add(count) = 0;
    }
    true
}

#[must_use]
pub struct MessageCapture {
    messages: MessageStream,
}

pub fn capture_messages() -> MessageCapture {
    let messages = Arc::new(Mutex::new(Vec::new()));
    MESSAGE_CAPTURES
        .lock()
        .expect("Mutex poisoned")
        .push(Arc::clone(&messages));
    MessageCapture { messages }
}

impl MessageCapture {
    pub fn messages(&self) -> Vec<String> {
        self.messages.lock().expect("Mutex poisoned").clone()
    }
}

impl Drop for MessageCapture {
    fn drop(&mut self) {
        MESSAGE_CAPTURES
            .lock()
            .expect("Mutex poisoned")
            .retain(|stream| !Arc::ptr_eq(stream, &self.messages));
    }
}

extern "C" {
    #[link_name = "draw_cave"]
    pub fn draw_cave();
}

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
    // Holding the history lock keeps concurrent stream and history writes in the same order.
    if let Some(stream) = MESSAGE_CAPTURES.lock().expect("Mutex poisoned").last() {
        stream.lock().expect("Mutex poisoned").push(message.clone());
    }
    guard.push_back(message);
    if guard.len() > MAX_MESSAGES {
        guard.pop_front();
    }
}

#[no_mangle]
pub extern "C" fn show_recorded_messages() {
    let guard = MESSAGE_RECORD.read().expect("RwLock poisoned");
    let items = guard
        .iter()
        .rev()
        .map(String::as_ref)
        .collect::<Vec<&str>>();
    draw_help_vec("Messages", &items);
    unsafe { draw_cave() }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::term;
    use serial_test::serial;

    pub(crate) struct TestState {
        history: LinkedList<String>,
        last_message: String,
    }

    impl TestState {
        pub(crate) fn new() -> Self {
            let state = Self {
                history: std::mem::take(&mut *MESSAGE_RECORD.write().unwrap()),
                last_message: term::test_last_msg_print(),
            };
            term::test_clear_last_msg_print();
            state
        }
    }

    impl Drop for TestState {
        fn drop(&mut self) {
            term::msg_print(self.last_message.clone());
            *MESSAGE_RECORD.write().unwrap() = std::mem::take(&mut self.history);
        }
    }

    #[test]
    #[serial]
    fn test_state_restores_history_and_last_message_after_panic() {
        let _state = TestState::new();
        term::msg_print("Original message");
        let result = std::panic::catch_unwind(|| {
            let _inner_state = TestState::new();
            assert!(MESSAGE_RECORD.read().unwrap().is_empty());
            assert_eq!(term::test_last_msg_print(), "");
            term::msg_print("Temporary message");
            panic!("Test failure");
        });
        assert!(result.is_err());
        let history: Vec<String> = MESSAGE_RECORD.read().unwrap().iter().cloned().collect();
        assert_eq!(history, ["Original message"]);
        assert_eq!(term::test_last_msg_print(), "Original message");
    }

    #[test]
    #[serial]
    fn capture_active_tracks_live_guards_and_panic_cleanup() {
        let _state = TestState::new();
        assert!(!message_capture_active());
        let outer = capture_messages();
        assert!(message_capture_active());
        let inner = capture_messages();
        drop(outer);
        assert!(message_capture_active());
        drop(inner);
        assert!(!message_capture_active());
        let result = std::panic::catch_unwind(|| {
            let _capture = capture_messages();
            assert!(message_capture_active());
            panic!("Scoped capture failure");
        });
        assert!(result.is_err());
        assert!(!message_capture_active());
    }

    #[test]
    #[serial]
    fn fresh_capture_is_empty() {
        let _state = TestState::new();
        let capture = capture_messages();
        assert!(capture.messages().is_empty());
    }

    #[test]
    #[serial]
    fn capture_preserves_message_order_and_contents() {
        let _state = TestState::new();
        let capture = capture_messages();
        record_message("First message".into());
        record_message("Second message".into());
        assert_eq!(capture.messages(), ["First message", "Second message"]);
    }

    #[test]
    #[serial]
    fn capture_is_unbounded_while_history_keeps_last_fifty() {
        let _state = TestState::new();
        let capture = capture_messages();
        let messages: Vec<String> = (0..MAX_MESSAGES + 2)
            .map(|index| format!("Message {index}"))
            .collect();
        for message in &messages {
            record_message(message.clone());
        }
        assert_eq!(capture.messages(), messages);
        let history: Vec<String> = MESSAGE_RECORD.read().unwrap().iter().cloned().collect();
        assert_eq!(history, messages[2..]);
    }

    #[test]
    #[serial]
    fn capture_retains_empty_and_space_messages() {
        let _state = TestState::new();
        let capture = capture_messages();
        record_message(String::new());
        record_message(" ".into());
        assert_eq!(capture.messages(), ["", " "]);
    }

    #[test]
    #[serial]
    fn second_capture_starts_empty() {
        let _state = TestState::new();
        {
            let capture = capture_messages();
            record_message("Previous capture".into());
            assert_eq!(capture.messages(), ["Previous capture"]);
        }
        let capture = capture_messages();
        assert!(capture.messages().is_empty());
        record_message("New capture".into());
        assert_eq!(capture.messages(), ["New capture"]);
    }

    #[test]
    #[serial]
    fn dropping_capture_restores_history_only_recording() {
        let _state = TestState::new();
        let capture = capture_messages();
        let stream = Arc::clone(&capture.messages);
        record_message("Captured".into());
        drop(capture);
        record_message("History only".into());
        assert_eq!(*stream.lock().unwrap(), ["Captured"]);
        let history: Vec<String> = MESSAGE_RECORD.read().unwrap().iter().cloned().collect();
        assert_eq!(history, ["Captured", "History only"]);
    }

    #[test]
    #[serial]
    fn panic_restores_history_only_recording() {
        let _state = TestState::new();
        let mut stream = None;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let capture = capture_messages();
            stream = Some(Arc::clone(&capture.messages));
            record_message("Before panic".into());
            panic!("Scoped capture failure");
        }));
        assert!(result.is_err());
        record_message("After panic".into());
        assert_eq!(*stream.unwrap().lock().unwrap(), ["Before panic"]);
        let capture = capture_messages();
        assert!(capture.messages().is_empty());
    }

    #[test]
    #[serial]
    fn nested_capture_restores_outer_stream() {
        let _state = TestState::new();
        let outer = capture_messages();
        record_message("Before inner".into());
        {
            let inner = capture_messages();
            record_message("Inner".into());
            assert_eq!(inner.messages(), ["Inner"]);
        }
        record_message("After inner".into());
        assert_eq!(outer.messages(), ["Before inner", "After inner"]);
    }

    #[test]
    #[serial]
    fn dropping_outer_capture_does_not_revive_it_after_inner_drop() {
        let _state = TestState::new();
        let outer = capture_messages();
        let outer_stream = Arc::clone(&outer.messages);
        record_message("Outer".into());
        let inner = capture_messages();
        let inner_stream = Arc::clone(&inner.messages);
        drop(outer);
        record_message("Inner".into());
        assert_eq!(inner.messages(), ["Inner"]);
        drop(inner);
        record_message("History only".into());
        assert_eq!(*outer_stream.lock().unwrap(), ["Outer"]);
        assert_eq!(*inner_stream.lock().unwrap(), ["Inner"]);
    }

    #[test]
    #[serial]
    fn rust_terminal_stub_feeds_capture_and_keeps_last_message() {
        let _state = TestState::new();
        let capture = capture_messages();
        term::msg_print("Rust message");
        assert_eq!(capture.messages(), ["Rust message"]);
        assert_eq!(term::test_last_msg_print(), "Rust message");
    }
}
