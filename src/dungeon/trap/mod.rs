//! Trap system.
//!
//! Trap data, placement, and activation dispatch ported from `src/traps.c`.
//!
//! ## Ported functionality
//!
//! - **Trap templates**: `TRAP_LIST_A`, `TRAP_LIST_B`, `RUBBLE` (static data)
//! - **Placement**: `place_trap_global`, `change_trap_global`, `place_rubble_global`
//! - **Activation**: `hit_trap`, open pits, arrows, and town entrance routing
//! - **C ABI wrappers**: placement and `hit_trap` (in `interop`)
//!
//! ## Not yet ported
//!
//! - Other trap effects (forwarded to C by the per-group handler modules)
//! - `trigger_trap` (chest traps)
//! - Town entrance implementations

#[cfg(not(test))]
mod activation;
mod basic;
pub mod data;
#[cfg(not(test))]
mod element;
mod globals;
#[cfg(not(test))]
mod movement;
mod placement;
#[cfg(not(test))]
mod spawn;
#[cfg(not(test))]
mod status;

// Export C ABI wrappers for the parts of traps.c we've already ported.
pub mod interop;

#[cfg(test)]
pub(crate) mod test_support;

pub use globals::change_trap_global;
pub use globals::place_rubble_global;
pub use globals::place_trap_global;
pub use placement::{place_trap_into_lists, TrapList};

#[cfg(test)]
mod data_tests;
