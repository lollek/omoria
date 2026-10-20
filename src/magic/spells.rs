//! C spell effects used by item-use actions; signatures match `spells.h`.

pub(crate) const SE_MONSTER: libc::c_int = 21;

extern "C" {
    pub(crate) fn gain_stat(stat: libc::c_int, message: *const libc::c_char) -> bool;
    pub(crate) fn lose_stat(
        stat: libc::c_int,
        message: *const libc::c_char,
        second_message: *const libc::c_char,
    ) -> bool;
    pub(crate) fn restore_stat(stat: libc::c_int, message: *const libc::c_char) -> bool;
    pub(crate) fn hp_player(amount: libc::c_long, cause: *const libc::c_char) -> bool;
    pub(crate) fn detect_creatures(kind: libc::c_int) -> bool;
    pub(crate) fn cure_player_status_effect(counter: *mut i64) -> bool;
    pub(crate) fn lose_exp(amount: libc::c_long);
    pub(crate) fn restore_player_drained_levels() -> bool;
    pub(crate) fn detect_inv2(amount: libc::c_long) -> bool;
    pub(crate) fn slow_poison() -> bool;
}
