use super::{disarm, DisarmContext};
use crate::{
    dungeon::cell::with_global_cells,
    model::{Cave, Item, PlayerFlags},
};

struct GlobalDisarm;

impl DisarmContext for GlobalDisarm {
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
        unsafe { with_global_cells(|cells| cells.read(y, x)) }
    }

    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
        unsafe { with_global_cells(|cells| cells.write(y, x, cell, item)) }
    }

    fn remove_trap(&mut self, y: i64, x: i64) {
        extern "C" {
            fn pusht(index: libc::c_long);
        }
        unsafe {
            let index = with_global_cells(|cells| {
                let (cell, Some(_)) = cells.read(y, x)? else {
                    return None;
                };
                cells.cave[y as usize][x as usize].fm = 0;
                Some(cell.tptr)
            });
            if let Some(index) = index {
                pusht(i64::from(index));
                with_global_cells(|cells| cells.cave[y as usize][x as usize].tptr = 0);
            }
        }
    }

    fn skill(&mut self) -> i64 {
        i64::from(crate::player::disarm())
    }

    fn is_blind(&mut self) -> bool {
        extern "C" {
            static mut player_flags: PlayerFlags;
        }
        unsafe { player_flags.blind > 0 }
    }

    fn has_no_light(&mut self) -> bool {
        extern "C" {
            fn player_has_no_light() -> bool;
        }
        unsafe { player_has_no_light() }
    }

    fn is_confused(&mut self) -> bool {
        extern "C" {
            static mut player_flags: PlayerFlags;
        }
        unsafe { player_flags.confused > 0 }
    }

    fn message(&mut self, message: &str) {
        crate::term::msg_print(message);
    }

    fn add_exp(&mut self, amount: i64) {
        crate::player::add_experience(amount);
    }

    fn move_player(&mut self, direction: i64) {
        extern "C" {
            fn player_action_move(direction: libc::c_long);
        }
        unsafe { player_action_move(direction) }
    }

    fn redraw(&mut self, y: i64, x: i64) {
        extern "C" {
            fn lite_spot(y: libc::c_long, x: libc::c_long);
        }
        unsafe { lite_spot(y, x) }
    }

    fn redraw_stats(&mut self) {
        extern "C" {
            fn prt_stat_block();
        }
        unsafe { prt_stat_block() }
    }

    fn trigger_trap(&mut self, y: i64, x: i64) {
        extern "C" {
            fn trigger_trap(y: libc::c_long, x: libc::c_long);
        }
        unsafe { trigger_trap(y, x) }
    }
}

pub(super) fn disarm_global(y: i64, x: i64, direction: i64) {
    disarm(&mut GlobalDisarm, &mut crate::rng::randint, y, x, direction);
}
