#[cfg(test)]
mod tests {
    use crate::dungeon::trap::data::{TRAP_LIST_A, TRAP_LIST_B, TVAL_RUBBLE};
    use crate::dungeon::trap::{place_trap_global, place_trap_into_lists, TrapList};
    use crate::dungeon::trap::test_support;
    use crate::misc::rs2item_damage;
    use crate::model::{Cave, Item};

    const TLEN: usize = 10;

    const MAX_TRAPA: usize = 19;
    const MAX_TRAPB: usize = 20;

    const SUBVAL_OPEN_PIT: usize = 1;
    const SUBVAL_CLOSED_DOOR: usize = 19;

    fn assert_item_matches_template(item: &Item, tpl: &crate::dungeon::trap::data::TrapTemplate) {
        assert_eq!(item.tval as i64, tpl.tval);
        assert_eq!(item.subval, tpl.subval);

        assert_eq!(item.flags as u32, tpl.flags);
        assert_eq!(item.flags2 as u32, tpl.flags2);
        assert_eq!(item.level as i64, tpl.level);
        assert_eq!(item.weight as i64, tpl.weight);
        assert_eq!(item.number as i64, tpl.number);
        assert_eq!(item.tohit as i64, tpl.tohit);
        assert_eq!(item.todam as i64, tpl.todam);
        assert_eq!(item.ac as i64, tpl.ac);
        assert_eq!(item.toac as i64, tpl.toac);
        assert_eq!(item.p1, tpl.p1);
        assert_eq!(item.cost, tpl.cost);
        assert_eq!(item.damage, rs2item_damage(tpl.damage));
    }

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
        assert_eq!(TRAP_LIST_A.len(), MAX_TRAPA + 1);
        assert_eq!(TRAP_LIST_B.len(), MAX_TRAPB + 1);
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

    #[test]
    fn place_trap_typ1_uses_trap_list_a_and_sets_tptr() {
        const ALLOC_INDEX: u8 = 3;

        let mut tile = Cave::default();
        let mut t_list = vec![Item::default(); TLEN];

        place_trap_into_lists(&mut tile, &mut t_list, ALLOC_INDEX, TrapList::A, SUBVAL_OPEN_PIT);

        assert_eq!(tile.tptr, ALLOC_INDEX);

        let item = &t_list[ALLOC_INDEX as usize];
        let tpl = &TRAP_LIST_A[SUBVAL_OPEN_PIT];
        assert_item_matches_template(item, tpl);
    }

    #[test]
    fn place_trap_typ2_uses_trap_list_b_and_sets_tptr() {
        const ALLOC_INDEX: u8 = 4;

        let mut tile = Cave::default();
        let mut t_list = vec![Item::default(); TLEN];

        place_trap_into_lists(
            &mut tile,
            &mut t_list,
            ALLOC_INDEX,
            TrapList::B,
            SUBVAL_CLOSED_DOOR,
        );

        assert_eq!(tile.tptr, ALLOC_INDEX);

        let item = &t_list[ALLOC_INDEX as usize];
        let tpl = &TRAP_LIST_B[SUBVAL_CLOSED_DOOR];
        assert_item_matches_template(item, tpl);
    }

    #[test]
    fn place_trap_global_places_into_global_cave_and_t_list() {
        const Y: usize = 2;
        const X: usize = 3;
        const ALLOC_INDEX: u8 = 7;

        unsafe {
            test_support::set_next_alloc_index(ALLOC_INDEX);
            test_support::clear_tile(Y, X);

            place_trap_global(Y, X, TrapList::B, SUBVAL_CLOSED_DOOR);

            let tile = test_support::read_tile(Y, X);
            let item = test_support::read_item(ALLOC_INDEX);
            let tpl = &TRAP_LIST_B[SUBVAL_CLOSED_DOOR];

            assert_eq!(tile.tptr, ALLOC_INDEX);
            assert_item_matches_template(&item, tpl);
        }
    }
}
