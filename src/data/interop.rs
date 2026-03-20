use std::convert::TryFrom;
use crate::data;
use crate::model::{Class, Item, ItemType};

#[no_mangle]
pub extern "C" fn C_class_melee_bonus(class: i32) -> i8 {
    data::class::melee_bonus(&Class::try_from(class).unwrap())
}

#[no_mangle]
pub extern "C" fn C_class_ranged_bonus(class: i32) -> i8 {
    data::class::ranged_bonus(&Class::try_from(class).unwrap())
}

#[no_mangle]
pub extern "C" fn C_item_get_tchar(item_ptr: *const Item) -> pancurses::chtype {
    let item = unsafe { *item_ptr };
    let item_type = ItemType::try_from(item.tval).unwrap();
    data::item_type::symbol(&item_type, item.subval)
}

#[no_mangle]
pub extern "C" fn coin_value(currency: u8) -> i64 {
    data::currency::value(&crate::model::Currency::try_from(currency as usize).unwrap())
}
