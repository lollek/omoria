use crate::model::PlayerFlags;
use std::ffi::CStr;

const IS_RESTING: u64 = 0x00000200;

pub(super) unsafe fn rest_global(input: *const libc::c_char) -> bool {
    extern "C" {
        static mut player_flags: PlayerFlags;
        static mut search_flag: bool;
        static mut reset_flag: bool;
        static mut turn_counter: libc::c_long;
        fn search_off();
    }

    let request = super::parse_rest(unsafe { CStr::from_ptr(input) }.to_bytes());
    unsafe {
        if request.until_full {
            player_flags.resting_till_full = 1;
        }
        if request.turns > 0 {
            if search_flag {
                search_off();
            }
            player_flags.rest = request.turns;
            turn_counter += request.turns;
            player_flags.status |= IS_RESTING;
            true
        } else {
            reset_flag = true;
            false
        }
    }
}
