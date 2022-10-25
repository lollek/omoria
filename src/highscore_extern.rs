use crate::highscore;

#[no_mangle]
pub extern "C" fn C_highscore(max: u8) {
    highscore::highscore(max);
}
