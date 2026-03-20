use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoodSubType {
    Mushroom,
    MushroomOfPoison,
    MushroomOfBlindness,
    MushroomOfParanoia,
    MushroomOfConfusion,
    MushroomOfHallucination,
    MushroomOfCurePoison,
    MushroomOfCureBlindness,
    MushroomOfCureParanoia,
    MushroomOfCureConfusion,
    MushroomOfWeakness,
    MushroomOfUnhealth,
    MushroomOfRestoreConstitution,
    MushroomOfFirstAid,
    MushroomOfMinorCures,
    MushroomOfLightCures,
    MushroomOfRestoring,
    MushroomOfPoison2,
    MushroomOfHallucination2,
    MushroomOfCurePoison2,
    MushroomOfUnhealth2,
    MushroomOfCureSeriousWounds,
    PintOfFineGradeMush,
    RationOfFood,
    Mushroom2,
    HardBiscuit,
    BeefJerky,
    FineAle,
    FineWine,
    ElvishWaybread,
    Stew,
    GreenJelly,
    BerriesPoisonous,
    BerriesSmurfberries,
    BerriesGoodberries,
    EyeballOfNed,
}

impl From<FoodSubType> for usize {
    fn from(value: FoodSubType) -> usize {
        match value {
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
}

impl TryFrom<usize> for FoodSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            256 => Ok(FoodSubType::Mushroom),
            257 => Ok(FoodSubType::MushroomOfPoison),
            258 => Ok(FoodSubType::MushroomOfBlindness),
            259 => Ok(FoodSubType::MushroomOfParanoia),
            260 => Ok(FoodSubType::MushroomOfConfusion),
            261 => Ok(FoodSubType::MushroomOfHallucination),
            262 => Ok(FoodSubType::MushroomOfCurePoison),
            263 => Ok(FoodSubType::MushroomOfCureBlindness),
            264 => Ok(FoodSubType::MushroomOfCureParanoia),
            265 => Ok(FoodSubType::MushroomOfCureConfusion),
            266 => Ok(FoodSubType::MushroomOfWeakness),
            267 => Ok(FoodSubType::MushroomOfUnhealth),
            268 => Ok(FoodSubType::MushroomOfRestoreConstitution),
            269 => Ok(FoodSubType::MushroomOfFirstAid),
            270 => Ok(FoodSubType::MushroomOfMinorCures),
            271 => Ok(FoodSubType::MushroomOfLightCures),
            272 => Ok(FoodSubType::MushroomOfRestoring),
            273 => Ok(FoodSubType::MushroomOfPoison2),
            274 => Ok(FoodSubType::MushroomOfHallucination2),
            275 => Ok(FoodSubType::MushroomOfCurePoison2),
            276 => Ok(FoodSubType::MushroomOfUnhealth2),
            277 => Ok(FoodSubType::MushroomOfCureSeriousWounds),
            306 => Ok(FoodSubType::PintOfFineGradeMush),
            307 => Ok(FoodSubType::RationOfFood),
            308 => Ok(FoodSubType::Mushroom2),
            309 => Ok(FoodSubType::HardBiscuit),
            310 => Ok(FoodSubType::BeefJerky),
            311 => Ok(FoodSubType::FineAle),
            312 => Ok(FoodSubType::FineWine),
            313 => Ok(FoodSubType::ElvishWaybread),
            314 => Ok(FoodSubType::Stew),
            315 => Ok(FoodSubType::GreenJelly),
            316 => Ok(FoodSubType::BerriesPoisonous),
            317 => Ok(FoodSubType::BerriesSmurfberries),
            318 => Ok(FoodSubType::BerriesGoodberries),
            319 => Ok(FoodSubType::EyeballOfNed),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JunkFoodSubType {
    BoxOfPiranhaCrackers,
    CanOfOrcaCola,
    TwelvePoundTrollBuger,
    BagOfBrontosaurusChips,
    SliceOfPurpleMushroomPizza,
    PeanutButterAndGrapeJellySandwich,
    DragonSteak,
    VorpalBunnyThroatLozenge,
    DeepFriedGiantCentipede,
    PintOfBeetleJuice,
    BownOfBatStew,
    JarOfPickledLeeches,
    PackOfKittenMcNuggets,
}

impl From<JunkFoodSubType> for usize {
    fn from(value: JunkFoodSubType) -> usize {
        match value {
            JunkFoodSubType::BoxOfPiranhaCrackers => 257,
            JunkFoodSubType::CanOfOrcaCola => 258,
            JunkFoodSubType::TwelvePoundTrollBuger => 259,
            JunkFoodSubType::BagOfBrontosaurusChips => 260,
            JunkFoodSubType::SliceOfPurpleMushroomPizza => 261,
            JunkFoodSubType::PeanutButterAndGrapeJellySandwich => 262,
            JunkFoodSubType::DragonSteak => 263,
            JunkFoodSubType::VorpalBunnyThroatLozenge => 264,
            JunkFoodSubType::DeepFriedGiantCentipede => 265,
            JunkFoodSubType::PintOfBeetleJuice => 266,
            JunkFoodSubType::BownOfBatStew => 267,
            JunkFoodSubType::JarOfPickledLeeches => 268,
            JunkFoodSubType::PackOfKittenMcNuggets => 269,
        }
    }
}

impl TryFrom<usize> for JunkFoodSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            257 => Ok(JunkFoodSubType::BoxOfPiranhaCrackers),
            258 => Ok(JunkFoodSubType::CanOfOrcaCola),
            259 => Ok(JunkFoodSubType::TwelvePoundTrollBuger),
            260 => Ok(JunkFoodSubType::BagOfBrontosaurusChips),
            261 => Ok(JunkFoodSubType::SliceOfPurpleMushroomPizza),
            262 => Ok(JunkFoodSubType::PeanutButterAndGrapeJellySandwich),
            263 => Ok(JunkFoodSubType::DragonSteak),
            264 => Ok(JunkFoodSubType::VorpalBunnyThroatLozenge),
            265 => Ok(JunkFoodSubType::DeepFriedGiantCentipede),
            266 => Ok(JunkFoodSubType::PintOfBeetleJuice),
            267 => Ok(JunkFoodSubType::BownOfBatStew),
            268 => Ok(JunkFoodSubType::JarOfPickledLeeches),
            269 => Ok(JunkFoodSubType::PackOfKittenMcNuggets),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::{FoodSubType, JunkFoodSubType};

    #[test]
    fn test_food_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            FoodSubType::try_from(256usize).unwrap(),
            FoodSubType::Mushroom
        );
    }

    #[test]
    fn test_food_subtype_try_from_usize_rejects_unknown_value() {
        assert!(FoodSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_food_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(FoodSubType::EyeballOfNed), 319);
    }

    #[test]
    fn test_junk_food_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            JunkFoodSubType::try_from(257usize).unwrap(),
            JunkFoodSubType::BoxOfPiranhaCrackers
        );
    }

    #[test]
    fn test_junk_food_subtype_try_from_usize_rejects_unknown_value() {
        assert!(JunkFoodSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_junk_food_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(JunkFoodSubType::PackOfKittenMcNuggets), 269);
    }
}
