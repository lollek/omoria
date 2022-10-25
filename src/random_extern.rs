use libc;

use crate::random;

#[no_mangle]
pub extern "C" fn randint(maxval: libc::c_long) -> libc::c_long {
    random::randint(maxval)
}

#[no_mangle]
pub extern "C" fn rand_rep(num: libc::c_long, die: libc::c_long) -> libc::c_long {
    random::rand_rep(num, die)
}

#[no_mangle]
pub extern "C" fn randnor(mean: libc::c_long, stand: libc::c_long) -> libc::c_long {
    random::randnor(mean, stand)
}
