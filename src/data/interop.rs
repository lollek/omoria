use libc;

use crate::conversion;
use crate::data;
use crate::model::Item;

#[no_mangle]
pub extern "C" fn C_class_melee_bonus(class: libc::int32_t) -> libc::int8_t {
    data::class::melee_bonus(&conversion::class::from_usize(class as usize).unwrap())
}

#[no_mangle]
pub extern "C" fn C_class_ranged_bonus(class: libc::int32_t) -> libc::int8_t {
    data::class::ranged_bonus(&conversion::class::from_usize(class as usize).unwrap())
}

#[no_mangle]
pub extern "C" fn C_item_get_tchar(item_ptr: *const Item) -> pancurses::chtype {

    let item = unsafe { *item_ptr };
    let item_type = conversion::item_type::from_usize(item.tval.into()).unwrap();
    data::item_type::symbol(&item_type, item.subval)
}

#[no_mangle]
pub extern "C" fn coin_value(currency: libc::uint8_t) -> libc::int64_t {
    data::currency::value(&conversion::currency::from_usize(currency.into()).unwrap())
}
