use libc;

use debug;
use identification;

#[no_mangle]
pub extern fn is_type_identified(item_type: libc::uint8_t, subval: libc::int64_t) -> libc::uint8_t {
    identification::is_item_type_identified(item_type, subval) ? 255 : 0
}

#[no_mangle]
pub extern fn set_type_identified(item_type: libc::uint8_t, subval: libc::int64_t, is_identified: libc::uint8_t) {
    identification::set_item_type_identified(item_type, subval, is_identified == 0 ? false : true);
}
