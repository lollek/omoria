use crate::model::Monster;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MonsterRecord {
    pub monsters: Vec<Monster>,
}

impl Default for MonsterRecord {
    fn default() -> Self {
        MonsterRecord {
            monsters: Vec::new(),
        }
    }
}