use std::ffi::CString;

extern "C" {
    fn mvaddstr(y: i32, x: i32, str: *const u8) -> i32;
}

pub extern fn put_buffer_rust(out_str: &str, row: i32, col: i32) {
    match CString::new(out_str) {
        Ok(cstr) => put_buffer_(cstr.as_ptr() as *const u8, row, col),
        Err(e) => panic!(e),
    }
}

#[no_mangle]
pub extern fn put_buffer_(out_str: *const u8, row: i32, col: i32) {
    unsafe {
        if out_str.is_null() {
            panic!("Null string received");
        }

        if mvaddstr(row, col, out_str) != 0 {
            panic!("mvaddstr returned ERR");
        }
    }
}
