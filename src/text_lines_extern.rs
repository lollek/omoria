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

/// C-compatible implementation of `unquote` from `text_lines.c`.
///
/// Legacy behavior: if the string contains a `"` byte, it takes the prefix of the
/// string up to (and including) the first `"` and then appends the suffix starting
/// from the first `|` marker.
///
/// This is based on the original C implementation, which uses `pindex` (1-based)
/// but then applies the returned positions as if they were 0-based indices.
#[no_mangle]
pub unsafe extern "C" fn unquote(object_str: *mut c_char) {
    if object_str.is_null() {
        return;
    }

    let bytes_with_nul = CStr::from_ptr(object_str).to_bytes_with_nul();

    // pindex('"') > 0 gate.
    if bytes_with_nul.iter().take(bytes_with_nul.len() - 1).all(|&b| b != b'"') {
        return;
    }

    // Legacy indexing:
    // pos1/pos2 are conceptually 1-based, but are used as 0-based indices.
    // So we compute 0-based offsets that match those numeric results.
    let tilde_pos1_based = bytes_with_nul
        .iter()
        .take(bytes_with_nul.len() - 1)
        .position(|&b| b == b'~')
        .map(|ix| ix + 1)
        .unwrap_or(0);
    let pipe_pos1_based = bytes_with_nul
        .iter()
        .take(bytes_with_nul.len() - 1)
        .position(|&b| b == b'|')
        .map(|ix| ix + 1)
        .unwrap_or(0);

    let tilde_off = tilde_pos1_based as usize;
    let pipe_off = pipe_pos1_based as usize;

    if tilde_off >= bytes_with_nul.len() || pipe_off >= bytes_with_nul.len() {
        return;
    }

    // Emulate `sprintf(object_str, "%s%s", str1, str2)` where:
    // - str1 includes bytes [0..=tilde_off) (tilde_off is used as a byte *count*)
    // - str2 starts at &object_str[pipe_off]
    // This effectively drops bytes in [tilde_off..pipe_off).
    if pipe_off > tilde_off {
        libc::memmove(
            object_str.add(tilde_off) as *mut _,
            object_str.add(pipe_off) as *const _,
            bytes_with_nul.len() - pipe_off,
        );
    }
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

    #[test]
    fn unquote_when_no_quote_marker_is_noop() {
        let mut buf = make_buf("foo~bar|baz");
        unsafe { unquote(buf.as_mut_ptr()) };
        assert_eq!(read_buf(&buf), "foo~bar|baz");
    }

    #[test]
    fn unquote_removes_quoted_portion_by_joining_prefix_before_tilde_with_suffix_from_pipe() {
        let mut buf = make_buf("ab\"cd~EF|GHI");
        unsafe { unquote(buf.as_mut_ptr()) };
        assert_eq!(read_buf(&buf), "ab\"cd~GHI");
    }

    #[test]
    fn unquote_does_not_write_past_end_of_original_buffer() {
        // Model a fixed-size C buffer (like `char name[82]`) with sentinel bytes after it.
        // This avoids relying on Vec reallocation details and gives a stable nul position.
        let mut buf = vec![0 as c_char; 82];
        let src = make_buf("ab\"cd~EF|GHI");
        buf[..src.len()].copy_from_slice(&src);
        buf.extend([111 as c_char, 112 as c_char]);

        unsafe { unquote(buf.as_mut_ptr()) };
        assert_eq!(read_buf(&buf), "ab\"cd~GHI");

        // The bytes *after the fixed-size buffer* are outside the caller allocation;
        // unquote must not touch them.
        assert_eq!(buf[82], 111 as c_char);
        assert_eq!(buf[83], 112 as c_char);
    }
}
