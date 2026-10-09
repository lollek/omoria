use crate::model::Monster;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MonsterRecord {
    pub monsters: Vec<Monster>,
}
