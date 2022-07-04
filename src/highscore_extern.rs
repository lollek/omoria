use libc;

use crate::highscore;


#[no_mangle]
pub extern fn C_highscore(max: libc::uint8_t) {
    highscore::highscore(max);
}
