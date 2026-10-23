use libc::c_int;

pub const KEY_HOME: u8 = 0xf0;
pub const KEY_END: u8 = 0xf1;
pub const KEY_PAGE_UP: u8 = 0xf2;
pub const KEY_PAGE_DOWN: u8 = 0xf3;

pub fn translate_key(code: c_int) -> Option<u8> {
    match code {
        ncurses::KEY_UP => Some(b'8'),
        ncurses::KEY_DOWN => Some(b'2'),
        ncurses::KEY_LEFT => Some(b'4'),
        ncurses::KEY_RIGHT => Some(b'6'),
        ncurses::KEY_HOME => Some(KEY_HOME),
        ncurses::KEY_END => Some(KEY_END),
        ncurses::KEY_PPAGE => Some(KEY_PAGE_UP),
        ncurses::KEY_NPAGE => Some(KEY_PAGE_DOWN),
        0..=255 => Some(code as u8),
        _ => None,
    }
}

#[no_mangle]
pub extern "C" fn C_translate_key_code(code: c_int) -> c_int {
    translate_key(code).map_or(-1, |byte| byte as c_int)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrow_keys_map_to_existing_direction_keys() {
        assert_eq!(translate_key(ncurses::KEY_UP), Some(b'8'));
        assert_eq!(translate_key(ncurses::KEY_DOWN), Some(b'2'));
        assert_eq!(translate_key(ncurses::KEY_LEFT), Some(b'4'));
        assert_eq!(translate_key(ncurses::KEY_RIGHT), Some(b'6'));
    }

    #[test]
    fn navigation_keys_have_distinct_named_values() {
        assert_eq!(translate_key(ncurses::KEY_HOME), Some(KEY_HOME));
        assert_eq!(translate_key(ncurses::KEY_END), Some(KEY_END));
        assert_eq!(translate_key(ncurses::KEY_PPAGE), Some(KEY_PAGE_UP));
        assert_eq!(translate_key(ncurses::KEY_NPAGE), Some(KEY_PAGE_DOWN));
    }

    #[test]
    fn ordinary_input_bytes_pass_through_unchanged() {
        for byte in [0, 1, 8, 13, 27, b'0', b'9', b'a', b'Z', 255] {
            assert_eq!(translate_key(byte as c_int), Some(byte as u8));
        }
    }

    #[test]
    fn unknown_curses_codes_are_dropped() {
        assert_eq!(translate_key(ncurses::KEY_RESIZE), None);
        assert_eq!(translate_key(ncurses::KEY_MOUSE), None);
        assert_eq!(translate_key(256), None);
        assert_eq!(translate_key(c_int::MAX), None);
    }

    #[test]
    fn scripted_input_preserves_plain_bytes_and_drops_unknown_codes() {
        let input = [
            27,
            b'3' as c_int,
            b'a' as c_int,
            18,
            ncurses::KEY_RESIZE,
            ncurses::KEY_MOUSE,
            b'Z' as c_int,
        ];
        let output: Vec<_> = input
            .iter()
            .filter_map(|code| translate_key(*code))
            .collect();

        assert_eq!(output, [27, b'3', b'a', 18, b'Z']);
    }
}
