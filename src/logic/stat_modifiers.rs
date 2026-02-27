use crate::model::{Stat, StatBlock};

pub fn modifier_from_value(value: i16) -> i16 {
    (value - 10) / 2
}

pub fn modifier_from_stat(stat: Stat, stat_block: &StatBlock) -> i16 {
    modifier_from_value(stat_block.get(stat))
}

pub fn disarm(stat_block: &StatBlock) -> i16 {
    modifier_from_stat(Stat::Intelligence, stat_block) + 2 * modifier_from_stat(Stat::Dexterity, stat_block)
}

pub fn ac(stat_block: &StatBlock) -> i16 {
    modifier_from_stat(Stat::Dexterity, stat_block)
}

pub fn to_hit_bonus(stat_block: &StatBlock) -> i16 {
    (modifier_from_stat(Stat::Dexterity, stat_block) + modifier_from_stat(Stat::Strength, stat_block)) / 2
}

pub fn damage(stat_block: &StatBlock) -> i16 {
    modifier_from_stat(Stat::Strength, stat_block)
}

pub fn health(stat_block: &StatBlock) -> i16 {
    modifier_from_stat(Stat::Constitution, stat_block)
}

pub fn shop_prices(stat_block: &StatBlock) -> f32 {
    modifier_from_stat(Stat::Charisma, stat_block) as f32 * -0.02
}
