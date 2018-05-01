
#[repr(C)]
pub struct p_misc {
    pub xtr_wgt: i64,
    pub account: i64,
    pub money: i64,
}

#[no_mangle]
pub extern fn cc__get_stats_(player_misc: *mut p_misc) {
}
