//! C spell effects used by item-use actions; signatures match `spells.h`.
//!
//! Calls require exclusive access to initialized game globals. Stat and spell
//! kinds must be valid C enum values. String pointers must remain valid and
//! NUL-terminated for the call; mutable counter pointers must be exclusive.

/// C `spell_effect_t` selector for visible monster detection.
pub(crate) const SE_MONSTER: libc::c_int = 21;

extern "C" {
    /// Gains or restores a stat, recalculates bonuses, and returns true.
    /// `"X"` selects the default stat-specific message.
    pub(crate) fn gain_stat(stat: libc::c_int, message: *const libc::c_char) -> bool;
    /// Drains a stat unless sustained, prints the appropriate message, and
    /// returns true. `"X"` selects the default messages for either outcome.
    pub(crate) fn lose_stat(
        stat: libc::c_int,
        message: *const libc::c_char,
        second_message: *const libc::c_char,
    ) -> bool;
    /// Restores a drained stat if needed and returns true even if unchanged.
    /// `"X"` selects the default restoration message.
    pub(crate) fn restore_stat(stat: libc::c_int, message: *const libc::c_char) -> bool;
    /// Applies an HP change and its messages; returns the legacy spell result.
    /// `cause` is the damage description used for a negative amount.
    pub(crate) fn hp_player(amount: libc::c_long, cause: *const libc::c_char) -> bool;
    /// Reveals matching creatures on the current panel, reports the result,
    /// and returns whether any were detected.
    pub(crate) fn detect_creatures(kind: libc::c_int) -> bool;
    /// Reduces a supported status counter to one, returning whether it changed.
    /// `counter` must point to a live, writable player status counter.
    pub(crate) fn cure_player_status_effect(counter: *mut i64) -> bool;
    /// Removes experience through C level/stat recalculation and UI handling.
    pub(crate) fn lose_exp(amount: libc::c_long);
    /// Attempts drained-experience restoration using RNG and reports success.
    pub(crate) fn restore_player_drained_levels() -> bool;
    /// Adds to the timed see-invisible counter and returns true.
    pub(crate) fn detect_inv2(amount: libc::c_long) -> bool;
    /// Halves active poison (minimum one), reports it, and returns whether active.
    pub(crate) fn slow_poison() -> bool;
}
