use libc;

use magic;


#[no_mangle]
pub extern fn C_magic_spell_level(slot: libc::int32_t) -> libc::uint8_t {
    magic::spell(slot as usize).level
}

#[no_mangle]
pub extern fn C_magic_spell_mana(slot: libc::int32_t) -> libc::uint8_t {
    magic::spell(slot as usize).mana
}

#[no_mangle]
pub extern fn C_magic_spell_failchance(slot: libc::int32_t) -> libc::uint8_t {
    magic::spell(slot as usize).fail
}

#[no_mangle]
pub extern fn C_gain_mana() {
    magic::gain_mana_from_level_up();
}
