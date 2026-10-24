use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;

use crate::dungeon::cell::Cells;
use crate::model::{Cave, Item, ItemType};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub y: i64,
    pub x: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PathCell {
    pub known: bool,
    pub passable: bool,
}

impl PathCell {
    fn from_cave(cave: &Cave, item: Option<Item>) -> Self {
        let item_type = item.and_then(|item| ItemType::try_from(item.tval).ok());
        let blocked_object = matches!(
            item_type,
            Some(ItemType::SeenTrap) | Some(ItemType::ClosedDoor)
        );

        Self {
            known: cave.is_lit() || cave.moved != 0 || cave.cptr == 1,
            passable: cave.is_open() && !blocked_object,
        }
    }
}

pub trait PathMap {
    fn cell(&self, coordinate: Coordinate) -> Option<PathCell>;
}

struct FCellMap<'cells, 'map> {
    cells: &'cells Cells<'map>,
}

impl PathMap for FCellMap<'_, '_> {
    fn cell(&self, coordinate: Coordinate) -> Option<PathCell> {
        let (cave, item) = self.cells.read(coordinate.y, coordinate.x)?;
        Some(PathCell::from_cave(&cave, item))
    }
}

pub fn shortest_path(
    map: &impl PathMap,
    start: Coordinate,
    goal: Coordinate,
) -> Option<Vec<Coordinate>> {
    if !is_traversable(map, start) || !is_traversable(map, goal) {
        return None;
    }
    if start == goal {
        return Some(vec![start]);
    }

    let mut previous = HashMap::new();
    let mut frontier = VecDeque::new();
    previous.insert(start, start);
    frontier.push_back(start);

    while let Some(current) = frontier.pop_front() {
        for y_delta in -1..=1 {
            for x_delta in -1..=1 {
                if y_delta == 0 && x_delta == 0 {
                    continue;
                }
                let Some(y) = current.y.checked_add(y_delta) else {
                    continue;
                };
                let Some(x) = current.x.checked_add(x_delta) else {
                    continue;
                };
                let next = Coordinate { y, x };
                if previous.contains_key(&next) || !is_traversable(map, next) {
                    continue;
                }
                previous.insert(next, current);
                if next == goal {
                    return Some(reconstruct_path(start, goal, &previous));
                }
                frontier.push_back(next);
            }
        }
    }

    None
}

fn is_traversable(map: &impl PathMap, coordinate: Coordinate) -> bool {
    map.cell(coordinate)
        .is_some_and(|cell| cell.known && cell.passable)
}

fn reconstruct_path(
    start: Coordinate,
    goal: Coordinate,
    previous: &HashMap<Coordinate, Coordinate>,
) -> Vec<Coordinate> {
    let mut path = vec![goal];
    let mut current = goal;
    while current != start {
        current = previous[&current];
        path.push(current);
    }
    path.reverse();
    path
}

#[cfg(not(test))]
/// # Safety
/// C globals must be initialized and exclusively accessed on the game thread.
pub unsafe fn find_path(start: Coordinate, goal: Coordinate) -> Option<Vec<Coordinate>> {
    unsafe {
        crate::dungeon::cell::with_global_cells(|cells| {
            shortest_path(&FCellMap { cells }, start, goal)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPEN: PathCell = PathCell {
        known: true,
        passable: true,
    };

    struct Grid {
        cells: Vec<Vec<PathCell>>,
    }

    impl Grid {
        fn open(height: usize, width: usize) -> Self {
            Self {
                cells: vec![vec![OPEN; width]; height],
            }
        }

        fn set(&mut self, y: usize, x: usize, cell: PathCell) {
            self.cells[y][x] = cell;
        }
    }

    impl PathMap for Grid {
        fn cell(&self, coordinate: Coordinate) -> Option<PathCell> {
            usize::try_from(coordinate.y)
                .ok()
                .and_then(|y| self.cells.get(y))
                .and_then(|row| usize::try_from(coordinate.x).ok().and_then(|x| row.get(x)))
                .copied()
        }
    }

    fn coordinate(y: i64, x: i64) -> Coordinate {
        Coordinate { y, x }
    }

    #[test]
    fn finds_a_shortest_path_on_an_open_grid() {
        let grid = Grid::open(5, 5);
        let start = coordinate(0, 0);
        let goal = coordinate(4, 4);

        let path = shortest_path(&grid, start, goal).unwrap();

        assert_eq!(path.first(), Some(&start));
        assert_eq!(path.last(), Some(&goal));
        assert_eq!(path.len(), 5, "diagonal steps should be one move each");
    }

    #[test]
    fn does_not_cross_unknown_cells() {
        let mut grid = Grid::open(3, 5);
        for y in 0..3 {
            grid.set(
                y,
                2,
                PathCell {
                    known: false,
                    passable: true,
                },
            );
        }

        assert!(shortest_path(&grid, coordinate(1, 0), coordinate(1, 4)).is_none());
    }

    #[test]
    fn routes_around_known_traps_and_closed_doors() {
        let mut grid = Grid::open(7, 7);
        for (y, x) in [(2, 2), (3, 3)] {
            grid.set(
                y,
                x,
                PathCell {
                    known: true,
                    passable: false,
                },
            );
        }
        let trap = coordinate(2, 2);
        let door = coordinate(3, 3);

        let path = shortest_path(&grid, coordinate(1, 1), coordinate(5, 5)).unwrap();

        assert!(!path.contains(&trap));
        assert!(!path.contains(&door));
        assert!(path.len() > 5, "the direct diagonal route is blocked");
    }

    #[test]
    fn returns_no_path_when_a_wall_separates_the_endpoints() {
        let mut grid = Grid::open(3, 5);
        for y in 0..3 {
            grid.set(
                y,
                2,
                PathCell {
                    known: true,
                    passable: false,
                },
            );
        }

        assert!(shortest_path(&grid, coordinate(1, 0), coordinate(1, 4)).is_none());
    }

    #[test]
    fn start_equal_to_goal_returns_a_single_cell_path() {
        let grid = Grid::open(3, 3);
        let start = coordinate(1, 1);

        assert_eq!(shortest_path(&grid, start, start), Some(vec![start]));
    }

    #[test]
    fn diagonals_do_not_require_adjacent_orthogonal_cells_to_be_open() {
        let mut grid = Grid::open(2, 2);
        let blocked = PathCell {
            known: true,
            passable: false,
        };
        grid.set(0, 1, blocked);
        grid.set(1, 0, blocked);
        let start = coordinate(0, 0);
        let goal = coordinate(1, 1);

        assert_eq!(shortest_path(&grid, start, goal), Some(vec![start, goal]));
    }

    #[test]
    fn fcell_map_reports_known_passable_and_blocked_cells() {
        use crate::dungeon::cell::test_support::TestMap;

        let mut test_map = TestMap::reset();
        for y in 2..=5 {
            for x in 2..=5 {
                test_map.cave[y][x].fopen = 1;
                test_map.cave[y][x].pl = 1;
            }
        }
        test_map.cave[2][5].pl = 0;
        test_map.cave[3][2].fopen = 0;
        test_map.put(2, 3, 1, 102, b"a known trap");
        test_map.put(2, 4, 2, 105, b"a closed door");

        let cells = test_map.cells();
        let map = FCellMap { cells: &cells };

        assert_eq!(
            map.cell(coordinate(2, 2)),
            Some(PathCell {
                known: true,
                passable: true
            })
        );
        assert_eq!(
            map.cell(coordinate(2, 5)),
            Some(PathCell {
                known: false,
                passable: true
            })
        );
        assert_eq!(
            map.cell(coordinate(3, 2)),
            Some(PathCell {
                known: true,
                passable: false
            })
        );
        assert_eq!(
            map.cell(coordinate(2, 3)),
            Some(PathCell {
                known: true,
                passable: false
            })
        );
        assert_eq!(
            map.cell(coordinate(2, 4)),
            Some(PathCell {
                known: true,
                passable: false
            })
        );
    }

    #[test]
    fn fcell_map_treats_walked_dark_cells_as_known_but_not_unvisited_cells() {
        use crate::dungeon::cell::test_support::TestMap;

        let mut test_map = TestMap::reset();
        test_map.cave[2][2].fopen = 1;
        test_map.cave[2][2].moved = 1;
        test_map.cave[2][3].fopen = 1;

        let cells = test_map.cells();
        let map = FCellMap { cells: &cells };

        assert_eq!(
            map.cell(coordinate(2, 2)),
            Some(PathCell {
                known: true,
                passable: true,
            }),
            "a previously walked dark cell should remain pathable"
        );
        assert_eq!(
            map.cell(coordinate(2, 3)),
            Some(PathCell {
                known: false,
                passable: true,
            }),
            "an unvisited dark cell must remain unknown"
        );
    }

    #[test]
    fn shortest_path_routes_around_fcell_traps_and_closed_doors() {
        use crate::dungeon::cell::test_support::TestMap;

        let mut test_map = TestMap::reset();
        for y in 2..=5 {
            for x in 2..=5 {
                test_map.cave[y][x].fopen = 1;
                test_map.cave[y][x].pl = 1;
            }
        }
        test_map.put(3, 3, 1, 102, b"a known trap");
        test_map.put(4, 4, 2, 105, b"a closed door");

        let cells = test_map.cells();
        let map = FCellMap { cells: &cells };
        let trap = coordinate(3, 3);
        let door = coordinate(4, 4);

        let path = shortest_path(&map, coordinate(2, 2), coordinate(5, 5)).unwrap();

        assert!(!path.contains(&trap));
        assert!(!path.contains(&door));
        assert!(path.len() > 4, "the direct diagonal route is blocked");
    }
}
