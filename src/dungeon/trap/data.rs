//! Trap templates.
//!
//! Static trap definitions ported from `src/traps.c`.
//!
//! ## Indexing
//!
//! The C code uses 1-based indices for trap subvals (`randint(MAX_TRAPA)` returns 1..N).
//! The Rust arrays here are 0-based and contain only real traps (no bogus padding).
//! The `template_for` function in `placement.rs` handles the index translation.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrapTemplate {
    pub name: &'static str,
    pub tval: i64,
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
    tval: TVAL_RUBBLE,
    level: 0,
    subval: 1,
    damage: "0d0",
    cost: 0,
};

/// Trap templates from `traps.c` (`trap_lista`), without the bogus padding entry.
///
/// Length is `MAX_TRAPA` (19). C code passes 1-based subval; use `subval - 1` to index.
pub const TRAP_LIST_A: &[TrapTemplate] = &[
    TrapTemplate { name: "an open pit", tval: TVAL_SEEN_TRAP, level: 0, subval: 1, damage: "2d6", cost: -50 },
    TrapTemplate { name: "an arrow trap", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 2, damage: "1d8", cost: 0 },
    TrapTemplate { name: "a covered pit", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 3, damage: "2d6", cost: 0 },
    TrapTemplate { name: "a trap door", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 4, damage: "2d8", cost: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 5, damage: "1d4", cost: 0 },
    TrapTemplate { name: "a loose rock", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 6, damage: "0d0", cost: 0 },
    TrapTemplate { name: "a dart trap", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 7, damage: "1d4", cost: 0 },
    TrapTemplate { name: "a strange rune", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 8, damage: "0d0", cost: 0 },
    TrapTemplate { name: "some loose rock", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 9, damage: "2d6", cost: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 10, damage: "1d4", cost: 0 },
    TrapTemplate { name: "a strange rune", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 11, damage: "0d0", cost: 0 },
    TrapTemplate { name: "a blackened spot", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 12, damage: "4d6", cost: 0 },
    TrapTemplate { name: "some corroded rock", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 13, damage: "4d6", cost: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_UNSEEN_TRAP, level: 0, subval: 14, damage: "2d6", cost: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_UNSEEN_TRAP, level: 5, subval: 15, damage: "1d4", cost: 10 },
    TrapTemplate { name: "a gas trap", tval: TVAL_UNSEEN_TRAP, level: 5, subval: 16, damage: "1d8", cost: 5 },
    TrapTemplate { name: "a dart trap", tval: TVAL_UNSEEN_TRAP, level: 5, subval: 17, damage: "1d8", cost: 10 },
    TrapTemplate { name: "a dart trap", tval: TVAL_UNSEEN_TRAP, level: 5, subval: 18, damage: "1d8", cost: 10 },
    TrapTemplate { name: "a chute", tval: TVAL_UNSEEN_TRAP, level: 5, subval: 20, damage: "4d8", cost: 20 },
];

/// Trap templates from `traps.c` (`trap_listb`), without the bogus padding entry.
///
/// Length is `MAX_TRAPB` (20). C code passes 1-based subval; use `subval - 1` to index.
///
/// For list B traps:
/// - `level` represents the difficulty of disarming
/// - `cost` represents experience gained when disarmed (can be negative)
pub const TRAP_LIST_B: &[TrapTemplate] = &[
    TrapTemplate { name: "an open pit", tval: TVAL_SEEN_TRAP, level: 1, subval: 1, damage: "2d6", cost: -50 },
    TrapTemplate { name: "an arrow trap", tval: TVAL_SEEN_TRAP, level: 3, subval: 2, damage: "1d8", cost: -10 },
    TrapTemplate { name: "a covered pit", tval: TVAL_SEEN_TRAP, level: 2, subval: 3, damage: "2d6", cost: -40 },
    TrapTemplate { name: "a trap door", tval: TVAL_SEEN_TRAP, level: 5, subval: 4, damage: "2d8", cost: -25 },
    TrapTemplate { name: "a gas trap", tval: TVAL_SEEN_TRAP, level: 3, subval: 5, damage: "1d4", cost: 5 },
    TrapTemplate { name: "a loose rock", tval: TVAL_SEEN_TRAP, level: 0, subval: 6, damage: "0d0", cost: -90 },
    TrapTemplate { name: "a dart trap", tval: TVAL_SEEN_TRAP, level: 5, subval: 7, damage: "1d4", cost: 10 },
    TrapTemplate { name: "a strange rune", tval: TVAL_SEEN_TRAP, level: 5, subval: 8, damage: "0d0", cost: -10 },
    TrapTemplate { name: "some loose rock", tval: TVAL_SEEN_TRAP, level: 5, subval: 9, damage: "2d6", cost: -10 },
    TrapTemplate { name: "a gas trap", tval: TVAL_SEEN_TRAP, level: 10, subval: 10, damage: "1d4", cost: 5 },
    TrapTemplate { name: "a strange rune", tval: TVAL_SEEN_TRAP, level: 5, subval: 11, damage: "0d0", cost: -10 },
    TrapTemplate { name: "a blackened spot", tval: TVAL_SEEN_TRAP, level: 10, subval: 12, damage: "4d6", cost: 10 },
    TrapTemplate { name: "some corroded rock", tval: TVAL_SEEN_TRAP, level: 10, subval: 13, damage: "4d6", cost: 10 },
    TrapTemplate { name: "a gas trap", tval: TVAL_SEEN_TRAP, level: 5, subval: 14, damage: "2d6", cost: 5 },
    TrapTemplate { name: "a gas trap", tval: TVAL_SEEN_TRAP, level: 5, subval: 15, damage: "1d4", cost: 10 },
    TrapTemplate { name: "a gas trap", tval: TVAL_SEEN_TRAP, level: 5, subval: 16, damage: "1d8", cost: 5 },
    TrapTemplate { name: "a dart trap", tval: TVAL_SEEN_TRAP, level: 5, subval: 17, damage: "1d8", cost: 10 },
    TrapTemplate { name: "a dart trap", tval: TVAL_SEEN_TRAP, level: 5, subval: 18, damage: "1d8", cost: 10 },
    TrapTemplate { name: "a closed door", tval: TVAL_CLOSED_DOOR, level: 0, subval: 19, damage: "1d1", cost: 0 },
    TrapTemplate { name: "a chute", tval: TVAL_SEEN_TRAP, level: 5, subval: 20, damage: "4d8", cost: 20 },
];
