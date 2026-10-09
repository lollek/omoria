use super::{take_stairs, Direction, StairsContext};
use crate::dungeon::cell::with_global_cells;

extern "C" {
    static mut char_row: libc::c_long;
    static mut char_col: libc::c_long;
    static mut dun_level: libc::c_long;
    static mut moria_flag: bool;
}

struct GlobalStairs;

impl StairsContext for GlobalStairs {
    fn tile_tval(&mut self) -> Option<u8> {
        unsafe {
            let (y, x) = (char_row, char_col);
            with_global_cells(|cells| cells.read(y, x))
                .and_then(|(_, item)| item)
                .map(|item| item.tval)
        }
    }

    fn dun_level(&mut self) -> i64 {
        unsafe { dun_level }
    }

    fn set_dun_level(&mut self, level: i64) {
        unsafe { dun_level = level }
    }

    fn set_moria_flag(&mut self) {
        unsafe { moria_flag = true }
    }

    fn message(&mut self, message: &str) {
        crate::term::msg_print(message);
    }
}

pub(super) fn take_stairs_global(direction: Direction) {
    take_stairs(&mut GlobalStairs, direction, || crate::rng::randint(3));
}
