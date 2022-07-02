use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChimeTemplate {
    ChimeOfLight,
    ChimeOfDetectDoorsStairs,
    ChimeOfDetectTraps,
    ChimeOfTeleportation,
    ChimeOfThunderblast,
    ChimeOfSummonMonster,
    ChimeOfDisarming,
    ChimeOfAggravation,
    ChimeOfSlowMonster,
    ChimeOfSootheMonster,
    ChimeOfCureLightWound,
    ChimeOfChanging,
    ChimeOfRemoveCurse,
    ChimeOfCuring,
    ChimeOfDispelEvil,
    ChimeOfDarkness,
}

impl ChimeTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(ChimeTemplate::ChimeOfLight),
            Box::new(ChimeTemplate::ChimeOfDetectDoorsStairs),
            Box::new(ChimeTemplate::ChimeOfDetectTraps),
            Box::new(ChimeTemplate::ChimeOfTeleportation),
            Box::new(ChimeTemplate::ChimeOfThunderblast),
            Box::new(ChimeTemplate::ChimeOfSummonMonster),
            Box::new(ChimeTemplate::ChimeOfDisarming),
            Box::new(ChimeTemplate::ChimeOfAggravation),
            Box::new(ChimeTemplate::ChimeOfSlowMonster),
            Box::new(ChimeTemplate::ChimeOfSootheMonster),
            Box::new(ChimeTemplate::ChimeOfCureLightWound),
            Box::new(ChimeTemplate::ChimeOfChanging),
            Box::new(ChimeTemplate::ChimeOfRemoveCurse),
            Box::new(ChimeTemplate::ChimeOfCuring),
            Box::new(ChimeTemplate::ChimeOfDispelEvil),
            Box::new(ChimeTemplate::ChimeOfDarkness),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        ChimeTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(ChimeTemplate::ChimeOfLight),
            2 => Box::new(ChimeTemplate::ChimeOfDetectDoorsStairs),
            3 => Box::new(ChimeTemplate::ChimeOfDetectTraps),
            4 => Box::new(ChimeTemplate::ChimeOfTeleportation),
            5 => Box::new(ChimeTemplate::ChimeOfThunderblast),
            6 => Box::new(ChimeTemplate::ChimeOfSummonMonster),
            7 => Box::new(ChimeTemplate::ChimeOfDisarming),
            8 => Box::new(ChimeTemplate::ChimeOfAggravation),
            9 => Box::new(ChimeTemplate::ChimeOfSlowMonster),
            10 => Box::new(ChimeTemplate::ChimeOfSootheMonster),
            11 => Box::new(ChimeTemplate::ChimeOfCureLightWound),
            12 => Box::new(ChimeTemplate::ChimeOfChanging),
            13 => Box::new(ChimeTemplate::ChimeOfRemoveCurse),
            14 => Box::new(ChimeTemplate::ChimeOfCuring),
            15 => Box::new(ChimeTemplate::ChimeOfDispelEvil),
            16 => Box::new(ChimeTemplate::ChimeOfDarkness),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for ChimeTemplate {
    fn item_type(&self) -> model::ItemType { model::ItemType::Chime }
    fn flags1(&self) -> u64 { 0 }

    fn flags2(&self) -> u64 {
        match self {
            ChimeTemplate::ChimeOfLight => 0x00000001,
            ChimeTemplate::ChimeOfDetectDoorsStairs => 0x00000002,
            ChimeTemplate::ChimeOfDetectTraps => 0x00000004,
            ChimeTemplate::ChimeOfTeleportation => 0x00000008,
            ChimeTemplate::ChimeOfThunderblast => 0x00000010,
            ChimeTemplate::ChimeOfSummonMonster => 0x00000020,
            ChimeTemplate::ChimeOfDisarming => 0x00000040,
            ChimeTemplate::ChimeOfAggravation => 0x00000080,
            ChimeTemplate::ChimeOfSlowMonster => 0x00000100,
            ChimeTemplate::ChimeOfSootheMonster => 0x00000200,
            ChimeTemplate::ChimeOfCureLightWound => 0x00000400,
            ChimeTemplate::ChimeOfChanging => 0x00000800,
            ChimeTemplate::ChimeOfRemoveCurse => 0x00001000,
            ChimeTemplate::ChimeOfCuring => 0x00002000,
            ChimeTemplate::ChimeOfDispelEvil => 0x00004000,
            ChimeTemplate::ChimeOfDarkness => 0x00008000,
        }
    }

    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            ChimeTemplate::ChimeOfLight => 275,
            ChimeTemplate::ChimeOfDetectDoorsStairs => 375,
            ChimeTemplate::ChimeOfDetectTraps => 375,
            ChimeTemplate::ChimeOfTeleportation => 450,
            ChimeTemplate::ChimeOfThunderblast => 400,
            ChimeTemplate::ChimeOfSummonMonster => 0,
            ChimeTemplate::ChimeOfDisarming => 400,
            ChimeTemplate::ChimeOfAggravation => 0,
            ChimeTemplate::ChimeOfSlowMonster => 850,
            ChimeTemplate::ChimeOfSootheMonster => 800,
            ChimeTemplate::ChimeOfCureLightWound => 400,
            ChimeTemplate::ChimeOfChanging => 800,
            ChimeTemplate::ChimeOfRemoveCurse => 675,
            ChimeTemplate::ChimeOfCuring => 1100,
            ChimeTemplate::ChimeOfDispelEvil => 1300,
            ChimeTemplate::ChimeOfDarkness => 0,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            ChimeTemplate::ChimeOfLight => 1,
            ChimeTemplate::ChimeOfDetectDoorsStairs => 2,
            ChimeTemplate::ChimeOfDetectTraps => 3,
            ChimeTemplate::ChimeOfTeleportation => 4,
            ChimeTemplate::ChimeOfThunderblast => 5,
            ChimeTemplate::ChimeOfSummonMonster => 6,
            ChimeTemplate::ChimeOfDisarming => 7,
            ChimeTemplate::ChimeOfAggravation => 8,
            ChimeTemplate::ChimeOfSlowMonster => 9,
            ChimeTemplate::ChimeOfSootheMonster => 10,
            ChimeTemplate::ChimeOfCureLightWound => 11,
            ChimeTemplate::ChimeOfChanging => 12,
            ChimeTemplate::ChimeOfRemoveCurse => 13,
            ChimeTemplate::ChimeOfCuring => 14,
            ChimeTemplate::ChimeOfDispelEvil => 15,
            ChimeTemplate::ChimeOfDarkness => 16,
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
            ChimeTemplate::ChimeOfLight => 10,
            ChimeTemplate::ChimeOfDetectDoorsStairs => 15,
            ChimeTemplate::ChimeOfDetectTraps => 15,
            ChimeTemplate::ChimeOfTeleportation => 23,
            ChimeTemplate::ChimeOfThunderblast => 42,
            ChimeTemplate::ChimeOfSummonMonster => 10,
            ChimeTemplate::ChimeOfDisarming => 30,
            ChimeTemplate::ChimeOfAggravation => 15,
            ChimeTemplate::ChimeOfSlowMonster => 15,
            ChimeTemplate::ChimeOfSootheMonster => 15,
            ChimeTemplate::ChimeOfCureLightWound => 10,
            ChimeTemplate::ChimeOfChanging => 46,
            ChimeTemplate::ChimeOfRemoveCurse => 47,
            ChimeTemplate::ChimeOfCuring => 27,
            ChimeTemplate::ChimeOfDispelEvil => 49,
            ChimeTemplate::ChimeOfDarkness => 20,
        }
    }

    fn is_identified(&self) -> bool { false }
}
