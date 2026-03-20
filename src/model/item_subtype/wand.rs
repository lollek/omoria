use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WandSubType {
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

impl From<WandSubType> for usize {
    fn from(value: WandSubType) -> usize {
        match value {
            WandSubType::WandOfProbing => 25,
            WandSubType::WandOfLight => 1,
            WandSubType::WandOfLightningBolts => 2,
            WandSubType::WandOfFrostBolts => 3,
            WandSubType::WandOfFireBolts => 4,
            WandSubType::WandOfStoneToMud => 5,
            WandSubType::WandOfPolymorph => 6,
            WandSubType::WandOfHealMonster => 7,
            WandSubType::WandOfHasteMonster => 8,
            WandSubType::WandOfSlowMonster => 9,
            WandSubType::WandOfConfuseMonster => 10,
            WandSubType::WandOfSleepMonster => 11,
            WandSubType::WandOfDrainLife => 12,
            WandSubType::WandOfTrapDoorDestruction => 13,
            WandSubType::WandOfMagicMissile => 14,
            WandSubType::WandOfWallBuilding => 15,
            WandSubType::WandOfCloneMonster => 16,
            WandSubType::WandOfTeleportAway => 17,
            WandSubType::WandOfDisarming => 18,
            WandSubType::WandOfLightningBalls => 19,
            WandSubType::WandOfColdBalls => 20,
            WandSubType::WandOfFireBalls => 21,
            WandSubType::WandOfStinkingCloud => 22,
            WandSubType::WandOfAcidBalls => 23,
            WandSubType::WandOfWonder => 24,
        }
    }
}

impl TryFrom<usize> for WandSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            25 => Ok(WandSubType::WandOfProbing),
            1 => Ok(WandSubType::WandOfLight),
            2 => Ok(WandSubType::WandOfLightningBolts),
            3 => Ok(WandSubType::WandOfFrostBolts),
            4 => Ok(WandSubType::WandOfFireBolts),
            5 => Ok(WandSubType::WandOfStoneToMud),
            6 => Ok(WandSubType::WandOfPolymorph),
            7 => Ok(WandSubType::WandOfHealMonster),
            8 => Ok(WandSubType::WandOfHasteMonster),
            9 => Ok(WandSubType::WandOfSlowMonster),
            10 => Ok(WandSubType::WandOfConfuseMonster),
            11 => Ok(WandSubType::WandOfSleepMonster),
            12 => Ok(WandSubType::WandOfDrainLife),
            13 => Ok(WandSubType::WandOfTrapDoorDestruction),
            14 => Ok(WandSubType::WandOfMagicMissile),
            15 => Ok(WandSubType::WandOfWallBuilding),
            16 => Ok(WandSubType::WandOfCloneMonster),
            17 => Ok(WandSubType::WandOfTeleportAway),
            18 => Ok(WandSubType::WandOfDisarming),
            19 => Ok(WandSubType::WandOfLightningBalls),
            20 => Ok(WandSubType::WandOfColdBalls),
            21 => Ok(WandSubType::WandOfFireBalls),
            22 => Ok(WandSubType::WandOfStinkingCloud),
            23 => Ok(WandSubType::WandOfAcidBalls),
            24 => Ok(WandSubType::WandOfWonder),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::WandSubType;

    #[test]
    fn test_wand_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            WandSubType::try_from(1usize).unwrap(),
            WandSubType::WandOfLight
        );
    }

    #[test]
    fn test_wand_subtype_try_from_usize_rejects_unknown_value() {
        assert!(WandSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_wand_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(WandSubType::WandOfProbing), 25);
    }
}
