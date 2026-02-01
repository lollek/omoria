//! Trap placement helpers.

use crate::dungeon::trap::data;
use crate::misc::rs2item_damage;
use crate::model::{Cave, Item};

/// Which legacy trap table to use.
///
/// Maps to the `typ` parameter in the C `place_trap`:
/// - `typ == 1` → `trap_lista` (`TrapList::A`)
/// - `typ == 2` → `trap_listb` (`TrapList::B`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrapList {
    A,
    B,
}

/// Look up a trap template by list type and subval index.
fn template_for(list: TrapList, subval: usize) -> &'static data::TrapTemplate {
    match list {
        TrapList::A => &data::TRAP_LIST_A[subval],
        TrapList::B => &data::TRAP_LIST_B[subval],
    }
}

/// Copy all fields from a `TrapTemplate` into an `Item`.
///
/// This mirrors the C code's direct struct assignment (`t_list[cur_pos] = trap;`).
/// Fields not stored in `TrapTemplate` (because they're always zero for traps)
/// are explicitly set to zero here.
pub(crate) fn apply_template_to_item(item: &mut Item, template: &data::TrapTemplate) {
    debug_assert!(template.tval >= 0 && template.tval <= u8::MAX as i64);
    debug_assert!(template.level >= i8::MIN as i64 && template.level <= i8::MAX as i64);

    // Fields from the template (these vary per trap).
    item.tval = template.tval as u8;
    item.level = template.level as i8;
    item.subval = template.subval;
    item.damage = rs2item_damage(template.damage);
    item.cost = template.cost;

    // Fields always zero for traps (not stored in TrapTemplate).
    item.flags = 0;
    item.flags2 = 0;
    item.weight = 0;
    item.number = 0;
    item.tohit = 0;
    item.todam = 0;
    item.ac = 0;
    item.toac = 0;
    item.p1 = 0;
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
