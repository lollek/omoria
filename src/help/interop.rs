use crate::help::{help_page_lines, ident_char_description};
use libc::{c_char, c_int};
use std::ffi::CString;
use std::ptr;

lazy_static! {
    static ref HELP_TEXT: Vec<Vec<CString>> = (0..2)
        .map(|page| {
            help_page_lines(page)
                .expect("known help page")
                .iter()
                .map(|line| CString::new(line.text).expect("help text contains no NUL bytes"))
                .collect()
        })
        .collect();
    static ref IDENT_CHAR_TEXT: Vec<Vec<CString>> = b"tdw"
        .iter()
        .map(|&environment| {
            (0..=u8::MAX)
                .map(|character| {
                    CString::new(ident_char_description(environment, character).unwrap_or_default())
                        .expect("identification text contains no NUL bytes")
                })
                .collect()
        })
        .collect();
}

#[no_mangle]
pub extern "C" fn C_help_page_line_count(page: c_int) -> c_int {
    if page < 0 {
        return 0;
    }
    help_page_lines(page as usize)
        .map(|lines| lines.len() as c_int)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn C_help_page_line_row(page: c_int, line: c_int) -> c_int {
    if page < 0 || line < 0 {
        return 0;
    }
    let line = line as usize;
    help_page_lines(page as usize)
        .and_then(|lines| lines.get(line))
        .map(|line| line.row)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn C_help_page_line_text(page: c_int, line: c_int) -> *const c_char {
    if page < 0 || line < 0 {
        return ptr::null();
    }
    let line = line as usize;
    let Some(page_text) = HELP_TEXT.get(page as usize) else {
        return ptr::null();
    };
    page_text
        .get(line)
        .map(|text| text.as_ptr())
        .unwrap_or(ptr::null())
}

#[no_mangle]
pub extern "C" fn C_ident_char_description(
    environment: c_char,
    character: c_char,
) -> *const c_char {
    let environment = environment as u8;
    let character = character as u8;
    let environment_index = match environment {
        b't' => 0,
        b'd' => 1,
        b'w' => 2,
        _ => return ptr::null(),
    };
    if ident_char_description(environment, character).is_none() {
        return ptr::null();
    }

    IDENT_CHAR_TEXT[environment_index][character as usize].as_ptr()
}
