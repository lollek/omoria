//! TR2 forwarding boundary; effects remain owned by C.

extern "C" {
    fn ht__sleep_gas();
    fn ht__str_dart(damage: libc::c_long);
    fn ht__blind_gas();
    fn ht__confuse_gas();
    fn ht__slow_dart(damage: libc::c_long);
    fn ht__con_dart(damage: libc::c_long);
}

pub(super) fn sleep_gas() {
    unsafe { ht__sleep_gas() }
}

pub(super) fn strength_dart(damage: i64) {
    unsafe { ht__str_dart(damage) }
}

pub(super) fn blind_gas() {
    unsafe { ht__blind_gas() }
}

pub(super) fn confuse_gas() {
    unsafe { ht__confuse_gas() }
}

pub(super) fn slow_dart(damage: i64) {
    unsafe { ht__slow_dart(damage) }
}

pub(super) fn constitution_dart(damage: i64) {
    unsafe { ht__con_dart(damage) }
}
