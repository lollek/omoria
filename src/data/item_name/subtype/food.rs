use crate::data::item_name::helpers::{maybe_number_of, plural_es, plural_s};
use crate::model::item_subtype::{FoodSubType, ItemSubType, JunkFoodSubType};
use crate::model::Item;

pub fn food(item: &Item) -> String {
    let mut parts = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }
    parts.push(subtype_name(item).into());
    parts.join("")
}

fn subtype_name<'a>(item: &Item) -> String {
    match item.item_subtype() {
        ItemSubType::Food(subtype) => match subtype {
            FoodSubType::Mushroom
            | FoodSubType::MushroomOfPoison
            | FoodSubType::MushroomOfBlindness
            | FoodSubType::MushroomOfParanoia
            | FoodSubType::MushroomOfConfusion
            | FoodSubType::MushroomOfHallucination
            | FoodSubType::MushroomOfCurePoison
            | FoodSubType::MushroomOfCureBlindness
            | FoodSubType::MushroomOfCureParanoia
            | FoodSubType::MushroomOfCureConfusion
            | FoodSubType::MushroomOfWeakness
            | FoodSubType::MushroomOfUnhealth
            | FoodSubType::MushroomOfRestoreConstitution
            | FoodSubType::MushroomOfFirstAid
            | FoodSubType::MushroomOfMinorCures
            | FoodSubType::MushroomOfLightCures
            | FoodSubType::MushroomOfRestoring
            | FoodSubType::MushroomOfPoison2
            | FoodSubType::MushroomOfHallucination2
            | FoodSubType::MushroomOfCurePoison2
            | FoodSubType::MushroomOfUnhealth2
            | FoodSubType::MushroomOfCureSeriousWounds => format!("mushroom{}", plural_s(item)),
            FoodSubType::PintOfFineGradeMush => {
                format!("pint{} of fine grade mush", plural_s(item))
            }
            FoodSubType::RationOfFood => format!("ration{} of food", plural_s(item)),
            FoodSubType::Mushroom2 => format!("mushroom{}", plural_s(item)),
            FoodSubType::HardBiscuit => format!("hard biscuit{}", plural_s(item)),
            FoodSubType::BeefJerky => "beef jerky".into(),
            FoodSubType::FineAle => "fine ale".into(),
            FoodSubType::FineWine => "fine wine".into(),
            FoodSubType::ElvishWaybread => "elvish waybread".into(),
            FoodSubType::Stew => "stew".into(),
            FoodSubType::GreenJelly => "green jelly".into(),
            FoodSubType::BerriesPoisonous => "handful of berries".into(),
            FoodSubType::BerriesSmurfberries => "handful of smurfberries".into(),
            FoodSubType::BerriesGoodberries => "handful of goodberries".into(),
            FoodSubType::EyeballOfNed => "eyeball of ned".into(),
        },
        ItemSubType::JunkFood(subtype) => match subtype {
            JunkFoodSubType::BoxOfPiranhaCrackers => {
                format!("box{} of piranha crackers", plural_es(item))
            }
            JunkFoodSubType::CanOfOrcaCola => format!("can{} of orca cola", plural_s(item)),
            JunkFoodSubType::TwelvePoundTrollBuger => {
                format!("twelve pound troll burger{}", plural_s(item))
            }
            JunkFoodSubType::BagOfBrontosaurusChips => {
                format!("bag{} of brontosaurus chips", plural_s(item))
            }
            JunkFoodSubType::SliceOfPurpleMushroomPizza => {
                format!("slice{} of purple mushroom pizza", plural_s(item))
            }
            JunkFoodSubType::PeanutButterAndGrapeJellySandwich => {
                format!("peanut butter and grape jelly sandwich{}", plural_es(item))
            }
            JunkFoodSubType::DragonSteak => format!("dragon steak{}", plural_s(item)),
            JunkFoodSubType::VorpalBunnyThroatLozenge => {
                format!("vorpal bunny throat lozenge{}", plural_s(item))
            }
            JunkFoodSubType::DeepFriedGiantCentipede => {
                format!("deep fried giant centipede{}", plural_s(item))
            }
            JunkFoodSubType::PintOfBeetleJuice => format!("pint{} of beetle juice", plural_s(item)),
            JunkFoodSubType::BownOfBatStew => format!("bowl{} of bat stew", plural_s(item)),
            JunkFoodSubType::JarOfPickledLeeches => {
                format!("jar{} of pickled leeches", plural_s(item))
            }
            JunkFoodSubType::PackOfKittenMcNuggets => {
                format!("pack{} of kitten mc nuggets", plural_s(item))
            }
        },
        _ => panic!("Expected food item, got {:?}", item.item_type()),
    }
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item::{self, template::FoodTemplate, ItemQuality};

    #[test]
    fn test_ration_of_food() {
        let mut item =
            generate_item::generate(Box::new(FoodTemplate::RationOfFood), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "ration of food");

        item.number = 0;
        assert_eq!(generate(&item), "no more rations of food");

        item.number = 5;
        assert_eq!(generate(&item), "5 rations of food");
    }
}
