use super::step::{Obstacle, StepOutcome};
use crate::model::ItemType;
use libc::c_long;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct MoveDirection {
    pub dir: c_long,
    pub scrambled: c_long,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct MoveResult {
    pub row: c_long,
    pub col: c_long,
    pub kind: c_long,
    pub obstacle: c_long,
    pub consumes_turn: c_long,
}

impl From<(i64, bool)> for MoveDirection {
    fn from((dir, scrambled): (i64, bool)) -> Self {
        Self {
            dir,
            scrambled: scrambled.into(),
        }
    }
}

impl From<StepOutcome> for MoveResult {
    fn from(outcome: StepOutcome) -> Self {
        let (row, col, kind, obstacle) = match outcome {
            StepOutcome::OutOfBounds => (0, 0, 0, 0),
            StepOutcome::Attack { row, col } => (row, col, 1, 0),
            StepOutcome::Blocked { row, col, obstacle } => (
                row,
                col,
                2,
                match obstacle {
                    Some(Obstacle::Rubble) => u8::from(ItemType::Rubble).into(),
                    Some(Obstacle::ClosedDoor) => u8::from(ItemType::ClosedDoor).into(),
                    _ => 0,
                },
            ),
            StepOutcome::Moved { row, col } => (row, col, 3, 0),
        };
        Self {
            row,
            col,
            kind,
            obstacle,
            consumes_turn: outcome.consumes_turn().into(),
        }
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn C_player_move_direction(dir: c_long, confused: c_long) -> MoveDirection {
    super::step::resolve_direction(dir, confused).into()
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn C_player_move_resolve(dir: c_long, row: c_long, col: c_long) -> MoveResult {
    super::globals::resolve_step(dir, row, col).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_outcomes_match_c_kind_and_turn_contract() {
        for (outcome, kind, obstacle, consumes_turn) in [
            (StepOutcome::OutOfBounds, 0, 0, 0),
            (StepOutcome::Attack { row: 2, col: 3 }, 1, 0, 1),
            (
                StepOutcome::Blocked {
                    row: 2,
                    col: 3,
                    obstacle: None,
                },
                2,
                0,
                0,
            ),
            (
                StepOutcome::Blocked {
                    row: 2,
                    col: 3,
                    obstacle: Some(Obstacle::Rubble),
                },
                2,
                103,
                0,
            ),
            (
                StepOutcome::Blocked {
                    row: 2,
                    col: 3,
                    obstacle: Some(Obstacle::ClosedDoor),
                },
                2,
                105,
                0,
            ),
            (
                StepOutcome::Blocked {
                    row: 2,
                    col: 3,
                    obstacle: Some(Obstacle::Other),
                },
                2,
                0,
                0,
            ),
            (StepOutcome::Moved { row: 2, col: 3 }, 3, 0, 1),
        ] {
            let result = MoveResult::from(outcome);
            let (row, col) = if kind == 0 { (0, 0) } else { (2, 3) };
            assert_eq!(
                result,
                MoveResult {
                    row,
                    col,
                    kind,
                    obstacle,
                    consumes_turn
                }
            );
        }
    }

    #[test]
    fn scrambled_flag_matches_c_long_contract() {
        assert_eq!(
            MoveDirection::from((6, false)),
            MoveDirection {
                dir: 6,
                scrambled: 0
            }
        );
        assert_eq!(
            MoveDirection::from((5, true)),
            MoveDirection {
                dir: 5,
                scrambled: 1
            }
        );
    }
}
