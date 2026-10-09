#[cfg(not(test))]
pub fn resolve_step(dir: i64, row: i64, col: i64) -> super::step::StepOutcome {
    use super::step::{CaveMap, MoveState};
    use crate::{
        constants,
        model::{Cave, Item},
    };
    extern "C" {
        static mut cave: [[Cave; constants::MAX_WIDTH + 1]; constants::MAX_HEIGHT + 1];
        static mut t_list: [Item; constants::MAX_TALLOC + 1];
        static mut cur_height: libc::c_long;
        static mut cur_width: libc::c_long;
    }
    // The single-threaded C game resumes mutations only after these borrows end.
    unsafe {
        super::step::resolve_step(
            &CaveMap {
                cave: &*std::ptr::addr_of!(cave),
                items: &*std::ptr::addr_of!(t_list),
                height: cur_height,
                width: cur_width,
            },
            MoveState { row, col },
            dir,
        )
    }
}
