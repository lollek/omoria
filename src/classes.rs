use libc;

use types::{ Class };

#[no_mangle]
pub extern fn C_class_melee_bonus(class: libc::int32_t) -> libc::int8_t {
    Class::from(class as usize).melee_bonus()
}

#[no_mangle]
pub extern fn C_class_ranged_bonus(class: libc::int32_t) -> libc::int8_t {
    Class::from(class as usize).ranged_bonus()
}
