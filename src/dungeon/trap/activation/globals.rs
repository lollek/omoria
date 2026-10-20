use super::super::{basic::TrapEffects, TRAP_BASE_TO_HIT};
use crate::{
    dungeon::cell::with_global_cells,
    model::{ActivationKind, Item, PlayerFlags, StoreKind},
};

extern "C" {
    static mut search_flag: bool;
    static mut find_flag: bool;
    static mut player_flags: PlayerFlags;
    static mut char_row: libc::c_long;
    static mut char_col: libc::c_long;
    fn search_off();
    fn rest_off();
    fn change_trap(y: libc::c_long, x: libc::c_long);
    fn lite_spot(y: libc::c_long, x: libc::c_long);
    fn damroll(damage: *const libc::c_char) -> libc::c_long;
    fn player_pac() -> i16;
    fn player_ptoac() -> i16;
    fn managed_to_hit(
        base: libc::c_long,
        level: libc::c_long,
        bonus: libc::c_long,
        armor: libc::c_long,
    ) -> bool;
    fn check_store_hours_and_enter(store: libc::c_int, visual: libc::c_long);
    fn check_store_hours(store: libc::c_int, visual: libc::c_long) -> bool;
    fn enter_trading_post();
    fn enter_bank();
    fn enter_casino();
    fn enter_fortress();
    fn enter_house(y: libc::c_long, x: libc::c_long);
}

pub(super) struct GlobalTrap;

impl TrapEffects for GlobalTrap {
    fn feather_fall(&self) -> bool {
        unsafe { player_flags.ffall != 0 }
    }

    fn arrow_hits(&mut self) -> bool {
        unsafe {
            managed_to_hit(
                TRAP_BASE_TO_HIT,
                0,
                0,
                i64::from(player_pac()) + i64::from(player_ptoac()),
            )
        }
    }

    fn damage(&mut self, damage: i64, cause: &str) {
        let cause = std::ffi::CString::new(cause).expect("trap damage causes contain no NUL bytes");
        unsafe { crate::player::damage::take_hit(damage, cause.as_ptr()) }
    }

    fn message(&mut self, message: &'static str) {
        crate::term::msg_print(message);
    }
}

impl GlobalTrap {
    pub(super) fn message(&mut self, message: &'static str) {
        TrapEffects::message(self, message);
    }

    pub(super) fn store(&mut self, kind: StoreKind) {
        let store = libc::c_int::from(kind);
        unsafe { check_store_hours_and_enter(store, i64::from(store)) }
    }

    pub(super) fn store_open(&mut self, kind: StoreKind) -> bool {
        unsafe { check_store_hours(libc::c_int::from(kind), -1) }
    }

    pub(super) fn trading_post(&mut self) {
        unsafe { enter_trading_post() }
    }

    pub(super) fn bank(&mut self) {
        unsafe { enter_bank() }
    }

    pub(super) fn casino(&mut self) {
        unsafe { enter_casino() }
    }

    pub(super) fn fortress(&mut self) {
        unsafe { enter_fortress() }
    }

    pub(super) fn house(&mut self, y: i64, x: i64) {
        unsafe { enter_house(y, x) }
    }
}

unsafe fn read_item(y: i64, x: i64) -> Option<Item> {
    with_global_cells(|cells| cells.read(y, x).and_then(|(_, item)| item))
}

/// # Safety
/// Non-null pointers must be aligned and readable for one C long. All C globals
/// must be initialized and exclusively accessed on the game thread.
pub(in crate::dungeon::trap) unsafe fn hit_trap_global(
    y: *const libc::c_long,
    x: *const libc::c_long,
) {
    if y.is_null() || x.is_null() {
        crate::debug::error("hit_trap: null coordinate pointer");
        return;
    }
    let (y, x) = (*y, *x);
    let Some(item) = read_item(y, x) else {
        crate::debug::error(format!("hit_trap: invalid trap cell ({y}, {x})"));
        return;
    };
    if (item.tval as i64 == super::super::data::TVAL_UNSEEN_TRAP
        || item.tval as i64 == super::super::data::TVAL_SECRET_DOOR)
        && !(1..=20).contains(&item.subval)
    {
        crate::debug::error(format!(
            "hit_trap: invalid hidden trap subval {}",
            item.subval
        ));
        return;
    }

    if search_flag {
        search_off();
    }
    if player_flags.rest > 0 {
        rest_off();
    }
    change_trap(y, x);
    lite_spot(char_row, char_col);
    find_flag = false;

    // Release global borrows before callbacks, and read the replacement after reveal.
    let Some(item) = read_item(y, x) else {
        crate::debug::error(format!("hit_trap: missing revealed trap at ({y}, {x})"));
        return;
    };
    if !item.damage.contains(&0) {
        crate::debug::error(format!(
            "hit_trap: unterminated damage string at ({y}, {x})"
        ));
        return;
    }
    let damage = damroll(item.damage.as_ptr());
    // C reads the subval after damroll, rather than from the earlier snapshot.
    let Some(item) = read_item(y, x) else {
        crate::debug::error(format!(
            "hit_trap: missing trap after damage roll at ({y}, {x})"
        ));
        return;
    };
    super::dispatch(
        &mut GlobalTrap,
        ActivationKind::from(item.subval),
        damage,
        y,
        x,
    );
}
