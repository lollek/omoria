#[cfg(test)]
mod tests {
    use crate::dungeon::trap::data::{TRAP_LIST_A, TRAP_LIST_B, TVAL_RUBBLE};

    /// Phase 2.1 (RED): pin down basic expectations from `src/traps.c`.
    ///
    /// These counts should match the legacy arrays:
    /// - `trap_lista[MAX_TRAPA + 1]` where `MAX_TRAPA` is 19 → 20 entries (index 0 is "bogus").
    /// - `trap_listb[MAX_TRAPB + 1]` where `MAX_TRAPB` is 20 → 21 entries (index 0 is "bogus").
    ///
    /// This keeps our Rust port honest about legacy indexing.
    #[test]
    fn trap_template_lists_have_legacy_counts() {
        // MAX_TRAPA=19, MAX_TRAPB=20 in src/constants.h
        assert_eq!(TRAP_LIST_A.len(), 19 + 1);
        assert_eq!(TRAP_LIST_B.len(), 20 + 1);
    }

    /// Phase 2.1 (next step, RED): port the standalone `some_rubble` template.
    ///
    /// In C (traps.c):
    /// ```c
    /// static treasure_type some_rubble = {
    ///     "some rubble", rubble, 0x00000000, 0x00000000, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ///     "0d0",         0,      0};
    /// ```
    ///
    /// This test should fail until `RUBBLE` exists in `dungeon::trap::data`.
    #[test]
    fn rubble_template_matches_legacy_definition() {
        let rubble = crate::dungeon::trap::data::RUBBLE;
        assert_eq!(rubble.name, "some rubble");
        assert_eq!(rubble.tval, TVAL_RUBBLE);
        assert_eq!(rubble.damage, "0d0");
        assert_eq!(rubble.cost, 0);
    }
}
