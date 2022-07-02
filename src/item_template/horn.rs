use model;
use item_template;

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
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
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

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        HornTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(HornTemplate::HornOfBubbles),
            2 => Box::new(HornTemplate::HornOfCalling),
            3 => Box::new(HornTemplate::HornOfSoftSounds),
            4 => Box::new(HornTemplate::HornOfBlasting),
            5 => Box::new(HornTemplate::HornOfCold),
            6 => Box::new(HornTemplate::HornOfHeat),
            7 => Box::new(HornTemplate::HornOfGas),
            8 => Box::new(HornTemplate::HornOfRecall),
            9 => Box::new(HornTemplate::HornOfChaos),
            10 => Box::new(HornTemplate::HornOfGlue),
            11 => Box::new(HornTemplate::HornOfValhalla),
            12 => Box::new(HornTemplate::HornOfTritons),
            13 => Box::new(HornTemplate::HornOfFog),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for HornTemplate {
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
