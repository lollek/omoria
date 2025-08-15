use crate::model::item_subtype::WandSubType;

pub fn from_usize(subtype: usize) -> Option<WandSubType> {
    match subtype {
        25 => Some(WandSubType::WandOfProbing),
        1 => Some(WandSubType::WandOfLight),
        2 => Some(WandSubType::WandOfLightningBolts),
        3 => Some(WandSubType::WandOfFrostBolts),
        4 => Some(WandSubType::WandOfFireBolts),
        5 => Some(WandSubType::WandOfStoneToMud),
        6 => Some(WandSubType::WandOfPolymorph),
        7 => Some(WandSubType::WandOfHealMonster),
        8 => Some(WandSubType::WandOfHasteMonster),
        9 => Some(WandSubType::WandOfSlowMonster),
        10 => Some(WandSubType::WandOfConfuseMonster),
        11 => Some(WandSubType::WandOfSleepMonster),
        12 => Some(WandSubType::WandOfDrainLife),
        13 => Some(WandSubType::WandOfTrapDoorDestruction),
        14 => Some(WandSubType::WandOfMagicMissile),
        15 => Some(WandSubType::WandOfWallBuilding),
        16 => Some(WandSubType::WandOfCloneMonster),
        17 => Some(WandSubType::WandOfTeleportAway),
        18 => Some(WandSubType::WandOfDisarming),
        19 => Some(WandSubType::WandOfLightningBalls),
        20 => Some(WandSubType::WandOfColdBalls),
        21 => Some(WandSubType::WandOfFireBalls),
        22 => Some(WandSubType::WandOfStinkingCloud),
        23 => Some(WandSubType::WandOfAcidBalls),
        24 => Some(WandSubType::WandOfWonder),
        _ => None,
    }
}

pub fn to_usize(subtype: &WandSubType) -> usize {
    match subtype {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_conversion() {
        (0..1000).for_each(|i| {
            if let Some(subtype) = from_usize(i) {
                assert_eq!(i, to_usize(&subtype));
            }
        })
    }
}
