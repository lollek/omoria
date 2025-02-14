use crate::model::{Race, Sex};
use crate::{data, random};

pub fn generate(race: &Race, sex: &Sex) -> u16 {
    random::randnor(
        data::race::weight_base(&race, sex) as i64,
        data::race::weight_modifier(&race, sex) as i64,
    ) as u16
}
