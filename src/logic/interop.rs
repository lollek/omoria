use crate::conversion;
use crate::logic::use_item;
use crate::model::Item;

#[no_mangle]
pub extern "C" fn C_class_can_use_item(class: libc::c_int, item: *const Item) -> bool {
    use_item::class_can_use_item(
        &conversion::class::from_usize(class as usize).unwrap(),
        unsafe { &*item })
}

#[no_mangle]
pub extern "C" fn C_gain_mana() {
    super::level_up::gain_mana_from_level_up();
}
