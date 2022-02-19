use std::borrow::Cow;

use model;
use item_template;
use logic::item_name;

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
    GreenJelly,
    BerriesPoisonous,
    BerriesSmurfberries,
    BerriesGoodberries,
    EyeballOfNed,
}

impl FoodTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
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
            Box::new(FoodTemplate::GreenJelly),
            Box::new(FoodTemplate::BerriesPoisonous),
            Box::new(FoodTemplate::BerriesSmurfberries),
            Box::new(FoodTemplate::BerriesGoodberries),
            Box::new(FoodTemplate::EyeballOfNed),
            ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        FoodTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            256 => Box::new(FoodTemplate::Mushroom),
            257 => Box::new(FoodTemplate::MushroomOfPoison),
            258 => Box::new(FoodTemplate::MushroomOfBlindness),
            259 => Box::new(FoodTemplate::MushroomOfParanoia),
            260 => Box::new(FoodTemplate::MushroomOfConfusion),
            261 => Box::new(FoodTemplate::MushroomOfHallucination),
            262 => Box::new(FoodTemplate::MushroomOfCurePoison),
            263 => Box::new(FoodTemplate::MushroomOfCureBlindness),
            264 => Box::new(FoodTemplate::MushroomOfCureParanoia),
            265 => Box::new(FoodTemplate::MushroomOfCureConfusion),
            266 => Box::new(FoodTemplate::MushroomOfWeakness),
            267 => Box::new(FoodTemplate::MushroomOfUnhealth),
            268 => Box::new(FoodTemplate::MushroomOfRestoreConstitution),
            269 => Box::new(FoodTemplate::MushroomOfFirstAid),
            270 => Box::new(FoodTemplate::MushroomOfMinorCures),
            271 => Box::new(FoodTemplate::MushroomOfLightCures),
            272 => Box::new(FoodTemplate::MushroomOfRestoring),
            273 => Box::new(FoodTemplate::MushroomOfPoison2),
            274 => Box::new(FoodTemplate::MushroomOfHallucination2),
            275 => Box::new(FoodTemplate::MushroomOfCurePoison2),
            276 => Box::new(FoodTemplate::MushroomOfUnhealth2),
            277 => Box::new(FoodTemplate::MushroomOfCureSeriousWounds),
            306 => Box::new(FoodTemplate::PintOfFineGradeMush),
            307 => Box::new(FoodTemplate::RationOfFood),
            308 => Box::new(FoodTemplate::Mushroom2),
            309 => Box::new(FoodTemplate::HardBiscuit),
            310 => Box::new(FoodTemplate::BeefJerky),
            311 => Box::new(FoodTemplate::FineAle),
            312 => Box::new(FoodTemplate::FineWine),
            313 => Box::new(FoodTemplate::ElvishWaybread),
            314 => Box::new(FoodTemplate::Stew),
            315 => Box::new(FoodTemplate::GreenJelly),
            316 => Box::new(FoodTemplate::BerriesPoisonous),
            317 => Box::new(FoodTemplate::BerriesSmurfberries),
            318 => Box::new(FoodTemplate::BerriesGoodberries),
            319 => Box::new(FoodTemplate::EyeballOfNed),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for FoodTemplate {
    fn name(&self, item: &model::Item) -> String {
        let plural_s = || if item.number == 1 { "" } else { "s" };
        let mut parts = Vec::new();
        parts.push(item_name::number_of(item));
        parts.push(Cow::from(match self {
            FoodTemplate::Mushroom |
            FoodTemplate::Mushroom2 |
            FoodTemplate::MushroomOfPoison |
            FoodTemplate::MushroomOfBlindness |
            FoodTemplate::MushroomOfParanoia |
            FoodTemplate::MushroomOfConfusion |
            FoodTemplate::MushroomOfHallucination |
            FoodTemplate::MushroomOfCurePoison |
            FoodTemplate::MushroomOfCureBlindness |
            FoodTemplate::MushroomOfCureParanoia |
            FoodTemplate::MushroomOfCureConfusion |
            FoodTemplate::MushroomOfWeakness |
            FoodTemplate::MushroomOfUnhealth |
            FoodTemplate::MushroomOfRestoreConstitution |
            FoodTemplate::MushroomOfFirstAid |
            FoodTemplate::MushroomOfMinorCures |
            FoodTemplate::MushroomOfLightCures |
            FoodTemplate::MushroomOfRestoring |
            FoodTemplate::MushroomOfPoison2 |
            FoodTemplate::MushroomOfHallucination2 |
            FoodTemplate::MushroomOfCurePoison2 |
            FoodTemplate::MushroomOfUnhealth2 |
            FoodTemplate::MushroomOfCureSeriousWounds
                => format!("Mushroom{}", plural_s()),
            FoodTemplate::PintOfFineGradeMush
                => format!("Pint{} of Fine Grade Mush", plural_s()),
            FoodTemplate::RationOfFood
                => format!("Ration{} of Food", plural_s()),
            FoodTemplate::HardBiscuit
                => format!("Hard Biscuit{}", plural_s()),
            FoodTemplate::BeefJerky
                => format!("Strip{} of Beef Jerky", plural_s()),
            FoodTemplate::FineAle
                => format!("Pint{} of Fine Ale", plural_s()),
            FoodTemplate::FineWine
                => format!("Pint{} of Fine Wine", plural_s()),
            FoodTemplate::ElvishWaybread
                => format!("Piece{} of Elvish Waybread", plural_s()),
            FoodTemplate::Stew
                => format!("Stew{}", plural_s()),
            FoodTemplate::GreenJelly
                => format!("Green Jelly{}", plural_s()),
            FoodTemplate::BerriesPoisonous |
            FoodTemplate::BerriesSmurfberries |
            FoodTemplate::BerriesGoodberries
                => format!("Handful{} of Berries", plural_s()),
            FoodTemplate::EyeballOfNed
                => format!("Eyeball{}", plural_s()),
        }));

        if self.is_identified() {
            parts.push(Cow::from(match self {
                FoodTemplate::MushroomOfPoison => " of Poison",
                FoodTemplate::MushroomOfBlindness => " of Blindness",
                FoodTemplate::MushroomOfParanoia => " of Paranoia",
                FoodTemplate::MushroomOfConfusion => " of Confusion",
                FoodTemplate::MushroomOfHallucination => " of Hallucination",
                FoodTemplate::MushroomOfCurePoison => " of Cure Poison",
                FoodTemplate::MushroomOfCureBlindness => " of Cure Blindness",
                FoodTemplate::MushroomOfCureParanoia => " of Cure Paranoia",
                FoodTemplate::MushroomOfCureConfusion => " of Cure Confusion",
                FoodTemplate::MushroomOfWeakness => " of Weakness",
                FoodTemplate::MushroomOfUnhealth => " of Unhealth",
                FoodTemplate::MushroomOfRestoreConstitution => " of Restore Constitution",
                FoodTemplate::MushroomOfFirstAid => " of First-Aid",
                FoodTemplate::MushroomOfMinorCures => " of Minor Cures",
                FoodTemplate::MushroomOfLightCures => " of Light Cures",
                FoodTemplate::MushroomOfRestoring => " of Restoring",
                FoodTemplate::MushroomOfPoison2 => " of Poison",
                FoodTemplate::MushroomOfHallucination2 => " of Hallucination",
                FoodTemplate::MushroomOfCurePoison2 => " of Cure Poison",
                FoodTemplate::MushroomOfUnhealth2 => " of Unhealth",
                FoodTemplate::MushroomOfCureSeriousWounds => " of Cure Serious Wounds",
                FoodTemplate::BerriesPoisonous => " (Poisonous)",
                FoodTemplate::BerriesSmurfberries => " (Smurfberries)",
                FoodTemplate::BerriesGoodberries => " (Goodberries)",
                FoodTemplate::EyeballOfNed => " of Ned",
                _ => "",
            }));
        }
        parts.join("")
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Food }
    fn flags1(&self) -> u64 { 0 }

    fn flags2(&self) -> u64 {
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
            FoodTemplate::GreenJelly => 0x22400060,
            FoodTemplate::BerriesPoisonous => 0x0C0000000,
            FoodTemplate::BerriesSmurfberries => 0x10400000,
            FoodTemplate::BerriesGoodberries => 0x10C00080,
            FoodTemplate::EyeballOfNed => 0x00000053,

        }
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
            FoodTemplate::GreenJelly => 4000,
            FoodTemplate::BerriesPoisonous => 1000,
            FoodTemplate::BerriesSmurfberries => 1000,
            FoodTemplate::BerriesGoodberries => 1000,
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
            FoodTemplate::GreenJelly => 50,
            FoodTemplate::BerriesPoisonous => 0,
            FoodTemplate::BerriesSmurfberries => 0,
            FoodTemplate::BerriesGoodberries => 0,
            FoodTemplate::EyeballOfNed => 50,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            FoodTemplate::Mushroom => 256,
            FoodTemplate::MushroomOfPoison => 257,
            FoodTemplate::MushroomOfBlindness => 258,
            FoodTemplate::MushroomOfParanoia => 259,
            FoodTemplate::MushroomOfConfusion => 260,
            FoodTemplate::MushroomOfHallucination => 261,
            FoodTemplate::MushroomOfCurePoison => 262,
            FoodTemplate::MushroomOfCureBlindness => 263,
            FoodTemplate::MushroomOfCureParanoia => 264,
            FoodTemplate::MushroomOfCureConfusion => 265,
            FoodTemplate::MushroomOfWeakness => 266,
            FoodTemplate::MushroomOfUnhealth => 267,
            FoodTemplate::MushroomOfRestoreConstitution => 268,
            FoodTemplate::MushroomOfFirstAid => 269,
            FoodTemplate::MushroomOfMinorCures => 270,
            FoodTemplate::MushroomOfLightCures => 271,
            FoodTemplate::MushroomOfRestoring => 272,
            FoodTemplate::MushroomOfPoison2 => 273,
            FoodTemplate::MushroomOfHallucination2 => 274,
            FoodTemplate::MushroomOfCurePoison2 => 275,
            FoodTemplate::MushroomOfUnhealth2 => 276,
            FoodTemplate::MushroomOfCureSeriousWounds => 277,
            FoodTemplate::PintOfFineGradeMush => 306,
            FoodTemplate::RationOfFood => 307,
            FoodTemplate::Mushroom2 => 308,
            FoodTemplate::HardBiscuit => 309,
            FoodTemplate::BeefJerky => 310,
            FoodTemplate::FineAle => 311,
            FoodTemplate::FineWine => 312,
            FoodTemplate::ElvishWaybread => 313,
            FoodTemplate::Stew => 314,
            FoodTemplate::GreenJelly => 315,
            FoodTemplate::BerriesPoisonous => 316,
            FoodTemplate::BerriesSmurfberries => 317,
            FoodTemplate::BerriesGoodberries => 318,
            FoodTemplate::EyeballOfNed => 319,
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
            FoodTemplate::GreenJelly => 3,
            FoodTemplate::BerriesPoisonous => 3,
            FoodTemplate::BerriesSmurfberries => 3,
            FoodTemplate::BerriesGoodberries => 3,
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
            FoodTemplate::GreenJelly => 1,
            FoodTemplate::BerriesPoisonous => 1,
            FoodTemplate::BerriesSmurfberries => 1,
            FoodTemplate::BerriesGoodberries => 1,
            FoodTemplate::EyeballOfNed => 2,

        }
    }

    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }

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
            FoodTemplate::GreenJelly => "0d0",
            FoodTemplate::BerriesPoisonous => "0d0",
            FoodTemplate::BerriesSmurfberries => "0d0",
            FoodTemplate::BerriesGoodberries => "0d0",
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
            FoodTemplate::GreenJelly => 30,
            FoodTemplate::BerriesPoisonous => 15,
            FoodTemplate::BerriesSmurfberries => 28,
            FoodTemplate::BerriesGoodberries => 255,
            FoodTemplate::EyeballOfNed => 255,
        }
    }

    fn is_identified(&self) -> bool { false }
}
