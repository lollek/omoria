extern crate rand;

use random::rand::Rng;

#[no_mangle]
pub extern fn randint(maxval: i64) -> i64 {
    if maxval > 0 {
        rand::thread_rng().gen_range(0, maxval) + 1
    } else {
        0
    }
}

#[no_mangle]
pub extern fn rand_rep(num: i64, die: i64) -> i64 {
    (0..num).fold(0, |sum, _val| sum + randint(die))
}

#[no_mangle]
pub extern fn randnor(mean: i64, stand: i64) -> i64 {
    ((
            (-2.0 * (randint(9999999) as f64 / 10000000.0).ln()).sqrt() *
            (6.283 * (randint(9999999) as f64 / 10000000.0)).cos() *
            stand as f64
     ) + mean as f64) as i64
}