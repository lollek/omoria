use std::ffi::CStr;

use libc::c_char;

use crate::model::{InventoryItem, Item};

const BAG_DESCRIP_BUF_SIZE: usize = 134;

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

/// C-compatible implementation of `bag_descrip` from `text_lines.c`.
///
/// Signature is maintained for C call sites:
/// `char *bag_descrip(const treas_rec *bag, char result[134]);`
#[no_mangle]
pub unsafe extern "C" fn bag_descrip(bag: *const InventoryItem, result: *mut c_char) -> *mut c_char {
    if result.is_null() {
        return result;
    }
    if bag.is_null() {
        // We expect callers to pass a valid bag; keep behavior defined for tests.
        libc::strcpy(result, b" (empty)\0".as_ptr() as *const c_char);
        return result;
    }

    let bag_node = &*bag;

    // Empty if no next item, or next item exists but isn't in the bag.
    if bag_node.next.is_null() || unsafe { &*bag_node.next }.is_in == 0 {
        libc::strcpy(result, b" (empty)\0".as_ptr() as *const c_char);
        return result;
    }

    let mut item_count: i64 = 0;
    let mut total_weight: i64 = 0;

    let mut cursor: *mut InventoryItem = bag_node.next;
    while !cursor.is_null() {
        let node = &*cursor;
        if node.is_in == 0 {
            break;
        }
        let n = node.data.number as i64;
        item_count += n;
        total_weight += (node.data.weight as i64) * n;
        cursor = node.next;
    }

    let capacity = bag_node.data.p1;
    let percent_full: i64 = if capacity > 0 {
        (total_weight * 100) / capacity
    } else {
        0
    };

    // Provide a NUL-terminated suffix for `snprintf`.
    let suffix_cstr: *const c_char = if item_count != 1 {
        b"s\0".as_ptr() as *const c_char
    } else {
        b"\0".as_ptr() as *const c_char
    };
    // `result` is a fixed-size caller buffer in C; use snprintf to avoid overflow.
    libc::snprintf(
        result,
        BAG_DESCRIP_BUF_SIZE,
        b" (%ld%% full, containing %ld item%s)\0".as_ptr() as *const c_char,
        percent_full,
        item_count,
        suffix_cstr,
    );
    result
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::*;
    fn item_default() -> Item {
        // `Item` is `Copy`, so we can build a zeroed-ish default for tests.
        Item {
            name: [0; 70],
            tval: 0,
            flags2: 0,
            flags: 0,
            p1: 0,
            cost: 0,
            subval: 0,
            weight: 0,
            number: 0,
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: [0; 7],
            level: 0,
            identified: 0,
        }
    }

    fn inventory_item_default() -> InventoryItem {
        InventoryItem {
            data: item_default(),
            ok: 0,
            insides: 0,
            is_in: 0,
            next: std::ptr::null_mut(),
        }
    }

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

    #[test]
    fn bag_descrip_empty_when_bag_next_is_null() {
        let bag = InventoryItem {
            next: std::ptr::null_mut(),
            ..inventory_item_default()
        };

        let mut out = vec![0 as c_char; BAG_DESCRIP_BUF_SIZE];
        let ret = unsafe { bag_descrip(&bag as *const InventoryItem, out.as_mut_ptr()) };

        assert_eq!(ret, out.as_mut_ptr());
        assert_eq!(read_buf(&out), " (empty)");
    }

    #[test]
    fn bag_descrip_empty_when_bag_next_is_not_in() {
        let next = Box::new(InventoryItem {
            is_in: 0,
            ..inventory_item_default()
        });
        let bag = InventoryItem {
            next: (&*next) as *const InventoryItem as *mut InventoryItem,
            ..inventory_item_default()
        };

        let mut out = vec![0 as c_char; BAG_DESCRIP_BUF_SIZE];
        let ret = unsafe { bag_descrip(&bag as *const InventoryItem, out.as_mut_ptr()) };

        assert_eq!(ret, out.as_mut_ptr());
        assert_eq!(read_buf(&out), " (empty)");
    }

    #[test]
    fn bag_descrip_one_item_formats_singular_item_suffix() {
        let item = Box::new(InventoryItem {
            is_in: 1,
            data: Item {
                number: 1,
                weight: 10,
                ..item_default()
            },
            next: std::ptr::null_mut(),
            ..inventory_item_default()
        });

        let bag = InventoryItem {
            data: Item {
                p1: 100, // capacity
                ..item_default()
            },
            next: (&*item) as *const InventoryItem as *mut InventoryItem,
            ..inventory_item_default()
        };

        let mut out = vec![0 as c_char; BAG_DESCRIP_BUF_SIZE];
        unsafe { bag_descrip(&bag as *const InventoryItem, out.as_mut_ptr()) };

        // wgt=10, capacity=100 => 10%
        assert_eq!(read_buf(&out), " (10% full, containing 1 item)");
    }

    #[test]
    fn bag_descrip_multiple_items_formats_plural_items_and_sums_weight_and_count() {
        let item2 = Box::new(InventoryItem {
            is_in: 1,
            data: Item {
                number: 2,
                weight: 7,
                ..item_default()
            },
            next: std::ptr::null_mut(),
            ..inventory_item_default()
        });
        let item1 = Box::new(InventoryItem {
            is_in: 1,
            data: Item {
                number: 3,
                weight: 5,
                ..item_default()
            },
            next: (&*item2) as *const InventoryItem as *mut InventoryItem,
            ..inventory_item_default()
        });

        let bag = InventoryItem {
            data: Item {
                p1: 100, // capacity
                ..item_default()
            },
            next: (&*item1) as *const InventoryItem as *mut InventoryItem,
            ..inventory_item_default()
        };

        let mut out = vec![0 as c_char; BAG_DESCRIP_BUF_SIZE];
        unsafe { bag_descrip(&bag as *const InventoryItem, out.as_mut_ptr()) };

        // count = 3 + 2 = 5
        // weight = 3*5 + 2*7 = 29 => 29%
        assert_eq!(read_buf(&out), " (29% full, containing 5 items)");
    }
}
