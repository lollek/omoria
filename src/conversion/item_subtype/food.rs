use crate::model::item_subtype::FoodSubType;

pub fn from_usize(subtype: usize) -> Option<FoodSubType> {
    match subtype {
        256 => Some(FoodSubType::Mushroom),
        257 => Some(FoodSubType::MushroomOfPoison),
        258 => Some(FoodSubType::MushroomOfBlindness),
        259 => Some(FoodSubType::MushroomOfParanoia),
        260 => Some(FoodSubType::MushroomOfConfusion),
        261 => Some(FoodSubType::MushroomOfHallucination),
        262 => Some(FoodSubType::MushroomOfCurePoison),
        263 => Some(FoodSubType::MushroomOfCureBlindness),
        264 => Some(FoodSubType::MushroomOfCureParanoia),
        265 => Some(FoodSubType::MushroomOfCureConfusion),
        266 => Some(FoodSubType::MushroomOfWeakness),
        267 => Some(FoodSubType::MushroomOfUnhealth),
        268 => Some(FoodSubType::MushroomOfRestoreConstitution),
        269 => Some(FoodSubType::MushroomOfFirstAid),
        270 => Some(FoodSubType::MushroomOfMinorCures),
        271 => Some(FoodSubType::MushroomOfLightCures),
        272 => Some(FoodSubType::MushroomOfRestoring),
        273 => Some(FoodSubType::MushroomOfPoison2),
        274 => Some(FoodSubType::MushroomOfHallucination2),
        275 => Some(FoodSubType::MushroomOfCurePoison2),
        276 => Some(FoodSubType::MushroomOfUnhealth2),
        277 => Some(FoodSubType::MushroomOfCureSeriousWounds),
        306 => Some(FoodSubType::PintOfFineGradeMush),
        307 => Some(FoodSubType::RationOfFood),
        308 => Some(FoodSubType::Mushroom2),
        309 => Some(FoodSubType::HardBiscuit),
        310 => Some(FoodSubType::BeefJerky),
        311 => Some(FoodSubType::FineAle),
        312 => Some(FoodSubType::FineWine),
        313 => Some(FoodSubType::ElvishWaybread),
        314 => Some(FoodSubType::Stew),
        315 => Some(FoodSubType::GreenJelly),
        316 => Some(FoodSubType::BerriesPoisonous),
        317 => Some(FoodSubType::BerriesSmurfberries),
        318 => Some(FoodSubType::BerriesGoodberries),
        319 => Some(FoodSubType::EyeballOfNed),
        _ => None,
    }
}

pub fn to_usize(subtype: FoodSubType) -> usize {
    match subtype {
        FoodSubType::Mushroom => 256,
        FoodSubType::MushroomOfPoison => 257,
        FoodSubType::MushroomOfBlindness => 258,
        FoodSubType::MushroomOfParanoia => 259,
        FoodSubType::MushroomOfConfusion => 260,
        FoodSubType::MushroomOfHallucination => 261,
        FoodSubType::MushroomOfCurePoison => 262,
        FoodSubType::MushroomOfCureBlindness => 263,
        FoodSubType::MushroomOfCureParanoia => 264,
        FoodSubType::MushroomOfCureConfusion => 265,
        FoodSubType::MushroomOfWeakness => 266,
        FoodSubType::MushroomOfUnhealth => 267,
        FoodSubType::MushroomOfRestoreConstitution => 268,
        FoodSubType::MushroomOfFirstAid => 269,
        FoodSubType::MushroomOfMinorCures => 270,
        FoodSubType::MushroomOfLightCures => 271,
        FoodSubType::MushroomOfRestoring => 272,
        FoodSubType::MushroomOfPoison2 => 273,
        FoodSubType::MushroomOfHallucination2 => 274,
        FoodSubType::MushroomOfCurePoison2 => 275,
        FoodSubType::MushroomOfUnhealth2 => 276,
        FoodSubType::MushroomOfCureSeriousWounds => 277,
        FoodSubType::PintOfFineGradeMush => 306,
        FoodSubType::RationOfFood => 307,
        FoodSubType::Mushroom2 => 308,
        FoodSubType::HardBiscuit => 309,
        FoodSubType::BeefJerky => 310,
        FoodSubType::FineAle => 311,
        FoodSubType::FineWine => 312,
        FoodSubType::ElvishWaybread => 313,
        FoodSubType::Stew => 314,
        FoodSubType::GreenJelly => 315,
        FoodSubType::BerriesPoisonous => 316,
        FoodSubType::BerriesSmurfberries => 317,
        FoodSubType::BerriesGoodberries => 318,
        FoodSubType::EyeballOfNed => 319,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_conversion() {
        (0..1000).for_each(|i| {
            if let Some(subtype) = from_usize(i) {
                assert_eq!(i, to_usize(subtype));
            }
        })
    }
}
