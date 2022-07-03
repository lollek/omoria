use std::convert::TryInto;

use libc;

use conversion;
use data;
use debug;
use player;

#[no_mangle]
pub extern fn C_player_knows_spell(slot: libc::int32_t) -> libc::uint8_t {
    debug::enter("C_player_knows_spell");
    let ret = match player::knows_spell(slot as usize) {
        true => 255,
        false => 0,
    };
    debug::leave("C_player_knows_spell");
    return ret;
}

#[no_mangle]
pub extern fn C_player_set_knows_spell(slot: libc::int32_t, yn: libc::uint8_t) {
    debug::enter("C_player_set_knows_spell");
    player::set_knows_spell(slot as usize, yn != 0);
    debug::leave("C_player_set_knows_spell");
}

#[no_mangle]
pub extern fn C_player_uses_magic(magic_type: libc::int32_t) -> libc::uint8_t {
    debug::enter("C_player_uses_magic");
    let ret = if data::class::magic_type(&player::class()) == Some(conversion::magic::from_usize(magic_type.try_into().unwrap()).unwrap()) {
        255
    } else {
        0
    };
    debug::leave("C_player_uses_magic");
    return ret;
}

#[no_mangle]
pub extern fn C_player_add_exp(num: libc::c_long) {
    debug::enter("C_player_add_exp");
    player::add_experience(num);
    debug::leave("C_player_add_exp");
}

#[no_mangle]
pub extern fn C_player_recalc_stats() {
    debug::enter("C_player_recalc_stats");
    player::recalc_curr_stats();
    debug::leave("C_player_recalc_stats");
}

#[no_mangle]
pub extern fn C_player_max_bulk() -> libc::uint16_t {
    debug::enter("C_player_max_bulk");
    let ret = player::max_bulk();
    debug::leave("C_player_max_bulk");
    return ret;
}

#[no_mangle]
pub extern fn C_player_dmg_from_str() -> libc::int16_t {
    debug::enter("C_player_dmg_from_str");
    let ret = player::dmg_from_str();
    debug::leave("C_player_dmg_from_str");
    return ret;
}

#[no_mangle]
pub extern fn C_player_disarm_from_dex() -> libc::int16_t {
    debug::enter("C_player_disarm_from_dex");
    let ret = player::disarm_from_dex();
    debug::leave("C_player_disarm_from_dex");
    return ret;
}

#[no_mangle]
pub extern fn C_player_get_stat(stat: libc::uint8_t) -> libc::int16_t {
    debug::enter(&format!("C_player_get_stat. stat={}", stat));
    let ret = player::get_stat(conversion::stat::from_usize(stat.into()).unwrap());
    debug::leave("C_player_get_stat");
    return ret;
}

#[no_mangle]
pub extern fn C_player_mod_from_stat(stat: libc::uint8_t) -> libc::int16_t {
    debug::enter("C_player_mod_from_stat");
    let ret = player::modifier_from_stat(conversion::stat::from_usize(stat.into()).unwrap());
    debug::leave("C_player_mod_from_stat");
    return ret;
}

#[no_mangle]
pub extern fn C_player_hp_from_con() -> libc::int16_t {
    debug::enter("C_player_hp_from_con");
    let ret = player::hp_from_con();
    debug::leave("C_player_hp_from_con");
    return ret;
}

#[no_mangle]
pub extern fn C_player_cost_modifier_from_charisma() -> libc::c_float {
    debug::enter("C_player_cost_modifier_from_charisma");
    let ret = player::cost_modifier_from_charisma();
    debug::leave("C_player_cost_modifier_from_charisma");
    return ret;
}

#[no_mangle]
pub extern fn C_player_tohit_from_stats() -> libc::int16_t {
    debug::enter("C_player_tohit_from_stats");
    let ret = player::tohit_from_stats();
    debug::leave("C_player_tohit_from_stats");
    return ret;
}

#[no_mangle]
pub extern fn C_player_ac_from_dex() -> libc::int16_t {
    debug::enter("C_player_ac_from_dex");
    let ret = player::ac_from_dex();
    debug::leave("C_player_ac_from_dex");
    return ret;
}

#[no_mangle]
pub extern fn C_player_modify_lost_stat(stat: libc::uint8_t, amount: libc::int16_t) {
    debug::enter("C_player_modify_lost_stat");
    player::modify_lost_stat(conversion::stat::from_usize(stat.into()).unwrap(), amount);
    debug::leave("C_player_modify_lost_stat");
}

#[no_mangle]
pub extern fn C_player_reset_lost_stat(stat: libc::uint8_t) {
    debug::enter("C_player_reset_lost_stat");
    player::reset_lost_stat(conversion::stat::from_usize(stat.into()).unwrap());
    debug::leave("C_player_reset_lost_stat");
}

#[no_mangle]
pub extern fn C_player_has_lost_stat(stat: libc::uint8_t) -> libc::uint8_t {
    debug::enter("C_player_has_lost_stat");
    let ret = match player::has_lost_stat(conversion::stat::from_usize(stat.into()).unwrap()) {
        true => 255,
        false => 0,
    };
    debug::leave("C_player_has_lost_stat");
    return ret;
}

#[no_mangle]
pub extern fn C_player_mod_stat(stat: libc::uint8_t, modifier: libc::int16_t) {
    debug::enter("C_player_mod_stat");
    player::mod_stat(conversion::stat::from_usize(stat.into()).unwrap(), modifier);
    debug::leave("C_player_mod_stat");
}

#[no_mangle]
pub extern fn C_player_mod_perm_stat(stat: libc::uint8_t, modifier: libc::int16_t) {
    debug::enter("C_player_mod_perm_stat");
    player::mod_perm_stat(conversion::stat::from_usize(stat.into()).unwrap(), modifier);
    debug::leave("C_player_mod_perm_stat");
}

#[no_mangle]
pub extern fn C_player_roll_hp_for_levelup() -> libc::int16_t {
    debug::enter("C_player_roll_hp_for_levelup");
    let ret = player::roll_hp_for_levelup();
    debug::leave("C_player_roll_hp_for_levelup");
    return ret;
}

#[no_mangle]
pub extern fn C_player_set_extra_bulk_carry(new_value: libc::uint16_t) {
    debug::enter("C_player_set_extra_bulk_carry");
    player::set_extra_bulk_carry(new_value);
    debug::leave("C_player_set_extra_bulk_carry");
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
