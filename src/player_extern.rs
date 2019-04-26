use libc;

use player;
use types::{ Magic, Stat };

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

#[no_mangle]
pub extern fn C_player_add_exp(num: libc::c_long) {
    player::add_experience(num);
}

#[no_mangle]
pub extern fn C_player_recalc_stats() {
    player::recalc_curr_stats();
}

#[no_mangle]
pub extern fn C_player_max_bulk() -> libc::uint16_t {
    player::max_bulk()
}

#[no_mangle]
pub extern fn C_player_dmg_from_str() -> libc::int16_t {
    player::dmg_from_str()
}

#[no_mangle]
pub extern fn C_player_disarm_from_dex() -> libc::int16_t {
    player::disarm_from_dex()
}

#[no_mangle]
pub extern fn C_player_get_stat(stat: libc::uint8_t) -> libc::int16_t {
    player::get_stat(Stat::from(stat))
}

#[no_mangle]
pub extern fn C_player_mod_from_stat(stat: libc::uint8_t) -> libc::int16_t {
    player::modifier_from_stat(Stat::from(stat))
}

#[no_mangle]
pub extern fn C_player_hp_from_con() -> libc::int16_t {
    player::hp_from_con()
}

#[no_mangle]
pub extern fn C_player_cost_modifier_from_charisma() -> libc::c_float {
    player::cost_modifier_from_charisma()
}

#[no_mangle]
pub extern fn C_player_tohit_from_stats() -> libc::int16_t {
    player::tohit_from_stats()
}

#[no_mangle]
pub extern fn C_player_ac_from_dex() -> libc::int16_t {
    player::ac_from_dex()
}

#[no_mangle]
pub extern fn C_player_modify_lost_stat(stat: libc::uint8_t, amount: libc::int16_t) {
    player::modify_lost_stat(Stat::from(stat), amount);
}

#[no_mangle]
pub extern fn C_player_reset_lost_stat(stat: libc::uint8_t) {
    player::reset_lost_stat(Stat::from(stat));
}

#[no_mangle]
pub extern fn C_player_has_lost_stat(stat: libc::uint8_t) -> libc::uint8_t {
    match player::has_lost_stat(Stat::from(stat)) {
        true => 255,
        false => 0,
    }
}
