
#[no_mangle]
pub extern fn squish_stat(stat: i32) -> u8 {
    if stat > 250 {
        250
    } else if stat < 0 {
        0
    } else {
        stat as u8
    }
}
