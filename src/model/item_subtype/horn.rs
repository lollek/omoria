use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HornSubType {
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

impl From<HornSubType> for usize {
    fn from(value: HornSubType) -> usize {
        match value {
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
}

impl TryFrom<usize> for HornSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HornSubType::HornOfBubbles),
            2 => Ok(HornSubType::HornOfCalling),
            3 => Ok(HornSubType::HornOfSoftSounds),
            4 => Ok(HornSubType::HornOfBlasting),
            5 => Ok(HornSubType::HornOfCold),
            6 => Ok(HornSubType::HornOfHeat),
            7 => Ok(HornSubType::HornOfGas),
            8 => Ok(HornSubType::HornOfRecall),
            9 => Ok(HornSubType::HornOfChaos),
            10 => Ok(HornSubType::HornOfGlue),
            11 => Ok(HornSubType::HornOfValhalla),
            12 => Ok(HornSubType::HornOfTritons),
            13 => Ok(HornSubType::HornOfFog),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::HornSubType;

    #[test]
    fn test_horn_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            HornSubType::try_from(1usize).unwrap(),
            HornSubType::HornOfBubbles
        );
    }

    #[test]
    fn test_horn_subtype_try_from_usize_rejects_unknown_value() {
        assert!(HornSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_horn_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(HornSubType::HornOfFog), 13);
    }
}
