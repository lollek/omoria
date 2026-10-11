use super::{open, OpenContext};
use crate::{
    dungeon::cell::with_global_cells,
    model::{Cave, Item, PlayerFlags},
};

struct GlobalOpen;

impl OpenContext for GlobalOpen {
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
        unsafe { with_global_cells(|cells| cells.read(y, x)) }
    }

    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
        unsafe { with_global_cells(|cells| cells.write(y, x, cell, item)) }
    }

    fn open_door(&mut self) -> Item {
        extern "C" {
            static mut door_list: [Item; 3];
        }
        unsafe { door_list[0] }
    }

    fn disarm(&mut self) -> i64 {
        i64::from(crate::player::disarm())
    }

    fn confused(&mut self) -> bool {
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

    fn redraw_stats(&mut self) {
        extern "C" {
            fn prt_stat_block();
        }
        unsafe { prt_stat_block() }
    }

    fn redraw(&mut self, y: i64, x: i64) {
        extern "C" {
            fn lite_spot(y: libc::c_long, x: libc::c_long);
        }
        unsafe { lite_spot(y, x) }
    }

    fn trigger_trap(&mut self, y: i64, x: i64) {
        extern "C" {
            fn trigger_trap(y: libc::c_long, x: libc::c_long);
        }
        unsafe { trigger_trap(y, x) }
    }

    fn treasure(&mut self, y: i64, x: i64, flags: u64, level: i64) {
        extern "C" {
            static mut dun_level: libc::c_long;
            fn monster_death(y: libc::c_long, x: libc::c_long, flags: libc::c_ulong);
        }
        unsafe {
            let previous = dun_level;
            dun_level = level;
            monster_death(y, x, flags);
            dun_level = previous;
        }
    }
}

pub(super) fn open_global(y: i64, x: i64) {
    open(&mut GlobalOpen, &mut || crate::rng::randint(100), y, x);
}
