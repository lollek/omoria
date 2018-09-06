use libc;
use std::ffi::CString;

use constants;

lazy_static! {
    static ref IMORIA_CSTR: CString = CString::new(constants::IMORIA_VERSION).unwrap();
    static ref OMORIA_CSTR: CString = CString::new(constants::OMORIA_VERSION).unwrap();
}

#[no_mangle]
pub extern fn imoria_version() -> *const libc::c_char {
    IMORIA_CSTR.as_ptr()
}

#[no_mangle]
pub extern fn omoria_version() -> *const libc::c_char {
    OMORIA_CSTR.as_ptr()
}
