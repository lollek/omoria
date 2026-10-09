use crate::constants;
use crate::model::Item;
use crate::model::{Cave, ItemType};
use crate::rng::randint_with_rng;
use rand::Rng;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obstacle {
    Rubble,
    ClosedDoor,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MoveCell {
    pub open: bool,
    pub monster: bool,
    pub obstacle: Option<Obstacle>,
}

impl MoveCell {
    pub fn from_cave(cell: &Cave, item_tval: u8) -> Self {
        Self {
            open: cell.fopen != 0,
            monster: cell.cptr >= 2,
            obstacle: if cell.tptr == 0 {
                None
            } else {
                Some(match ItemType::try_from(item_tval) {
                    Ok(ItemType::Rubble) => Obstacle::Rubble,
                    Ok(ItemType::ClosedDoor) => Obstacle::ClosedDoor,
                    _ => Obstacle::Other,
                })
            },
        }
    }
}

pub trait MoveMap {
    fn height(&self) -> i64;
    fn width(&self) -> i64;
    fn cell(&self, row: i64, col: i64) -> MoveCell;
}

pub struct CaveMap<'a> {
    pub cave: &'a [[Cave; constants::MAX_WIDTH + 1]],
    pub items: &'a [Item],
    pub height: i64,
    pub width: i64,
}

impl MoveMap for CaveMap<'_> {
    fn height(&self) -> i64 {
        self.height.min(self.cave.len() as i64 - 1)
    }

    fn width(&self) -> i64 {
        self.width.min(constants::MAX_WIDTH as i64)
    }

    fn cell(&self, row: i64, col: i64) -> MoveCell {
        let cell = &self.cave[row as usize][col as usize];
        let item_tval = self
            .items
            .get(cell.tptr as usize)
            .map_or(0, |item| item.tval);
        MoveCell::from_cave(cell, item_tval)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MoveState {
    pub row: i64,
    pub col: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepOutcome {
    OutOfBounds,
    Attack {
        row: i64,
        col: i64,
    },
    Blocked {
        row: i64,
        col: i64,
        obstacle: Option<Obstacle>,
    },
    Moved {
        row: i64,
        col: i64,
    },
}

impl StepOutcome {
    pub fn consumes_turn(self) -> bool {
        matches!(self, Self::Attack { .. } | Self::Moved { .. })
    }
}

pub fn resolve_step(map: &impl MoveMap, state: MoveState, dir: i64) -> StepOutcome {
    let row_delta = match dir {
        1..=3 => 1,
        7..=9 => -1,
        _ => 0,
    };
    let col_delta = match dir {
        1 | 4 | 7 => -1,
        3 | 6 | 9 => 1,
        _ => 0,
    };
    let Some(row) = state.row.checked_add(row_delta) else {
        return StepOutcome::OutOfBounds;
    };
    let Some(col) = state.col.checked_add(col_delta) else {
        return StepOutcome::OutOfBounds;
    };
    if row < 1 || row > map.height() || col < 1 || col > map.width() {
        return StepOutcome::OutOfBounds;
    }
    let cell = map.cell(row, col);
    if cell.monster {
        StepOutcome::Attack { row, col }
    } else if !cell.open {
        StepOutcome::Blocked {
            row,
            col,
            obstacle: cell.obstacle,
        }
    } else {
        StepOutcome::Moved { row, col }
    }
}

#[cfg(not(test))]
pub fn resolve_direction(dir: i64, confused: i64) -> (i64, bool) {
    resolve_direction_with_rng(&mut rand::thread_rng(), dir, confused)
}

pub fn resolve_direction_with_rng(rng: &mut impl Rng, dir: i64, confused: i64) -> (i64, bool) {
    if confused > 0 && dir != 5 && randint_with_rng(rng, 4) > 1 {
        (randint_with_rng(rng, 9), true)
    } else {
        (dir, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, StdRng};

    struct Map {
        cells: [[MoveCell; 4]; 4],
    }

    impl Map {
        fn open() -> Self {
            Self {
                cells: [[MoveCell {
                    open: true,
                    monster: false,
                    obstacle: None,
                }; 4]; 4],
            }
        }
    }

    impl MoveMap for Map {
        fn height(&self) -> i64 {
            3
        }
        fn width(&self) -> i64 {
            3
        }
        fn cell(&self, row: i64, col: i64) -> MoveCell {
            self.cells[row as usize][col as usize]
        }
    }

    #[test]
    fn legal_steps_follow_keypad_directions_and_consume_turns() {
        let map = Map::open();
        for (dir, row, col) in [
            (1, 3, 1),
            (2, 3, 2),
            (3, 3, 3),
            (4, 2, 1),
            (5, 2, 2),
            (6, 2, 3),
            (7, 1, 1),
            (8, 1, 2),
            (9, 1, 3),
            (0, 2, 2),
            (10, 2, 2),
        ] {
            let outcome = resolve_step(&map, MoveState { row: 2, col: 2 }, dir);
            assert_eq!(outcome, StepOutcome::Moved { row, col });
            assert!(outcome.consumes_turn());
        }
    }

    #[test]
    fn blocked_steps_preserve_obstacles_without_consuming_turns() {
        for obstacle in [
            None,
            Some(Obstacle::Rubble),
            Some(Obstacle::ClosedDoor),
            Some(Obstacle::Other),
        ] {
            let mut map = Map::open();
            map.cells[2][3] = MoveCell {
                open: false,
                monster: false,
                obstacle,
            };
            let outcome = resolve_step(&map, MoveState { row: 2, col: 2 }, 6);
            assert_eq!(
                outcome,
                StepOutcome::Blocked {
                    row: 2,
                    col: 3,
                    obstacle
                }
            );
            assert!(!outcome.consumes_turn());
        }
    }

    #[test]
    fn monster_takes_priority_over_blocked_floor() {
        let mut map = Map::open();
        map.cells[2][3].monster = true;
        map.cells[2][3].open = false;
        let outcome = resolve_step(&map, MoveState { row: 2, col: 2 }, 6);
        assert_eq!(outcome, StepOutcome::Attack { row: 2, col: 3 });
        assert!(outcome.consumes_turn());
    }

    #[test]
    fn map_edges_do_not_consume_turns_or_read_outside_map() {
        for (row, col, dir) in [(1, 2, 8), (3, 2, 2), (2, 1, 4), (2, 3, 6)] {
            let outcome = resolve_step(&Map::open(), MoveState { row, col }, dir);
            assert_eq!(outcome, StepOutcome::OutOfBounds);
            assert!(!outcome.consumes_turn());
        }
    }

    #[test]
    fn independent_states_and_maps_do_not_affect_each_other() {
        let first = MoveState { row: 2, col: 2 };
        let second = MoveState { row: 1, col: 1 };
        let mut first_map = Map::open();
        first_map.cells[2][3].open = false;
        assert!(matches!(
            resolve_step(&first_map, first, 6),
            StepOutcome::Blocked { .. }
        ));
        assert_eq!(
            resolve_step(&Map::open(), second, 6),
            StepOutcome::Moved { row: 1, col: 2 }
        );
        assert_eq!(first, MoveState { row: 2, col: 2 });
    }

    #[test]
    fn confusion_is_deterministic_and_directions_stay_valid() {
        let mut first = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut second = StdRng::from_seed(&[1, 2, 3, 4][..]);
        for _ in 0..100 {
            let outcome = resolve_direction_with_rng(&mut first, 6, 1);
            assert_eq!(outcome, resolve_direction_with_rng(&mut second, 6, 1));
            assert!((1..=9).contains(&outcome.0));
            if !outcome.1 {
                assert_eq!(outcome.0, 6);
            }
        }
    }

    #[test]
    fn unconfused_or_stationary_steps_do_not_draw_randomness() {
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut untouched = rng;
        assert_eq!(resolve_direction_with_rng(&mut rng, 6, 0), (6, false));
        assert_eq!(resolve_direction_with_rng(&mut rng, 5, 1), (5, false));
        assert_eq!(rng.next_u64(), untouched.next_u64());
    }

    #[test]
    fn confusion_matches_legacy_rolls_and_rng_consumption() {
        for seed in 1..=4 {
            let mut actual = StdRng::from_seed(&[seed, 2, 3, 4][..]);
            let mut legacy = actual;
            for _ in 0..100 {
                let expected = if randint_with_rng(&mut legacy, 4) > 1 {
                    (randint_with_rng(&mut legacy, 9), true)
                } else {
                    (6, false)
                };
                assert_eq!(resolve_direction_with_rng(&mut actual, 6, 1), expected);
            }
            assert_eq!(actual.next_u64(), legacy.next_u64());
        }
    }

    #[test]
    fn cave_conversion_respects_occupancy_and_item_presence() {
        let mut cave = Cave::default();
        for cptr in [0, 1, 2, 255] {
            cave.cptr = cptr;
            assert_eq!(MoveCell::from_cave(&cave, 103).monster, cptr >= 2);
        }
        assert_eq!(MoveCell::from_cave(&cave, 103).obstacle, None);
        cave.tptr = 1;
        cave.fopen = 1;
        for (tval, obstacle) in [
            (103, Obstacle::Rubble),
            (105, Obstacle::ClosedDoor),
            (0, Obstacle::Other),
        ] {
            let cell = MoveCell::from_cave(&cave, tval);
            assert_eq!(cell.obstacle, Some(obstacle));
            assert!(cell.open);
        }
    }

    #[test]
    fn legacy_cave_adapter_uses_explicit_state_and_item_records() {
        let mut cave = vec![[Cave::default(); constants::MAX_WIDTH + 1]; 4];
        cave[2][3].tptr = 1;
        let mut items = vec![Item::default(); 2];
        items[1].tval = 105;
        let map = CaveMap {
            cave: &cave,
            items: &items,
            height: 3,
            width: 3,
        };
        assert_eq!(
            resolve_step(&map, MoveState { row: 2, col: 2 }, 6),
            StepOutcome::Blocked {
                row: 2,
                col: 3,
                obstacle: Some(Obstacle::ClosedDoor)
            }
        );
    }

    #[test]
    fn legacy_cave_adapter_bounds_reads_by_actual_storage() {
        let cave = vec![[Cave::default(); constants::MAX_WIDTH + 1]; 2];
        let map = CaveMap {
            cave: &cave,
            items: &[],
            height: i64::MAX,
            width: i64::MAX,
        };
        assert_eq!(
            resolve_step(&map, MoveState { row: 1, col: 1 }, 2),
            StepOutcome::OutOfBounds
        );
        assert_eq!(
            resolve_step(
                &map,
                MoveState {
                    row: 1,
                    col: constants::MAX_WIDTH as i64
                },
                6
            ),
            StepOutcome::OutOfBounds
        );
    }

    #[test]
    fn overflowing_coordinates_are_out_of_bounds() {
        assert_eq!(
            resolve_step(
                &Map::open(),
                MoveState {
                    row: i64::MAX,
                    col: 2
                },
                2
            ),
            StepOutcome::OutOfBounds
        );
        assert_eq!(
            resolve_step(
                &Map::open(),
                MoveState {
                    row: 2,
                    col: i64::MIN
                },
                4
            ),
            StepOutcome::OutOfBounds
        );
    }
}
