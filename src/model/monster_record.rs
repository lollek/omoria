use crate::model::Monster;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MonsterRecord {
    pub monsters: Vec<Monster>,
}
