use crate::highscore;


#[no_mangle]
pub extern fn C_highscore(max: u8) {
    highscore::highscore(max);
}
