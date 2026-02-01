//! Trap placement helpers.

use crate::dungeon::trap::data;
use crate::misc::rs2item_damage;
use crate::model::{Cave, Item};

/// Which legacy trap list type to use.
///
/// Maps to the `typ` parameter in the C `place_trap`:
/// - `typ == 1` → unseen traps (`TrapList::A`)
/// - `typ == 2` → seen traps (`TrapList::B`)
///
/// The actual trap data comes from a single unified list; only the `tval` differs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrapList {
    /// Unseen/hidden traps (tval = TVAL_UNSEEN_TRAP)
    A,
    /// Seen/visible traps (tval = TVAL_SEEN_TRAP)
    B,
}

/// Look up a trap template by subval.
///
/// The `subval` parameter is 1-based (as passed from C code).
/// This function translates to 0-based indexing for the Rust array.
fn template_for(subval: usize) -> &'static data::TrapTemplate {
    let index = subval - 1;
    &data::TRAP_LIST[index]
}

/// Determine the tval for a trap based on list type and subval.
///
/// Most traps use UNSEEN_TRAP (list A) or SEEN_TRAP (list B).
/// Special cases:
/// - Open pit (subval 1): always TVAL_SEEN_TRAP (always visible)
/// - Closed door (subval 19): always TVAL_CLOSED_DOOR
fn tval_for(list: TrapList, subval: i64) -> i64 {
    if subval == data::SUBVAL_CLOSED_DOOR {
        data::TVAL_CLOSED_DOOR
    } else if subval == data::SUBVAL_OPEN_PIT {
        // Open pits are always visible
        data::TVAL_SEEN_TRAP
    } else {
        match list {
            TrapList::A => data::TVAL_UNSEEN_TRAP,
            TrapList::B => data::TVAL_SEEN_TRAP,
        }
    }
}

/// Copy all fields from a `TrapTemplate` into an `Item`.
///
/// This mirrors the C code's direct struct assignment (`t_list[cur_pos] = trap;`).
/// Fields not stored in `TrapTemplate` (because they're always zero for traps)
/// are explicitly set to zero here.
///
/// The `tval` is passed separately because it depends on the list type (A vs B).
pub(crate) fn apply_template_to_item(item: &mut Item, template: &data::TrapTemplate, tval: i64) {
    debug_assert!(tval >= 0 && tval <= u8::MAX as i64);
    debug_assert!(template.level >= i8::MIN as i64 && template.level <= i8::MAX as i64);

    // Fields from the template (these vary per trap).
    item.tval = tval as u8;
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

/// Apply rubble template to an item.
///
/// Rubble always uses TVAL_RUBBLE.
pub(crate) fn apply_rubble_to_item(item: &mut Item) {
    apply_template_to_item(item, &data::RUBBLE, data::TVAL_RUBBLE);
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
    let template = template_for(subval);
    let tval = tval_for(list, template.subval);

    tile.tptr = alloc_index;

    let idx = alloc_index as usize;
    apply_template_to_item(&mut t_list[idx], template, tval);
}
