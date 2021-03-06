use libc;

use types::Class;
use types::item::Item;

#[no_mangle]
pub extern fn C_class_melee_bonus(class: libc::int32_t) -> libc::int8_t {
    Class::from(class as usize).melee_bonus()
}

#[no_mangle]
pub extern fn C_class_ranged_bonus(class: libc::int32_t) -> libc::int8_t {
    Class::from(class as usize).ranged_bonus()
}

#[no_mangle]
pub extern fn C_class_can_use_item(class: libc::int32_t, item: *const Item) -> libc::uint8_t {
    match Class::from(class as usize).can_use_item(unsafe { &*item }) {
        true => 255,
        false => 0,
    }
}
