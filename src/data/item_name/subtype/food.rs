use crate::model::item_subtype::{FoodSubType, ItemSubType, JunkFoodSubType};
use crate::model::Item;

pub fn food(item: &Item) -> String {
    match item.item_subtype() {
        ItemSubType::Food(subtype) => match subtype {
            FoodSubType::Mushroom => "mushroom",
            FoodSubType::MushroomOfPoison => "mushroom",
            FoodSubType::MushroomOfBlindness => "mushroom",
            FoodSubType::MushroomOfParanoia => "mushroom",
            FoodSubType::MushroomOfConfusion => "mushroom",
            FoodSubType::MushroomOfHallucination => "mushroom",
            FoodSubType::MushroomOfCurePoison => "mushroom",
            FoodSubType::MushroomOfCureBlindness => "mushroom",
            FoodSubType::MushroomOfCureParanoia => "mushroom",
            FoodSubType::MushroomOfCureConfusion => "mushroom",
            FoodSubType::MushroomOfWeakness => "mushroom",
            FoodSubType::MushroomOfUnhealth => "mushroom",
            FoodSubType::MushroomOfRestoreConstitution => "mushroom",
            FoodSubType::MushroomOfFirstAid => "mushroom",
            FoodSubType::MushroomOfMinorCures => "mushroom",
            FoodSubType::MushroomOfLightCures => "mushroom",
            FoodSubType::MushroomOfRestoring => "mushroom",
            FoodSubType::MushroomOfPoison2 => "mushroom",
            FoodSubType::MushroomOfHallucination2 => "mushroom",
            FoodSubType::MushroomOfCurePoison2 => "mushroom",
            FoodSubType::MushroomOfUnhealth2 => "mushroom",
            FoodSubType::MushroomOfCureSeriousWounds => "mushroom",
            FoodSubType::PintOfFineGradeMush => "pint of fine grade mush",
            FoodSubType::RationOfFood => "ration of food",
            FoodSubType::Mushroom2 => "mushroom",
            FoodSubType::HardBiscuit => "hard biscuit",
            FoodSubType::BeefJerky => "beef jerky",
            FoodSubType::FineAle => "fine ale",
            FoodSubType::FineWine => "fine wine",
            FoodSubType::ElvishWaybread => "elvish waybread",
            FoodSubType::Stew => "stew",
            FoodSubType::GreenJelly => "green jelly",
            FoodSubType::BerriesPoisonous => "berries",
            FoodSubType::BerriesSmurfberries => "smurfberries",
            FoodSubType::BerriesGoodberries => "goodberries",
            FoodSubType::EyeballOfNed => "eyeball of ned",
        },
        ItemSubType::JunkFood(subtype) => match subtype {
            JunkFoodSubType::BoxOfPiranhaCrackers => "box of piranha crackers",
            JunkFoodSubType::CanOfOrcaCola => "can of orca cola",
            JunkFoodSubType::TwelvePoundTrollBuger => "twelve pound troll buger",
            JunkFoodSubType::BagOfBrontosaurusChips => "bag of brontosaurus chips",
            JunkFoodSubType::SliceOfPurpleMushroomPizza => "slice of purple mushroom pizza",
            JunkFoodSubType::PeanutButterAndGrapeJellySandwich => {
                "peanut butter and grape jelly sandwich"
            }
            JunkFoodSubType::DragonSteak => "dragon steak",
            JunkFoodSubType::VorpalBunnyThroatLozenge => "vorpal bunny throat lozenge",
            JunkFoodSubType::DeepFriedGiantCentipede => "deep fried giant centipede",
            JunkFoodSubType::PintOfBeetleJuice => "pint of beetle juice",
            JunkFoodSubType::BownOfBatStew => "bowl of bat stew",
            JunkFoodSubType::JarOfPickledLeeches => "jar of pickled leeches",
            JunkFoodSubType::PackOfKittenMcNuggets => "pack of kitten mc nuggets",
        },
        _ => panic!("Expected food item, got {:?}", item.item_type()),
    }
    .to_string()
}
