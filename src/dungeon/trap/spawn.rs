//! TR5 forwarding boundary; effects remain owned by C.

extern "C" {
    fn ht__hidden_object(y: libc::c_long, x: libc::c_long);
    fn ht__rockfall(damage: libc::c_long, y: libc::c_long, x: libc::c_long);
    fn ht__summon_monster(y: libc::c_long, x: libc::c_long);
}

pub(super) fn hidden_object(y: i64, x: i64) {
    unsafe { ht__hidden_object(y, x) }
}

pub(super) fn rockfall(damage: i64, y: i64, x: i64) {
    unsafe { ht__rockfall(damage, y, x) }
}

pub(super) fn summon_monster(y: i64, x: i64) {
    unsafe { ht__summon_monster(y, x) }
}
