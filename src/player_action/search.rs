use crate::misc::{c_string_lossy, remove_first_c_string_byte};
use crate::model::{Cave, Item};

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

trait SearchContext {
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)>;
    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item);
    fn reveal(&mut self, y: i64, x: i64);
    fn message(&mut self, message: &str);
    fn stop_running(&mut self);
}

#[derive(Default)]
struct SearchState {
    blind: i64,
    confused: i64,
    no_light: bool,
}

#[cfg(test)]
fn search_with_rng(
    context: &mut impl SearchContext,
    rng: &mut impl rand::Rng,
    y: i64,
    x: i64,
    chance: i64,
    state: SearchState,
) {
    search(
        context,
        || crate::rng::randint_with_rng(rng, 100),
        y,
        x,
        chance,
        state,
    );
}

fn search(
    context: &mut impl SearchContext,
    mut roll: impl FnMut() -> i64,
    player_y: i64,
    player_x: i64,
    mut chance: i64,
    state: SearchState,
) {
    if state.blind > 0 {
        context.message("You are incapable of searching while blind.");
        return;
    }
    if state.confused.saturating_add(state.blind) > 0 {
        chance = (chance as f64 / 10.0).trunc() as i64;
    } else if state.no_light {
        chance = (chance as f64 / 5.0).trunc() as i64;
    }
    for y in player_y.saturating_sub(1)..=player_y.saturating_add(1) {
        for x in player_x.saturating_sub(1)..=player_x.saturating_add(1) {
            let Some((mut cell, item)) = context.read(y, x) else {
                continue;
            };
            if (y, x) == (player_y, player_x) || roll() >= chance {
                continue;
            }
            let Some(mut item) = item else {
                continue;
            };
            match item.tval as i64 {
                crate::dungeon::trap::data::TVAL_UNSEEN_TRAP => {
                    context.message(&format!("You have found {}.", c_string_lossy(&item.name)));
                    context.reveal(y, x);
                    context.stop_running();
                }
                crate::dungeon::trap::data::TVAL_SECRET_DOOR => {
                    context.message("You have found a secret door.");
                    cell.fval = 5;
                    context.write(y, x, cell, item);
                    context.reveal(y, x);
                    context.stop_running();
                }
                tval if tval == u8::from(crate::model::ItemType::Chest) as i64
                    && item.flags > 1
                    && remove_first_c_string_byte(&mut item.name, b'^') =>
                {
                    context.write(y, x, cell, item);
                    context.message("You have discovered a trap on the chest!");
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::cell::test_support::TestMap;
    use crate::dungeon::trap::{change_trap_global, test_support as traps};
    use rand::{Rng, SeedableRng, StdRng};

    struct Context {
        map: TestMap,
        messages: Vec<String>,
        running: bool,
        revealed: Vec<(i64, i64)>,
        visited: Vec<(i64, i64)>,
    }

    impl Context {
        fn new() -> Self {
            Self {
                map: TestMap::reset(),
                messages: Vec::new(),
                running: true,
                revealed: Vec::new(),
                visited: Vec::new(),
            }
        }

        fn search(&mut self, chance: i64, state: SearchState) {
            let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
            search_with_rng(self, &mut rng, 3, 3, chance, state);
        }
    }

    impl SearchContext for Context {
        fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
            let result = self.map.cells().read(y, x);
            if result.is_some() {
                self.visited.push((y, x));
            }
            result
        }
        fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
            self.map.cells().write(y, x, cell, item);
        }
        fn reveal(&mut self, y: i64, x: i64) {
            let old = self.map.cave[y as usize][x as usize].tptr as usize;
            let item = self.map.items[old];
            let new_index = 8 + self.revealed.len() as u8;
            unsafe {
                traps::reset_side_effect_counters();
                traps::set_tile_tptr(y as usize, x as usize, old as u8);
                traps::write_item_tval_subval(old as u8, item.tval, item.subval);
                traps::set_next_alloc_index(new_index);
                change_trap_global(y as usize, x as usize);
                self.map.cave[y as usize][x as usize].tptr =
                    traps::read_tile(y as usize, x as usize).tptr;
                self.map.items[new_index as usize] = traps::read_item(new_index);
                assert_eq!(traps::pusht_called(), 1);
                assert_eq!(traps::last_pusht_index(), old as u8);
                assert_eq!(traps::lite_spot_called(), 1);
                assert_eq!(traps::last_lite_spot_yx(), (y as usize, x as usize));
                traps::clear_tile(y as usize, x as usize);
                traps::reset_side_effect_counters();
            }
            self.revealed.push((y, x));
        }
        fn message(&mut self, message: &str) {
            self.messages.push(message.to_owned());
        }
        fn stop_running(&mut self) {
            self.running = false;
        }
    }

    #[test]
    #[serial_test::serial]
    fn seeded_search_reveals_trap_and_secret_door() {
        let mut context = Context::new();
        context.map.put(2, 2, 1, 101, b"a hidden trap");
        context.map.put(2, 3, 2, 109, b"a secret door");
        context.map.items[2].subval = 19;
        context.search(101, SearchState::default());
        assert_eq!(context.revealed, [(2, 2), (2, 3)]);
        assert_eq!(
            context.messages,
            [
                "You have found a hidden trap.",
                "You have found a secret door."
            ]
        );
        assert!(!context.running);
        assert_eq!(context.map.cave[2][3].fval, 5);
        assert_eq!(context.map.items[8].tval, 102);
        assert_eq!(context.map.items[9].tval, 105);
    }

    #[test]
    fn blind_search_prints_message_without_changes_or_rng_draws() {
        let mut context = Context::new();
        context.map.put(2, 2, 1, 101, b"a hidden trap");
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut untouched = StdRng::from_seed(&[1, 2, 3, 4][..]);
        search_with_rng(
            &mut context,
            &mut rng,
            3,
            3,
            101,
            SearchState {
                blind: 1,
                ..SearchState::default()
            },
        );
        assert_eq!(rng.gen::<u64>(), untouched.gen::<u64>());
        assert_eq!(
            context.messages,
            ["You are incapable of searching while blind."]
        );
        assert!(context.revealed.is_empty());
        assert!(context.running);
        assert_eq!(context.map.items[1].tval, 101);
    }

    #[test]
    fn search_marks_only_trapped_chests_known() {
        let mut context = Context::new();
        context.map.put(2, 2, 1, 2, b"a ^ch^est");
        context.map.items[1].flags = 16;
        context.map.put(2, 3, 2, 2, b"a ^locked chest");
        context.map.items[2].flags = 1;
        context.search(101, SearchState::default());
        assert_eq!(
            context.map.items[1].name[..9]
                .iter()
                .map(|byte| *byte as u8)
                .collect::<Vec<_>>(),
            b"a ch^est\0"
        );
        assert_eq!(context.map.items[2].name[2] as u8, b'^');
        assert_eq!(
            context.messages,
            ["You have discovered a trap on the chest!"]
        );
        assert!(context.running);
    }

    #[test]
    #[serial_test::serial]
    fn edge_search_skips_border_and_player_cell() {
        let mut context = Context::new();
        context.map.put(1, 2, 1, 101, b"border trap");
        context.map.put(2, 2, 2, 101, b"player trap");
        context.map.put(2, 3, 3, 101, b"adjacent trap");
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut draws = 0;
        search(
            &mut context,
            || {
                draws += 1;
                crate::rng::randint_with_rng(&mut rng, 100)
            },
            2,
            2,
            101,
            SearchState::default(),
        );
        assert_eq!(context.revealed, [(2, 3)]);
        assert_eq!(draws, 3);
        assert_eq!(context.visited, [(2, 2), (2, 3), (3, 2), (3, 3)]);
        assert!(context.map.cells().read(-1, 2).is_none());
        assert!(context.map.cells().read(6, 2).is_none());
    }

    #[test]
    fn confusion_and_darkness_truncate_and_do_not_stack() {
        for (chance, state, adjusted) in [
            (
                1009,
                SearchState {
                    confused: 1,
                    no_light: true,
                    ..SearchState::default()
                },
                100,
            ),
            (
                504,
                SearchState {
                    no_light: true,
                    ..SearchState::default()
                },
                100,
            ),
            (
                19,
                SearchState {
                    confused: 1,
                    ..SearchState::default()
                },
                1,
            ),
            (
                9,
                SearchState {
                    no_light: true,
                    ..SearchState::default()
                },
                1,
            ),
            (
                -19,
                SearchState {
                    confused: 1,
                    ..SearchState::default()
                },
                -1,
            ),
        ] {
            let mut context = Context::new();
            for (index, (y, x)) in [
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 2),
                (3, 4),
                (4, 2),
                (4, 3),
                (4, 4),
            ]
            .iter()
            .enumerate()
            {
                context.map.put(*y, *x, index as u8 + 1, 2, b"^chest");
                context.map.items[index + 1].flags = 16;
            }
            let mut expected_rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
            let expected = (0..8)
                .filter(|_| crate::rng::randint_with_rng(&mut expected_rng, 100) < adjusted)
                .count();
            context.search(chance, state);
            assert_eq!(context.messages.len(), expected);
            assert_eq!(
                context.map.items[1].name[0] as u8 != b'^',
                crate::rng::randint_with_rng(&mut StdRng::from_seed(&[1, 2, 3, 4][..]), 100)
                    < adjusted
            );
        }
    }

    #[test]
    fn search_draws_for_every_valid_neighbor_even_without_an_item() {
        let mut context = Context::new();
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut expected = StdRng::from_seed(&[1, 2, 3, 4][..]);
        for _ in 0..8 {
            crate::rng::randint_with_rng(&mut expected, 100);
        }
        search_with_rng(&mut context, &mut rng, 3, 3, 50, SearchState::default());
        assert_eq!(rng.gen::<u64>(), expected.gen::<u64>());
        assert_eq!(
            context.visited,
            [
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 2),
                (3, 3),
                (3, 4),
                (4, 2),
                (4, 3),
                (4, 4)
            ]
        );
    }

    #[test]
    fn penalty_truncation_and_strict_threshold_match_c() {
        for (chance, confused, no_light, roll, found) in [
            (10, 0, false, 10, false),
            (11, 0, false, 10, true),
            (109, 1, true, 10, false),
            (110, 1, true, 10, true),
            (54, 0, true, 10, false),
            (55, 0, true, 10, true),
            (19, 1, false, 1, false),
            (9, 0, true, 1, false),
        ] {
            let mut context = Context::new();
            context.map.put(2, 2, 1, 2, b"^chest");
            context.map.items[1].flags = 16;
            search(
                &mut context,
                || roll,
                3,
                3,
                chance,
                SearchState {
                    confused,
                    no_light,
                    ..SearchState::default()
                },
            );
            assert_eq!(
                !context.messages.is_empty(),
                found,
                "chance={chance}, confused={confused}, no_light={no_light}"
            );
        }
    }

    #[test]
    fn known_chest_and_unrelated_items_are_unchanged() {
        let mut context = Context::new();
        context.map.put(2, 2, 1, 2, b"a chest");
        context.map.items[1].flags = 16;
        context.map.put(2, 3, 2, 20, b"^unrelated item");
        context.map.items[2].flags = 16;
        context.search(101, SearchState::default());
        assert!(context.messages.is_empty());
        assert_eq!(context.map.items[2].name[0] as u8, b'^');
        assert!(context.running);
    }
}
