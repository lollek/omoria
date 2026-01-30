use libc;
use std::ffi::CStr;

// ---- Pure Rust helpers (unit-testable logic) ----

/// Pascal `index` function.
///
/// Returns the (1-based) position of the first occurrence of `needle` in `haystack`.
/// Returns 0 if not found.
///
/// Note: This is the *Rust* convenience helper. The C ABI wrapper is exported as `pindex`.
pub fn pindex_str(haystack: &str, needle: char) -> i64 {
    // Legacy semantics are 1-based, 0 if not found.
    match haystack.chars().position(|c| c == needle) {
        Some(ix) => (ix as i64) + 1,
        None => 0,
    }
}

/// Tests if `value` is present in `set`.
///
/// Legacy details:
/// - `oset` is a fixed-size set (`MAX_OBJ_SET` entries).
/// - It is assumed to be sorted ascending.
/// - Values > `obj` imply remaining entries can't match.
///
/// We keep the representation compatible with C (`obj_set` is `uint8_t[MAX_OBJ_SET]`).
///
/// Project convention: `0` entries are padding and are *not* members.
pub fn is_in_set(value: i64, set: &[u8]) -> bool {
    // In C, `obj_set` elements are `u8` and the algorithm assumes sorted ascending.
    // For the Rust port we keep behavior for the common, sorted case, but we don't
    // *depend* on ordering to avoid turning malformed sets into hard-to-debug issues.
    if value <= 0 || value > u8::MAX as i64 {
        // Project convention: 0 means "padding" in obj_set (not a real member).
        return false;
    }
    let value_u8 = value as u8;

    set.iter().any(|&v| v != 0 && v == value_u8)
}

/// Returns true if `ch` is a vowel (A,E,I,O,U), case-insensitive.
pub fn is_vowel_char(ch: char) -> bool {
    matches!(
        ch,
        'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U'
    )
}

// ---- C ABI drop-in wrappers ----
//
// These functions intentionally keep the original C symbol names so that
// legacy C call-sites can link against Rust without any source changes.
//
// IMPORTANT: to avoid duplicate symbols at link time, `src/pascal.c` must not
// be compiled/linked when these are enabled (see Makefile).

#[no_mangle]
pub unsafe extern "C" fn pindex(s1: *const libc::c_char, c1: libc::c_char) -> libc::c_long {
    if s1.is_null() {
        return 0;
    }

    // Legacy C matches bytes, not Unicode scalar values.
    let bytes = CStr::from_ptr(s1).to_bytes();
    let needle = c1 as u8;

    match bytes.iter().position(|&b| b == needle) {
        Some(ix) => (ix as libc::c_long) + 1,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn is_vowel(a_char: libc::c_char) -> bool {
    // Preserve legacy semantics: only ASCII vowels count.
    matches!(
        a_char as u8,
        b'a' | b'A' | b'e' | b'E' | b'i' | b'I' | b'o' | b'O' | b'u' | b'U'
    )
}

#[no_mangle]
pub unsafe extern "C" fn is_in(obj: libc::c_long, oset: *const u8) -> bool {
    if oset.is_null() {
        return false;
    }

    // `obj_set` is `uint8_t[MAX_OBJ_SET]` in C; treat 0 as padding.
    // MAX_OBJ_SET is 25 (see constants.h). We hardcode 25 here to avoid
    // binding in C headers at build time.
    const MAX_OBJ_SET: usize = 25;
    let set = std::slice::from_raw_parts(oset, MAX_OBJ_SET);

    if obj <= 0 || obj > u8::MAX as libc::c_long {
        return false;
    }
    let value_u8 = obj as u8;

    set.iter().any(|&v| v != 0 && v == value_u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn pindex_returns_one_based_index_when_found() {
        assert_eq!(pindex_str("abcd", 'a'), 1);
        assert_eq!(pindex_str("abcd", 'c'), 3);
    }

    #[test]
    fn pindex_returns_zero_when_not_found() {
        assert_eq!(pindex_str("abcd", 'z'), 0);
    }

    #[test]
    fn pindex_handles_empty_string() {
        assert_eq!(pindex_str("", 'a'), 0);
    }

    #[test]
    fn is_vowel_is_case_insensitive() {
        for ch in ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'] {
            assert!(is_vowel_char(ch), "expected {} to be a vowel", ch);
        }
        for ch in ['b', 'y', 'Z', '0', '?'] {
            assert!(!is_vowel_char(ch), "expected {} to not be a vowel", ch);
        }
    }

    #[test]
    fn is_in_finds_value_in_sorted_obj_set() {
        // C type is `uint8_t obj_set[MAX_OBJ_SET]`; 0s are commonly used as tail padding.
        let oset: [u8; 8] = [1, 3, 5, 9, 0, 0, 0, 0];
        assert!(is_in_set(1, &oset));
        assert!(is_in_set(9, &oset));
        assert!(!is_in_set(2, &oset));
        assert!(!is_in_set(10, &oset));
    }

    #[test]
    fn is_in_returns_false_for_empty_set() {
        let oset: [u8; 8] = [0; 8];
        assert!(!is_in_set(0, &oset));
        assert!(!is_in_set(1, &oset));
    }

    #[test]
    fn is_in_accepts_large_obj_but_set_is_u8() {
        let oset: [u8; 8] = [250, 251, 252, 0, 0, 0, 0, 0];
        assert!(is_in_set(252, &oset));
        assert!(!is_in_set(253, &oset));
    }

    // These tests validate the exported C ABI surface (`#[no_mangle] extern "C"`).
    // They intentionally call the ABI functions directly (with C-compatible inputs)
    // so we catch accidental signature/behavior changes early.
    #[test]
    fn abi_pindex_matches_legacy_c_semantics_for_bytes() {
        let s = CString::new("abcd").unwrap();
        unsafe {
            assert_eq!(super::pindex(s.as_ptr(), 'a' as libc::c_char), 1);
            assert_eq!(super::pindex(s.as_ptr(), 'c' as libc::c_char), 3);
            assert_eq!(super::pindex(s.as_ptr(), 'z' as libc::c_char), 0);
            assert_eq!(super::pindex(std::ptr::null(), 'a' as libc::c_char), 0);
        }
    }

    #[test]
    fn abi_is_vowel_matches_legacy_ascii_rules() {
        unsafe {
            assert!(super::is_vowel('a' as libc::c_char));
            assert!(super::is_vowel('A' as libc::c_char));
            assert!(!super::is_vowel('b' as libc::c_char));
            assert!(!super::is_vowel('?' as libc::c_char));
        }
    }

    #[test]
    fn abi_is_in_treats_zero_as_padding() {
        // MAX_OBJ_SET is 25 in the C headers; the ABI wrapper reads exactly 25 bytes.
        let mut set = [0u8; 25];
        set[0] = 1;
        set[1] = 3;
        set[2] = 5;

        unsafe {
            assert!(super::is_in(1, set.as_ptr()));
            assert!(!super::is_in(2, set.as_ptr()));
            assert!(!super::is_in(0, set.as_ptr()));
            assert!(!super::is_in(-1, set.as_ptr()));
            assert!(!super::is_in(256, set.as_ptr()));
            assert!(!super::is_in(1, std::ptr::null()));
        }
    }
}
