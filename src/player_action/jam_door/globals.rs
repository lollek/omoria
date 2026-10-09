use super::{jam, JamContext};
use crate::{
    dungeon::cell::with_global_cells,
    model::{Cave, InventoryItem, Item, ItemType},
};

/// The selected pointer belongs to C's inventory on the single game thread.
/// It is used only after a successful lookup and cleared immediately on deletion.
struct GlobalJam {
    spike: *mut InventoryItem,
}

impl JamContext for GlobalJam {
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
        unsafe { with_global_cells(|cells| cells.read(y, x)) }
    }

    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
        unsafe { with_global_cells(|cells| cells.write(y, x, cell, item)) }
    }

    fn find_spike(&mut self) -> Option<u16> {
        extern "C" {
            fn inventory_find_range(
                item_types: *const u8,
                inner: bool,
                first: *mut *mut InventoryItem,
                count: *mut libc::c_long,
            ) -> bool;
        }
        let mut item_types = [0; 25];
        item_types[0] = ItemType::Spike.into();
        let mut count = 0;
        if unsafe { inventory_find_range(item_types.as_ptr(), false, &mut self.spike, &mut count) }
        {
            unsafe { self.spike.as_ref().map(|spike| spike.data.number) }
        } else {
            None
        }
    }

    fn set_spike_count(&mut self, count: u16) {
        unsafe { (*self.spike).data.number = count }
    }

    fn destroy_spike(&mut self) {
        extern "C" {
            fn inven_destroy(item: *mut InventoryItem);
        }
        unsafe { inven_destroy(self.spike) }
        self.spike = std::ptr::null_mut();
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
        crate::misc::c_string_lossy(&name)
    }

    fn message(&mut self, message: &str) {
        crate::term::msg_print(message);
    }

    fn redraw_stats(&mut self) {
        extern "C" {
            fn prt_stat_block();
        }
        unsafe { prt_stat_block() }
    }
}

pub(super) fn jam_global(y: i64, x: i64) {
    jam(
        &mut GlobalJam {
            spike: std::ptr::null_mut(),
        },
        y,
        x,
    );
}
