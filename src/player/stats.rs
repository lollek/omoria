use crate::model::{Stat, StatBlock};

use crate::player;
use crate::player::data::PLAYER;

pub fn set_perm_stats(block: StatBlock) {
    PLAYER.try_write().unwrap().perm_stats = block;
}

pub fn curr_stats() -> StatBlock {
    let mut stats = PLAYER.try_read().unwrap().curr_stats;
    if player::is_raging() {
        stats.strength += 4;
        stats.constitution += 4;
    }
    stats
}

pub fn recalc_curr_stats() {
    let perm_stats = PLAYER.try_read().unwrap().perm_stats;
    let mod_stats = PLAYER.try_read().unwrap().mod_stats;
    let lost_stats = lost_stats();
    let mut curr_stats = StatBlock::new(0);
    for stat in Stat::iter() {
        let curr_stat = perm_stats.get(stat) + mod_stats.get(stat) - lost_stats.get(stat);
        curr_stats.set(stat, curr_stat);
    }
    PLAYER.try_write().unwrap().curr_stats = curr_stats;
}

pub fn get_stat(stat: Stat) -> i16 {
    curr_stats().get(stat)
}

pub fn tohit_from_stats() -> i16 {
    (modifier_from_stat(Stat::Strength) + modifier_from_stat(Stat::Dexterity)) / 2
}

pub fn disarm_from_dex() -> i16 {
    modifier_from_stat(Stat::Dexterity)
}

pub fn modifier_from_stat(stat: Stat) -> i16 {
    (get_stat(stat) - 10) / 2
}

pub fn dmg_from_str() -> i16 {
    modifier_from_stat(Stat::Strength)
}

pub fn hp_from_con() -> i16 {
    modifier_from_stat(Stat::Constitution)
}

pub fn cost_modifier_from_charisma() -> f32 {
    modifier_from_stat(Stat::Charisma) as f32 * -0.02
}

pub fn modify_lost_stat(stat: Stat, amount: i16) {
    let mut stats = PLAYER.try_write().unwrap().lost_stats;
    let old_val = stats.get(stat);
    stats.set(stat, old_val + amount);
}

pub fn reset_lost_stat(stat: Stat) {
    PLAYER.try_write().unwrap().lost_stats.set(stat, 0);
}

pub fn lost_stats() -> StatBlock {
    let mut stats = PLAYER.try_read().unwrap().lost_stats;
    if player::is_fatigued() {
        stats.strength += 2;
        stats.constitution += 2;
    }
    stats
}

pub fn has_lost_stat(stat: Stat) -> bool {
    lost_stats().get(stat) != 0
}

pub fn rage_rounds_from_con() -> i16 {
    modifier_from_stat(Stat::Constitution)
}

pub fn ac_from_dex() -> i16 {
    let mut ac = modifier_from_stat(Stat::Dexterity);
    if player::is_raging() {
        ac -= 2;
    }
    ac
}
