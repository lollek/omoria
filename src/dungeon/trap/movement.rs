//! TR4 forwarding boundary; effects remain owned by C.

extern "C" {
    fn ht__covered_pit(damage: libc::c_long, y: libc::c_long, x: libc::c_long);
    fn ht__trap_door(damage: libc::c_long);
    fn ht__teleport();
    fn ht__chute(damage: libc::c_long);
    fn ht__whirlpool(damage: libc::c_long);
}

pub(super) fn covered_pit(damage: i64, y: i64, x: i64) {
    unsafe { ht__covered_pit(damage, y, x) }
}

pub(super) fn trap_door(damage: i64) {
    unsafe { ht__trap_door(damage) }
}

pub(super) fn teleport() {
    unsafe { ht__teleport() }
}

pub(super) fn chute(damage: i64) {
    unsafe { ht__chute(damage) }
}

pub(super) fn whirlpool(damage: i64) {
    unsafe { ht__whirlpool(damage) }
}
