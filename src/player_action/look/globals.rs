use super::{look, LookContext};
use crate::{
    dungeon::cell::with_global_cells,
    model::{m_list, Cave, Item, PlayerFlags},
};

struct GlobalLook;

impl LookContext for GlobalLook {
    fn blind(&mut self) -> bool {
        extern "C" {
            static player_flags: PlayerFlags;
        }
        unsafe { player_flags.blind >= 1 }
    }

    fn step(&mut self, direction: i64, y: &mut i64, x: &mut i64) {
        extern "C" {
            fn move_dir(direction: libc::c_int, y: *mut libc::c_long, x: *mut libc::c_long)
                -> bool;
        }
        unsafe { move_dir(direction as libc::c_int, y, x) };
    }

    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
        unsafe {
            with_global_cells(|cells| {
                if y < 1 || y > cells.height || x < 1 || x > cells.width {
                    return None;
                }
                let cell = *cells.cave.get(y as usize)?.get(x as usize)?;
                let item = (cell.tptr != 0)
                    .then(|| cells.items.get(cell.tptr as usize).copied())
                    .flatten();
                Some((cell, item))
            })
        }
    }

    fn monster_name(&mut self, index: u8) -> Option<String> {
        extern "C" {
            fn monster_template_get_name(index: libc::c_long) -> *const libc::c_char;
        }
        let monster = unsafe { (*std::ptr::addr_of!(m_list)).get(index as usize).copied()? };
        if monster.is_seen == 0 {
            return None;
        }
        let name = unsafe { monster_template_get_name(monster.mptr.into()) };
        Some(
            unsafe { std::ffi::CStr::from_ptr(name) }
                .to_string_lossy()
                .into_owned(),
        )
    }

    fn item_name(&mut self, item: Item) -> String {
        extern "C" {
            fn item_name(out: *mut [libc::c_char; 82], item: *const Item);
        }
        let mut name = [0; 82];
        unsafe { item_name(&mut name, &item) };
        crate::misc::c_string_lossy(&name)
    }

    fn message(&mut self, message: &str) {
        crate::term::msg_print(message);
    }
}

/// Uses initialized C globals on the exclusive game thread.
pub(super) fn look_global(direction: i64) {
    extern "C" {
        static char_row: libc::c_long;
        static char_col: libc::c_long;
    }
    let (row, col) = unsafe { (char_row, char_col) };
    look(&mut GlobalLook, direction, row, col);
}
