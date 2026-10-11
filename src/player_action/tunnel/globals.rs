use super::{read_target, tunnel, TunnelContext};
use crate::{
    dungeon::cell::with_global_cells,
    model::{Cave, Item, Stat},
};

struct GlobalTunnel;

impl TunnelContext for GlobalTunnel {
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
        unsafe { with_global_cells(|cells| read_target(cells, y, x)) }
    }

    fn wall(&mut self, y: i64, x: i64, ability: i64, difficulty: i64) -> bool {
        extern "C" {
            fn twall(
                y: libc::c_long,
                x: libc::c_long,
                ability: libc::c_long,
                difficulty: libc::c_long,
            ) -> bool;
        }
        unsafe { twall(y, x, ability, difficulty) }
    }

    fn remove_rubble(&mut self, y: i64, x: i64, index: u8) {
        extern "C" {
            fn pusht(index: libc::c_long);
        }
        unsafe {
            pusht(i64::from(index));
            with_global_cells(|cells| {
                let cell = &mut cells.cave[y as usize][x as usize];
                cell.tptr = 0;
                cell.fm = 0;
                cell.fopen = 1;
            });
        }
    }

    fn loot(&mut self, y: i64, x: i64) {
        extern "C" {
            fn place_random_dungeon_item(y: libc::c_long, x: libc::c_long);
        }
        unsafe { place_random_dungeon_item(y, x) }
    }

    fn lit(&mut self, y: i64, x: i64) -> bool {
        extern "C" {
            fn test_light(y: libc::c_long, x: libc::c_long) -> bool;
        }
        unsafe { test_light(y, x) }
    }

    fn redraw(&mut self, y: i64, x: i64) {
        extern "C" {
            fn lite_spot(y: libc::c_long, x: libc::c_long);
        }
        unsafe { lite_spot(y, x) }
    }

    fn search(&mut self) {
        extern "C" {
            static mut char_row: libc::c_long;
            static mut char_col: libc::c_long;
        }
        unsafe {
            super::super::search::globals::search_global(
                char_row,
                char_col,
                i64::from(crate::player::curr_search_skill()),
            );
        }
    }

    fn message(&mut self, message: &str) {
        crate::term::msg_print(message);
    }
}

pub(super) fn tunnel_global(y: i64, x: i64) {
    let weapon = unsafe { *crate::equipment::get_item(crate::equipment::Slot::Primary) };
    tunnel(
        &mut GlobalTunnel,
        &mut crate::rng::randint,
        y,
        x,
        i64::from(crate::player::get_stat(Stat::Strength)),
        weapon,
    );
}
