//! Test helpers for trap placement.
//!
#![cfg(test)]

use crate::dungeon::trap::globals::{
    TEST_ALLOC_INDEX, TEST_CAVE, TEST_T_LIST, TEST_LITE_SPOT_CALLED, TEST_PUSHT_CALLED,
    TEST_LAST_LITE_SPOT_YX, TEST_LAST_PUSHT_INDEX,
};
use crate::model::{Cave, Item};

use crate::dungeon::trap::data::TrapTemplate;
use crate::dungeon::trap::placement::apply_template_to_item;

pub unsafe fn set_next_alloc_index(index: u8) {
    TEST_ALLOC_INDEX = index;
}

pub unsafe fn clear_tile(y: usize, x: usize) {
    TEST_CAVE[y][x].tptr = 0;
}

pub unsafe fn set_tile_tptr(y: usize, x: usize, index: u8) {
    TEST_CAVE[y][x].tptr = index;
}

pub unsafe fn read_tile(y: usize, x: usize) -> Cave {
    TEST_CAVE[y][x]
}

pub unsafe fn read_item(index: u8) -> Item {
    TEST_T_LIST[index as usize]
}

pub unsafe fn write_item_tval_subval(index: u8, tval: u8, subval: i64) {
    let item = &mut TEST_T_LIST[index as usize];
    item.tval = tval;
    item.subval = subval;
}

pub unsafe fn write_item_from_template(index: u8, tpl: &TrapTemplate) {
    apply_template_to_item(&mut TEST_T_LIST[index as usize], tpl);
}

pub unsafe fn reset_side_effect_counters() {
    TEST_PUSHT_CALLED = 0;
    TEST_LITE_SPOT_CALLED = 0;
    TEST_LAST_PUSHT_INDEX = 0;
    TEST_LAST_LITE_SPOT_YX = (0, 0);
}

pub unsafe fn pusht_called() -> usize {
    TEST_PUSHT_CALLED
}

pub unsafe fn lite_spot_called() -> usize {
    TEST_LITE_SPOT_CALLED
}

pub unsafe fn last_pusht_index() -> u8 {
    TEST_LAST_PUSHT_INDEX
}

pub unsafe fn last_lite_spot_yx() -> (usize, usize) {
    TEST_LAST_LITE_SPOT_YX
}
