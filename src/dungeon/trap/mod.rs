//! Trap system (WIP).

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
