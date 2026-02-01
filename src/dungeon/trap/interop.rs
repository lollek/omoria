//! C ABI wrapper functions for trap-related operations.
//!
//! These functions exist to let the C code call into the Rust implementation during the
//! migration, so we can delete the corresponding implementations from `src/traps.c`.

use crate::dungeon::trap::{change_trap_global, place_rubble_global, place_trap_global, TrapList};

#[no_mangle]
pub extern "C" fn place_trap(y: libc::c_long, x: libc::c_long, typ: libc::c_long, subval: libc::c_long) {
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
