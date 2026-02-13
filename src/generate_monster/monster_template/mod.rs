//! Monster template system.
//!
//! This module contains monster template data and attribute query logic
//! ported from `src/generate_monster/monster_template.c`.
//!
//! ## Ported functionality
//!
//! - **MonsterAttribute enum**: All monster attribute flags
//! - **MonsterTemplate struct**: Static monster definition (repr(C) compatible)
//! - **has_attribute()**: Query attribute from bit fields
//! - **MONSTER_TEMPLATES**: Static array of all monster definitions
//!
//! ## C Interop
//!
//! The `interop` module provides C ABI wrappers so existing C code can
//! access the Rust data and functions.

mod attribute;
mod template;

pub use attribute::MonsterAttribute;
pub use template::MonsterTemplate;

#[cfg(test)]
mod tests;
