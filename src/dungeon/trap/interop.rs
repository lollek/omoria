//! C ABI wrapper functions for trap-related operations.
//!
//! These exports let remaining C callers reach the Rust trap implementation. Their
//! prototypes live in `src/traps.h`.

use crate::dungeon::trap::{change_trap_global, place_rubble_global, place_trap_global, TrapList};

/// Activates the trap at the supplied coordinates.
///
/// # Safety
/// Non-null pointers must be aligned and readable for one C long during the call.
/// C globals must be initialized and exclusively accessed on the game thread.
#[cfg(not(test))]
#[no_mangle]
pub unsafe extern "C" fn hit_trap(y: *const libc::c_long, x: *const libc::c_long) {
    super::activation::hit_trap_global(y, x);
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn trigger_trap(y: libc::c_long, x: libc::c_long) {
    unsafe { super::chest::trigger_trap_global(y, x) }
}

#[no_mangle]
pub extern "C" fn place_trap(
    y: libc::c_long,
    x: libc::c_long,
    typ: libc::c_long,
    subval: libc::c_long,
) {
    let list = if typ == 1 { TrapList::A } else { TrapList::B };
    // Mirrors the C behavior: this mutates global dungeon state.
    unsafe {
        place_trap_global(y as usize, x as usize, list, subval as usize);
    }
}

#[no_mangle]
pub extern "C" fn change_trap(y: libc::c_long, x: libc::c_long) {
    unsafe {
        change_trap_global(y as usize, x as usize);
    }
}

#[no_mangle]
pub extern "C" fn place_rubble(y: libc::c_long, x: libc::c_long) {
    unsafe {
        place_rubble_global(y as usize, x as usize);
    }
}
