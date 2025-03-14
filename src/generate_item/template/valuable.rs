use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{GemSubType, ItemSubType, JewelrySubType, WearableGemSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ValuableTemplate {
    GemOfTeleportation,
    GemOfResistCold,
    GemOfResistAcid,
    GemOfSeeInvisible,
    GemOfStealth,
    GemOfSlowDigestion,
    GemOfProtectFire,
    GemOfDetectMonsters,
    GemOfDispelEvil,
    GemOfDarkness,
    GemOfAcidBalls,
    GemOfDetectInvisible,
    GemOfIdentify,
    GemOfLight,
    GemOfSummoning,
    GemOfRemoveCurse,
    GemOfAnnihilation,
    GemOfRecall,
    FineAgate,
    FineDiamond,
    RoughDiamond,
    RoughSapphire,
    FineSapphire,
    SmallBagOfOpals,
    SmallBagOfSapphires,
    SmallPouchOfDiamonds,
    LargeSackOfPearls,
    LargeSackOfSapphires,
    LargePouchOfDiamonds,
    SmallGoldPendant,
    SmallMithrilPendant,
    LargeMithrilGarterBelt,
    SmallSilverPendant,
}

impl ValuableTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(ValuableTemplate::GemOfTeleportation),
            Box::new(ValuableTemplate::GemOfResistCold),
            Box::new(ValuableTemplate::GemOfResistAcid),
            Box::new(ValuableTemplate::GemOfSeeInvisible),
            Box::new(ValuableTemplate::GemOfStealth),
            Box::new(ValuableTemplate::GemOfSlowDigestion),
            Box::new(ValuableTemplate::GemOfProtectFire),
            Box::new(ValuableTemplate::GemOfDetectMonsters),
            Box::new(ValuableTemplate::GemOfDispelEvil),
            Box::new(ValuableTemplate::GemOfDarkness),
            Box::new(ValuableTemplate::GemOfAcidBalls),
            Box::new(ValuableTemplate::GemOfDetectInvisible),
            Box::new(ValuableTemplate::GemOfIdentify),
            Box::new(ValuableTemplate::GemOfLight),
            Box::new(ValuableTemplate::GemOfSummoning),
            Box::new(ValuableTemplate::GemOfRemoveCurse),
            Box::new(ValuableTemplate::GemOfAnnihilation),
            Box::new(ValuableTemplate::GemOfRecall),
            Box::new(ValuableTemplate::FineAgate),
            Box::new(ValuableTemplate::FineDiamond),
            Box::new(ValuableTemplate::RoughDiamond),
            Box::new(ValuableTemplate::RoughSapphire),
            Box::new(ValuableTemplate::FineSapphire),
            Box::new(ValuableTemplate::SmallBagOfOpals),
            Box::new(ValuableTemplate::SmallBagOfSapphires),
            Box::new(ValuableTemplate::SmallPouchOfDiamonds),
            Box::new(ValuableTemplate::LargeSackOfPearls),
            Box::new(ValuableTemplate::LargeSackOfSapphires),
            Box::new(ValuableTemplate::LargePouchOfDiamonds),
            Box::new(ValuableTemplate::SmallGoldPendant),
            Box::new(ValuableTemplate::SmallMithrilPendant),
            Box::new(ValuableTemplate::LargeMithrilGarterBelt),
            Box::new(ValuableTemplate::SmallSilverPendant),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        ValuableTemplate::vec().into_iter()
    }
}

impl ItemTemplate for ValuableTemplate {
    fn name(&self) -> &str {
        match self {
            ValuableTemplate::GemOfTeleportation => "& Finely cut| of Teleportation^",
            ValuableTemplate::GemOfResistCold => "& Finely cut| of Resist Cold^",
            ValuableTemplate::GemOfResistAcid => "& Finely cut| of Resist Acid^",
            ValuableTemplate::GemOfSeeInvisible => "& Finely cut| of See Invisible^",
            ValuableTemplate::GemOfStealth => "& Finely cut| of Stealth^",
            ValuableTemplate::GemOfSlowDigestion => "& Finely cut| of Slow Digestion^",
            ValuableTemplate::GemOfProtectFire => "& Finely cut| of Lordly Protection (FIRE)^",
            ValuableTemplate::GemOfDetectMonsters => {
                "& Finely cut| of Detect Monsters^ (%P1 charges)"
            }
            ValuableTemplate::GemOfDispelEvil => "& Finely cut| of Dispel Evil^ (%P1 charges)",
            ValuableTemplate::GemOfDarkness => "& Finely cut| of Darkness^ (%P1 charges)",
            ValuableTemplate::GemOfAcidBalls => "& Finely cut| of Acid Balls^ (%P1 charges)",
            ValuableTemplate::GemOfDetectInvisible => {
                "& Finely cut| of Detect Invisible^ (%P1 charges)"
            }
            ValuableTemplate::GemOfIdentify => "& Finely cut| of Identify^ (%P1 charges)",
            ValuableTemplate::GemOfLight => "& Finely cut| of Light^ (%P1 charges)",
            ValuableTemplate::GemOfSummoning => "& Finely cut| of Summoning^ (%P1 charges)",
            ValuableTemplate::GemOfRemoveCurse => "& Finely cut| of Remove Curse^ (%P1 charges)",
            ValuableTemplate::GemOfAnnihilation => "& Finely cut| of Annihilation^ (%P1 charges)",
            ValuableTemplate::GemOfRecall => "& Finely cut| of Recall^ (%P1 charges)",
            ValuableTemplate::FineAgate => "& Finely cut Agate~^",
            ValuableTemplate::FineDiamond => "& Finely cut Diamond~^",
            ValuableTemplate::RoughDiamond => "& Rough cut Diamond~^",
            ValuableTemplate::RoughSapphire => "& Rough cut Sapphire~^",
            ValuableTemplate::FineSapphire => "& Finely cut Sapphire~^",
            ValuableTemplate::SmallBagOfOpals => "& Small bag~ of Opals^",
            ValuableTemplate::SmallBagOfSapphires => "& Small bag~ of Sapphires^",
            ValuableTemplate::SmallPouchOfDiamonds => "& Small pouch~ of Diamonds^",
            ValuableTemplate::LargeSackOfPearls => "& Large sack~ of Pearls^",
            ValuableTemplate::LargeSackOfSapphires => "& Large sack~ of Sapphires^",
            ValuableTemplate::LargePouchOfDiamonds => "& Large pouch~ of Diamonds^",
            ValuableTemplate::SmallGoldPendant => "& Small gold pendant~^",
            ValuableTemplate::SmallMithrilPendant => "& Small mithril pendant~^",
            ValuableTemplate::LargeMithrilGarterBelt => "& Large mithril garter-belt~^",
            ValuableTemplate::SmallSilverPendant => "& Small silver pendant~",
        }
    }

    fn item_type(&self) -> model::ItemType {
        match self {
            ValuableTemplate::GemOfTeleportation => model::ItemType::WearableGem,
            ValuableTemplate::GemOfResistCold => model::ItemType::WearableGem,
            ValuableTemplate::GemOfResistAcid => model::ItemType::WearableGem,
            ValuableTemplate::GemOfSeeInvisible => model::ItemType::WearableGem,
            ValuableTemplate::GemOfStealth => model::ItemType::WearableGem,
            ValuableTemplate::GemOfSlowDigestion => model::ItemType::WearableGem,
            ValuableTemplate::GemOfProtectFire => model::ItemType::WearableGem,
            ValuableTemplate::GemOfDetectMonsters => model::ItemType::Gem,
            ValuableTemplate::GemOfDispelEvil => model::ItemType::Gem,
            ValuableTemplate::GemOfDarkness => model::ItemType::Gem,
            ValuableTemplate::GemOfAcidBalls => model::ItemType::Gem,
            ValuableTemplate::GemOfDetectInvisible => model::ItemType::Gem,
            ValuableTemplate::GemOfIdentify => model::ItemType::Gem,
            ValuableTemplate::GemOfLight => model::ItemType::Gem,
            ValuableTemplate::GemOfSummoning => model::ItemType::Gem,
            ValuableTemplate::GemOfRemoveCurse => model::ItemType::Gem,
            ValuableTemplate::GemOfAnnihilation => model::ItemType::Gem,
            ValuableTemplate::GemOfRecall => model::ItemType::Gem,
            ValuableTemplate::FineAgate => model::ItemType::Gem,
            ValuableTemplate::FineDiamond => model::ItemType::Gem,
            ValuableTemplate::RoughDiamond => model::ItemType::Gem,
            ValuableTemplate::RoughSapphire => model::ItemType::Gem,
            ValuableTemplate::FineSapphire => model::ItemType::Gem,
            ValuableTemplate::SmallBagOfOpals => model::ItemType::Gem,
            ValuableTemplate::SmallBagOfSapphires => model::ItemType::Gem,
            ValuableTemplate::SmallPouchOfDiamonds => model::ItemType::Gem,
            ValuableTemplate::LargeSackOfPearls => model::ItemType::Gem,
            ValuableTemplate::LargeSackOfSapphires => model::ItemType::Gem,
            ValuableTemplate::LargePouchOfDiamonds => model::ItemType::Gem,
            ValuableTemplate::SmallGoldPendant => model::ItemType::Jewelry,
            ValuableTemplate::SmallMithrilPendant => model::ItemType::Jewelry,
            ValuableTemplate::LargeMithrilGarterBelt => model::ItemType::Jewelry,
            ValuableTemplate::SmallSilverPendant => model::ItemType::Jewelry,
        }
    }

    fn flags1(&self) -> u64 {
        match self {
            ValuableTemplate::GemOfTeleportation => 0x00000400,
            ValuableTemplate::GemOfResistCold => 0x00200000,
            ValuableTemplate::GemOfResistAcid => 0x00100000,
            ValuableTemplate::GemOfSeeInvisible => 0x01000000,
            ValuableTemplate::GemOfStealth => 0x00000100,
            ValuableTemplate::GemOfSlowDigestion => 0x00000800,
            ValuableTemplate::GemOfProtectFire => 0x00000800,
            ValuableTemplate::GemOfDetectMonsters => 0x00040000,
            ValuableTemplate::GemOfDispelEvil => 0x00080000,
            ValuableTemplate::GemOfDarkness => 0x00100000,
            ValuableTemplate::GemOfAcidBalls => 0x00200000,
            ValuableTemplate::GemOfDetectInvisible => 0x00400000,
            ValuableTemplate::GemOfIdentify => 0x00800000,
            ValuableTemplate::GemOfLight => 0x01000000,
            ValuableTemplate::GemOfSummoning => 0x02000000,
            ValuableTemplate::GemOfRemoveCurse => 0x04000000,
            ValuableTemplate::GemOfAnnihilation => 0x08000000,
            ValuableTemplate::GemOfRecall => 0x10000000,
            ValuableTemplate::FineAgate => 0,
            ValuableTemplate::FineDiamond => 0,
            ValuableTemplate::RoughDiamond => 0,
            ValuableTemplate::RoughSapphire => 0,
            ValuableTemplate::FineSapphire => 0,
            ValuableTemplate::SmallBagOfOpals => 0,
            ValuableTemplate::SmallBagOfSapphires => 0,
            ValuableTemplate::SmallPouchOfDiamonds => 0,
            ValuableTemplate::LargeSackOfPearls => 0,
            ValuableTemplate::LargeSackOfSapphires => 0,
            ValuableTemplate::LargePouchOfDiamonds => 0,
            ValuableTemplate::SmallGoldPendant => 0,
            ValuableTemplate::SmallMithrilPendant => 0,
            ValuableTemplate::LargeMithrilGarterBelt => 0,
            ValuableTemplate::SmallSilverPendant => 0,
        }
    }

    fn flags2(&self) -> u64 {
        0
    }

    fn p1(&self) -> i64 {
        0
    }

    fn cost(&self) -> i64 {
        match self {
            ValuableTemplate::GemOfTeleportation => 300,
            ValuableTemplate::GemOfResistCold => 250,
            ValuableTemplate::GemOfResistAcid => 250,
            ValuableTemplate::GemOfSeeInvisible => 350,
            ValuableTemplate::GemOfStealth => 300,
            ValuableTemplate::GemOfSlowDigestion => 200,
            ValuableTemplate::GemOfProtectFire => 1200,
            ValuableTemplate::GemOfDetectMonsters => 350,
            ValuableTemplate::GemOfDispelEvil => 1200,
            ValuableTemplate::GemOfDarkness => 0,
            ValuableTemplate::GemOfAcidBalls => 1800,
            ValuableTemplate::GemOfDetectInvisible => 225,
            ValuableTemplate::GemOfIdentify => 400,
            ValuableTemplate::GemOfLight => 300,
            ValuableTemplate::GemOfSummoning => 0,
            ValuableTemplate::GemOfRemoveCurse => 700,
            ValuableTemplate::GemOfAnnihilation => 1000,
            ValuableTemplate::GemOfRecall => 1200,
            ValuableTemplate::FineAgate => 50,
            ValuableTemplate::FineDiamond => 500,
            ValuableTemplate::RoughDiamond => 100,
            ValuableTemplate::RoughSapphire => 40,
            ValuableTemplate::FineSapphire => 250,
            ValuableTemplate::SmallBagOfOpals => 250,
            ValuableTemplate::SmallBagOfSapphires => 450,
            ValuableTemplate::SmallPouchOfDiamonds => 1000,
            ValuableTemplate::LargeSackOfPearls => 650,
            ValuableTemplate::LargeSackOfSapphires => 600,
            ValuableTemplate::LargePouchOfDiamonds => 2000,
            ValuableTemplate::SmallGoldPendant => 75,
            ValuableTemplate::SmallMithrilPendant => 350,
            ValuableTemplate::LargeMithrilGarterBelt => 1500,
            ValuableTemplate::SmallSilverPendant => 60,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            ValuableTemplate::GemOfTeleportation => {
                ItemSubType::WearableGem(WearableGemSubType::GemOfTeleportation)
            }
            ValuableTemplate::GemOfResistCold => {
                ItemSubType::WearableGem(WearableGemSubType::GemOfResistCold)
            }
            ValuableTemplate::GemOfResistAcid => {
                ItemSubType::WearableGem(WearableGemSubType::GemOfResistAcid)
            }
            ValuableTemplate::GemOfSeeInvisible => {
                ItemSubType::WearableGem(WearableGemSubType::GemOfSeeInvisible)
            }
            ValuableTemplate::GemOfStealth => {
                ItemSubType::WearableGem(WearableGemSubType::GemOfStealth)
            }
            ValuableTemplate::GemOfSlowDigestion => {
                ItemSubType::WearableGem(WearableGemSubType::GemOfSlowDigestion)
            }
            ValuableTemplate::GemOfProtectFire => {
                ItemSubType::WearableGem(WearableGemSubType::GemOfProtectFire)
            }
            ValuableTemplate::GemOfDetectMonsters => {
                ItemSubType::Gem(GemSubType::GemOfDetectMonsters)
            }
            ValuableTemplate::GemOfDispelEvil => ItemSubType::Gem(GemSubType::GemOfDispelEvil),
            ValuableTemplate::GemOfDarkness => ItemSubType::Gem(GemSubType::GemOfDarkness),
            ValuableTemplate::GemOfAcidBalls => ItemSubType::Gem(GemSubType::GemOfAcidBalls),
            ValuableTemplate::GemOfDetectInvisible => {
                ItemSubType::Gem(GemSubType::GemOfDetectInvisible)
            }
            ValuableTemplate::GemOfIdentify => ItemSubType::Gem(GemSubType::GemOfIdentify),
            ValuableTemplate::GemOfLight => ItemSubType::Gem(GemSubType::GemOfLight),
            ValuableTemplate::GemOfSummoning => ItemSubType::Gem(GemSubType::GemOfSummoning),
            ValuableTemplate::GemOfRemoveCurse => ItemSubType::Gem(GemSubType::GemOfRemoveCurse),
            ValuableTemplate::GemOfAnnihilation => ItemSubType::Gem(GemSubType::GemOfAnnihilation),
            ValuableTemplate::GemOfRecall => ItemSubType::Gem(GemSubType::GemOfRecall),
            ValuableTemplate::FineAgate => ItemSubType::Gem(GemSubType::FineAgate),
            ValuableTemplate::FineDiamond => ItemSubType::Gem(GemSubType::FineDiamond),
            ValuableTemplate::RoughDiamond => ItemSubType::Gem(GemSubType::RoughDiamond),
            ValuableTemplate::RoughSapphire => ItemSubType::Gem(GemSubType::RoughSapphire),
            ValuableTemplate::FineSapphire => ItemSubType::Gem(GemSubType::FineSapphire),
            ValuableTemplate::SmallBagOfOpals => ItemSubType::Gem(GemSubType::SmallBagOfOpals),
            ValuableTemplate::SmallBagOfSapphires => {
                ItemSubType::Gem(GemSubType::SmallBagOfSapphires)
            }
            ValuableTemplate::SmallPouchOfDiamonds => {
                ItemSubType::Gem(GemSubType::SmallPouchOfDiamonds)
            }
            ValuableTemplate::LargeSackOfPearls => ItemSubType::Gem(GemSubType::LargeSackOfPearls),
            ValuableTemplate::LargeSackOfSapphires => {
                ItemSubType::Gem(GemSubType::LargeSackOfSapphires)
            }
            ValuableTemplate::LargePouchOfDiamonds => {
                ItemSubType::Gem(GemSubType::LargePouchOfDiamonds)
            }
            ValuableTemplate::SmallGoldPendant => {
                ItemSubType::Jewelry(JewelrySubType::SmallGoldPendant)
            }
            ValuableTemplate::SmallMithrilPendant => {
                ItemSubType::Jewelry(JewelrySubType::SmallMithrilPendant)
            }
            ValuableTemplate::LargeMithrilGarterBelt => {
                ItemSubType::Jewelry(JewelrySubType::LargeMithrilGarterBelt)
            }
            ValuableTemplate::SmallSilverPendant => {
                ItemSubType::Jewelry(JewelrySubType::SmallSilverPendant)
            }
        }
    }

    fn weight(&self) -> u16 {
        match self {
            ValuableTemplate::GemOfTeleportation => 5,
            ValuableTemplate::GemOfResistCold => 5,
            ValuableTemplate::GemOfResistAcid => 5,
            ValuableTemplate::GemOfSeeInvisible => 5,
            ValuableTemplate::GemOfStealth => 5,
            ValuableTemplate::GemOfSlowDigestion => 5,
            ValuableTemplate::GemOfProtectFire => 5,
            ValuableTemplate::GemOfDetectMonsters => 5,
            ValuableTemplate::GemOfDispelEvil => 5,
            ValuableTemplate::GemOfDarkness => 5,
            ValuableTemplate::GemOfAcidBalls => 5,
            ValuableTemplate::GemOfDetectInvisible => 5,
            ValuableTemplate::GemOfIdentify => 5,
            ValuableTemplate::GemOfLight => 5,
            ValuableTemplate::GemOfSummoning => 5,
            ValuableTemplate::GemOfRemoveCurse => 5,
            ValuableTemplate::GemOfAnnihilation => 5,
            ValuableTemplate::GemOfRecall => 5,
            ValuableTemplate::FineAgate => 5,
            ValuableTemplate::FineDiamond => 5,
            ValuableTemplate::RoughDiamond => 5,
            ValuableTemplate::RoughSapphire => 5,
            ValuableTemplate::FineSapphire => 5,
            ValuableTemplate::SmallBagOfOpals => 5,
            ValuableTemplate::SmallBagOfSapphires => 5,
            ValuableTemplate::SmallPouchOfDiamonds => 5,
            ValuableTemplate::LargeSackOfPearls => 35,
            ValuableTemplate::LargeSackOfSapphires => 5,
            ValuableTemplate::LargePouchOfDiamonds => 5,
            ValuableTemplate::SmallGoldPendant => 5,
            ValuableTemplate::SmallMithrilPendant => 5,
            ValuableTemplate::LargeMithrilGarterBelt => 5,
            ValuableTemplate::SmallSilverPendant => 5,
        }
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
        "1d1"
    }

    fn item_level(&self) -> u8 {
        match self {
            ValuableTemplate::GemOfTeleportation => 5,
            ValuableTemplate::GemOfResistCold => 14,
            ValuableTemplate::GemOfResistAcid => 14,
            ValuableTemplate::GemOfSeeInvisible => 40,
            ValuableTemplate::GemOfStealth => 35,
            ValuableTemplate::GemOfSlowDigestion => 14,
            ValuableTemplate::GemOfProtectFire => 40,
            ValuableTemplate::GemOfDetectMonsters => 14,
            ValuableTemplate::GemOfDispelEvil => 49,
            ValuableTemplate::GemOfDarkness => 7,
            ValuableTemplate::GemOfAcidBalls => 50,
            ValuableTemplate::GemOfDetectInvisible => 47,
            ValuableTemplate::GemOfIdentify => 40,
            ValuableTemplate::GemOfLight => 2,
            ValuableTemplate::GemOfSummoning => 3,
            ValuableTemplate::GemOfRemoveCurse => 25,
            ValuableTemplate::GemOfAnnihilation => 40,
            ValuableTemplate::GemOfRecall => 22,
            ValuableTemplate::FineAgate => 5,
            ValuableTemplate::FineDiamond => 35,
            ValuableTemplate::RoughDiamond => 15,
            ValuableTemplate::RoughSapphire => 7,
            ValuableTemplate::FineSapphire => 12,
            ValuableTemplate::SmallBagOfOpals => 10,
            ValuableTemplate::SmallBagOfSapphires => 15,
            ValuableTemplate::SmallPouchOfDiamonds => 45,
            ValuableTemplate::LargeSackOfPearls => 25,
            ValuableTemplate::LargeSackOfSapphires => 30,
            ValuableTemplate::LargePouchOfDiamonds => 65,
            ValuableTemplate::SmallGoldPendant => 5,
            ValuableTemplate::SmallMithrilPendant => 10,
            ValuableTemplate::LargeMithrilGarterBelt => 45,
            ValuableTemplate::SmallSilverPendant => 5,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
