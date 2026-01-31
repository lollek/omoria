//! Trap templates.
//!
//! Phase 2.1: port the static trap definitions from `src/traps.c`.
//!
//! Notes on legacy indexing:
//! - C defines these arrays as `MAX_TRAP* + 1` and keeps a "bogus" element at index 0.
//! - Many call sites use 1-based random selection (e.g. `randint(MAX_TRAPA)`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrapTemplate {
    pub name: &'static str,
    pub tval: i64,
    pub flags: u32,
    pub flags2: u32,
    pub level: i64,
    pub weight: i64,
    pub subval: i64,
    pub number: i64,
    pub tohit: i64,
    pub todam: i64,
    pub ac: i64,
    pub toac: i64,
    pub p1: i64,
    pub damage: &'static str,
    pub cost: i64,
    pub special: i64,
}

/// Legacy `tval` values from `src/constants.h`.
///
/// We keep these as integers for now to keep this module independent of C header bindings.
pub const TVAL_UNSEEN_TRAP: i64 = 101;
pub const TVAL_SEEN_TRAP: i64 = 102;
pub const TVAL_RUBBLE: i64 = 103;
pub const TVAL_CLOSED_DOOR: i64 = 105;

/// Standalone rubble template from `traps.c` (`some_rubble`).
pub const RUBBLE: TrapTemplate = TrapTemplate {
    name: "some rubble",
    tval: TVAL_RUBBLE,
    flags: 0x00000000,
    flags2: 0x00000000,
    level: 0,
    weight: 0,
    subval: 1,
    number: 0,
    tohit: 0,
    todam: 0,
    ac: 0,
    toac: 0,
    p1: 0,
    damage: "0d0",
    cost: 0,
    special: 0,
};

/// Trap templates from `traps.c` (`trap_lista`).
///
/// Length is `MAX_TRAPA + 1` (20). Index 0 is a bogus padding entry.
pub const TRAP_LIST_A: &[TrapTemplate] = &[
    TrapTemplate { name: "bogus trap a", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 1, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d6", cost: -50, special: 0 },
    TrapTemplate { name: "an open pit", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 1, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d6", cost: -50, special: 0 },
    TrapTemplate { name: "an arrow trap", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 2, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d8", cost: 0, special: 0 },
    TrapTemplate { name: "a covered pit", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 3, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d6", cost: 0, special: 0 },
    TrapTemplate { name: "a trap door", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 4, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d8", cost: 0, special: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 5, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d4", cost: 0, special: 0 },
    TrapTemplate { name: "a loose rock", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 6, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "0d0", cost: 0, special: 0 },
    TrapTemplate { name: "a dart trap", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 7, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d4", cost: 0, special: 0 },
    TrapTemplate { name: "a strange rune", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 8, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "0d0", cost: 0, special: 0 },
    TrapTemplate { name: "some loose rock", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 9, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d6", cost: 0, special: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 10, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d4", cost: 0, special: 0 },
    TrapTemplate { name: "a strange rune", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 11, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "0d0", cost: 0, special: 0 },
    TrapTemplate { name: "a blackened spot", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 12, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "4d6", cost: 0, special: 0 },
    TrapTemplate { name: "some corroded rock", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 13, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "4d6", cost: 0, special: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 14, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d6", cost: 0, special: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 15, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d4", cost: 10, special: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 16, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d8", cost: 5, special: 0 },
    TrapTemplate { name: "a dart trap", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 17, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d8", cost: 10, special: 0 },
    TrapTemplate { name: "a dart trap", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 18, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d8", cost: 10, special: 0 },
    TrapTemplate { name: "a chute", tval: TVAL_UNSEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 20, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "4d8", cost: 20, special: 0 },
];

/// Trap templates from `traps.c` (`trap_listb`).
///
/// Length is `MAX_TRAPB + 1` (21). Index 0 is a bogus padding entry.
pub const TRAP_LIST_B: &[TrapTemplate] = &[
    TrapTemplate { name: "bogus trap b", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 1, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d6", cost: -50, special: 0 },
    TrapTemplate { name: "an open pit", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 1, weight: 0, subval: 1, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d6", cost: -50, special: 0 },
    TrapTemplate { name: "an arrow trap", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 3, weight: 0, subval: 2, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d8", cost: -10, special: 0 },
    TrapTemplate { name: "a covered pit", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 2, weight: 0, subval: 3, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d6", cost: -40, special: 0 },
    TrapTemplate { name: "a trap door", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 4, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d8", cost: -25, special: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 3, weight: 0, subval: 5, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d4", cost: 5, special: 0 },
    TrapTemplate { name: "a loose rock", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 6, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "0d0", cost: -90, special: 0 },
    TrapTemplate { name: "a dart trap", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 7, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d4", cost: 10, special: 0 },
    TrapTemplate { name: "a strange rune", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 8, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "0d0", cost: -10, special: 0 },
    TrapTemplate { name: "some loose rock", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 9, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d6", cost: -10, special: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 10, weight: 0, subval: 10, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d4", cost: 5, special: 0 },
    TrapTemplate { name: "a strange rune", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 11, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "0d0", cost: -10, special: 0 },
    TrapTemplate { name: "a blackened spot", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 10, weight: 0, subval: 12, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "4d6", cost: 10, special: 0 },
    TrapTemplate { name: "some corroded rock", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 10, weight: 0, subval: 13, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "4d6", cost: 10, special: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 14, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "2d6", cost: 5, special: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 15, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d4", cost: 10, special: 0 },
    TrapTemplate { name: "a gas trap", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 16, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d8", cost: 5, special: 0 },
    TrapTemplate { name: "a dart trap", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 17, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d8", cost: 10, special: 0 },
    TrapTemplate { name: "a dart trap", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 18, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d8", cost: 10, special: 0 },
    TrapTemplate { name: "a closed door", tval: TVAL_CLOSED_DOOR, flags: 0x00000000, flags2: 0x00000000, level: 0, weight: 0, subval: 19, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "1d1", cost: 0, special: 0 },
    TrapTemplate { name: "a chute", tval: TVAL_SEEN_TRAP, flags: 0x00000000, flags2: 0x00000000, level: 5, weight: 0, subval: 20, number: 0, tohit: 0, todam: 0, ac: 0, toac: 0, p1: 0, damage: "4d8", cost: 20, special: 0 },
];

/// Trap templates excluding the legacy bogus entry at index 0.
///
/// This is usually what you want when iterating over real traps.
pub fn trap_list_a_real() -> &'static [TrapTemplate] {
    &TRAP_LIST_A[1..]
}

/// Trap templates excluding the legacy bogus entry at index 0.
///
/// This is usually what you want when iterating over real traps / features.
pub fn trap_list_b_real() -> &'static [TrapTemplate] {
    &TRAP_LIST_B[1..]
}
