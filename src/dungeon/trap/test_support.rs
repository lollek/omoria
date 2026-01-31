//! Test helpers for trap placement.
//!
#![cfg(test)]

use crate::dungeon::trap::globals::{TEST_ALLOC_INDEX, TEST_CAVE, TEST_T_LIST};
use crate::model::{Cave, Item};

pub unsafe fn set_next_alloc_index(index: u8) {
    TEST_ALLOC_INDEX = index;
}

pub unsafe fn clear_tile(y: usize, x: usize) {
    TEST_CAVE[y][x].tptr = 0;
}

pub unsafe fn read_tile(y: usize, x: usize) -> Cave {
    TEST_CAVE[y][x]
}

pub unsafe fn read_item(index: u8) -> Item {
    TEST_T_LIST[index as usize]
}
