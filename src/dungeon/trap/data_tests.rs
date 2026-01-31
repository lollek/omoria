#[cfg(test)]
mod tests {
    use crate::dungeon::trap::data::{TRAP_LIST_A, TRAP_LIST_B};

    /// Phase 2.1 (RED): pin down basic expectations from `src/traps.c`.
    ///
    /// These counts should match the legacy arrays:
    /// - `trap_lista[MAX_TRAPA + 1]` where `MAX_TRAPA` is 19 → 20 entries (index 0 is "bogus").
    /// - `trap_listb[MAX_TRAPB + 1]` where `MAX_TRAPB` is 20 → 21 entries (index 0 is "bogus").
    ///
    /// This test should fail until we've ported the data.
    #[test]
    fn trap_template_lists_have_legacy_counts() {
        assert_eq!(TRAP_LIST_A.len(), 20);
        assert_eq!(TRAP_LIST_B.len(), 21);
    }
}
