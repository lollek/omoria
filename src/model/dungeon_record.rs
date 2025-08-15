use libc;

use crate::model::{Cave, TreasureAndCoordinate};

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DungeonRecord {
    pub cur_height: libc::c_long,
    pub cur_width: libc::c_long,
    pub max_panel_rows: libc::c_long,
    pub max_panel_cols: libc::c_long,
    pub cave: Vec<Cave>,
    pub treasure: Vec<TreasureAndCoordinate>,
    pub dun_level: libc::c_long,
    pub mon_tot_mult: libc::c_long,
    pub turn: libc::c_long,
    pub randes_seed: libc::c_long,
}

impl Default for DungeonRecord {
    fn default() -> Self {
        DungeonRecord {
            cur_height: 0,
            cur_width: 0,
            max_panel_rows: 0,
            max_panel_cols: 0,
            cave: Vec::new(),
            treasure: Vec::new(),
            dun_level: 0,
            mon_tot_mult: 0,
            turn: 0,
            randes_seed: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let dungeon_record = DungeonRecord::default();
        serde_json::to_string(&dungeon_record).expect("Failed to serialize DungeonRecord");
    }
}