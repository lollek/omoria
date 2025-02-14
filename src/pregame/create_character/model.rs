use crate::model::{GameTime, StatBlock};

pub(crate) struct StatsFromRace {
    pub age_plain: u16,
    pub age_game_time: GameTime,
    pub birthdate: GameTime,
    pub disarm_modifier: i16,
    pub experience_factor: f32,
    pub height: u16,
    pub history: String,
    pub infravision: i64,
    pub melee_bonus: i16,
    pub ranged_bonus: i16,
    pub save_modifier: i16,
    pub search_frequency: i16,
    pub social_class: i16,
    pub stat_block: StatBlock,
    pub stealth_modifier: i16,
    pub swim_speed: i64,
    pub weight: u16,
}

