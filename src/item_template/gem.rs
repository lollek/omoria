use std::borrow::Cow;

use model;
use item_template;
use logic::item_name;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum GemTemplate {
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
}

impl GemTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(GemTemplate::GemOfDetectMonsters),
            Box::new(GemTemplate::GemOfDispelEvil),
            Box::new(GemTemplate::GemOfDarkness),
            Box::new(GemTemplate::GemOfAcidBalls),
            Box::new(GemTemplate::GemOfDetectInvisible),
            Box::new(GemTemplate::GemOfIdentify),
            Box::new(GemTemplate::GemOfLight),
            Box::new(GemTemplate::GemOfSummoning),
            Box::new(GemTemplate::GemOfRemoveCurse),
            Box::new(GemTemplate::GemOfAnnihilation),
            Box::new(GemTemplate::GemOfRecall),
            Box::new(GemTemplate::FineAgate),
            Box::new(GemTemplate::FineDiamond),
            Box::new(GemTemplate::RoughDiamond),
            Box::new(GemTemplate::RoughSapphire),
            Box::new(GemTemplate::FineSapphire),
            Box::new(GemTemplate::SmallBagOfOpals),
            Box::new(GemTemplate::SmallBagOfSapphires),
            Box::new(GemTemplate::SmallPouchOfDiamonds),
            Box::new(GemTemplate::LargeSackOfPearls),
            Box::new(GemTemplate::LargeSackOfSapphires),
            Box::new(GemTemplate::LargePouchOfDiamonds),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        GemTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(GemTemplate::GemOfDetectMonsters),
            2 => Box::new(GemTemplate::GemOfDispelEvil),
            3 => Box::new(GemTemplate::GemOfDarkness),
            4 => Box::new(GemTemplate::GemOfAcidBalls),
            5 => Box::new(GemTemplate::GemOfDetectInvisible),
            6 => Box::new(GemTemplate::GemOfIdentify),
            7 => Box::new(GemTemplate::GemOfLight),
            8 => Box::new(GemTemplate::GemOfSummoning),
            9 => Box::new(GemTemplate::GemOfRemoveCurse),
            10 => Box::new(GemTemplate::GemOfAnnihilation),
            11 => Box::new(GemTemplate::GemOfRecall),
            257 => Box::new(GemTemplate::FineAgate),
            258 => Box::new(GemTemplate::FineDiamond),
            259 => Box::new(GemTemplate::RoughDiamond),
            260 => Box::new(GemTemplate::RoughSapphire),
            261 => Box::new(GemTemplate::FineSapphire),
            262 => Box::new(GemTemplate::SmallBagOfOpals),
            263 => Box::new(GemTemplate::SmallBagOfSapphires),
            264 => Box::new(GemTemplate::SmallPouchOfDiamonds),
            265 => Box::new(GemTemplate::LargeSackOfPearls),
            266 => Box::new(GemTemplate::LargeSackOfSapphires),
            267 => Box::new(GemTemplate::LargePouchOfDiamonds),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for GemTemplate {
    fn name(&self, item: &model::Item) -> String {
        let plural_s = || if item.number == 1 { "" } else { "s" };

        let mut parts = Vec::new();
        parts.push(item_name::number_of(item));
        parts.push(
            match self {
                GemTemplate::GemOfDetectMonsters |
                GemTemplate::GemOfDispelEvil |
                GemTemplate::GemOfDarkness |
                GemTemplate::GemOfAcidBalls |
                GemTemplate::GemOfDetectInvisible |
                GemTemplate::GemOfIdentify |
                GemTemplate::GemOfLight |
                GemTemplate::GemOfSummoning |
                GemTemplate::GemOfRemoveCurse |
                GemTemplate::GemOfAnnihilation |
                GemTemplate::GemOfRecall =>
                Cow::from("Finely Cut Gem"),
                GemTemplate::FineAgate =>
                    Cow::from(format!("Finely cut Agate{}", plural_s())),
                GemTemplate::FineDiamond =>
                    Cow::from(format!("Finely cut Diamond{}", plural_s())),
                GemTemplate::RoughDiamond =>
                    Cow::from(format!("Rough cut Diamond{}", plural_s())),
                GemTemplate::RoughSapphire =>
                    Cow::from(format!("Rough cut Sapphire{}", plural_s())),
                GemTemplate::FineSapphire =>
                    Cow::from(format!("Finely cut Sapphire{}", plural_s())),
                GemTemplate::SmallBagOfOpals =>
                    Cow::from(format!("Small bag{} of Opals", plural_s())),
                GemTemplate::SmallBagOfSapphires =>
                    Cow::from(format!("Small bag{} of Sapphires", plural_s())),
                GemTemplate::SmallPouchOfDiamonds =>
                    Cow::from(format!("Small pouch{} of Diamonds", plural_s())),
                GemTemplate::LargeSackOfPearls =>
                    Cow::from(format!("Large sack{} of Pearls", plural_s())),
                GemTemplate::LargeSackOfSapphires =>
                    Cow::from(format!("Large sack{} of Sapphires", plural_s())),
                GemTemplate::LargePouchOfDiamonds =>
                    Cow::from(format!("Large pouch{} of Diamonds", plural_s())),
            });
        if self.is_identified() {
            parts.push(Cow::from(
                    match self {
                        GemTemplate::GemOfDetectMonsters => " of Detect Monsters",
                        GemTemplate::GemOfDispelEvil => " of Dispel Evil",
                        GemTemplate::GemOfDarkness => " of Darkness",
                        GemTemplate::GemOfAcidBalls => " of Acid Balls",
                        GemTemplate::GemOfDetectInvisible => " of Detect Invisible",
                        GemTemplate::GemOfIdentify => " of Identify",
                        GemTemplate::GemOfLight => " of Light",
                        GemTemplate::GemOfSummoning => " of Summoning",
                        GemTemplate::GemOfRemoveCurse => " of Remove Curse",
                        GemTemplate::GemOfAnnihilation => " of Annihilation",
                        GemTemplate::GemOfRecall => " of Recall",
                        _ => "",
                    }));
        }
        if item.is_identified() {
            parts.push(
                    match self {
                        GemTemplate::GemOfDetectMonsters |
                        GemTemplate::GemOfDispelEvil |
                        GemTemplate::GemOfDarkness |
                        GemTemplate::GemOfAcidBalls |
                        GemTemplate::GemOfDetectInvisible |
                        GemTemplate::GemOfIdentify |
                        GemTemplate::GemOfLight |
                        GemTemplate::GemOfSummoning |
                        GemTemplate::GemOfRemoveCurse |
                        GemTemplate::GemOfAnnihilation |
                        GemTemplate::GemOfRecall =>
                            item_name::charges(item),
                        _ => Cow::from(""),
                    });
        }
        parts.join("")
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Gem }

    fn flags1(&self) -> u64 { 0 }

    fn flags2(&self) -> u64 {
        match self {
            GemTemplate::GemOfDetectMonsters => 0x00040000,
            GemTemplate::GemOfDispelEvil => 0x00080000,
            GemTemplate::GemOfDarkness => 0x00100000,
            GemTemplate::GemOfAcidBalls => 0x00200000,
            GemTemplate::GemOfDetectInvisible => 0x00400000,
            GemTemplate::GemOfIdentify => 0x00800000,
            GemTemplate::GemOfLight => 0x01000000,
            GemTemplate::GemOfSummoning => 0x02000000,
            GemTemplate::GemOfRemoveCurse => 0x04000000,
            GemTemplate::GemOfAnnihilation => 0x08000000,
            GemTemplate::GemOfRecall => 0x10000000,
            GemTemplate::FineAgate => 0,
            GemTemplate::FineDiamond => 0,
            GemTemplate::RoughDiamond => 0,
            GemTemplate::RoughSapphire => 0,
            GemTemplate::FineSapphire => 0,
            GemTemplate::SmallBagOfOpals => 0,
            GemTemplate::SmallBagOfSapphires => 0,
            GemTemplate::SmallPouchOfDiamonds => 0,
            GemTemplate::LargeSackOfPearls => 0,
            GemTemplate::LargeSackOfSapphires => 0,
            GemTemplate::LargePouchOfDiamonds => 0,
        }
    }

    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            GemTemplate::GemOfDetectMonsters => 350,
            GemTemplate::GemOfDispelEvil => 1200,
            GemTemplate::GemOfDarkness => 0,
            GemTemplate::GemOfAcidBalls => 1800,
            GemTemplate::GemOfDetectInvisible => 225,
            GemTemplate::GemOfIdentify => 400,
            GemTemplate::GemOfLight => 300,
            GemTemplate::GemOfSummoning => 0,
            GemTemplate::GemOfRemoveCurse => 700,
            GemTemplate::GemOfAnnihilation => 1000,
            GemTemplate::GemOfRecall => 1200,
            GemTemplate::FineAgate => 50,
            GemTemplate::FineDiamond => 500,
            GemTemplate::RoughDiamond => 100,
            GemTemplate::RoughSapphire => 40,
            GemTemplate::FineSapphire => 250,
            GemTemplate::SmallBagOfOpals => 250,
            GemTemplate::SmallBagOfSapphires => 450,
            GemTemplate::SmallPouchOfDiamonds => 1000,
            GemTemplate::LargeSackOfPearls => 650,
            GemTemplate::LargeSackOfSapphires => 600,
            GemTemplate::LargePouchOfDiamonds => 2000,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            GemTemplate::GemOfDetectMonsters => 1,
            GemTemplate::GemOfDispelEvil => 2,
            GemTemplate::GemOfDarkness => 3,
            GemTemplate::GemOfAcidBalls => 4,
            GemTemplate::GemOfDetectInvisible => 5,
            GemTemplate::GemOfIdentify => 6,
            GemTemplate::GemOfLight => 7,
            GemTemplate::GemOfSummoning => 8,
            GemTemplate::GemOfRemoveCurse => 9,
            GemTemplate::GemOfAnnihilation => 10,
            GemTemplate::GemOfRecall => 11,
            GemTemplate::FineAgate => 257,
            GemTemplate::FineDiamond => 258,
            GemTemplate::RoughDiamond => 259,
            GemTemplate::RoughSapphire => 260,
            GemTemplate::FineSapphire => 261,
            GemTemplate::SmallBagOfOpals => 262,
            GemTemplate::SmallBagOfSapphires => 263,
            GemTemplate::SmallPouchOfDiamonds => 264,
            GemTemplate::LargeSackOfPearls => 265,
            GemTemplate::LargeSackOfSapphires => 266,
            GemTemplate::LargePouchOfDiamonds => 267,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            GemTemplate::GemOfDetectMonsters => 5,
            GemTemplate::GemOfDispelEvil => 5,
            GemTemplate::GemOfDarkness => 5,
            GemTemplate::GemOfAcidBalls => 5,
            GemTemplate::GemOfDetectInvisible => 5,
            GemTemplate::GemOfIdentify => 5,
            GemTemplate::GemOfLight => 5,
            GemTemplate::GemOfSummoning => 5,
            GemTemplate::GemOfRemoveCurse => 5,
            GemTemplate::GemOfAnnihilation => 5,
            GemTemplate::GemOfRecall => 5,
            GemTemplate::FineAgate => 5,
            GemTemplate::FineDiamond => 5,
            GemTemplate::RoughDiamond => 5,
            GemTemplate::RoughSapphire => 5,
            GemTemplate::FineSapphire => 5,
            GemTemplate::SmallBagOfOpals => 5,
            GemTemplate::SmallBagOfSapphires => 5,
            GemTemplate::SmallPouchOfDiamonds => 5,
            GemTemplate::LargeSackOfPearls => 35,
            GemTemplate::LargeSackOfSapphires => 5,
            GemTemplate::LargePouchOfDiamonds => 5,
        }
    }

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }

    fn item_level(&self) -> u8 {
        match self {
            GemTemplate::GemOfDetectMonsters => 14,
            GemTemplate::GemOfDispelEvil => 49,
            GemTemplate::GemOfDarkness => 7,
            GemTemplate::GemOfAcidBalls => 50,
            GemTemplate::GemOfDetectInvisible => 47,
            GemTemplate::GemOfIdentify => 40,
            GemTemplate::GemOfLight => 2,
            GemTemplate::GemOfSummoning => 3,
            GemTemplate::GemOfRemoveCurse => 25,
            GemTemplate::GemOfAnnihilation => 40,
            GemTemplate::GemOfRecall => 22,
            GemTemplate::FineAgate => 5,
            GemTemplate::FineDiamond => 35,
            GemTemplate::RoughDiamond => 15,
            GemTemplate::RoughSapphire => 7,
            GemTemplate::FineSapphire => 12,
            GemTemplate::SmallBagOfOpals => 10,
            GemTemplate::SmallBagOfSapphires => 15,
            GemTemplate::SmallPouchOfDiamonds => 45,
            GemTemplate::LargeSackOfPearls => 25,
            GemTemplate::LargeSackOfSapphires => 30,
            GemTemplate::LargePouchOfDiamonds => 65,
        }
    }

    fn is_identified(&self) -> bool { false }
}

