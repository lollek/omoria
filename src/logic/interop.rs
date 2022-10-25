use crate::conversion;
use crate::logic::use_item;
use crate::model::Item;

#[no_mangle]
pub extern "C" fn C_class_can_use_item(class: i32, item: *const Item) -> u8 {
    match use_item::class_can_use_item(
        &conversion::class::from_usize(class as usize).unwrap(),
        unsafe { &*item },
    ) {
        true => 255,
        false => 0,
    }
}

#[no_mangle]
pub extern "C" fn C_gain_mana() {
    super::level_up::gain_mana_from_level_up();
}
