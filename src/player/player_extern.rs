use std::convert::TryInto;

use libc;

use crate::conversion;
use crate::data;
use crate::debug;
use crate::player;

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
    if data::class::magic_type(&player::class()) == Some(conversion::magic::from_usize(magic_type.try_into().unwrap()).unwrap()) {
        255
    } else {
        0
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
    player::get_stat(conversion::stat::from_usize(stat.into()).unwrap())
}

#[no_mangle]
pub extern fn C_player_mod_from_stat(stat: libc::uint8_t) -> libc::int16_t {
    player::modifier_from_stat(conversion::stat::from_usize(stat.into()).unwrap())
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
    player::modify_lost_stat(conversion::stat::from_usize(stat.into()).unwrap(), amount);
}

#[no_mangle]
pub extern fn C_player_reset_lost_stat(stat: libc::uint8_t) {
    player::reset_lost_stat(conversion::stat::from_usize(stat.into()).unwrap());
}

#[no_mangle]
pub extern fn C_player_has_lost_stat(stat: libc::uint8_t) -> libc::uint8_t {
    match player::has_lost_stat(conversion::stat::from_usize(stat.into()).unwrap()) {
        true => 255,
        false => 0,
    }
}

#[no_mangle]
pub extern fn C_player_mod_stat(stat: libc::uint8_t, modifier: libc::int16_t) {
    player::mod_stat(conversion::stat::from_usize(stat.into()).unwrap(), modifier);
}

#[no_mangle]
pub extern fn C_player_mod_perm_stat(stat: libc::uint8_t, modifier: libc::int16_t) {
    player::mod_perm_stat(conversion::stat::from_usize(stat.into()).unwrap(), modifier);
}

#[no_mangle]
pub extern fn C_player_roll_hp_for_levelup() -> libc::int16_t {
    player::roll_hp_for_levelup()
}

#[no_mangle]
pub extern fn C_player_set_extra_bulk_carry(new_value: libc::uint16_t) {
    player::set_extra_bulk_carry(new_value);
}

#[no_mangle]
pub extern fn C_player_mod_search_skill(modifier: libc::int16_t) {
    player::mod_search_skill(modifier);
}

#[no_mangle]
pub extern fn C_player_curr_search_skill() -> libc::int16_t {
    player::curr_search_skill()
}

#[no_mangle]
pub extern fn C_player_current_hp() -> libc::int16_t {
    player::current_hp()
}

#[no_mangle]
pub extern fn C_player_max_hp() -> libc::int16_t {
    player::max_hp()
}

#[no_mangle]
pub extern fn C_player_reset_current_hp() {
    player::reset_current_hp()
}

#[no_mangle]
pub extern fn C_player_modify_max_hp(modifier: libc::int16_t) {
    player::modify_max_hp(modifier);
}

#[no_mangle]
pub extern fn C_player_modify_current_hp(modifier: libc::c_float) {
    player::modify_current_hp(modifier);
}

#[no_mangle]
pub extern fn C_player_regen_hp(percent: libc::c_float) {
    player::regen_hp(percent);
}
