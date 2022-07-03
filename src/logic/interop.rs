use conversion;
use libc;
use logic::use_item;
use model::Item;

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
