#[cfg(test)]
mod tests {
    use serial_test::serial;
    use crate::dungeon::trap::data::{TRAP_LIST_A, TRAP_LIST_B, TVAL_RUBBLE};
    use crate::dungeon::trap::{place_trap_global, place_trap_into_lists, TrapList};
    use crate::dungeon::trap::place_rubble_global;
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
        assert_eq!(item.level as i64, tpl.level);
        assert_eq!(item.cost, tpl.cost);
        assert_eq!(item.damage, rs2item_damage(tpl.damage));

        // These fields are always zero for traps (not stored in TrapTemplate).
        assert_eq!(item.flags, 0);
        assert_eq!(item.flags2, 0);
        assert_eq!(item.weight, 0);
        assert_eq!(item.number, 0);
        assert_eq!(item.tohit, 0);
        assert_eq!(item.todam, 0);
        assert_eq!(item.ac, 0);
        assert_eq!(item.toac, 0);
        assert_eq!(item.p1, 0);
    }

    /// Validate that trap list counts match the legacy C arrays.
    ///
    /// - `trap_lista[MAX_TRAPA + 1]` where `MAX_TRAPA` is 19 → 20 entries (index 0 is "bogus").
    /// - `trap_listb[MAX_TRAPB + 1]` where `MAX_TRAPB` is 20 → 21 entries (index 0 is "bogus").
    #[test]
    fn trap_template_lists_have_legacy_counts() {
        // MAX_TRAPA=19, MAX_TRAPB=20 in src/constants.h
        assert_eq!(TRAP_LIST_A.len(), MAX_TRAPA + 1);
        assert_eq!(TRAP_LIST_B.len(), MAX_TRAPB + 1);
    }

    /// Validate that the standalone rubble template matches the C definition.
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

    #[test]
    #[serial]
    fn change_trap_global_replaces_unseen_trap_with_seen_variant_and_restores_slot() {
        // C change_trap:
        // - if t_list[cave[y][x].tptr].tval is in {unseen_trap, secret_door}
        // - it places the corresponding type-B trap at the same (y,x)
        // - then pushes the old t_list index back (so it can be reused)
        //
        // We can assert:
        // - tile's tptr changes to the newly allocated index
        // - the new item matches TRAP_LIST_B[subval] (all fields)
        // - pusht is called with the old t_list index (free-list restored)
        // - lite_spot is called for the affected tile
        // - the old slot is cleared (blank_treasure behavior)

        const Y: usize = 4;
        const X: usize = 5;
        const INITIAL_INDEX: u8 = 10;
        const NEW_INDEX: u8 = 11;

        // Use an UNSEEN trap from list A (arrow trap has subval=2).
        const SUBVAL_ARROW_TRAP: usize = 2;

        unsafe {
            // Arrange initial placement in the global arrays.
            test_support::clear_tile(Y, X);
            test_support::reset_side_effect_counters();

            // Put an unseen trap in the existing slot.
            let tpl_a = &TRAP_LIST_A[SUBVAL_ARROW_TRAP];
            test_support::set_tile_tptr(Y, X, INITIAL_INDEX);
            test_support::write_item_from_template(INITIAL_INDEX, tpl_a);

            // Configure allocator to return a new slot.
            test_support::set_next_alloc_index(NEW_INDEX);

            // Act.
            crate::dungeon::trap::change_trap_global(Y, X);

            // Assert.
            let tile = test_support::read_tile(Y, X);
            assert_eq!(tile.tptr, NEW_INDEX);

            let new_item = test_support::read_item(NEW_INDEX);
            let tpl_b = &TRAP_LIST_B[SUBVAL_ARROW_TRAP];
            assert_item_matches_template(&new_item, tpl_b);

            assert_eq!(test_support::pusht_called(), 1);
            assert_eq!(test_support::last_pusht_index(), INITIAL_INDEX);
            assert_eq!(test_support::lite_spot_called(), 1);
            assert_eq!(test_support::last_lite_spot_yx(), (Y, X));

            // Old slot should be cleared by pusht (blank_treasure).
            let old_item = test_support::read_item(INITIAL_INDEX);
            assert_eq!(old_item.tval, 0);
            assert_eq!(old_item.subval, 0);
        }
    }

    #[test]
    #[serial]
    fn change_trap_global_replaces_secret_door_with_seen_variant_and_restores_slot() {
        const Y: usize = 6;
        const X: usize = 7;
        const INITIAL_INDEX: u8 = 12;
        const NEW_INDEX: u8 = 13;

        // Subval must map into TRAP_LIST_B; any normal trap subval works.
        const SUBVAL_ARROW_TRAP: usize = 2;

        unsafe {
            test_support::clear_tile(Y, X);
            test_support::reset_side_effect_counters();

            test_support::set_tile_tptr(Y, X, INITIAL_INDEX);

            // Seed a "secret door" entry in t_list.
            test_support::write_item_tval_subval(
                INITIAL_INDEX,
                crate::dungeon::trap::data::TVAL_SECRET_DOOR as u8,
                TRAP_LIST_A[SUBVAL_ARROW_TRAP].subval,
            );

            test_support::set_next_alloc_index(NEW_INDEX);

            crate::dungeon::trap::change_trap_global(Y, X);

            let tile = test_support::read_tile(Y, X);
            assert_eq!(tile.tptr, NEW_INDEX);

            let new_item = test_support::read_item(NEW_INDEX);
            let tpl_b = &TRAP_LIST_B[SUBVAL_ARROW_TRAP];
            assert_item_matches_template(&new_item, tpl_b);

            assert_eq!(test_support::pusht_called(), 1);
            assert_eq!(test_support::last_pusht_index(), INITIAL_INDEX);
            assert_eq!(test_support::lite_spot_called(), 1);
            assert_eq!(test_support::last_lite_spot_yx(), (Y, X));

            let old_item = test_support::read_item(INITIAL_INDEX);
            assert_eq!(old_item.tval, 0);
            assert_eq!(old_item.subval, 0);
        }
    }

    #[test]
    #[serial]
    fn place_rubble_global_allocates_slot_sets_fopen_false_and_sets_rubble_item() {
        const Y: usize = 8;
        const X: usize = 9;
        const ALLOC_INDEX: u8 = 20;

        unsafe {
            test_support::clear_tile(Y, X);
            test_support::reset_side_effect_counters();
            test_support::set_next_alloc_index(ALLOC_INDEX);

            place_rubble_global(Y, X);

            let tile = test_support::read_tile(Y, X);
            assert_eq!(tile.tptr, ALLOC_INDEX);
            assert_eq!(tile.fopen, 0);

            let item = test_support::read_item(ALLOC_INDEX);
            assert_item_matches_template(&item, &crate::dungeon::trap::data::RUBBLE);
        }
    }

    #[test]
    #[serial]
    fn change_trap_global_does_nothing_when_tile_has_no_object() {
        const Y: usize = 10;
        const X: usize = 11;

        unsafe {
            test_support::clear_tile(Y, X);
            test_support::reset_side_effect_counters();

            crate::dungeon::trap::change_trap_global(Y, X);

            // Should be a no-op: no pusht, no lite_spot, tile unchanged.
            assert_eq!(test_support::pusht_called(), 0);
            assert_eq!(test_support::lite_spot_called(), 0);
            assert_eq!(test_support::read_tile(Y, X).tptr, 0);
        }
    }

    #[test]
    #[serial]
    fn change_trap_global_does_nothing_when_item_is_already_seen_trap() {
        const Y: usize = 12;
        const X: usize = 13;
        const INDEX: u8 = 25;

        unsafe {
            test_support::clear_tile(Y, X);
            test_support::reset_side_effect_counters();
            test_support::set_tile_tptr(Y, X, INDEX);

            // Seed an already-visible trap (TVAL_SEEN_TRAP) — should not trigger change.
            test_support::write_item_tval_subval(
                INDEX,
                crate::dungeon::trap::data::TVAL_SEEN_TRAP as u8,
                1,
            );

            crate::dungeon::trap::change_trap_global(Y, X);

            // Should be a no-op.
            assert_eq!(test_support::pusht_called(), 0);
            assert_eq!(test_support::lite_spot_called(), 0);
            assert_eq!(test_support::read_tile(Y, X).tptr, INDEX);
        }
    }
}
