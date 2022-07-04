use crate::conversion;
use libc;
use crate::logic::use_item;
use crate::model::Item;

#[no_mangle]
pub extern "C" fn C_class_can_use_item(class: libc::int32_t, item: *const Item) -> libc::uint8_t {
    match use_item::class_can_use_item(
        &conversion::class::from_usize(class as usize).unwrap(),
        unsafe { &*item },
    ) {
        true => 255,
        false => 0,
    }
}

#[no_mangle]
pub extern fn C_gain_mana() {
    super::level_up::gain_mana_from_level_up();
}
