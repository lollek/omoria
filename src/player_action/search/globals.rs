use super::{search, SearchContext, SearchState};
use crate::dungeon::{cell::with_global_cells, trap::change_trap_global};
use crate::model::{Cave, Item, PlayerFlags};

struct GlobalSearch;

impl SearchContext for GlobalSearch {
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
        unsafe { with_global_cells(|cells| cells.read(y, x)) }
    }

    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
        unsafe { with_global_cells(|cells| cells.write(y, x, cell, item)) }
    }

    fn reveal(&mut self, y: i64, x: i64) {
        unsafe { change_trap_global(y as usize, x as usize) }
    }

    fn message(&mut self, message: &str) {
        crate::term::msg_print(message);
    }

    fn stop_running(&mut self) {
        extern "C" {
            static mut find_flag: bool;
        }
        unsafe {
            find_flag = false;
        }
    }
}

pub(super) fn search_global(y: i64, x: i64, chance: i64) {
    extern "C" {
        static mut player_flags: PlayerFlags;
        fn player_has_no_light() -> bool;
    }
    let state = unsafe {
        SearchState {
            blind: player_flags.blind,
            confused: player_flags.confused,
            no_light: player_flags.blind <= 0
                && player_flags.confused.saturating_add(player_flags.blind) <= 0
                && player_has_no_light(),
        }
    };
    search(
        &mut GlobalSearch,
        || crate::rng::randint(100),
        y,
        x,
        chance,
        state,
    );
}
