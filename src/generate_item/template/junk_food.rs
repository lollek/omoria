use super::super::item_template::ItemTemplate;
use crate::model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum JunkFoodTemplate {
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

impl JunkFoodTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(JunkFoodTemplate::BoxOfPiranhaCrackers),
            Box::new(JunkFoodTemplate::CanOfOrcaCola),
            Box::new(JunkFoodTemplate::TwelvePoundTrollBuger),
            Box::new(JunkFoodTemplate::BagOfBrontosaurusChips),
            Box::new(JunkFoodTemplate::SliceOfPurpleMushroomPizza),
            Box::new(JunkFoodTemplate::PeanutButterAndGrapeJellySandwich),
            Box::new(JunkFoodTemplate::DragonSteak),
            Box::new(JunkFoodTemplate::VorpalBunnyThroatLozenge),
            Box::new(JunkFoodTemplate::DeepFriedGiantCentipede),
            Box::new(JunkFoodTemplate::PintOfBeetleJuice),
            Box::new(JunkFoodTemplate::BownOfBatStew),
            Box::new(JunkFoodTemplate::JarOfPickledLeeches),
            Box::new(JunkFoodTemplate::PackOfKittenMcNuggets),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        JunkFoodTemplate::vec().into_iter()
    }
}

impl ItemTemplate for JunkFoodTemplate {
    fn name(&self) -> &str {
        match self {
            JunkFoodTemplate::BoxOfPiranhaCrackers => "& Box~ of Piranha Crackers",
            JunkFoodTemplate::CanOfOrcaCola => "& Can~ of Orca-Cola",
            JunkFoodTemplate::TwelvePoundTrollBuger => "& Twelve-Pound Troll Burger~",
            JunkFoodTemplate::BagOfBrontosaurusChips => "& Bag~ of Brontosaurus Chips",
            JunkFoodTemplate::SliceOfPurpleMushroomPizza => "& Slice~ of Purple Mushroom Pizza",
            JunkFoodTemplate::PeanutButterAndGrapeJellySandwich => {
                "& Peanut Butter and Grape Jelly Sandwich~"
            }
            JunkFoodTemplate::DragonSteak => "& Dragon Steak~",
            JunkFoodTemplate::VorpalBunnyThroatLozenge => "& Vorpal Bunny Throat Lozenge~",
            JunkFoodTemplate::DeepFriedGiantCentipede => "& Deep-Fried Giant Centipede~",
            JunkFoodTemplate::PintOfBeetleJuice => "& Pint~ of Beetle Juice",
            JunkFoodTemplate::BownOfBatStew => "& Bowl~ of Bat Stew",
            JunkFoodTemplate::JarOfPickledLeeches => "& Jar~ of Pickled Leeches",
            JunkFoodTemplate::PackOfKittenMcNuggets => "& Pack~ of Kitten McNuggets",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::JunkFood
    }

    fn flags1(&self) -> u64 {
        match self {
            JunkFoodTemplate::BoxOfPiranhaCrackers => 0x00000001,
            JunkFoodTemplate::CanOfOrcaCola => 0x00000002,
            JunkFoodTemplate::TwelvePoundTrollBuger => 0x00000001,
            JunkFoodTemplate::BagOfBrontosaurusChips => 0x00000001,
            JunkFoodTemplate::SliceOfPurpleMushroomPizza => 0x00000001,
            JunkFoodTemplate::PeanutButterAndGrapeJellySandwich => 0x00000001,
            JunkFoodTemplate::DragonSteak => 0x00000001,
            JunkFoodTemplate::VorpalBunnyThroatLozenge => 0x00000001,
            JunkFoodTemplate::DeepFriedGiantCentipede => 0x00000001,
            JunkFoodTemplate::PintOfBeetleJuice => 0x00000002,
            JunkFoodTemplate::BownOfBatStew => 0x00000001,
            JunkFoodTemplate::JarOfPickledLeeches => 0x00000001,
            JunkFoodTemplate::PackOfKittenMcNuggets => 0x00000001,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            JunkFoodTemplate::BoxOfPiranhaCrackers => 0x40000000,
            JunkFoodTemplate::CanOfOrcaCola => 0x40000000,
            JunkFoodTemplate::TwelvePoundTrollBuger => 0x40000000,
            JunkFoodTemplate::BagOfBrontosaurusChips => 0x40000000,
            JunkFoodTemplate::SliceOfPurpleMushroomPizza => 0x40000400,
            JunkFoodTemplate::PeanutButterAndGrapeJellySandwich => 0x40000000,
            JunkFoodTemplate::DragonSteak => 0x50000000,
            JunkFoodTemplate::VorpalBunnyThroatLozenge => 0x40000000,
            JunkFoodTemplate::DeepFriedGiantCentipede => 0x40000000,
            JunkFoodTemplate::PintOfBeetleJuice => 0x40000000,
            JunkFoodTemplate::BownOfBatStew => 0x40000000,
            JunkFoodTemplate::JarOfPickledLeeches => 0x40000000,
            JunkFoodTemplate::PackOfKittenMcNuggets => 0x40000000,
        }
    }

    fn p1(&self) -> i64 {
        match self {
            JunkFoodTemplate::BoxOfPiranhaCrackers => 1500,
            JunkFoodTemplate::CanOfOrcaCola => 500,
            JunkFoodTemplate::TwelvePoundTrollBuger => 7500,
            JunkFoodTemplate::BagOfBrontosaurusChips => 3000,
            JunkFoodTemplate::SliceOfPurpleMushroomPizza => 1500,
            JunkFoodTemplate::PeanutButterAndGrapeJellySandwich => 1000,
            JunkFoodTemplate::DragonSteak => 5000,
            JunkFoodTemplate::VorpalBunnyThroatLozenge => 50,
            JunkFoodTemplate::DeepFriedGiantCentipede => 750,
            JunkFoodTemplate::PintOfBeetleJuice => 1000,
            JunkFoodTemplate::BownOfBatStew => 2000,
            JunkFoodTemplate::JarOfPickledLeeches => 1500,
            JunkFoodTemplate::PackOfKittenMcNuggets => 1500,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            JunkFoodTemplate::BoxOfPiranhaCrackers => 4,
            JunkFoodTemplate::CanOfOrcaCola => 4,
            JunkFoodTemplate::TwelvePoundTrollBuger => 15,
            JunkFoodTemplate::BagOfBrontosaurusChips => 12,
            JunkFoodTemplate::SliceOfPurpleMushroomPizza => 8,
            JunkFoodTemplate::PeanutButterAndGrapeJellySandwich => 5,
            JunkFoodTemplate::DragonSteak => 15,
            JunkFoodTemplate::VorpalBunnyThroatLozenge => 2,
            JunkFoodTemplate::DeepFriedGiantCentipede => 5,
            JunkFoodTemplate::PintOfBeetleJuice => 4,
            JunkFoodTemplate::BownOfBatStew => 6,
            JunkFoodTemplate::JarOfPickledLeeches => 5,
            JunkFoodTemplate::PackOfKittenMcNuggets => 8,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            JunkFoodTemplate::BoxOfPiranhaCrackers => 257,
            JunkFoodTemplate::CanOfOrcaCola => 258,
            JunkFoodTemplate::TwelvePoundTrollBuger => 259,
            JunkFoodTemplate::BagOfBrontosaurusChips => 260,
            JunkFoodTemplate::SliceOfPurpleMushroomPizza => 261,
            JunkFoodTemplate::PeanutButterAndGrapeJellySandwich => 262,
            JunkFoodTemplate::DragonSteak => 263,
            JunkFoodTemplate::VorpalBunnyThroatLozenge => 264,
            JunkFoodTemplate::DeepFriedGiantCentipede => 265,
            JunkFoodTemplate::PintOfBeetleJuice => 266,
            JunkFoodTemplate::BownOfBatStew => 267,
            JunkFoodTemplate::JarOfPickledLeeches => 268,
            JunkFoodTemplate::PackOfKittenMcNuggets => 269,
        }
    }

    fn weight(&self) -> u16 {
        1
    }
    fn number(&self) -> u16 {
        1
    }
    fn modifier_to_hit(&self) -> i16 {
        0
    }
    fn modifier_to_damage(&self) -> i16 {
        0
    }
    fn base_ac(&self) -> i16 {
        0
    }
    fn modifier_to_ac(&self) -> i16 {
        0
    }

    fn damage(&self) -> &str {
        match self {
            JunkFoodTemplate::BoxOfPiranhaCrackers => "0d0",
            JunkFoodTemplate::CanOfOrcaCola => "0d0",
            JunkFoodTemplate::TwelvePoundTrollBuger => "0d0",
            JunkFoodTemplate::BagOfBrontosaurusChips => "0d0",
            JunkFoodTemplate::SliceOfPurpleMushroomPizza => "2d6",
            JunkFoodTemplate::PeanutButterAndGrapeJellySandwich => "0d0",
            JunkFoodTemplate::DragonSteak => "0d0",
            JunkFoodTemplate::VorpalBunnyThroatLozenge => "0d0",
            JunkFoodTemplate::DeepFriedGiantCentipede => "0d0",
            JunkFoodTemplate::PintOfBeetleJuice => "0d0",
            JunkFoodTemplate::BownOfBatStew => "0d0",
            JunkFoodTemplate::JarOfPickledLeeches => "0d0",
            JunkFoodTemplate::PackOfKittenMcNuggets => "0d0",
        }
    }

    fn item_level(&self) -> u8 {
        1
    }
    fn is_identified(&self) -> bool {
        false
    }
}
