use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum WandTemplate {
    WandOfProbing,
    WandOfLight,
    WandOfLightningBolts,
    WandOfFrostBolts,
    WandOfFireBolts,
    WandOfStoneToMud,
    WandOfPolymorph,
    WandOfHealMonster,
    WandOfHasteMonster,
    WandOfSlowMonster,
    WandOfConfuseMonster,
    WandOfSleepMonster,
    WandOfDrainLife,
    WandOfTrapDoorDestruction,
    WandOfMagicMissile,
    WandOfWallBuilding,
    WandOfCloneMonster,
    WandOfTeleportAway,
    WandOfDisarming,
    WandOfLightningBalls,
    WandOfColdBalls,
    WandOfFireBalls,
    WandOfStinkingCloud,
    WandOfAcidBalls,
    WandOfWonder,
}

impl WandTemplate {
    pub fn vec() -> Vec<Box<dyn template::Template>> {
        vec![
            Box::new(WandTemplate::WandOfProbing),
            Box::new(WandTemplate::WandOfLight),
            Box::new(WandTemplate::WandOfLightningBolts),
            Box::new(WandTemplate::WandOfFrostBolts),
            Box::new(WandTemplate::WandOfFireBolts),
            Box::new(WandTemplate::WandOfStoneToMud),
            Box::new(WandTemplate::WandOfPolymorph),
            Box::new(WandTemplate::WandOfHealMonster),
            Box::new(WandTemplate::WandOfHasteMonster),
            Box::new(WandTemplate::WandOfSlowMonster),
            Box::new(WandTemplate::WandOfConfuseMonster),
            Box::new(WandTemplate::WandOfSleepMonster),
            Box::new(WandTemplate::WandOfDrainLife),
            Box::new(WandTemplate::WandOfTrapDoorDestruction),
            Box::new(WandTemplate::WandOfMagicMissile),
            Box::new(WandTemplate::WandOfWallBuilding),
            Box::new(WandTemplate::WandOfCloneMonster),
            Box::new(WandTemplate::WandOfTeleportAway),
            Box::new(WandTemplate::WandOfDisarming),
            Box::new(WandTemplate::WandOfLightningBalls),
            Box::new(WandTemplate::WandOfColdBalls),
            Box::new(WandTemplate::WandOfFireBalls),
            Box::new(WandTemplate::WandOfStinkingCloud),
            Box::new(WandTemplate::WandOfAcidBalls),
            Box::new(WandTemplate::WandOfWonder),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn template::Template>> {
        WandTemplate::vec().into_iter()
    }
}

impl template::Template for WandTemplate {
    fn name(&self) -> &str {
        match self {
            WandTemplate::WandOfProbing => "& %M Wand| of Probing^ (%P1 charges)",
            WandTemplate::WandOfLight => "& %M Wand| of Light^ (%P1 charges)",
            WandTemplate::WandOfLightningBolts => "& %M Wand| of Lightning Bolts^ (%P1 charges)",
            WandTemplate::WandOfFrostBolts => "& %M Wand| of Frost Bolts^ (%P1 charges)",
            WandTemplate::WandOfFireBolts => "& %M Wand| of Fire Bolts^ (%P1 charges)",
            WandTemplate::WandOfStoneToMud => "& %M Wand| of Stone-to-Mud^ (%P1 charges)",
            WandTemplate::WandOfPolymorph => "& %M Wand| of Polymorph^ (%P1 charges)",
            WandTemplate::WandOfHealMonster => "& %M Wand| of Heal Monster^ (%P1 charges)",
            WandTemplate::WandOfHasteMonster => "& %M Wand| of Haste Monster^ (%P1 charges)",
            WandTemplate::WandOfSlowMonster => "& %M Wand| of Slow Monster^ (%P1 charges)",
            WandTemplate::WandOfConfuseMonster => "& %M Wand| of Confuse Monster^ (%P1 charges)",
            WandTemplate::WandOfSleepMonster => "& %M Wand| of Sleep Monster^ (%P1 charges)",
            WandTemplate::WandOfDrainLife => "& %M Wand| of Drain Life^ (%P1 charges)",
            WandTemplate::WandOfTrapDoorDestruction => "& %M Wand| of Trap/Door destruction^ (%P1 charges)",
            WandTemplate::WandOfMagicMissile => "& %M Wand| of Magic Missile^ (%P1 charges)",
            WandTemplate::WandOfWallBuilding => "& %M Wand| of Wall Building^ (%P1 charges)",
            WandTemplate::WandOfCloneMonster => "& %M Wand| of Clone Monster^ (%P1 charges)",
            WandTemplate::WandOfTeleportAway => "& %M Wand| of Teleport Away^ (%P1 charges)",
            WandTemplate::WandOfDisarming => "& %M Wand| of Disarming^ (%P1 charges)",
            WandTemplate::WandOfLightningBalls => "& %M Wand| of Lightning Balls^ (%P1 charges)",
            WandTemplate::WandOfColdBalls => "& %M Wand| of Cold Balls^ (%P1 charges)",
            WandTemplate::WandOfFireBalls => "& %M Wand| of Fire Balls^ (%P1 charges)",
            WandTemplate::WandOfStinkingCloud => "& %M Wand| of Stinking Cloud^ (%P1 charges)",
            WandTemplate::WandOfAcidBalls => "& %M Wand| of Acid Balls^ (%P1 charges)",
            WandTemplate::WandOfWonder => "& %M Wand| of Wonder^ (%P1 charges)",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Wand }
    fn flags1(&self) -> u64 { 0 }

    fn flags2(&self) -> u64 {
        match self {
            WandTemplate::WandOfProbing => 0x01000000,
            WandTemplate::WandOfLight => 0x00000001,
            WandTemplate::WandOfLightningBolts => 0x00000002,
            WandTemplate::WandOfFrostBolts => 0x00000004,
            WandTemplate::WandOfFireBolts => 0x00000008,
            WandTemplate::WandOfStoneToMud => 0x00000010,
            WandTemplate::WandOfPolymorph => 0x00000020,
            WandTemplate::WandOfHealMonster => 0x00000040,
            WandTemplate::WandOfHasteMonster => 0x00000080,
            WandTemplate::WandOfSlowMonster => 0x00000100,
            WandTemplate::WandOfConfuseMonster => 0x00000200,
            WandTemplate::WandOfSleepMonster => 0x00000400,
            WandTemplate::WandOfDrainLife => 0x00000800,
            WandTemplate::WandOfTrapDoorDestruction => 0x00001000,
            WandTemplate::WandOfMagicMissile => 0x00002000,
            WandTemplate::WandOfWallBuilding => 0x00004000,
            WandTemplate::WandOfCloneMonster => 0x00008000,
            WandTemplate::WandOfTeleportAway => 0x00010000,
            WandTemplate::WandOfDisarming => 0x00020000,
            WandTemplate::WandOfLightningBalls => 0x00040000,
            WandTemplate::WandOfColdBalls => 0x00080000,
            WandTemplate::WandOfFireBalls => 0x00100000,
            WandTemplate::WandOfStinkingCloud => 0x00200000,
            WandTemplate::WandOfAcidBalls => 0x00400000,
            WandTemplate::WandOfWonder => 0x00800000,
        }
    }

    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            WandTemplate::WandOfProbing => 1500,
            WandTemplate::WandOfLight => 200,
            WandTemplate::WandOfLightningBolts => 600,
            WandTemplate::WandOfFrostBolts => 800,
            WandTemplate::WandOfFireBolts => 1000,
            WandTemplate::WandOfStoneToMud => 300,
            WandTemplate::WandOfPolymorph => 400,
            WandTemplate::WandOfHealMonster => 0,
            WandTemplate::WandOfHasteMonster => 0,
            WandTemplate::WandOfSlowMonster => 500,
            WandTemplate::WandOfConfuseMonster => 400,
            WandTemplate::WandOfSleepMonster => 500,
            WandTemplate::WandOfDrainLife => 2500,
            WandTemplate::WandOfTrapDoorDestruction => 100,
            WandTemplate::WandOfMagicMissile => 200,
            WandTemplate::WandOfWallBuilding => 1400,
            WandTemplate::WandOfCloneMonster => 0,
            WandTemplate::WandOfTeleportAway => 350,
            WandTemplate::WandOfDisarming => 700,
            WandTemplate::WandOfLightningBalls => 1200,
            WandTemplate::WandOfColdBalls => 1500,
            WandTemplate::WandOfFireBalls => 1800,
            WandTemplate::WandOfStinkingCloud => 400,
            WandTemplate::WandOfAcidBalls => 2650,
            WandTemplate::WandOfWonder => 250,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            WandTemplate::WandOfProbing => 25,
            WandTemplate::WandOfLight => 1,
            WandTemplate::WandOfLightningBolts => 2,
            WandTemplate::WandOfFrostBolts => 3,
            WandTemplate::WandOfFireBolts => 4,
            WandTemplate::WandOfStoneToMud => 5,
            WandTemplate::WandOfPolymorph => 6,
            WandTemplate::WandOfHealMonster => 7,
            WandTemplate::WandOfHasteMonster => 8,
            WandTemplate::WandOfSlowMonster => 9,
            WandTemplate::WandOfConfuseMonster => 10,
            WandTemplate::WandOfSleepMonster => 11,
            WandTemplate::WandOfDrainLife => 12,
            WandTemplate::WandOfTrapDoorDestruction => 13,
            WandTemplate::WandOfMagicMissile => 14,
            WandTemplate::WandOfWallBuilding => 15,
            WandTemplate::WandOfCloneMonster => 16,
            WandTemplate::WandOfTeleportAway => 17,
            WandTemplate::WandOfDisarming => 18,
            WandTemplate::WandOfLightningBalls => 19,
            WandTemplate::WandOfColdBalls => 20,
            WandTemplate::WandOfFireBalls => 21,
            WandTemplate::WandOfStinkingCloud => 22,
            WandTemplate::WandOfAcidBalls => 23,
            WandTemplate::WandOfWonder => 24,
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
            WandTemplate::WandOfProbing => 30,
            WandTemplate::WandOfLight => 0,
            WandTemplate::WandOfLightningBolts => 15,
            WandTemplate::WandOfFrostBolts => 20,
            WandTemplate::WandOfFireBolts => 30,
            WandTemplate::WandOfStoneToMud => 12,
            WandTemplate::WandOfPolymorph => 20,
            WandTemplate::WandOfHealMonster => 2,
            WandTemplate::WandOfHasteMonster => 2,
            WandTemplate::WandOfSlowMonster => 2,
            WandTemplate::WandOfConfuseMonster => 2,
            WandTemplate::WandOfSleepMonster => 7,
            WandTemplate::WandOfDrainLife => 50,
            WandTemplate::WandOfTrapDoorDestruction => 12,
            WandTemplate::WandOfMagicMissile => 2,
            WandTemplate::WandOfWallBuilding => 25,
            WandTemplate::WandOfCloneMonster => 2,
            WandTemplate::WandOfTeleportAway => 20,
            WandTemplate::WandOfDisarming => 20,
            WandTemplate::WandOfLightningBalls => 35,
            WandTemplate::WandOfColdBalls => 40,
            WandTemplate::WandOfFireBalls => 50,
            WandTemplate::WandOfStinkingCloud => 5,
            WandTemplate::WandOfAcidBalls => 48,
            WandTemplate::WandOfWonder => 2,
        }
    }

    fn is_identified(&self) -> bool { false }
}
