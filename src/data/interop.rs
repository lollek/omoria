use conversion;
use data;
use libc;

#[no_mangle]
pub extern "C" fn C_class_melee_bonus(class: libc::int32_t) -> libc::int8_t {
    data::class::melee_bonus(&conversion::class::from_usize(class as usize).unwrap())
}

#[no_mangle]
pub extern "C" fn C_class_ranged_bonus(class: libc::int32_t) -> libc::int8_t {
    data::class::ranged_bonus(&conversion::class::from_usize(class as usize).unwrap())
}