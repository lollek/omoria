use super::{toggle_light_source, LightContext};
use crate::{equipment, model::PlayerFlags};
use std::ffi::CString;

extern "C" {
    static mut player_flags: PlayerFlags;
    static mut player_light: u8;
    static mut reset_flag: u8;
    static mut char_row: libc::c_long;
    static mut char_col: libc::c_long;
    fn prt_light_on();
    fn msg_print(message: *const libc::c_char);
    fn dungeon_light_move(
        old_row: libc::c_long,
        old_col: libc::c_long,
        new_row: libc::c_long,
        new_col: libc::c_long,
    );
}

struct GlobalLight;

impl LightContext for GlobalLight {
    fn reset_turn(&mut self) {
        unsafe { reset_flag = 1 }
    }

    fn light_source(&self) -> (i64, i64) {
        let light = unsafe { *equipment::get_item(equipment::Slot::Light) };
        (light.tval.into(), light.p1)
    }

    fn light_on(&self) -> bool {
        unsafe { player_flags.light_on != 0 }
    }

    fn set_light(&mut self, on: bool) {
        unsafe {
            player_flags.light_on = u8::from(on);
            player_light = u8::from(on);
        }
    }

    fn status(&mut self) {
        unsafe { prt_light_on() }
    }

    fn message(&mut self, message: &str) {
        let message = CString::new(message).expect("light messages contain no NUL bytes");
        unsafe { msg_print(message.as_ptr()) }
    }

    fn redraw(&mut self) {
        unsafe { dungeon_light_move(char_row, char_col, char_row, char_col) }
    }
}

pub(super) fn toggle_global() {
    toggle_light_source(&mut GlobalLight);
}
