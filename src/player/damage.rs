//! F-DMG: the C-owned damage and display operations used by acid and corrosion.

use crate::model::{Item, ItemType};

extern "C" {
    #[link_name = "take_hit"]
    fn c_take_hit(damage: libc::c_long, cause: *const libc::c_char);
    #[link_name = "py_bonuses"]
    fn c_py_bonuses(item: *const Item, factor: libc::c_long);
    #[link_name = "prt_stat_block"]
    fn c_prt_stat_block();
    #[link_name = "inven_damage"]
    fn c_inven_damage(types: *const u8, chance: libc::c_long) -> libc::c_long;
    static mut blank_treasure: Item;
}

/// Changes HP, cancels searching/resting, flushes input, and redraws surviving
/// players. Invulnerability blocks HP loss; negative HP sets death/cause flags.
///
/// # Safety
/// Call on the exclusive game thread with initialized C globals and a valid
/// NUL-terminated `cause` string that lives through the call.
pub(crate) unsafe fn take_hit(damage: libc::c_long, cause: *const libc::c_char) {
    c_take_hit(damage, cause);
}

/// Recalculates equipment-derived flags, stats, and armor without applying an
/// item's stat delta. May request stat redraws; does not apply HP damage.
///
/// # Safety
/// Call on the exclusive game thread with initialized player/equipment globals.
pub(crate) unsafe fn recalculate_bonuses() {
    c_py_bonuses(std::ptr::addr_of!(blank_treasure), 0);
}

/// Redraws the stat block using C player globals; does not apply HP damage.
///
/// # Safety
/// Call on the exclusive game thread with initialized player and terminal state.
pub(crate) unsafe fn redraw_stats() {
    c_prt_stat_block();
}

/// Rolls the legacy strict `randint(100) < chance` check for matching pack
/// entries, destroys one item per successful entry, and returns the count.
/// Mutates inventory links/counts/weight and `inven_temp`; does not damage HP.
///
/// # Safety
/// Call on the exclusive game thread with a valid initialized inventory list.
pub(crate) unsafe fn damage_inventory(types: &[ItemType], chance: libc::c_long) -> libc::c_long {
    let mut codes: Vec<u8> = types.iter().copied().map(u8::from).collect();
    codes.push(0);
    c_inven_damage(codes.as_ptr(), chance)
}
