//! Trap templates.
//!
//! Static trap definitions ported from `src/traps.c`.
//!
//! ## Indexing
//!
//! The C code uses 1-based indices for trap subvals (`randint(MAX_TRAPA)` returns 1..N).
//! The Rust array here is 0-based; `template_for` in `placement.rs` handles the translation.
//!
//! ## List A vs List B
//!
//! The C code has two nearly-identical trap arrays (`trap_lista` and `trap_listb`).
//! The only meaningful difference is `tval`:
//! - List A traps are **unseen** (hidden until triggered)
//! - List B traps are **seen** (visible, e.g. after being triggered or detected)
//!
//! We store only one list (`TRAP_LIST`) with the "seen" values, and `apply_template_to_item`
//! sets `tval` based on which list type is requested.
//!
//! ## Simplified template struct
//!
//! The original C `treasure_type` has many fields, but for traps most are always zero.
//! `TrapTemplate` only stores fields that actually vary; `apply_template_to_item` sets
//! the constant fields (flags, weight, tohit, etc.) to zero when copying.

/// Trap template with only the fields that vary across trap definitions.
///
/// When applied to an `Item`, the omitted fields (flags, flags2, weight, number,
/// tohit, todam, ac, toac, p1, special) are set to zero.
///
/// Note: `tval` is not stored here because it depends on whether the trap is
/// placed as "unseen" (list A) or "seen" (list B).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrapTemplate {
    pub name: &'static str,
    pub level: i64,
    pub subval: i64,
    pub damage: &'static str,
    pub cost: i64,
}

/// Legacy `tval` values from `src/constants.h`.
///
/// We keep these as integers for now to keep this module independent of C header bindings.
pub const TVAL_UNSEEN_TRAP: i64 = 101;
pub const TVAL_SEEN_TRAP: i64 = 102;
pub const TVAL_RUBBLE: i64 = 103;
pub const TVAL_CLOSED_DOOR: i64 = 105;
pub const TVAL_SECRET_DOOR: i64 = 109;

/// Standalone rubble template from `traps.c` (`some_rubble`).
pub const RUBBLE: TrapTemplate = TrapTemplate {
    name: "some rubble",
    level: 0,
    subval: 1,
    damage: "0d0",
    cost: 0,
};

/// Unified trap template list, combining data from C's `trap_lista` and `trap_listb`.
///
/// Length is 20 (MAX_TRAPB). C code passes 1-based subval; use `subval - 1` to index.
///
/// The `tval` field is NOT stored here - it's set at placement time:
/// - Open pit (subval 1): always `TVAL_SEEN_TRAP` (always visible)
/// - Closed door (subval 19): always `TVAL_CLOSED_DOOR`
/// - List A (unseen): `TVAL_UNSEEN_TRAP`
/// - List B (seen): `TVAL_SEEN_TRAP`
///
/// Field meanings:
/// - `level`: difficulty of disarming (used when trap is visible)
/// - `cost`: experience gained when disarmed (can be negative)
pub const TRAP_LIST: &[TrapTemplate] = &[
    TrapTemplate { name: "an open pit", level: 1, subval: 1, damage: "2d6", cost: -50 },
    TrapTemplate { name: "an arrow trap", level: 3, subval: 2, damage: "1d8", cost: -10 },
    TrapTemplate { name: "a covered pit", level: 2, subval: 3, damage: "2d6", cost: -40 },
    TrapTemplate { name: "a trap door", level: 5, subval: 4, damage: "2d8", cost: -25 },
    TrapTemplate { name: "a gas trap", level: 3, subval: 5, damage: "1d4", cost: 5 },
    TrapTemplate { name: "a loose rock", level: 0, subval: 6, damage: "0d0", cost: -90 },
    TrapTemplate { name: "a dart trap", level: 5, subval: 7, damage: "1d4", cost: 10 },
    TrapTemplate { name: "a strange rune", level: 5, subval: 8, damage: "0d0", cost: -10 },
    TrapTemplate { name: "some loose rock", level: 5, subval: 9, damage: "2d6", cost: -10 },
    TrapTemplate { name: "a gas trap", level: 10, subval: 10, damage: "1d4", cost: 5 },
    TrapTemplate { name: "a strange rune", level: 5, subval: 11, damage: "0d0", cost: -10 },
    TrapTemplate { name: "a blackened spot", level: 10, subval: 12, damage: "4d6", cost: 10 },
    TrapTemplate { name: "some corroded rock", level: 10, subval: 13, damage: "4d6", cost: 10 },
    TrapTemplate { name: "a gas trap", level: 5, subval: 14, damage: "2d6", cost: 5 },
    TrapTemplate { name: "a gas trap", level: 5, subval: 15, damage: "1d4", cost: 10 },
    TrapTemplate { name: "a gas trap", level: 5, subval: 16, damage: "1d8", cost: 5 },
    TrapTemplate { name: "a dart trap", level: 5, subval: 17, damage: "1d8", cost: 10 },
    TrapTemplate { name: "a dart trap", level: 5, subval: 18, damage: "1d8", cost: 10 },
    TrapTemplate { name: "a closed door", level: 0, subval: 19, damage: "1d1", cost: 0 },
    TrapTemplate { name: "a chute", level: 5, subval: 20, damage: "4d8", cost: 20 },
];

/// Subval for the open pit entry (always visible, uses TVAL_SEEN_TRAP).
pub const SUBVAL_OPEN_PIT: i64 = 1;

/// Subval for the closed door entry (special case with different tval).
pub const SUBVAL_CLOSED_DOOR: i64 = 19;

