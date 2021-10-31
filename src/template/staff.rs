use misc;
use model;

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
    pub fn iter() -> impl Iterator<Item=StaffTemplate> {
        [
            StaffTemplate::StaffOfLight,
            StaffTemplate::StaffOfDoorStairLocation,
            StaffTemplate::StaffOfTrapLocation,
            StaffTemplate::StaffOfTreasureLocation,
            StaffTemplate::StaffOfObjectLocation,
            StaffTemplate::StaffOfTeleportation,
            StaffTemplate::StaffOfEarthquakes,
            StaffTemplate::StaffOfSummoning,
            StaffTemplate::StaffOfDestruction,
            StaffTemplate::StaffOfStarlite,
            StaffTemplate::StaffOfHasteMonsters,
            StaffTemplate::StaffOfSlowMonsters,
            StaffTemplate::StaffOfSleepMonsters,
            StaffTemplate::StaffOfCureLightWounds,
            StaffTemplate::StaffOfDetectInvisible,
            StaffTemplate::StaffOfSpeed,
            StaffTemplate::StaffOfSlowness,
            StaffTemplate::StaffOfMassPolymorph,
            StaffTemplate::StaffOfRemoveCurse,
            StaffTemplate::StaffOfDetectEvil,
            StaffTemplate::StaffOfCuring,
            StaffTemplate::StaffOfDispelEvil,
            StaffTemplate::StaffOfDarkness,
            StaffTemplate::StaffOfIdentify,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::Horn as u8,
            flags: 0,
            flags2: self.flags2(),
            p1: 0,
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: 50,
            number: 1,
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage("1d2"),
            level: 0,
            identified: 0,
        }
    }

    fn name(&self) -> &str {
        match self {
            StaffTemplate::StaffOfLight => "& %W Staff| of Light^ (%P1 charges)",
            StaffTemplate::StaffOfDoorStairLocation => "& %W Staff| of Door/Stair Location^ (%P1 charges)",
            StaffTemplate::StaffOfTrapLocation => "& %W Staff| of Trap Location^ (%P1 charges)",
            StaffTemplate::StaffOfTreasureLocation => "& %W Staff| of Treasure Location^ (%P1 charges)",
            StaffTemplate::StaffOfObjectLocation => "& %W Staff| of Object Location^ (%P1 charges)",
            StaffTemplate::StaffOfTeleportation => "& %W Staff| of Teleportation^ (%P1 charges)",
            StaffTemplate::StaffOfEarthquakes => "& %W Staff| of Earthquakes^ (%P1 charges)",
            StaffTemplate::StaffOfSummoning => "& %W Staff| of Summoning^ (%P1 charges)",
            StaffTemplate::StaffOfDestruction => "& %W Staff| of *Destruction*^ (%P1 charges)",
            StaffTemplate::StaffOfStarlite => "& %W Staff| of Starlite^ (%P1 charges)",
            StaffTemplate::StaffOfHasteMonsters => "& %W Staff| of Haste Monsters^ (%P1 charges)",
            StaffTemplate::StaffOfSlowMonsters => "& %W Staff| of Slow Monsters^ (%P1 charges)",
            StaffTemplate::StaffOfSleepMonsters => "& %W Staff| of Sleep Monsters^ (%P1 charges)",
            StaffTemplate::StaffOfCureLightWounds => "& %W Staff| of Cure Light Wounds^ (%P1 charges)",
            StaffTemplate::StaffOfDetectInvisible => "& %W Staff| of Detect Invisible^ (%P1 charges)",
            StaffTemplate::StaffOfSpeed => "& %W Staff| of Speed^ (%P1 charges)",
            StaffTemplate::StaffOfSlowness => "& %W Staff| of Slowness^ (%P1 charges)",
            StaffTemplate::StaffOfMassPolymorph => "& %W Staff| of Mass Polymorph^ (%P1 charges)",
            StaffTemplate::StaffOfRemoveCurse => "& %W Staff| of Remove Curse^ (%P1 charges)",
            StaffTemplate::StaffOfDetectEvil => "& %W Staff| of Detect Evil^ (%P1 charges)",
            StaffTemplate::StaffOfCuring => "& %W Staff| of Curing^ (%P1 charges)",
            StaffTemplate::StaffOfDispelEvil => "& %W Staff| of Dispel Evil^ (%P1 charges)",
            StaffTemplate::StaffOfDarkness => "& %W Staff| of Darkness^ (%P1 charges)",
            StaffTemplate::StaffOfIdentify => "& %W Staff| of Identify^ (%P1 charges)",
        }
    }

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

    fn subval(&self) -> i64 {
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

    fn level(&self) -> u8 {
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
}

