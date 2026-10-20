/// Uses initialized C globals on the exclusive game thread.
/// `cause` must point to a valid NUL-terminated string for the entire call.
#[no_mangle]
pub extern "C" fn corrode_gas(cause: *const libc::c_char) {
    super::globals::corrode_gas(cause);
}

/// Uses initialized C globals on the exclusive game thread.
/// `cause` must point to a valid NUL-terminated string for the entire call.
#[no_mangle]
pub extern "C" fn acid_dam(damage: libc::c_long, cause: *const libc::c_char) {
    super::globals::acid_dam(damage, cause);
}

/// Uses initialized C globals on the exclusive game thread.
/// `cause` must point to a valid NUL-terminated string for the entire call.
#[no_mangle]
pub extern "C" fn poison_gas(damage: libc::c_long, cause: *const libc::c_char) {
    super::globals::poison_gas(damage, cause);
}

/// Uses initialized C globals on the exclusive game thread.
/// `cause` must point to a valid NUL-terminated string for the entire call.
#[no_mangle]
pub extern "C" fn fire_dam(damage: libc::c_long, cause: *const libc::c_char) {
    super::globals::fire_dam(damage, cause);
}

/// Uses initialized C globals on the exclusive game thread.
/// `cause` must point to a valid NUL-terminated string for the entire call.
#[no_mangle]
pub extern "C" fn cold_dam(damage: libc::c_long, cause: *const libc::c_char) {
    super::globals::cold_dam(damage, cause);
}

/// Uses initialized C globals on the exclusive game thread.
/// `cause` must point to a valid NUL-terminated string for the entire call.
#[no_mangle]
pub extern "C" fn light_dam(damage: libc::c_long, cause: *const libc::c_char) {
    super::globals::light_dam(damage, cause);
}
