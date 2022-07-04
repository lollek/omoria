use crate::model::Monster;

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct MonsterRecord {
    pub monsters: Vec<Monster>,
}
