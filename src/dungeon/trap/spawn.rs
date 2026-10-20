//! Spawning and rockfall trap effects.

use crate::{constants, model::Cave};
use std::ffi::CString;

extern "C" {
    static mut cave: [[Cave; constants::MAX_WIDTH + 1]; constants::MAX_HEIGHT + 1];
    static mut char_row: libc::c_long;
    static mut char_col: libc::c_long;
    static water_set: [u8; 25];

    #[link_name = "pusht"]
    fn c_pusht(index: libc::c_long);
    fn place_random_dungeon_item(y: libc::c_long, x: libc::c_long);
    fn randint(max: libc::c_long) -> libc::c_long;
    fn summon_land_monster(y: *mut i64, x: *mut i64, asleep: bool) -> bool;
    fn summon_water_monster(y: *mut i64, x: *mut i64, asleep: bool) -> bool;
}

fn clear_field_mark_and_read_object(y: i64, x: i64) -> u8 {
    unsafe {
        let cave_cells = &mut *std::ptr::addr_of_mut!(cave);
        cave_cells[y as usize][x as usize].fm = 0;
        cave_cells[y as usize][x as usize].tptr
    }
}

pub(super) fn hidden_object(y: i64, x: i64) {
    let object_index = clear_field_mark_and_read_object(y, x);
    unsafe {
        c_pusht(object_index as libc::c_long);
        place_random_dungeon_item(y as libc::c_long, x as libc::c_long);
    }
    crate::term::msg_print("Hmmm, there was something under this rock.");
}

pub(super) fn rockfall(damage: i64, y: i64, x: i64) {
    let cause = CString::new("falling rock").expect("static trap cause has no NUL bytes");
    unsafe {
        crate::player::damage::take_hit(damage as libc::c_long, cause.as_ptr());
    }

    let object_index = unsafe { (&*std::ptr::addr_of!(cave))[y as usize][x as usize].tptr };
    unsafe {
        c_pusht(object_index as libc::c_long);
    }
    unsafe {
        crate::dungeon::trap::place_rubble_global(y as usize, x as usize);
    }
    crate::term::msg_print("You are hit by falling rock");
}

pub(super) fn summon_monster(_y: i64, _x: i64) {
    let object_index = clear_field_mark_and_read_object(_y, _x);
    unsafe {
        c_pusht(object_index as libc::c_long);
        (&mut *std::ptr::addr_of_mut!(cave))[_y as usize][_x as usize].tptr = 0;
    }

    let mut attempt = 1;
    loop {
        if attempt > 2 + unsafe { randint(3) } {
            break;
        }
        attempt += 1;

        let mut monster_y = unsafe { char_row };
        let mut monster_x = unsafe { char_col };
        let floor =
            unsafe { (&*std::ptr::addr_of!(cave))[monster_y as usize][monster_x as usize].fval };
        let is_water = unsafe {
            crate::pascal::is_in(
                floor as libc::c_long,
                std::ptr::addr_of!(water_set).cast::<u8>(),
            )
        };
        unsafe {
            if is_water {
                summon_water_monster(&mut monster_y, &mut monster_x, false);
            } else {
                summon_land_monster(&mut monster_y, &mut monster_x, false);
            }
        }
    }
}
