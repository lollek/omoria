use crate::model;
use super::super::item_template::ItemTemplate;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum HornTemplate {
    HornOfBubbles,
    HornOfCalling,
    HornOfSoftSounds,
    HornOfBlasting,
    HornOfCold,
    HornOfHeat,
    HornOfGas,
    HornOfRecall,
    HornOfChaos,
    HornOfGlue,
    HornOfValhalla,
    HornOfTritons,
    HornOfFog,
}

impl HornTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(HornTemplate::HornOfBubbles),
            Box::new(HornTemplate::HornOfCalling),
            Box::new(HornTemplate::HornOfSoftSounds),
            Box::new(HornTemplate::HornOfBlasting),
            Box::new(HornTemplate::HornOfCold),
            Box::new(HornTemplate::HornOfHeat),
            Box::new(HornTemplate::HornOfGas),
            Box::new(HornTemplate::HornOfRecall),
            Box::new(HornTemplate::HornOfChaos),
            Box::new(HornTemplate::HornOfGlue),
            Box::new(HornTemplate::HornOfValhalla),
            Box::new(HornTemplate::HornOfTritons),
            Box::new(HornTemplate::HornOfFog),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn ItemTemplate>> {
        HornTemplate::vec().into_iter()
    }
}

impl ItemTemplate for HornTemplate {
    fn name(&self) -> &str {
        match self {
            HornTemplate::HornOfBubbles => "& Horn| of Bubbles^ (%P1 charges)",
            HornTemplate::HornOfCalling => "& Horn| of Calling^ (%P1 charges)",
            HornTemplate::HornOfSoftSounds => "& Horn| of Soft Sounds^ (%P1 charges)",
            HornTemplate::HornOfBlasting => "& Horn| of *Blasting*^ (%P1 charges)",
            HornTemplate::HornOfCold => "& Horn| of Cold^ (%P1 charges)",
            HornTemplate::HornOfHeat => "& Horn| of Heat^ (%P1 charges)",
            HornTemplate::HornOfGas => "& Horn| of Gas^ (%P1 charges)",
            HornTemplate::HornOfRecall => "& Horn| of Recall^ (%P1 charges)",
            HornTemplate::HornOfChaos => "& Horn| of *Chaos*^ (%P1 charges)",
            HornTemplate::HornOfGlue => "& Horn| of Glue^ (%P1 charges)",
            HornTemplate::HornOfValhalla => "& Horn| of Valhalla^ (%P1 charges)",
            HornTemplate::HornOfTritons => "& Horn| of Tritons^ (%P1 charges)",
            HornTemplate::HornOfFog => "& Horn| of Fog^ (%P1 charges)",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Horn }
    fn flags1(&self) -> u64 { 0 }

    fn flags2(&self) -> u64 {
        match self {
            HornTemplate::HornOfBubbles => 0x00010000,
            HornTemplate::HornOfCalling => 0x00020000,
            HornTemplate::HornOfSoftSounds => 0x00040000,
            HornTemplate::HornOfBlasting => 0x00080000,
            HornTemplate::HornOfCold => 0x00100000,
            HornTemplate::HornOfHeat => 0x00200000,
            HornTemplate::HornOfGas => 0x00400000,
            HornTemplate::HornOfRecall => 0x00800000,
            HornTemplate::HornOfChaos => 0x01000000,
            HornTemplate::HornOfGlue => 0x02000000,
            HornTemplate::HornOfValhalla => 0x04000000,
            HornTemplate::HornOfTritons => 0x08000000,
            HornTemplate::HornOfFog => 0x10000000,
        }
    }

    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            HornTemplate::HornOfBubbles => 0,
            HornTemplate::HornOfCalling => 0,
            HornTemplate::HornOfSoftSounds => 600,
            HornTemplate::HornOfBlasting => 2600,
            HornTemplate::HornOfCold => 1000,
            HornTemplate::HornOfHeat => 1000,
            HornTemplate::HornOfGas => 900,
            HornTemplate::HornOfRecall => 1200,
            HornTemplate::HornOfChaos => 800,
            HornTemplate::HornOfGlue => 0,
            HornTemplate::HornOfValhalla => 2700,
            HornTemplate::HornOfTritons => 200,
            HornTemplate::HornOfFog => 500,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            HornTemplate::HornOfBubbles => 1,
            HornTemplate::HornOfCalling => 2,
            HornTemplate::HornOfSoftSounds => 3,
            HornTemplate::HornOfBlasting => 4,
            HornTemplate::HornOfCold => 5,
            HornTemplate::HornOfHeat => 6,
            HornTemplate::HornOfGas => 7,
            HornTemplate::HornOfRecall => 8,
            HornTemplate::HornOfChaos => 9,
            HornTemplate::HornOfGlue => 10,
            HornTemplate::HornOfValhalla => 11,
            HornTemplate::HornOfTritons => 12,
            HornTemplate::HornOfFog => 13,
        }
    }

    fn weight(&self) -> u16 { 2 }
    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }

    fn item_level(&self) -> u8 {
        match self {
            HornTemplate::HornOfBubbles => 15,
            HornTemplate::HornOfCalling => 10,
            HornTemplate::HornOfSoftSounds => 8,
            HornTemplate::HornOfBlasting => 49,
            HornTemplate::HornOfCold => 40,
            HornTemplate::HornOfHeat => 40,
            HornTemplate::HornOfGas => 35,
            HornTemplate::HornOfRecall => 30,
            HornTemplate::HornOfChaos => 43,
            HornTemplate::HornOfGlue => 20,
            HornTemplate::HornOfValhalla => 50,
            HornTemplate::HornOfTritons => 15,
            HornTemplate::HornOfFog => 25,
        }
    }

    fn is_identified(&self) -> bool { false }
}
