use std::convert::TryInto;

use libc;

use crate::data;
use crate::player;
use crate::conversion;

#[no_mangle]
pub extern "C" fn C_player_knows_spell(slot: i32) -> u8 {
    match player::knows_spell(slot as usize) {
        true => 255,
        false => 0,
    }
}

#[no_mangle]
pub extern "C" fn C_player_set_knows_spell(slot: i32, yn: u8) {
    player::set_knows_spell(slot as usize, yn != 0);
}

#[no_mangle]
pub extern "C" fn C_player_uses_magic(magic_type: i32) -> u8 {
    if data::class::magic_type(&player::class())
        == Some(conversion::magic::from_usize(magic_type.try_into().unwrap()).unwrap())
    {
        255
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn C_player_add_exp(num: libc::c_long) {
    player::add_experience(num);
}

#[no_mangle]
pub extern "C" fn C_player_recalc_stats() {
    player::recalc_curr_stats();
}

#[no_mangle]
pub extern "C" fn C_player_max_bulk() -> u16 {
    player::max_bulk()
}

#[no_mangle]
pub extern "C" fn C_player_dmg_from_str() -> i16 {
    player::dmg_from_str()
}

#[no_mangle]
pub extern "C" fn C_player_disarm_from_dex() -> i16 {
    player::disarm_from_dex()
}

#[no_mangle]
pub extern "C" fn C_player_get_stat(stat: u8) -> i16 {
    player::get_stat(conversion::stat::from_usize(stat.into()).unwrap())
}

#[no_mangle]
pub extern "C" fn C_player_mod_from_stat(stat: u8) -> i16 {
    player::modifier_from_stat(conversion::stat::from_usize(stat.into()).unwrap())
}

#[no_mangle]
pub extern "C" fn C_player_hp_from_con() -> i16 {
    player::hp_from_con()
}

#[no_mangle]
pub extern "C" fn C_player_cost_modifier_from_charisma() -> libc::c_float {
    player::cost_modifier_from_charisma()
}

#[no_mangle]
pub extern "C" fn C_player_tohit_from_stats() -> i16 {
    player::tohit_from_stats()
}

#[no_mangle]
pub extern "C" fn C_player_ac_from_dex() -> i16 {
    player::ac_from_dex()
}

#[no_mangle]
pub extern "C" fn C_player_modify_lost_stat(stat: u8, amount: i16) {
    player::modify_lost_stat(conversion::stat::from_usize(stat.into()).unwrap(), amount);
}

#[no_mangle]
pub extern "C" fn C_player_reset_lost_stat(stat: u8) {
    player::reset_lost_stat(conversion::stat::from_usize(stat.into()).unwrap());
}

#[no_mangle]
pub extern "C" fn C_player_has_lost_stat(stat: u8) -> u8 {
    match player::has_lost_stat(conversion::stat::from_usize(stat.into()).unwrap()) {
        true => 255,
        false => 0,
    }
}

#[no_mangle]
pub extern "C" fn C_player_mod_stat(stat: u8, modifier: i16) {
    player::mod_stat(conversion::stat::from_usize(stat.into()).unwrap(), modifier);
}

#[no_mangle]
pub extern "C" fn C_player_mod_perm_stat(stat: u8, modifier: i16) {
    player::mod_perm_stat(conversion::stat::from_usize(stat.into()).unwrap(), modifier);
}

#[no_mangle]
pub extern "C" fn C_player_roll_hp_for_levelup() -> i16 {
    player::roll_hp_for_levelup()
}

#[no_mangle]
pub extern "C" fn C_player_change_extra_bulk_carry(modifier: i16) {
    player::modify_extra_bulk_carry(modifier);
}

#[no_mangle]
pub extern "C" fn C_player_mod_search_skill(modifier: i16) {
    player::mod_search_skill(modifier);
}

#[no_mangle]
pub extern "C" fn C_player_curr_search_skill() -> i16 {
    player::curr_search_skill()
}

#[no_mangle]
pub extern "C" fn C_player_current_hp() -> i16 {
    player::current_hp()
}

#[no_mangle]
pub extern "C" fn C_player_max_hp() -> i16 {
    player::max_hp()
}

#[no_mangle]
pub extern "C" fn C_player_reset_current_hp() {
    player::reset_current_hp()
}

#[no_mangle]
pub extern "C" fn C_player_modify_max_hp(modifier: i16) {
    player::modify_max_hp(modifier);
}

#[no_mangle]
pub extern "C" fn C_player_modify_current_hp(modifier: libc::c_float) {
    player::modify_current_hp(modifier);
}

#[no_mangle]
pub extern "C" fn C_player_regen_hp(percent: libc::c_float) {
    player::regen_hp(percent);
}
