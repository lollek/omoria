use crate::model::{Race, Sex};
use crate::{data, rng};

pub fn generate(race: &Race, sex: &Sex) -> u16 {
    rng::randnor(
        data::race::weight_base(&race, sex) as i64,
        data::race::weight_modifier(&race, sex) as i64,
    ) as u16
}
