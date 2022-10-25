use crate::model::StatBlock;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub spells_known: Vec<bool>,
    pub rage_rounds_spent: u8,
    pub is_raging: bool,
    pub rage_exhaustion_rounds_left: u8,
    pub curr_stats: StatBlock,
    pub lost_stats: StatBlock,
    pub mod_stats: StatBlock,
    pub perm_stats: StatBlock,
    pub save_counter: u64,
    pub extra_bulk_carry: u16,
    pub search_modifier: i16,
    pub max_hp_last_calc: i16, // Last time we checked, what was max hp?
    pub max_hp: i16,
    pub current_hp: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            spells_known: [false; 40].to_vec(),
            rage_rounds_spent: 0,
            is_raging: false,
            rage_exhaustion_rounds_left: 0,
            curr_stats: StatBlock::new(0),
            lost_stats: StatBlock::new(0),
            mod_stats: StatBlock::new(0),
            perm_stats: StatBlock::new(0),
            save_counter: 0,
            extra_bulk_carry: 0,
            search_modifier: 0,
            max_hp_last_calc: 0,
            max_hp: 0,
            current_hp: 0.0,
        }
    }
}
