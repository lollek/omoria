use libc;
use std::ffi::CString;

use crate::constants;

lazy_static! {
    static ref IMORIA_CSTR: CString = CString::new(constants::IMORIA_VERSION).unwrap();
    static ref OMORIA_CSTR: CString = CString::new(constants::OMORIA_VERSION).unwrap();
}

#[no_mangle]
pub extern "C" fn imoria_version() -> *const libc::c_char {
    IMORIA_CSTR.as_ptr()
}

#[no_mangle]
pub extern "C" fn omoria_version() -> *const libc::c_char {
    OMORIA_CSTR.as_ptr()
}
