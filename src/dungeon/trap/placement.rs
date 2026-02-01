//! Trap placement helpers.

use crate::dungeon::trap::data;
use crate::misc::rs2item_damage;
use crate::model::{Cave, Item};

/// Which legacy trap table to use.
///
/// This maps to the `typ` parameter in the C version of `place_trap`:
/// - `typ == 1` → `trap_lista` (A)
/// - `typ == 2` → `trap_listb` (B)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrapList {
    A,
    B,
}

fn template_for(list: TrapList, subval: usize) -> &'static data::TrapTemplate {
    match list {
        TrapList::A => &data::TRAP_LIST_A[subval],
        TrapList::B => &data::TRAP_LIST_B[subval],
    }
}

pub(crate) fn apply_template_to_item(item: &mut Item, template: &data::TrapTemplate) {
    // Port the legacy `treasure_type` assignment used by C `place_trap`.
    // This intentionally does not try to “fix” any legacy truncations.
    debug_assert!(template.tval >= 0 && template.tval <= u8::MAX as i64);
    debug_assert!(template.level >= i8::MIN as i64 && template.level <= i8::MAX as i64);
    debug_assert!(template.weight >= 0 && template.weight <= u16::MAX as i64);
    debug_assert!(template.number >= 0 && template.number <= u16::MAX as i64);
    debug_assert!(template.tohit >= i16::MIN as i64 && template.tohit <= i16::MAX as i64);
    debug_assert!(template.todam >= i16::MIN as i64 && template.todam <= i16::MAX as i64);
    debug_assert!(template.ac >= i16::MIN as i64 && template.ac <= i16::MAX as i64);
    debug_assert!(template.toac >= i16::MIN as i64 && template.toac <= i16::MAX as i64);

    item.tval = template.tval as u8;
    item.flags = template.flags as u64;
    item.flags2 = template.flags2 as u64;
    item.level = template.level as i8;
    item.weight = template.weight as u16;
    item.subval = template.subval;
    item.number = template.number as u16;
    item.tohit = template.tohit as i16;
    item.todam = template.todam as i16;
    item.ac = template.ac as i16;
    item.toac = template.toac as i16;
    item.p1 = template.p1;
    item.cost = template.cost;
    item.damage = rs2item_damage(template.damage);
}

/// Place a trap template into the provided cave tile and `t_list` slot.
///
/// This is a Rust-port-friendly, side-effect-minimized variant of the C `place_trap`.
///
/// In the legacy C implementation, `subval` is 1-based and index 0 is reserved.
pub fn place_trap_into_lists(
    tile: &mut Cave,
    t_list: &mut [Item],
    alloc_index: u8,
    list: TrapList,
    subval: usize,
) {
    let template = template_for(list, subval);

    tile.tptr = alloc_index;

    let idx = alloc_index as usize;
    apply_template_to_item(&mut t_list[idx], template);
}
