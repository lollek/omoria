use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{FoodSubType, ItemSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum FoodTemplate {
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
    JollyGreenJelly,
    GreenJelly,
    BerriesPoisonous,
    BerriesSmurfberries,
    BerriesSmurfberries2,
    BerriesGoodberries,
    BerriesGoodberries2,
    EyeballOfNed,
}

impl FoodTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(FoodTemplate::Mushroom),
            Box::new(FoodTemplate::MushroomOfPoison),
            Box::new(FoodTemplate::MushroomOfBlindness),
            Box::new(FoodTemplate::MushroomOfParanoia),
            Box::new(FoodTemplate::MushroomOfConfusion),
            Box::new(FoodTemplate::MushroomOfHallucination),
            Box::new(FoodTemplate::MushroomOfCurePoison),
            Box::new(FoodTemplate::MushroomOfCureBlindness),
            Box::new(FoodTemplate::MushroomOfCureParanoia),
            Box::new(FoodTemplate::MushroomOfCureConfusion),
            Box::new(FoodTemplate::MushroomOfWeakness),
            Box::new(FoodTemplate::MushroomOfUnhealth),
            Box::new(FoodTemplate::MushroomOfRestoreConstitution),
            Box::new(FoodTemplate::MushroomOfFirstAid),
            Box::new(FoodTemplate::MushroomOfMinorCures),
            Box::new(FoodTemplate::MushroomOfLightCures),
            Box::new(FoodTemplate::MushroomOfRestoring),
            Box::new(FoodTemplate::MushroomOfPoison2),
            Box::new(FoodTemplate::MushroomOfHallucination2),
            Box::new(FoodTemplate::MushroomOfCurePoison2),
            Box::new(FoodTemplate::MushroomOfUnhealth2),
            Box::new(FoodTemplate::MushroomOfCureSeriousWounds),
            Box::new(FoodTemplate::PintOfFineGradeMush),
            Box::new(FoodTemplate::RationOfFood),
            Box::new(FoodTemplate::Mushroom2),
            Box::new(FoodTemplate::HardBiscuit),
            Box::new(FoodTemplate::BeefJerky),
            Box::new(FoodTemplate::FineAle),
            Box::new(FoodTemplate::FineWine),
            Box::new(FoodTemplate::ElvishWaybread),
            Box::new(FoodTemplate::Stew),
            Box::new(FoodTemplate::JollyGreenJelly),
            Box::new(FoodTemplate::GreenJelly),
            Box::new(FoodTemplate::BerriesPoisonous),
            Box::new(FoodTemplate::BerriesSmurfberries),
            Box::new(FoodTemplate::BerriesSmurfberries2),
            Box::new(FoodTemplate::BerriesGoodberries),
            Box::new(FoodTemplate::BerriesGoodberries2),
            Box::new(FoodTemplate::EyeballOfNed),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        FoodTemplate::vec().into_iter()
    }
}

impl ItemTemplate for FoodTemplate {
    fn name(&self) -> &str {
        match self {
            FoodTemplate::Mushroom => "& Mushroom~",
            FoodTemplate::MushroomOfPoison => "& Mushroom~| of Poison",
            FoodTemplate::MushroomOfBlindness => "& Mushroom~| of Blindness",
            FoodTemplate::MushroomOfParanoia => "& Mushroom~| of Paranoia",
            FoodTemplate::MushroomOfConfusion => "& Mushroom~| of Confusion",
            FoodTemplate::MushroomOfHallucination => "& Mushroom~| of Hallucination",
            FoodTemplate::MushroomOfCurePoison => "& Mushroom~| of Cure Poison",
            FoodTemplate::MushroomOfCureBlindness => "& Mushroom~| of Cure Blindness",
            FoodTemplate::MushroomOfCureParanoia => "& Mushroom~| of Cure Paranoia",
            FoodTemplate::MushroomOfCureConfusion => "& Mushroom~| of Cure Confusion",
            FoodTemplate::MushroomOfWeakness => "& Mushroom~| of Weakness",
            FoodTemplate::MushroomOfUnhealth => "& Mushroom~| of Unhealth",
            FoodTemplate::MushroomOfRestoreConstitution => "& Mushroom~| of Restore Constitution",
            FoodTemplate::MushroomOfFirstAid => "& Mushroom~| of First-Aid",
            FoodTemplate::MushroomOfMinorCures => "& Mushroom~| of Minor Cures",
            FoodTemplate::MushroomOfLightCures => "& Mushroom~| of Light Cures",
            FoodTemplate::MushroomOfRestoring => "& Mushroom~| of Restoring",
            FoodTemplate::MushroomOfPoison2 => "& Mushroom~| of Poison",
            FoodTemplate::MushroomOfHallucination2 => "& Mushroom~| of Hallucination",
            FoodTemplate::MushroomOfCurePoison2 => "& Mushroom~| of Cure Poison",
            FoodTemplate::MushroomOfUnhealth2 => "& Mushroom~| of Unhealth",
            FoodTemplate::MushroomOfCureSeriousWounds => "& Mushroom~| of Cure Serious Wounds",
            FoodTemplate::PintOfFineGradeMush => "& pint~ of fine grade mush",
            FoodTemplate::RationOfFood => "& Ration~ of Food",
            FoodTemplate::Mushroom2 => "& Mushroom~",
            FoodTemplate::HardBiscuit => "& Hard Biscuit~",
            FoodTemplate::BeefJerky => "& Strip~ of Beef Jerky",
            FoodTemplate::FineAle => "& Pint~ of Fine Ale",
            FoodTemplate::FineWine => "& Pint~ of Fine Wine",
            FoodTemplate::ElvishWaybread => "& Piece~ of Elvish Waybread",
            FoodTemplate::Stew => "& Stew~",
            FoodTemplate::JollyGreenJelly => "& Jolly Green Jelly~| (Ho Ho Ho!)",
            FoodTemplate::GreenJelly => "& Green Jelly~",
            FoodTemplate::BerriesPoisonous => "& Handful~ of Berries| (Poisonous)",
            FoodTemplate::BerriesSmurfberries => "& Handful~ of Berries| (Smurfberries)",
            FoodTemplate::BerriesSmurfberries2 => "& Handful~ of Berries| (Smurfberries)",
            FoodTemplate::BerriesGoodberries => "& Handful~ of Berries| (Goodberries)",
            FoodTemplate::BerriesGoodberries2 => "& Handful~ of Berries| (Goodberries)",
            FoodTemplate::EyeballOfNed => "& Eyeball~| of Ned",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Food
    }
    fn flags1(&self) -> u64 {
        match self {
            FoodTemplate::Mushroom => 0,
            FoodTemplate::MushroomOfPoison => 0x00000001,
            FoodTemplate::MushroomOfBlindness => 0x00000002,
            FoodTemplate::MushroomOfParanoia => 0x00000004,
            FoodTemplate::MushroomOfConfusion => 0x00000008,
            FoodTemplate::MushroomOfHallucination => 0x00000010,
            FoodTemplate::MushroomOfCurePoison => 0x00000020,
            FoodTemplate::MushroomOfCureBlindness => 0x00000040,
            FoodTemplate::MushroomOfCureParanoia => 0x00000080,
            FoodTemplate::MushroomOfCureConfusion => 0x00000100,
            FoodTemplate::MushroomOfWeakness => 0x00000200,
            FoodTemplate::MushroomOfUnhealth => 0x00000400,
            FoodTemplate::MushroomOfRestoreConstitution => 0x00010000,
            FoodTemplate::MushroomOfFirstAid => 0x00020000,
            FoodTemplate::MushroomOfMinorCures => 0x00040000,
            FoodTemplate::MushroomOfLightCures => 0x00080000,
            FoodTemplate::MushroomOfRestoring => 0x001F8040,
            FoodTemplate::MushroomOfPoison2 => 0x00000001,
            FoodTemplate::MushroomOfHallucination2 => 0x00000010,
            FoodTemplate::MushroomOfCurePoison2 => 0x00000020,
            FoodTemplate::MushroomOfUnhealth2 => 0x00000400,
            FoodTemplate::MushroomOfCureSeriousWounds => 0x01800000,
            FoodTemplate::PintOfFineGradeMush => 0x00000000,
            FoodTemplate::RationOfFood => 0,
            FoodTemplate::Mushroom2 => 0,
            FoodTemplate::HardBiscuit => 0,
            FoodTemplate::BeefJerky => 0,
            FoodTemplate::FineAle => 0,
            FoodTemplate::FineWine => 0,
            FoodTemplate::ElvishWaybread => 0x21800020,
            FoodTemplate::Stew => 0x330001C0,
            FoodTemplate::JollyGreenJelly => 0x224001C0,
            FoodTemplate::GreenJelly => 0x22400060,
            FoodTemplate::BerriesPoisonous => 0x0C0000000,
            FoodTemplate::BerriesSmurfberries => 0x10400000,
            FoodTemplate::BerriesSmurfberries2 => 0x30400000,
            FoodTemplate::BerriesGoodberries => 0x10C00080,
            FoodTemplate::BerriesGoodberries2 => 0x30C00080,
            FoodTemplate::EyeballOfNed => 0x00000053,
        }
    }

    fn flags2(&self) -> u64 {
        0
    }

    fn p1(&self) -> i64 {
        match self {
            FoodTemplate::Mushroom => 3000,
            FoodTemplate::MushroomOfPoison => 500,
            FoodTemplate::MushroomOfBlindness => 500,
            FoodTemplate::MushroomOfParanoia => 500,
            FoodTemplate::MushroomOfConfusion => 500,
            FoodTemplate::MushroomOfHallucination => 500,
            FoodTemplate::MushroomOfCurePoison => 500,
            FoodTemplate::MushroomOfCureBlindness => 500,
            FoodTemplate::MushroomOfCureParanoia => 500,
            FoodTemplate::MushroomOfCureConfusion => 500,
            FoodTemplate::MushroomOfWeakness => 500,
            FoodTemplate::MushroomOfUnhealth => 500,
            FoodTemplate::MushroomOfRestoreConstitution => 500,
            FoodTemplate::MushroomOfFirstAid => 500,
            FoodTemplate::MushroomOfMinorCures => 500,
            FoodTemplate::MushroomOfLightCures => 500,
            FoodTemplate::MushroomOfRestoring => 500,
            FoodTemplate::MushroomOfPoison2 => 1200,
            FoodTemplate::MushroomOfHallucination2 => 1200,
            FoodTemplate::MushroomOfCurePoison2 => 1200,
            FoodTemplate::MushroomOfUnhealth2 => 1200,
            FoodTemplate::MushroomOfCureSeriousWounds => 1200,
            FoodTemplate::PintOfFineGradeMush => 1500,
            FoodTemplate::RationOfFood => 5000,
            FoodTemplate::Mushroom2 => 3000,
            FoodTemplate::HardBiscuit => 500,
            FoodTemplate::BeefJerky => 1750,
            FoodTemplate::FineAle => 500,
            FoodTemplate::FineWine => 400,
            FoodTemplate::ElvishWaybread => 3500,
            FoodTemplate::Stew => 2000,
            FoodTemplate::JollyGreenJelly => 4000,
            FoodTemplate::GreenJelly => 4000,
            FoodTemplate::BerriesPoisonous => 1000,
            FoodTemplate::BerriesSmurfberries => 1000,
            FoodTemplate::BerriesSmurfberries2 => 1000,
            FoodTemplate::BerriesGoodberries => 1000,
            FoodTemplate::BerriesGoodberries2 => 1000,
            FoodTemplate::EyeballOfNed => 200,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            FoodTemplate::Mushroom => 2,
            FoodTemplate::MushroomOfPoison => 0,
            FoodTemplate::MushroomOfBlindness => 0,
            FoodTemplate::MushroomOfParanoia => 0,
            FoodTemplate::MushroomOfConfusion => 0,
            FoodTemplate::MushroomOfHallucination => 0,
            FoodTemplate::MushroomOfCurePoison => 60,
            FoodTemplate::MushroomOfCureBlindness => 50,
            FoodTemplate::MushroomOfCureParanoia => 25,
            FoodTemplate::MushroomOfCureConfusion => 50,
            FoodTemplate::MushroomOfWeakness => 0,
            FoodTemplate::MushroomOfUnhealth => 50,
            FoodTemplate::MushroomOfRestoreConstitution => 350,
            FoodTemplate::MushroomOfFirstAid => 5,
            FoodTemplate::MushroomOfMinorCures => 20,
            FoodTemplate::MushroomOfLightCures => 30,
            FoodTemplate::MushroomOfRestoring => 1000,
            FoodTemplate::MushroomOfPoison2 => 0,
            FoodTemplate::MushroomOfHallucination2 => 0,
            FoodTemplate::MushroomOfCurePoison2 => 75,
            FoodTemplate::MushroomOfUnhealth2 => 25,
            FoodTemplate::MushroomOfCureSeriousWounds => 75,
            FoodTemplate::PintOfFineGradeMush => 0,
            FoodTemplate::RationOfFood => 3,
            FoodTemplate::Mushroom2 => 2,
            FoodTemplate::HardBiscuit => 1,
            FoodTemplate::BeefJerky => 2,
            FoodTemplate::FineAle => 1,
            FoodTemplate::FineWine => 2,
            FoodTemplate::ElvishWaybread => 10,
            FoodTemplate::Stew => 0,
            FoodTemplate::JollyGreenJelly => 0,
            FoodTemplate::GreenJelly => 50,
            FoodTemplate::BerriesPoisonous => 0,
            FoodTemplate::BerriesSmurfberries => 0,
            FoodTemplate::BerriesSmurfberries2 => 0,
            FoodTemplate::BerriesGoodberries => 0,
            FoodTemplate::BerriesGoodberries2 => 0,
            FoodTemplate::EyeballOfNed => 50,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            FoodTemplate::Mushroom => ItemSubType::Food(FoodSubType::Mushroom),
            FoodTemplate::MushroomOfPoison => ItemSubType::Food(FoodSubType::MushroomOfPoison),
            FoodTemplate::MushroomOfBlindness => {
                ItemSubType::Food(FoodSubType::MushroomOfBlindness)
            }
            FoodTemplate::MushroomOfParanoia => ItemSubType::Food(FoodSubType::MushroomOfParanoia),
            FoodTemplate::MushroomOfConfusion => {
                ItemSubType::Food(FoodSubType::MushroomOfConfusion)
            }
            FoodTemplate::MushroomOfHallucination => {
                ItemSubType::Food(FoodSubType::MushroomOfHallucination)
            }
            FoodTemplate::MushroomOfCurePoison => {
                ItemSubType::Food(FoodSubType::MushroomOfCurePoison)
            }
            FoodTemplate::MushroomOfCureBlindness => {
                ItemSubType::Food(FoodSubType::MushroomOfCureBlindness)
            }
            FoodTemplate::MushroomOfCureParanoia => {
                ItemSubType::Food(FoodSubType::MushroomOfCureParanoia)
            }
            FoodTemplate::MushroomOfCureConfusion => {
                ItemSubType::Food(FoodSubType::MushroomOfCureConfusion)
            }
            FoodTemplate::MushroomOfWeakness => ItemSubType::Food(FoodSubType::MushroomOfWeakness),
            FoodTemplate::MushroomOfUnhealth => ItemSubType::Food(FoodSubType::MushroomOfUnhealth),
            FoodTemplate::MushroomOfRestoreConstitution => {
                ItemSubType::Food(FoodSubType::MushroomOfRestoreConstitution)
            }
            FoodTemplate::MushroomOfFirstAid => ItemSubType::Food(FoodSubType::MushroomOfFirstAid),
            FoodTemplate::MushroomOfMinorCures => {
                ItemSubType::Food(FoodSubType::MushroomOfMinorCures)
            }
            FoodTemplate::MushroomOfLightCures => {
                ItemSubType::Food(FoodSubType::MushroomOfLightCures)
            }
            FoodTemplate::MushroomOfRestoring => {
                ItemSubType::Food(FoodSubType::MushroomOfRestoring)
            }
            FoodTemplate::MushroomOfPoison2 => ItemSubType::Food(FoodSubType::MushroomOfPoison2),
            FoodTemplate::MushroomOfHallucination2 => {
                ItemSubType::Food(FoodSubType::MushroomOfHallucination2)
            }
            FoodTemplate::MushroomOfCurePoison2 => {
                ItemSubType::Food(FoodSubType::MushroomOfCurePoison2)
            }
            FoodTemplate::MushroomOfUnhealth2 => {
                ItemSubType::Food(FoodSubType::MushroomOfUnhealth2)
            }
            FoodTemplate::MushroomOfCureSeriousWounds => {
                ItemSubType::Food(FoodSubType::MushroomOfCureSeriousWounds)
            }
            FoodTemplate::PintOfFineGradeMush => {
                ItemSubType::Food(FoodSubType::PintOfFineGradeMush)
            }
            FoodTemplate::RationOfFood => ItemSubType::Food(FoodSubType::RationOfFood),
            FoodTemplate::Mushroom2 => ItemSubType::Food(FoodSubType::Mushroom2),
            FoodTemplate::HardBiscuit => ItemSubType::Food(FoodSubType::HardBiscuit),
            FoodTemplate::BeefJerky => ItemSubType::Food(FoodSubType::BeefJerky),
            FoodTemplate::FineAle => ItemSubType::Food(FoodSubType::FineAle),
            FoodTemplate::FineWine => ItemSubType::Food(FoodSubType::FineWine),
            FoodTemplate::ElvishWaybread => ItemSubType::Food(FoodSubType::ElvishWaybread),
            FoodTemplate::Stew => ItemSubType::Food(FoodSubType::Stew),
            FoodTemplate::JollyGreenJelly => ItemSubType::Food(FoodSubType::GreenJelly),
            FoodTemplate::GreenJelly => ItemSubType::Food(FoodSubType::GreenJelly),
            FoodTemplate::BerriesPoisonous => ItemSubType::Food(FoodSubType::BerriesPoisonous),
            FoodTemplate::BerriesSmurfberries => {
                ItemSubType::Food(FoodSubType::BerriesSmurfberries)
            }
            FoodTemplate::BerriesSmurfberries2 => {
                ItemSubType::Food(FoodSubType::BerriesSmurfberries)
            }
            FoodTemplate::BerriesGoodberries => ItemSubType::Food(FoodSubType::BerriesGoodberries),
            FoodTemplate::BerriesGoodberries2 => ItemSubType::Food(FoodSubType::BerriesGoodberries),
            FoodTemplate::EyeballOfNed => ItemSubType::Food(FoodSubType::EyeballOfNed),
        }
    }

    fn weight(&self) -> u16 {
        match self {
            FoodTemplate::Mushroom => 5,
            FoodTemplate::MushroomOfPoison => 1,
            FoodTemplate::MushroomOfBlindness => 1,
            FoodTemplate::MushroomOfParanoia => 1,
            FoodTemplate::MushroomOfConfusion => 1,
            FoodTemplate::MushroomOfHallucination => 1,
            FoodTemplate::MushroomOfCurePoison => 1,
            FoodTemplate::MushroomOfCureBlindness => 1,
            FoodTemplate::MushroomOfCureParanoia => 1,
            FoodTemplate::MushroomOfCureConfusion => 1,
            FoodTemplate::MushroomOfWeakness => 1,
            FoodTemplate::MushroomOfUnhealth => 1,
            FoodTemplate::MushroomOfRestoreConstitution => 1,
            FoodTemplate::MushroomOfFirstAid => 1,
            FoodTemplate::MushroomOfMinorCures => 1,
            FoodTemplate::MushroomOfLightCures => 1,
            FoodTemplate::MushroomOfRestoring => 1,
            FoodTemplate::MushroomOfPoison2 => 1,
            FoodTemplate::MushroomOfHallucination2 => 1,
            FoodTemplate::MushroomOfCurePoison2 => 1,
            FoodTemplate::MushroomOfUnhealth2 => 1,
            FoodTemplate::MushroomOfCureSeriousWounds => 1,
            FoodTemplate::PintOfFineGradeMush => 10,
            FoodTemplate::RationOfFood => 10,
            FoodTemplate::Mushroom2 => 5,
            FoodTemplate::HardBiscuit => 2,
            FoodTemplate::BeefJerky => 2,
            FoodTemplate::FineAle => 10,
            FoodTemplate::FineWine => 10,
            FoodTemplate::ElvishWaybread => 3,
            FoodTemplate::Stew => 3,
            FoodTemplate::JollyGreenJelly => 3,
            FoodTemplate::GreenJelly => 3,
            FoodTemplate::BerriesPoisonous => 3,
            FoodTemplate::BerriesSmurfberries => 3,
            FoodTemplate::BerriesSmurfberries2 => 3,
            FoodTemplate::BerriesGoodberries => 3,
            FoodTemplate::BerriesGoodberries2 => 3,
            FoodTemplate::EyeballOfNed => 2,
        }
    }

    fn number(&self) -> u16 {
        match self {
            FoodTemplate::Mushroom => 1,
            FoodTemplate::MushroomOfPoison => 1,
            FoodTemplate::MushroomOfBlindness => 1,
            FoodTemplate::MushroomOfParanoia => 1,
            FoodTemplate::MushroomOfConfusion => 1,
            FoodTemplate::MushroomOfHallucination => 1,
            FoodTemplate::MushroomOfCurePoison => 1,
            FoodTemplate::MushroomOfCureBlindness => 1,
            FoodTemplate::MushroomOfCureParanoia => 1,
            FoodTemplate::MushroomOfCureConfusion => 1,
            FoodTemplate::MushroomOfWeakness => 1,
            FoodTemplate::MushroomOfUnhealth => 1,
            FoodTemplate::MushroomOfRestoreConstitution => 1,
            FoodTemplate::MushroomOfFirstAid => 1,
            FoodTemplate::MushroomOfMinorCures => 1,
            FoodTemplate::MushroomOfLightCures => 1,
            FoodTemplate::MushroomOfRestoring => 1,
            FoodTemplate::MushroomOfPoison2 => 1,
            FoodTemplate::MushroomOfHallucination2 => 1,
            FoodTemplate::MushroomOfCurePoison2 => 1,
            FoodTemplate::MushroomOfUnhealth2 => 1,
            FoodTemplate::MushroomOfCureSeriousWounds => 1,
            FoodTemplate::PintOfFineGradeMush => 1,
            FoodTemplate::RationOfFood => 1,
            FoodTemplate::Mushroom2 => 1,
            FoodTemplate::HardBiscuit => 1,
            FoodTemplate::BeefJerky => 1,
            FoodTemplate::FineAle => 1,
            FoodTemplate::FineWine => 1,
            FoodTemplate::ElvishWaybread => 1,
            FoodTemplate::Stew => 1,
            FoodTemplate::JollyGreenJelly => 1,
            FoodTemplate::GreenJelly => 1,
            FoodTemplate::BerriesPoisonous => 1,
            FoodTemplate::BerriesSmurfberries => 1,
            FoodTemplate::BerriesSmurfberries2 => 1,
            FoodTemplate::BerriesGoodberries => 1,
            FoodTemplate::BerriesGoodberries2 => 1,
            FoodTemplate::EyeballOfNed => 2,
        }
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
            FoodTemplate::Mushroom => "0d0",
            FoodTemplate::MushroomOfPoison => "0d0",
            FoodTemplate::MushroomOfBlindness => "0d0",
            FoodTemplate::MushroomOfParanoia => "0d0",
            FoodTemplate::MushroomOfConfusion => "0d0",
            FoodTemplate::MushroomOfHallucination => "0d0",
            FoodTemplate::MushroomOfCurePoison => "0d0",
            FoodTemplate::MushroomOfCureBlindness => "0d0",
            FoodTemplate::MushroomOfCureParanoia => "0d0",
            FoodTemplate::MushroomOfCureConfusion => "0d0",
            FoodTemplate::MushroomOfWeakness => "0d0",
            FoodTemplate::MushroomOfUnhealth => "10d10",
            FoodTemplate::MushroomOfRestoreConstitution => "0d0",
            FoodTemplate::MushroomOfFirstAid => "0d0",
            FoodTemplate::MushroomOfMinorCures => "0d0",
            FoodTemplate::MushroomOfLightCures => "0d0",
            FoodTemplate::MushroomOfRestoring => "0d0",
            FoodTemplate::MushroomOfPoison2 => "0d0",
            FoodTemplate::MushroomOfHallucination2 => "0d0",
            FoodTemplate::MushroomOfCurePoison2 => "0d0",
            FoodTemplate::MushroomOfUnhealth2 => "6d8",
            FoodTemplate::MushroomOfCureSeriousWounds => "0d0",
            FoodTemplate::PintOfFineGradeMush => "0d0",
            FoodTemplate::RationOfFood => "0d0",
            FoodTemplate::Mushroom2 => "0d0",
            FoodTemplate::HardBiscuit => "0d0",
            FoodTemplate::BeefJerky => "0d0",
            FoodTemplate::FineAle => "0d0",
            FoodTemplate::FineWine => "0d0",
            FoodTemplate::ElvishWaybread => "0d0",
            FoodTemplate::Stew => "0d0",
            FoodTemplate::JollyGreenJelly => "0d0",
            FoodTemplate::GreenJelly => "0d0",
            FoodTemplate::BerriesPoisonous => "0d0",
            FoodTemplate::BerriesSmurfberries => "0d0",
            FoodTemplate::BerriesSmurfberries2 => "0d0",
            FoodTemplate::BerriesGoodberries => "0d0",
            FoodTemplate::BerriesGoodberries2 => "0d0",
            FoodTemplate::EyeballOfNed => "6d5",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            FoodTemplate::Mushroom => 255,
            FoodTemplate::MushroomOfPoison => 40,
            FoodTemplate::MushroomOfBlindness => 255,
            FoodTemplate::MushroomOfParanoia => 8,
            FoodTemplate::MushroomOfConfusion => 20,
            FoodTemplate::MushroomOfHallucination => 255,
            FoodTemplate::MushroomOfCurePoison => 6,
            FoodTemplate::MushroomOfCureBlindness => 20,
            FoodTemplate::MushroomOfCureParanoia => 255,
            FoodTemplate::MushroomOfCureConfusion => 255,
            FoodTemplate::MushroomOfWeakness => 255,
            FoodTemplate::MushroomOfUnhealth => 255,
            FoodTemplate::MushroomOfRestoreConstitution => 50,
            FoodTemplate::MushroomOfFirstAid => 1,
            FoodTemplate::MushroomOfMinorCures => 255,
            FoodTemplate::MushroomOfLightCures => 9,
            FoodTemplate::MushroomOfRestoring => 7,
            FoodTemplate::MushroomOfPoison2 => 10,
            FoodTemplate::MushroomOfHallucination2 => 6,
            FoodTemplate::MushroomOfCurePoison2 => 12,
            FoodTemplate::MushroomOfUnhealth2 => 8,
            FoodTemplate::MushroomOfCureSeriousWounds => 19,
            FoodTemplate::PintOfFineGradeMush => 16,
            FoodTemplate::RationOfFood => 6,
            FoodTemplate::Mushroom2 => 13,
            FoodTemplate::HardBiscuit => 18,
            FoodTemplate::BeefJerky => 10,
            FoodTemplate::FineAle => 7,
            FoodTemplate::FineWine => 9,
            FoodTemplate::ElvishWaybread => 7,
            FoodTemplate::Stew => 15,
            FoodTemplate::JollyGreenJelly => 20,
            FoodTemplate::GreenJelly => 30,
            FoodTemplate::BerriesPoisonous => 15,
            FoodTemplate::BerriesSmurfberries => 28,
            FoodTemplate::BerriesSmurfberries2 => 7,
            FoodTemplate::BerriesGoodberries => 255,
            FoodTemplate::BerriesGoodberries2 => 0,
            FoodTemplate::EyeballOfNed => 255,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
