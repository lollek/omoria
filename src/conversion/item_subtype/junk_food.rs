use crate::model::item_subtype::JunkFoodSubType;

pub fn from_usize(subtype: usize) -> Option<JunkFoodSubType> {
    match subtype {
        257 => Some(JunkFoodSubType::BoxOfPiranhaCrackers),
        258 => Some(JunkFoodSubType::CanOfOrcaCola),
        259 => Some(JunkFoodSubType::TwelvePoundTrollBuger),
        260 => Some(JunkFoodSubType::BagOfBrontosaurusChips),
        261 => Some(JunkFoodSubType::SliceOfPurpleMushroomPizza),
        262 => Some(JunkFoodSubType::PeanutButterAndGrapeJellySandwich),
        263 => Some(JunkFoodSubType::DragonSteak),
        264 => Some(JunkFoodSubType::VorpalBunnyThroatLozenge),
        265 => Some(JunkFoodSubType::DeepFriedGiantCentipede),
        266 => Some(JunkFoodSubType::PintOfBeetleJuice),
        267 => Some(JunkFoodSubType::BownOfBatStew),
        268 => Some(JunkFoodSubType::JarOfPickledLeeches),
        269 => Some(JunkFoodSubType::PackOfKittenMcNuggets),
        _ => None,
    }
}

pub fn to_usize(subtype: JunkFoodSubType) -> usize {
    match subtype {
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
