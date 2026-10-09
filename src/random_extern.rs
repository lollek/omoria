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

#[no_mangle]
pub extern "C" fn C_seeded_rng_begin(seed: u64) {
    rng::begin_c_seeded_rng(seed)
}

#[no_mangle]
pub extern "C" fn C_seeded_rng_end() {
    rng::end_c_seeded_rng()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_seeded_rng_begin_drives_c_randint_until_end() {
        let expected: Vec<libc::c_long> = {
            let _guard = rng::use_seeded_rng(9);
            (0..4).map(|_| randint(1_000)).collect()
        };
        C_seeded_rng_begin(9);
        let actual: Vec<libc::c_long> = (0..4).map(|_| randint(1_000)).collect();
        C_seeded_rng_end();
        assert_eq!(actual, expected);
        assert_eq!(rng::active_seeded_rng_count(), 0);
    }

    #[test]
    fn c_seeded_rng_end_without_begin_is_a_no_op() {
        C_seeded_rng_end();
        assert_eq!(rng::active_seeded_rng_count(), 0);
    }

    #[test]
    fn c_seeded_rng_end_restores_outer_seed_in_lifo_order() {
        let expected: Vec<libc::c_long> = {
            let _guard = rng::use_seeded_rng(1);
            (0..3).map(|_| randint(1_000)).collect()
        };
        C_seeded_rng_begin(1);
        C_seeded_rng_begin(2);
        C_seeded_rng_end();
        let actual: Vec<libc::c_long> = (0..3).map(|_| randint(1_000)).collect();
        C_seeded_rng_end();
        assert_eq!(actual, expected);
    }
}
