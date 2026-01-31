use libc;

use crate::rng;

#[no_mangle]
pub extern "C" fn randint(maxval: libc::c_long) -> libc::c_long {
    rng::randint(maxval)
}

#[no_mangle]
pub extern "C" fn rand_rep(num: libc::c_long, die: libc::c_long) -> libc::c_long {
    rng::rand_rep(num, die)
}

#[no_mangle]
pub extern "C" fn randnor(mean: libc::c_long, stand: libc::c_long) -> libc::c_long {
    rng::randnor(mean, stand)
}
