use libc;

use player;

#[no_mangle]
pub extern fn C_magic_spell_level(slot: libc::int32_t) -> libc::uint8_t {
    super::spell::from_usize_or_blank(player::class(), slot as usize).level
}

#[no_mangle]
pub extern fn C_magic_spell_mana(slot: libc::int32_t) -> libc::uint8_t {
    super::spell::from_usize_or_blank(player::class(), slot as usize).mana
}

#[no_mangle]
pub extern fn C_magic_spell_failchance(slot: libc::int32_t) -> libc::uint8_t {
    super::spell::from_usize_or_blank(player::class(), slot as usize).fail
}