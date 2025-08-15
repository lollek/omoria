use crate::model::item_subtype::HornSubType;

pub fn from_usize(subtype: usize) -> Option<HornSubType> {
    match subtype {
        1 => Some(HornSubType::HornOfBubbles),
        2 => Some(HornSubType::HornOfCalling),
        3 => Some(HornSubType::HornOfSoftSounds),
        4 => Some(HornSubType::HornOfBlasting),
        5 => Some(HornSubType::HornOfCold),
        6 => Some(HornSubType::HornOfHeat),
        7 => Some(HornSubType::HornOfGas),
        8 => Some(HornSubType::HornOfRecall),
        9 => Some(HornSubType::HornOfChaos),
        10 => Some(HornSubType::HornOfGlue),
        11 => Some(HornSubType::HornOfValhalla),
        12 => Some(HornSubType::HornOfTritons),
        13 => Some(HornSubType::HornOfFog),
        _ => None,
    }
}

pub fn to_usize(subtype: &HornSubType) -> usize {
    match subtype {
        HornSubType::HornOfBubbles => 1,
        HornSubType::HornOfCalling => 2,
        HornSubType::HornOfSoftSounds => 3,
        HornSubType::HornOfBlasting => 4,
        HornSubType::HornOfCold => 5,
        HornSubType::HornOfHeat => 6,
        HornSubType::HornOfGas => 7,
        HornSubType::HornOfRecall => 8,
        HornSubType::HornOfChaos => 9,
        HornSubType::HornOfGlue => 10,
        HornSubType::HornOfValhalla => 11,
        HornSubType::HornOfTritons => 12,
        HornSubType::HornOfFog => 13,
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
