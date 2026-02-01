//! Trap system.
//!
//! This module contains trap-related data and placement logic ported from `src/traps.c`.
//!
//! ## Ported functionality
//!
//! - **Trap templates**: `TRAP_LIST_A`, `TRAP_LIST_B`, `RUBBLE` (static data)
//! - **Placement**: `place_trap_global`, `change_trap_global`, `place_rubble_global`
//! - **C ABI wrappers**: `place_trap`, `change_trap`, `place_rubble` (in `interop`)
//!
//! ## Not yet ported
//!
//! - Trap effect handlers (`ht__*` functions)
//! - `hit_trap` dispatcher
//! - `trigger_trap` (chest traps)
//! - Town entrance logic (cases 101-123)

pub mod data;
mod globals;
mod placement;

// Export C ABI wrappers for the parts of traps.c we've already ported.
pub mod interop;

#[cfg(test)]
pub(crate) mod test_support;

pub use globals::place_trap_global;
pub use globals::change_trap_global;
pub use globals::place_rubble_global;
pub use placement::{place_trap_into_lists, TrapList};

#[cfg(test)]
mod data_tests;
