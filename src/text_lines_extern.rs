use std::ffi::CStr;

use libc::c_char;

/// Removes the first occurrence of `needle` from the nul-terminated C string at `s`.
///
/// Safety: `s` must be a valid, writable nul-terminated string.
///
/// Notes:
/// - Matches legacy `insert_str` behavior: removes only the first occurrence.
/// - Writes back into the original buffer without changing its length (shifts left),
///   so it can’t overflow the caller’s allocation.
unsafe fn remove_first_marker_in_place(s: *mut c_char, needle: u8) {
    if s.is_null() {
        return;
    }

    // Include the trailing nul so we can memmove it too.
    let bytes_with_nul = CStr::from_ptr(s).to_bytes_with_nul();
    if let Some(pos) = bytes_with_nul.iter().position(|&b| b == needle) {
        // Shift everything after the marker left by one, including the trailing nul.
        // This stays within the original buffer.
        libc::memmove(
            s.add(pos) as *mut _,
            s.add(pos + 1) as *const _,
            bytes_with_nul.len() - (pos + 1),
        );
    }
}

/// C-compatible implementation of `known1` from `text_lines.c`.
///
/// Removes the first occurrence of the `|` marker from the string, if present.
#[no_mangle]
pub unsafe extern "C" fn known1(object_str: *mut c_char) {
    remove_first_marker_in_place(object_str, b'|');
}

/// C-compatible implementation of `known2` from `text_lines.c`.
///
/// Removes the first occurrence of the `^` marker from the string, if present.
#[no_mangle]
pub unsafe extern "C" fn known2(object_str: *mut c_char) {
    remove_first_marker_in_place(object_str, b'^');
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::*;

    fn make_buf(s: &str) -> Vec<c_char> {
        let mut v = s.as_bytes().iter().map(|&b| b as c_char).collect::<Vec<_>>();
        v.push(0);
        v
    }

    fn read_buf(buf: &[c_char]) -> String {
        unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap().to_string() }
    }

    #[test]
    fn known1_removes_first_pipe_marker_in_place() {
        let mut buf = make_buf("foo|bar");
        unsafe { known1(buf.as_mut_ptr()) };
        assert_eq!(read_buf(&buf), "foobar");
    }

    #[test]
    fn known2_removes_first_caret_marker_in_place() {
        let mut buf = make_buf("foo^bar");
        unsafe { known2(buf.as_mut_ptr()) };
        assert_eq!(read_buf(&buf), "foobar");
    }

    #[test]
    fn known1_leaves_string_unchanged_when_absent() {
        let mut buf = make_buf("foobar");
        unsafe { known1(buf.as_mut_ptr()) };
        assert_eq!(read_buf(&buf), "foobar");
    }

    #[test]
    fn known2_leaves_string_unchanged_when_absent() {
        let mut buf = make_buf("foobar");
        unsafe { known2(buf.as_mut_ptr()) };
        assert_eq!(read_buf(&buf), "foobar");
    }

    #[test]
    fn known1_only_removes_first_occurrence_like_insert_str() {
        let mut buf = make_buf("a|b|c");
        unsafe { known1(buf.as_mut_ptr()) };
        assert_eq!(read_buf(&buf), "ab|c");
    }

    #[test]
    fn known2_only_removes_first_occurrence_like_insert_str() {
        let mut buf = make_buf("a^b^c");
        unsafe { known2(buf.as_mut_ptr()) };
        assert_eq!(read_buf(&buf), "ab^c");
    }

    #[test]
    fn known1_does_not_write_past_end_of_original_buffer() {
        // Extra garbage after the nul should remain unchanged.
        let mut buf = make_buf("a|b");
        buf.extend([123 as c_char, 124 as c_char]);
        unsafe { known1(buf.as_mut_ptr()) };
        assert_eq!(read_buf(&buf), "ab");
        assert_eq!(buf[4], 123 as c_char);
        assert_eq!(buf[5], 124 as c_char);
    }
}
