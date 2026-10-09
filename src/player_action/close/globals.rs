use super::{close, CloseContext};
use crate::{
    dungeon::cell::with_global_cells,
    model::{Cave, Item},
};

struct GlobalClose;

impl CloseContext for GlobalClose {
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
        unsafe { with_global_cells(|cells| cells.read(y, x)) }
    }

    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
        unsafe { with_global_cells(|cells| cells.write(y, x, cell, item)) }
    }

    fn closed_door(&mut self) -> Item {
        extern "C" {
            static mut door_list: [Item; 3];
        }
        unsafe { door_list[1] }
    }

    fn monster_name(&mut self, index: u8) -> String {
        extern "C" {
            fn find_monster_name(
                name: *mut libc::c_char,
                index: libc::c_long,
                begin_sentence: bool,
            );
        }
        let mut name = [0; 82];
        unsafe { find_monster_name(name.as_mut_ptr(), index.into(), true) }
        let bytes = name
            .iter()
            .take_while(|byte| **byte != 0)
            .map(|byte| *byte as u8)
            .collect::<Vec<_>>();
        String::from_utf8_lossy(&bytes).into_owned()
    }

    fn message(&mut self, message: &str) {
        crate::term::msg_print(message);
    }

    fn redraw(&mut self, y: i64, x: i64) {
        extern "C" {
            fn lite_spot(y: libc::c_long, x: libc::c_long);
        }
        unsafe { lite_spot(y, x) }
    }
}

pub(super) fn close_global(y: i64, x: i64) {
    close(&mut GlobalClose, y, x);
}
