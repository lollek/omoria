use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum StaffTemplate {
    StaffOfLight,
    StaffOfDoorStairLocation,
    StaffOfTrapLocation,
    StaffOfTreasureLocation,
    StaffOfObjectLocation,
    StaffOfTeleportation,
    StaffOfEarthquakes,
    StaffOfSummoning,
    StaffOfDestruction,
    StaffOfStarlite,
    StaffOfHasteMonsters,
    StaffOfSlowMonsters,
    StaffOfSleepMonsters,
    StaffOfCureLightWounds,
    StaffOfDetectInvisible,
    StaffOfSpeed,
    StaffOfSlowness,
    StaffOfMassPolymorph,
    StaffOfRemoveCurse,
    StaffOfDetectEvil,
    StaffOfCuring,
    StaffOfDispelEvil,
    StaffOfDarkness,
    StaffOfIdentify,
}


impl StaffTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(StaffTemplate::StaffOfLight),
            Box::new(StaffTemplate::StaffOfDoorStairLocation),
            Box::new(StaffTemplate::StaffOfTrapLocation),
            Box::new(StaffTemplate::StaffOfTreasureLocation),
            Box::new(StaffTemplate::StaffOfObjectLocation),
            Box::new(StaffTemplate::StaffOfTeleportation),
            Box::new(StaffTemplate::StaffOfEarthquakes),
            Box::new(StaffTemplate::StaffOfSummoning),
            Box::new(StaffTemplate::StaffOfDestruction),
            Box::new(StaffTemplate::StaffOfStarlite),
            Box::new(StaffTemplate::StaffOfHasteMonsters),
            Box::new(StaffTemplate::StaffOfSlowMonsters),
            Box::new(StaffTemplate::StaffOfSleepMonsters),
            Box::new(StaffTemplate::StaffOfCureLightWounds),
            Box::new(StaffTemplate::StaffOfDetectInvisible),
            Box::new(StaffTemplate::StaffOfSpeed),
            Box::new(StaffTemplate::StaffOfSlowness),
            Box::new(StaffTemplate::StaffOfMassPolymorph),
            Box::new(StaffTemplate::StaffOfRemoveCurse),
            Box::new(StaffTemplate::StaffOfDetectEvil),
            Box::new(StaffTemplate::StaffOfCuring),
            Box::new(StaffTemplate::StaffOfDispelEvil),
            Box::new(StaffTemplate::StaffOfDarkness),
            Box::new(StaffTemplate::StaffOfIdentify),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        StaffTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(StaffTemplate::StaffOfLight),
            2 => Box::new(StaffTemplate::StaffOfDoorStairLocation),
            3 => Box::new(StaffTemplate::StaffOfTrapLocation),
            4 => Box::new(StaffTemplate::StaffOfTreasureLocation),
            5 => Box::new(StaffTemplate::StaffOfObjectLocation),
            6 => Box::new(StaffTemplate::StaffOfTeleportation),
            7 => Box::new(StaffTemplate::StaffOfEarthquakes),
            8 => Box::new(StaffTemplate::StaffOfSummoning),
            10 => Box::new(StaffTemplate::StaffOfDestruction),
            11 => Box::new(StaffTemplate::StaffOfStarlite),
            12 => Box::new(StaffTemplate::StaffOfHasteMonsters),
            13 => Box::new(StaffTemplate::StaffOfSlowMonsters),
            14 => Box::new(StaffTemplate::StaffOfSleepMonsters),
            15 => Box::new(StaffTemplate::StaffOfCureLightWounds),
            16 => Box::new(StaffTemplate::StaffOfDetectInvisible),
            17 => Box::new(StaffTemplate::StaffOfSpeed),
            18 => Box::new(StaffTemplate::StaffOfSlowness),
            19 => Box::new(StaffTemplate::StaffOfMassPolymorph),
            20 => Box::new(StaffTemplate::StaffOfRemoveCurse),
            21 => Box::new(StaffTemplate::StaffOfDetectEvil),
            22 => Box::new(StaffTemplate::StaffOfCuring),
            23 => Box::new(StaffTemplate::StaffOfDispelEvil),
            25 => Box::new(StaffTemplate::StaffOfDarkness),
            26 => Box::new(StaffTemplate::StaffOfIdentify),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for StaffTemplate {
    fn item_type(&self) -> model::ItemType { model::ItemType::Staff }
    fn flags1(&self) -> u64 { 0 }

    fn flags2(&self) -> u64 {
        match self {
            StaffTemplate::StaffOfLight => 0x00000001,
            StaffTemplate::StaffOfDoorStairLocation => 0x00000002,
            StaffTemplate::StaffOfTrapLocation => 0x00000004,
            StaffTemplate::StaffOfTreasureLocation => 0x00000008,
            StaffTemplate::StaffOfObjectLocation => 0x00000010,
            StaffTemplate::StaffOfTeleportation => 0x00000020,
            StaffTemplate::StaffOfEarthquakes => 0x00000040,
            StaffTemplate::StaffOfSummoning => 0x00000080,
            StaffTemplate::StaffOfDestruction => 0x00000200,
            StaffTemplate::StaffOfStarlite => 0x00000400,
            StaffTemplate::StaffOfHasteMonsters => 0x00000800,
            StaffTemplate::StaffOfSlowMonsters => 0x00001000,
            StaffTemplate::StaffOfSleepMonsters => 0x00002000,
            StaffTemplate::StaffOfCureLightWounds => 0x00004000,
            StaffTemplate::StaffOfDetectInvisible => 0x00008000,
            StaffTemplate::StaffOfSpeed => 0x00010000,
            StaffTemplate::StaffOfSlowness => 0x00020000,
            StaffTemplate::StaffOfMassPolymorph => 0x00040000,
            StaffTemplate::StaffOfRemoveCurse => 0x00080000,
            StaffTemplate::StaffOfDetectEvil => 0x00100000,
            StaffTemplate::StaffOfCuring => 0x00200000,
            StaffTemplate::StaffOfDispelEvil => 0x00400000,
            StaffTemplate::StaffOfDarkness => 0x01000000,
            StaffTemplate::StaffOfIdentify => 0x02000000,
        }
    }

    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            StaffTemplate::StaffOfLight => 250,
            StaffTemplate::StaffOfDoorStairLocation => 350,
            StaffTemplate::StaffOfTrapLocation => 350,
            StaffTemplate::StaffOfTreasureLocation => 200,
            StaffTemplate::StaffOfObjectLocation => 200,
            StaffTemplate::StaffOfTeleportation => 400,
            StaffTemplate::StaffOfEarthquakes => 350,
            StaffTemplate::StaffOfSummoning => 0,
            StaffTemplate::StaffOfDestruction => 2500,
            StaffTemplate::StaffOfStarlite => 800,
            StaffTemplate::StaffOfHasteMonsters => 0,
            StaffTemplate::StaffOfSlowMonsters => 800,
            StaffTemplate::StaffOfSleepMonsters => 700,
            StaffTemplate::StaffOfCureLightWounds => 350,
            StaffTemplate::StaffOfDetectInvisible => 200,
            StaffTemplate::StaffOfSpeed => 800,
            StaffTemplate::StaffOfSlowness => 0,
            StaffTemplate::StaffOfMassPolymorph => 1750,
            StaffTemplate::StaffOfRemoveCurse => 500,
            StaffTemplate::StaffOfDetectEvil => 350,
            StaffTemplate::StaffOfCuring => 1000,
            StaffTemplate::StaffOfDispelEvil => 1200,
            StaffTemplate::StaffOfDarkness => 0,
            StaffTemplate::StaffOfIdentify => 1500,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            StaffTemplate::StaffOfLight => 1,
            StaffTemplate::StaffOfDoorStairLocation => 2,
            StaffTemplate::StaffOfTrapLocation => 3,
            StaffTemplate::StaffOfTreasureLocation => 4,
            StaffTemplate::StaffOfObjectLocation => 5,
            StaffTemplate::StaffOfTeleportation => 6,
            StaffTemplate::StaffOfEarthquakes => 7,
            StaffTemplate::StaffOfSummoning => 8,
            StaffTemplate::StaffOfDestruction => 10,
            StaffTemplate::StaffOfStarlite => 11,
            StaffTemplate::StaffOfHasteMonsters => 12,
            StaffTemplate::StaffOfSlowMonsters => 13,
            StaffTemplate::StaffOfSleepMonsters => 14,
            StaffTemplate::StaffOfCureLightWounds => 15,
            StaffTemplate::StaffOfDetectInvisible => 16,
            StaffTemplate::StaffOfSpeed => 17,
            StaffTemplate::StaffOfSlowness => 18,
            StaffTemplate::StaffOfMassPolymorph => 19,
            StaffTemplate::StaffOfRemoveCurse => 20,
            StaffTemplate::StaffOfDetectEvil => 21,
            StaffTemplate::StaffOfCuring => 22,
            StaffTemplate::StaffOfDispelEvil => 23,
            StaffTemplate::StaffOfDarkness => 25,
            StaffTemplate::StaffOfIdentify => 26,
        }
    }

    fn weight(&self) -> u16 { 50 }

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }

    fn damage(&self) -> &str { "1d2" }

    fn item_level(&self) -> u8 {
        match self {
            StaffTemplate::StaffOfLight => 5,
            StaffTemplate::StaffOfDoorStairLocation => 10,
            StaffTemplate::StaffOfTrapLocation => 10,
            StaffTemplate::StaffOfTreasureLocation => 5,
            StaffTemplate::StaffOfObjectLocation => 5,
            StaffTemplate::StaffOfTeleportation => 20,
            StaffTemplate::StaffOfEarthquakes => 40,
            StaffTemplate::StaffOfSummoning => 10,
            StaffTemplate::StaffOfDestruction => 50,
            StaffTemplate::StaffOfStarlite => 20,
            StaffTemplate::StaffOfHasteMonsters => 10,
            StaffTemplate::StaffOfSlowMonsters => 10,
            StaffTemplate::StaffOfSleepMonsters => 10,
            StaffTemplate::StaffOfCureLightWounds => 5,
            StaffTemplate::StaffOfDetectInvisible => 5,
            StaffTemplate::StaffOfSpeed => 40,
            StaffTemplate::StaffOfSlowness => 40,
            StaffTemplate::StaffOfMassPolymorph => 46,
            StaffTemplate::StaffOfRemoveCurse => 47,
            StaffTemplate::StaffOfDetectEvil => 20,
            StaffTemplate::StaffOfCuring => 25,
            StaffTemplate::StaffOfDispelEvil => 49,
            StaffTemplate::StaffOfDarkness => 5,
            StaffTemplate::StaffOfIdentify => 20,
        }
    }

    fn is_identified(&self) -> bool { false }
}

