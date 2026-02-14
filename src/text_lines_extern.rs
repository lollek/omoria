use std::ffi::CStr;
use std::ffi::CString;

use libc::c_char;

use crate::term;
use crate::model::{InventoryItem, Item};
use crate::data::item_name;
use crate::identification::set_identified;

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

    let tilde_off = tilde_pos1_based;
    let pipe_off = pipe_pos1_based;

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

/// C-compatible implementation of `identify` from `text_lines.c`.
///
/// This is a thin wrapper around [`identify_core`] that wires in:
/// - the C global item arrays / lists (`t_list`, `equipment`, `inventory_list`)
/// - the legacy side effect of marking the type identified.
#[no_mangle]
pub unsafe extern "C" fn identify(item_ptr: *mut Item) {
    if item_ptr.is_null() {
        return;
    }

    extern "C" {
        static mut t_list: [Item; crate::constants::MAX_TALLOC + 1];
        static mut equipment: [Item; crate::equipment::EQUIP_MAX];
        static mut inventory_list: *mut InventoryItem;
    }

    // SAFETY: caller passes a valid pointer to a live `Item`.
    let item = unsafe { &mut *item_ptr };

    // SAFETY: we access raw pointers and build temporary slices.
    let t_list_slice: &mut [Item] = unsafe {
        std::slice::from_raw_parts_mut(
            (&raw mut t_list) as *mut Item,
            crate::constants::MAX_TALLOC + 1,
        )
    };
    let equipment_slice: &mut [Item] = unsafe {
        std::slice::from_raw_parts_mut(
            (&raw mut equipment) as *mut Item,
            crate::equipment::EQUIP_MAX,
        )
    };

    identify_core(
        item,
        t_list_slice,
        equipment_slice,
        inventory_list,
    );
}

/// Testable core for legacy `identify()`.
///
/// Contract (legacy parity):
/// - If `item.name` does not contain a `|`, do nothing.
/// - Otherwise, for each matching entry (same tval + subval):
///   - apply `unquote` then `known1` to its name.
/// - Finally, call `mark_identified(item)`.
///
/// Note: This function is intentionally `pub(crate)` so we can unit test it without
/// linking against the C global variables.
pub(crate) fn identify_core(
    item: &mut Item,
    t_list: &mut [Item],
    equipment: &mut [Item],
    inventory_list: *mut InventoryItem,
) {
    // SAFETY NOTE: We assume all `Item.name` buffers we touch are NUL-terminated.
    // That matches the C code's expectations and is required for strstr/unquote/known1.

    // Legacy guard: if item name does not contain the '|' marker, do nothing.
    // (In C this was: `if (strstr(item->name, "|") == NULL) return;`)
    let item_name_has_pipe = unsafe {
        // SAFETY: `item.name` is a fixed-size C buffer; we expect it to be NUL-terminated.
        let s = item.name.as_ptr();
        !libc::strstr(s, b"|\0".as_ptr() as *const c_char).is_null()
    };
    if !item_name_has_pipe {
        return;
    }

    // Helper: apply unquote() and known1() to the provided C string buffer.
    unsafe fn unquote_then_known1(buf: *mut c_char) {
        unquote(buf);
        known1(buf);
    }

    // Update matching entries in t_list.
    for t in t_list.iter_mut() {
        if t.tval == item.tval && t.subval == item.subval {
            unsafe { unquote_then_known1(t.name.as_mut_ptr()) };
        }
    }

    // Update matching entries in equipment.
    for e in equipment.iter_mut() {
        if e.tval == item.tval && e.subval == item.subval {
            unsafe { unquote_then_known1(e.name.as_mut_ptr()) };
        }
    }

    // Update matching entries in the inventory linked list.
    // This mirrors: `for (treas_rec *curse = inventory_list; curse != NULL; curse = curse->next)`.
    let mut cursor = inventory_list;
    while !cursor.is_null() {
        // SAFETY: caller guarantees `inventory_list` is a valid list.
        let node = unsafe { &mut *cursor };
        if node.data.tval == item.tval && node.data.subval == item.subval {
            unsafe { unquote_then_known1(node.data.name.as_mut_ptr()) };
        }
        cursor = node.next;
    }

    // Final legacy side effect.
    let subtype = item.item_subtype().expect("Item has no subtype");
    set_identified(subtype, true)
}

/// C-compatible implementation of `msg_charges_remaining`.
///
/// Player-facing intent: show remaining charges only when the item is identified.
#[no_mangle]
pub unsafe extern "C" fn msg_charges_remaining(item_ptr: *const InventoryItem) {
    if item_ptr.is_null() {
        return;
    }

    // Intent-based behavior: only show remaining charges when the item is identified.
    // This aligns with player-facing intent of “identified item shows its charges”.
    let item = unsafe { &(*item_ptr).data };
    if !item.is_identified() {
        return;
    }

    let mut out_val = [0 as c_char; 82];
    libc::snprintf(
        out_val.as_mut_ptr(),
        out_val.len(),
        b"You have %ld charges remaining.\0".as_ptr() as *const c_char,
        item.p1,
    );

    // SAFETY: out_val is a valid NUL-terminated C string.
    let bytes = unsafe { CStr::from_ptr(out_val.as_ptr()) }.to_bytes();
    term::msg_print(bytes);
}

/// C-compatible implementation of `msg_remaining_of_item` from `text_lines.c`.
///
/// Legacy contract:
/// - Copy `item_ptr->data` into a temporary item.
/// - Decrement `number` *before* naming, so the message describes the remaining
///   stack after consuming/using one item.
/// - Print: `"You have <item_name(tmp_item)>."`.
#[no_mangle]
pub unsafe extern "C" fn msg_remaining_of_item(_item_ptr: *const InventoryItem) {
    if _item_ptr.is_null() {
        return;
    }

    let mut tmp_item = unsafe { (*_item_ptr).data };
    tmp_item.number = tmp_item.number.saturating_sub(1);

    let item_desc = item_name::generate(&tmp_item);
    let msg = format!("You have {}.", item_desc);
    // term::msg_print takes bytes; ensure NUL-free.
    let c = CString::new(msg).expect("msg_remaining_of_item output must not contain NUL");
    term::msg_print(c.as_bytes());
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

#[cfg(test)]
mod identify_core_tests {
    use serial_test::serial;
    use crate::conversion::{item_subtype, item_type};
    use crate::identification::is_identified;
    use crate::model::item_subtype::FoodSubType;
    use crate::model::ItemType;
    use super::*;

    fn unquote_then_known1(buf: &mut [i8]) {
        unsafe { unquote(buf.as_mut_ptr() as *mut c_char) };
        unsafe { known1(buf.as_mut_ptr() as *mut c_char) };
    }

    fn write_name(dst: &mut [i8], s: &[u8]) {
        for (d, &b) in dst.iter_mut().zip(s.iter()) {
            *d = b as i8;
        }
    }

    fn read_name(src: &[i8]) -> String {
        unsafe { CStr::from_ptr(src.as_ptr()).to_string_lossy().into_owned() }
    }

    fn mk_item_with_pipe() -> Item {
        let mut item = Item::default();
        item.tval = item_type::to_usize(ItemType::Food) as u8;
        item.subval = item_subtype::food::to_usize(&FoodSubType::RationOfFood) as i64;
        write_name(&mut item.name, b"foo|bar\0");
        item
    }

    fn mk_item_without_pipe() -> Item {
        let mut item = Item::default();
        item.tval = item_type::to_usize(ItemType::Food) as u8;
        item.subval = item_subtype::food::to_usize(&FoodSubType::RationOfFood) as i64;
        write_name(&mut item.name, b"foobar\0");
        item
    }

    #[serial]
    #[test]
    fn identify_core_noops_when_item_name_has_no_pipe_marker() {
        let mut item = mk_item_without_pipe();
        let item_sub_type = item.item_subtype().unwrap();
        set_identified(item_sub_type, false);

        let mut t_list = vec![Item::default(); 3];
        let mut equipment = vec![Item::default(); 2];

        let mut inv_a = Box::new(InventoryItem {
            data: Item::default(),
            ok: 0,
            insides: 0,
            is_in: 0,
            next: std::ptr::null_mut(),
        });
        write_name(&mut inv_a.data.name, b"ab\"cd~EF|GHI\0");
        let inv_head = &mut *inv_a as *mut InventoryItem;

        identify_core(
            &mut item,
            &mut t_list,
            &mut equipment,
            inv_head,
        );

        assert_eq!(read_name(&inv_a.data.name), "ab\"cd~EF|GHI");
        assert_eq!(is_identified(item_sub_type), false);

        // Prevent cross-test leakage if other tests run after this one.
        set_identified(item_sub_type, false);
    }

    #[test]
    fn identify_core_mutates_matching_entries_and_calls_mark_identified_once() {
        let mut item = mk_item_with_pipe();
        let item_sub_type = item.item_subtype().unwrap();
        set_identified(item_sub_type, false);

        let example_item_name_with_null = b"ab\"cd~EF|GHI\0";
        let example_item_name = "ab\"cd~EF|GHI";

        // Legacy: unquote then known1.
        // For this input string, unquote yields "ab\"cd~GHI" (drops the EF segment),
        // and known1 then does nothing because the '|' is already gone.
        // Result should be "ab\"cd~GHI";

        let mut expected = example_item_name_with_null.clone().map(|x| x as i8);
        unquote_then_known1(&mut expected);

        // t_list: entry 1 matches, entry 2 does not.
        let mut t_list = vec![Item::default(); 3];
        t_list[1].tval = item.tval;
        t_list[1].subval = item.subval;
        write_name(&mut t_list[1].name, example_item_name_with_null);

        t_list[2].tval = item.tval;
        t_list[2].subval = item.subval + 1;
        write_name(&mut t_list[2].name, example_item_name_with_null);

        // equipment: slot 0 matches, slot 1 does not.
        let mut equipment = vec![Item::default(); 2];
        equipment[0].tval = item.tval;
        equipment[0].subval = item.subval;
        write_name(&mut equipment[0].name, example_item_name_with_null);

        equipment[1].tval = item.tval + 1;
        equipment[1].subval = item.subval;
        write_name(&mut equipment[1].name, example_item_name_with_null);

        // inventory list: head matches, next does not.
        let mut inv_next = Box::new(InventoryItem {
            data: Item::default(),
            ok: 0,
            insides: 0,
            is_in: 0,
            next: std::ptr::null_mut(),
        });
        inv_next.data.tval = item.tval;
        inv_next.data.subval = item.subval + 1;
        write_name(&mut inv_next.data.name, example_item_name_with_null);

        let mut inv_head = Box::new(InventoryItem {
            data: Item::default(),
            ok: 0,
            insides: 0,
            is_in: 0,
            next: (&mut *inv_next) as *mut InventoryItem,
        });
        inv_head.data.tval = item.tval;
        inv_head.data.subval = item.subval;
        write_name(&mut inv_head.data.name, example_item_name_with_null);

        let inv_head_ptr = (&mut *inv_head) as *mut InventoryItem;

        identify_core(
            &mut item,
            &mut t_list,
            &mut equipment,
            inv_head_ptr,
        );

        assert_eq!(read_name(&t_list[1].name), read_name(&expected));
        assert_eq!(read_name(&t_list[2].name), example_item_name);

        assert_eq!(read_name(&equipment[0].name), read_name(&expected));
        assert_eq!(read_name(&equipment[1].name), example_item_name);

        unsafe {
            assert_eq!(read_name(&(*inv_head_ptr).data.name), read_name(&expected));
            assert_eq!(read_name(&(*(*inv_head_ptr).next).data.name), example_item_name);
        }

        assert_eq!(is_identified(item_sub_type), true);

        // Prevent cross-test leakage if other tests run after this one.
        set_identified(item_sub_type, false);
    }
}

#[cfg(test)]
mod msg_charges_remaining_tests {
    use super::*;
    use serial_test::serial;

    fn write_name(dst: &mut [i8], s: &[u8]) {
        for (d, &b) in dst.iter_mut().zip(s.iter()) {
            *d = b as i8;
        }
    }

    fn mk_item(name: &[u8], charges: i64, is_identified: bool) -> InventoryItem {
        let mut inv = InventoryItem {
            data: Item::default(),
            ok: 0,
            insides: 0,
            is_in: 0,
            next: std::ptr::null_mut(),
        };
        inv.data.p1 = charges;
        inv.data.set_identified(is_identified);
        write_name(&mut inv.data.name, name);
        inv
    }

    #[test]
    #[serial]
    fn msg_charges_remaining_prints_when_item_is_identified() {
        term::test_clear_last_msg_print();

        let inv = mk_item(b"staff of foo\0", 42, true);
        unsafe { msg_charges_remaining(&inv as *const InventoryItem) };

        assert_eq!(term::test_last_msg_print(), "You have 42 charges remaining.");

        // Prevent cross-test leakage if other tests run after this one.
        term::test_clear_last_msg_print();
    }

    #[test]
    #[serial]
    fn msg_charges_remaining_does_not_print_when_item_is_not_identified() {
        term::test_clear_last_msg_print();

        let inv = mk_item(b"staff of foo\0", 42, false);
        unsafe { msg_charges_remaining(&inv as *const InventoryItem) };

        // Ensure we don't see output, and also clear before assertion to avoid
        // test order leakage if other tests printed.
        assert_eq!(term::test_last_msg_print(), "");

        term::test_clear_last_msg_print();
    }
}

#[cfg(test)]
mod msg_remaining_of_item_tests {
    use super::*;
    use serial_test::serial;
    use crate::identification;

    #[test]
    #[serial]
    fn msg_remaining_of_item_prints_you_have_item_name_with_decremented_quantity() {
        term::test_clear_last_msg_print();

        // Avoid leaking/depending on global subtype identification across tests.
        let staff_light_subtype = crate::model::item_subtype::ItemSubType::Staff(
            crate::model::item_subtype::StaffSubType::StaffOfLight,
        );
        identification::set_identified(staff_light_subtype, false);

        let mut inv = InventoryItem {
            data: Item::default(),
            ok: 0,
            insides: 0,
            is_in: 0,
            next: std::ptr::null_mut(),
        };
        // Make this a valid staff so item_name::generate() can format it.
        inv.data.tval = crate::conversion::item_type::to_usize(crate::model::ItemType::Staff) as u8;
        inv.data.subval = crate::conversion::item_subtype::to_usize(
            &crate::model::item_subtype::ItemSubType::Staff(
                crate::model::item_subtype::StaffSubType::StaffOfLight,
            ),
        ) as i64;
        inv.data.set_identified(true);
        inv.data.p1 = 5;
        inv.data.number = 2;

        unsafe { msg_remaining_of_item(&inv as *const InventoryItem) };

        // When number is decremented from 2 -> 1, typical item naming will switch
        // from plural to singular, so we should see a singular item mention.
        // This assertion is intentionally tight: it checks the full user-facing sentence.
        // NOTE: This assertion is only stable if the subval maps correctly.
        assert_eq!(term::test_last_msg_print(), "You have staff of light (5 charges).");

        // This looks odd at first, but it matches the legacy C behavior: the function
        // decrements a *temporary copy* of the item for naming/printing ("remaining after
        // consuming one"), and must not modify the caller's inventory record.
        assert_eq!(inv.data.number, 2);

        identification::set_identified(staff_light_subtype, false);
    }
}
