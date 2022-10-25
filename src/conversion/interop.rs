use crate::player;

#[no_mangle]
pub extern fn C_magic_spell_level(slot: i32) -> u8 {
    super::spell::from_usize_or_blank(player::class(), slot as usize).level
}

#[no_mangle]
pub extern fn C_magic_spell_mana(slot: i32) -> u8 {
    super::spell::from_usize_or_blank(player::class(), slot as usize).mana
}

#[no_mangle]
pub extern fn C_magic_spell_failchance(slot: i32) -> u8 {
    super::spell::from_usize_or_blank(player::class(), slot as usize).fail
}
