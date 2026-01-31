//! Unsafe wrappers around legacy global state for trap placement.

use crate::constants;
use crate::dungeon::trap::placement::{place_trap_into_lists, TrapList};
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

/// Unsafe wrapper around the legacy global state + allocator (`popt`), mirroring the C `place_trap`.
///
/// This is intentionally `unsafe` because it mutates global state (`cave`, `t_list`).
pub unsafe fn place_trap_global(y: usize, x: usize, list: TrapList, subval: usize) {
    let alloc_index = popt_alloc_index();

    let cave = cave_global();
    let t_list = t_list_global();

    place_trap_into_lists(&mut cave[y][x], &mut t_list[..], alloc_index, list, subval);
}
