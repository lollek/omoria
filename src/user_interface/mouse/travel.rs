use std::sync::Mutex;

#[cfg(test)]
use crate::pathfinding::PathMap;
use crate::pathfinding::{self, Coordinate};
use crate::user_interface::mouse::{MouseButton, MouseEvent, MouseEventKind, MouseTarget};

lazy_static! {
    static ref ACTIVE_TRAVEL: Mutex<Option<TravelState>> = Mutex::new(None);
}

#[cfg(not(test))]
extern "C" {
    static mut char_row: libc::c_long;
    static mut char_col: libc::c_long;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TravelState {
    goal: Coordinate,
}

impl TravelState {
    pub(super) fn new(goal: Coordinate) -> Self {
        Self { goal }
    }

    #[cfg(test)]
    fn next_direction(
        &mut self,
        map: &impl PathMap,
        current: Coordinate,
        running: bool,
        key_pressed: bool,
    ) -> Option<i64> {
        if !running || key_pressed || current == self.goal {
            return None;
        }

        let path = pathfinding::shortest_path(map, current, self.goal)?;
        self.direction_from_path(current, Some(path), running, key_pressed)
    }

    fn direction_from_path(
        &self,
        current: Coordinate,
        path: Option<Vec<Coordinate>>,
        running: bool,
        key_pressed: bool,
    ) -> Option<i64> {
        if !running || key_pressed || current == self.goal {
            return None;
        }
        let path = path?;
        let next = *path.get(1)?;
        direction_between(current, next)
    }
}

fn direction_between(current: Coordinate, next: Coordinate) -> Option<i64> {
    match (next.y - current.y, next.x - current.x) {
        (-1, -1) => Some(7),
        (-1, 0) => Some(8),
        (-1, 1) => Some(9),
        (0, -1) => Some(4),
        (0, 1) => Some(6),
        (1, -1) => Some(1),
        (1, 0) => Some(2),
        (1, 1) => Some(3),
        _ => None,
    }
}

fn begin_travel_from_path(
    active: &mut Option<TravelState>,
    start: Coordinate,
    goal: Coordinate,
    path: Option<Vec<Coordinate>>,
) -> bool {
    if start == goal || path.is_none() {
        *active = None;
        return false;
    }
    *active = Some(TravelState::new(goal));
    true
}

fn target_from_click(event: MouseEvent, target: Option<MouseTarget>) -> Option<Coordinate> {
    if event.kind != MouseEventKind::Click(MouseButton::Left) {
        return None;
    }
    match target {
        Some(MouseTarget::Map { y, x }) => Some(Coordinate {
            y: y.into(),
            x: x.into(),
        }),
        _ => None,
    }
}

fn dispatch_travel_click(
    event: MouseEvent,
    target: Option<MouseTarget>,
    start_travel: impl FnOnce(Coordinate),
) {
    if let Some(goal) = target_from_click(event, target) {
        start_travel(goal);
    }
}

fn handle_click(event: MouseEvent, target: Option<MouseTarget>) {
    dispatch_travel_click(event, target, |goal| {
        let started = C_mouse_travel_begin(goal.y as libc::c_long, goal.x as libc::c_long);
        if started {
            crate::term::msg_print("Travel started.".to_string());
        } else {
            crate::term::msg_print("No known path to that location.".to_string());
        }
    });
}

pub(super) fn register() {
    super::register_handler(handle_click);
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn C_mouse_travel_begin(goal_y: libc::c_long, goal_x: libc::c_long) -> bool {
    let start = unsafe {
        Coordinate {
            y: char_row as i64,
            x: char_col as i64,
        }
    };
    let goal = Coordinate {
        y: goal_y as i64,
        x: goal_x as i64,
    };
    let path = if start == goal {
        None
    } else {
        unsafe { pathfinding::find_path(start, goal) }
    };
    begin_travel_from_path(
        &mut ACTIVE_TRAVEL.lock().expect("travel state lock poisoned"),
        start,
        goal,
        path,
    )
}

#[no_mangle]
#[cfg(test)]
pub extern "C" fn C_mouse_travel_begin(goal_y: libc::c_long, goal_x: libc::c_long) -> bool {
    tests::begin_travel_from_fixture(Coordinate {
        y: goal_y as i64,
        x: goal_x as i64,
    })
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn C_mouse_travel_is_active() -> bool {
    ACTIVE_TRAVEL
        .lock()
        .expect("travel state lock poisoned")
        .is_some()
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn C_mouse_travel_next_direction(
    current_y: libc::c_long,
    current_x: libc::c_long,
    running: bool,
    key_pressed: bool,
) -> libc::c_long {
    let current = Coordinate {
        y: current_y as i64,
        x: current_x as i64,
    };
    let mut active = ACTIVE_TRAVEL.lock().expect("travel state lock poisoned");
    let Some(state) = active.as_ref() else {
        return 0;
    };
    let path = unsafe { pathfinding::find_path(current, state.goal) };
    let direction = state.direction_from_path(current, path, running, key_pressed);
    if direction.is_none() {
        *active = None;
    }
    direction.unwrap_or(0) as libc::c_long
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn C_mouse_travel_arrived(current_y: libc::c_long, current_x: libc::c_long) -> bool {
    let current = Coordinate {
        y: current_y as i64,
        x: current_x as i64,
    };
    let mut active = ACTIVE_TRAVEL.lock().expect("travel state lock poisoned");
    let arrived = active.as_ref().is_some_and(|state| state.goal == current);
    if arrived {
        *active = None;
    }
    arrived
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn C_mouse_travel_cancel() {
    *ACTIVE_TRAVEL.lock().expect("travel state lock poisoned") = None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    struct ResetMouseDispatch;

    impl Drop for ResetMouseDispatch {
        fn drop(&mut self) {
            super::super::MOUSE_EVENTS.lock().unwrap().clear();
            super::super::MOUSE_HANDLERS.lock().unwrap().clear();
            *ACTIVE_TRAVEL.lock().unwrap() = None;
        }
    }

    const OPEN: pathfinding::PathCell = pathfinding::PathCell {
        known: true,
        passable: true,
    };

    struct Grid(Vec<Vec<pathfinding::PathCell>>);

    impl Grid {
        fn open(height: usize, width: usize) -> Self {
            Self(vec![vec![OPEN; width]; height])
        }

        fn block(&mut self, y: usize, x: usize) {
            self.0[y][x].passable = false;
        }
    }

    impl PathMap for Grid {
        fn cell(&self, coordinate: Coordinate) -> Option<pathfinding::PathCell> {
            usize::try_from(coordinate.y)
                .ok()
                .and_then(|y| self.0.get(y))
                .and_then(|row| usize::try_from(coordinate.x).ok().and_then(|x| row.get(x)))
                .copied()
        }
    }

    fn coordinate(y: i64, x: i64) -> Coordinate {
        Coordinate { y, x }
    }

    pub(super) fn begin_travel_from_fixture(goal: Coordinate) -> bool {
        let grid = Grid::open(5, 5);
        let start = coordinate(2, 2);
        begin_travel_from_path(
            &mut ACTIVE_TRAVEL.lock().unwrap(),
            start,
            goal,
            pathfinding::shortest_path(&grid, start, goal),
        )
    }

    #[test]
    fn advances_one_direction_and_replans_from_the_new_position() {
        let mut grid = Grid::open(3, 5);
        let mut travel = TravelState::new(coordinate(1, 4));

        assert_eq!(
            travel.next_direction(&grid, coordinate(1, 0), true, false),
            Some(9)
        );

        grid.block(0, 2);
        assert_eq!(
            travel.next_direction(&grid, coordinate(0, 1), true, false),
            Some(3)
        );
    }

    #[test]
    fn stops_when_arrived() {
        let grid = Grid::open(3, 5);
        let mut travel = TravelState::new(coordinate(1, 4));

        assert_eq!(
            travel.next_direction(&grid, coordinate(1, 4), true, false),
            None
        );
    }

    #[test]
    fn stops_when_running_is_disturbed() {
        let grid = Grid::open(3, 5);
        let mut travel = TravelState::new(coordinate(1, 4));

        assert_eq!(
            travel.next_direction(&grid, coordinate(1, 0), false, false),
            None
        );
    }

    #[test]
    fn stops_when_a_key_is_pressed() {
        let grid = Grid::open(3, 5);
        let mut travel = TravelState::new(coordinate(1, 4));

        assert_eq!(
            travel.next_direction(&grid, coordinate(1, 0), true, true),
            None
        );
    }

    #[test]
    fn stops_when_the_remaining_route_is_blocked() {
        let mut grid = Grid::open(3, 5);
        let mut travel = TravelState::new(coordinate(1, 4));

        for y in 0..3 {
            grid.block(y, 2);
        }
        assert_eq!(
            travel.next_direction(&grid, coordinate(1, 0), true, false),
            None
        );
    }

    #[test]
    fn only_left_clicks_on_map_cells_start_travel() {
        let left_click = MouseEvent {
            row: 8,
            col: 20,
            kind: MouseEventKind::Click(MouseButton::Left),
        };
        let map_target = Some(MouseTarget::Map { y: 5, x: 9 });

        assert_eq!(
            target_from_click(left_click, map_target),
            Some(coordinate(5, 9))
        );
        assert_eq!(
            target_from_click(
                MouseEvent {
                    kind: MouseEventKind::Click(MouseButton::Right),
                    ..left_click
                },
                map_target,
            ),
            None
        );
        assert_eq!(
            target_from_click(
                left_click,
                Some(MouseTarget::Sidebar {
                    panel: super::super::SidebarPanel::Inventory,
                    index: 2,
                }),
            ),
            None
        );
    }

    #[test]
    #[serial_test::serial]
    fn dispatched_left_map_click_starts_travel() {
        let _reset = ResetMouseDispatch;
        let _messages = crate::message::tests::TestState::new();
        super::super::register_travel_handler();
        super::super::C_mouse_event_push(
            1,
            14,
            (ncurses::BUTTON1_CLICKED | ncurses::BUTTON_SHIFT) as libc::c_ulong,
        );

        let layout = crate::layout::compute_layout(25, 150);
        let viewport =
            crate::user_interface::viewport::calculate_viewport(layout.map.unwrap(), 0, 0, 66, 198);
        assert_eq!(super::super::dispatch_pending_events(layout, viewport), 1);
        assert_eq!(
            *ACTIVE_TRAVEL.lock().unwrap(),
            Some(TravelState::new(coordinate(1, 1)))
        );
        assert_eq!(crate::term::test_last_msg_print(), "Travel started.");
    }

    #[test]
    #[serial_test::serial]
    fn map_click_without_a_path_reports_feedback() {
        let _reset = ResetMouseDispatch;
        let _messages = crate::message::tests::TestState::new();
        handle_click(
            MouseEvent {
                row: 8,
                col: 20,
                kind: MouseEventKind::Click(MouseButton::Left),
            },
            Some(MouseTarget::Map { y: 8, x: 8 }),
        );

        assert_eq!(
            crate::term::test_last_msg_print(),
            "No known path to that location."
        );
        assert!(ACTIVE_TRAVEL.lock().unwrap().is_none());
    }

    #[test]
    fn starts_only_when_a_path_leads_to_a_different_cell() {
        let grid = Grid::open(3, 5);
        let start = coordinate(1, 0);
        let goal = coordinate(1, 4);
        let mut active = None;

        assert!(begin_travel_from_path(
            &mut active,
            start,
            goal,
            pathfinding::shortest_path(&grid, start, goal),
        ));
        assert_eq!(active, Some(TravelState::new(goal)));

        assert!(!begin_travel_from_path(&mut active, start, goal, None));
        assert_eq!(active, None);
        assert!(!begin_travel_from_path(
            &mut active,
            start,
            start,
            Some(vec![start]),
        ));
    }
}
