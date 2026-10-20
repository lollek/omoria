//! TR3 forwarding boundary; effects remain owned by C.

extern "C" {
    fn ht__corrode_gas();
    fn ht__fire(damage: libc::c_long);
    fn ht__acid(damage: libc::c_long);
    fn ht__poison_gas(damage: libc::c_long);
}

pub(super) fn corrode_gas() {
    unsafe { ht__corrode_gas() }
}

pub(super) fn fire(damage: i64) {
    unsafe { ht__fire(damage) }
}

pub(super) fn acid(damage: i64) {
    unsafe { ht__acid(damage) }
}

pub(super) fn poison_gas(damage: i64) {
    unsafe { ht__poison_gas(damage) }
}
