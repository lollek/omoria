use libc;

use player;
use types::Magic;

#[no_mangle]
pub extern fn C_player_knows_spell(slot: libc::int32_t) -> libc::uint8_t {
    match player::knows_spell(slot as usize) {
        true => 255,
        false => 0,
    }
}

#[no_mangle]
pub extern fn C_player_set_knows_spell(slot: libc::int32_t, yn: libc::uint8_t) {
    player::set_knows_spell(slot as usize, yn != 0);
}

#[no_mangle]
pub extern fn C_player_uses_magic(magic_type: libc::int32_t) -> libc::uint8_t {
    match player::uses_magic(Magic::from(magic_type)) {
        true => 255,
        false => 0
    }
}

