//! Unsafe wrappers around legacy global state for trap placement.

use crate::constants;
use crate::dungeon::trap::data;
use crate::dungeon::trap::placement::{apply_template_to_item, place_trap_into_lists, TrapList};
use crate::model::{Cave, Item};

#[cfg(not(test))]
use libc;

#[cfg(not(test))]
unsafe fn cave_global() -> &'static mut [[Cave; constants::MAX_WIDTH + 1]; constants::MAX_HEIGHT + 1] {
    extern "C" {
        static mut cave: [[Cave; constants::MAX_WIDTH + 1]; constants::MAX_HEIGHT + 1];
    }
    &mut *(&raw mut cave)
}

#[cfg(not(test))]
unsafe fn t_list_global() -> &'static mut [Item; constants::MAX_TALLOC + 1] {
    extern "C" {
        static mut t_list: [Item; constants::MAX_TALLOC + 1];
    }
    &mut *(&raw mut t_list)
}

#[cfg(not(test))]
unsafe fn popt_alloc_index() -> u8 {
    extern "C" {
        #[link_name = "popt"]
        fn C_popt(index: *mut libc::c_long);
    }
    let mut index: libc::c_long = 0;
    C_popt(&mut index);
    index as u8
}

// --- test hooks ---

#[cfg(test)]
pub(crate) static mut TEST_ALLOC_INDEX: u8 = 0;

#[cfg(test)]
pub(crate) static mut TEST_PUSHT_CALLED: usize = 0;

#[cfg(test)]
pub(crate) static mut TEST_LITE_SPOT_CALLED: usize = 0;

#[cfg(test)]
pub(crate) static mut TEST_LAST_PUSHT_INDEX: u8 = 0;

#[cfg(test)]
pub(crate) static mut TEST_LAST_LITE_SPOT_YX: (usize, usize) = (0, 0);

#[cfg(test)]
pub(crate) static mut TEST_CAVE: [[Cave; constants::MAX_WIDTH + 1]; constants::MAX_HEIGHT + 1] =
    [[Cave { cptr: 0, tptr: 0, fval: 0, fopen: 0, fm: 0, pl: 0, tl: 0, moved: 0, oct: 0, h2o: 0 };
        constants::MAX_WIDTH + 1];
        constants::MAX_HEIGHT + 1];

#[cfg(test)]
pub(crate) static mut TEST_T_LIST: [Item; constants::MAX_TALLOC + 1] = [Item {
    name: [0; 70],
    tval: 0,
    flags2: 0,
    flags: 0,
    p1: 0,
    cost: 0,
    subval: 0,
    weight: 0,
    number: 1,
    tohit: 0,
    todam: 0,
    ac: 0,
    toac: 0,
    damage: [0; 7],
    level: 0,
    identified: 0,
};
    constants::MAX_TALLOC + 1];

#[cfg(test)]
unsafe fn cave_global() -> &'static mut [[Cave; constants::MAX_WIDTH + 1]; constants::MAX_HEIGHT + 1] {
    &mut *(&raw mut TEST_CAVE)
}

#[cfg(test)]
unsafe fn t_list_global() -> &'static mut [Item; constants::MAX_TALLOC + 1] {
    &mut *(&raw mut TEST_T_LIST)
}

#[cfg(test)]
unsafe fn popt_alloc_index() -> u8 {
    TEST_ALLOC_INDEX
}

#[cfg(test)]
unsafe fn pusht(index: u8) {
    TEST_PUSHT_CALLED += 1;
    TEST_LAST_PUSHT_INDEX = index;

    // Mirror the important observable part of C `pusht`: the freed slot becomes blank.
    // (We don't model the actual free-list pointer chain in Rust tests yet.)
    let slot = &mut TEST_T_LIST[index as usize];
    slot.tval = 0;
    slot.subval = 0;
}

#[cfg(test)]
unsafe fn lite_spot(y: usize, x: usize) {
    TEST_LITE_SPOT_CALLED += 1;
    TEST_LAST_LITE_SPOT_YX = (y, x);
}

#[cfg(not(test))]
unsafe fn pusht(index: u8) {
    extern "C" {
        #[link_name = "pusht"]
        fn C_pusht(index: libc::c_long);
    }
    C_pusht(index as libc::c_long);
}

#[cfg(not(test))]
unsafe fn lite_spot(y: usize, x: usize) {
    extern "C" {
        #[link_name = "lite_spot"]
        fn C_lite_spot(y: libc::c_long, x: libc::c_long);
    }
    C_lite_spot(y as libc::c_long, x as libc::c_long);
}

/// Unsafe wrapper around the legacy global state + allocator (`popt`), mirroring the C `place_trap`.
///
/// This is intentionally `unsafe` because it mutates global state (`cave`, `t_list`).
pub unsafe fn place_trap_global(y: usize, x: usize, list: TrapList, subval: usize) {
    let alloc_index = popt_alloc_index();

    let cave = cave_global();
    let t_list = t_list_global();
    place_trap_into_lists(&mut cave[y][x], &mut t_list[..], alloc_index, list, subval);
}

/// Unsafe wrapper mirroring the C `change_trap` behavior.
///
/// Reveals a hidden trap or secret door by replacing it with its visible counterpart.
///
/// Does nothing if:
/// - the tile has no object (`tptr == 0`)
/// - the object's `tval` is not `unseen_trap` or `secret_door`
///
/// Otherwise:
/// - places the corresponding `TRAP_LIST_B` entry at `(y, x)` using the same `subval`
/// - pushes the old t_list index back on the free list (`pusht`)
/// - refreshes the tile display (`lite_spot`)
pub unsafe fn change_trap_global(y: usize, x: usize) {
    let cave = cave_global();
    let t_list = t_list_global();

    let old_index = cave[y][x].tptr;
    if old_index == 0 {
        return;
    }

    let old_item = &t_list[old_index as usize];
    let tval = old_item.tval as i64;

    if tval != data::TVAL_UNSEEN_TRAP
        && tval != data::TVAL_SECRET_DOOR
    {
        return;
    }

    let subval = old_item.subval as usize;

    place_trap_global(y, x, TrapList::B, subval);
    pusht(old_index);
    lite_spot(y, x);
}

/// Unsafe wrapper mirroring the C `place_rubble` behavior.
pub unsafe fn place_rubble_global(y: usize, x: usize) {
    let alloc_index = popt_alloc_index();

    let cave = cave_global();
    let t_list = t_list_global();

    cave[y][x].tptr = alloc_index;
    apply_template_to_item(&mut t_list[alloc_index as usize], &data::RUBBLE);

    // Rubble blocks the tile.
    cave[y][x].fopen = 0;
}
