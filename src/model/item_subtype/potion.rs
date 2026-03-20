use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Potion1SubType {
    AppleJuice,
    Blindness,
    Boldliness,
    Charisma,
    Confusion,
    CureCriticalWounds,
    CureLightWounds,
    CureSeriousWounds,
    DetectInvisible,
    FleaBile,
    GainConstitution,
    GainDexterity,
    GainExperience,
    GainIntelligence,
    GainStrength,
    GainWisdom,
    HasteSelf,
    Healing,
    Heroism,
    InfraVision,
    Invulnerability,
    Learning,
    LoseIntelligence,
    LoseMemories,
    LoseWisdom,
    NeutralizePoison,
    Poison,
    ResistCold,
    ResistHeat,
    RestoreCharisma,
    RestoreConstitution,
    RestoreDexterity,
    RestoreIntelligence,
    RestoreLifeLevels,
    RestoreMana,
    RestoreStrength,
    RestoreWisdom,
    SaltWater,
    Sleep,
    SlimeMoldJuice,
    SlowPoison,
    Slowness,
    SuperHeroism,
    Ugliness,
    Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Potion2SubType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlaskOfOilSubType {
    FlaskOfOil,
}

impl From<FlaskOfOilSubType> for usize {
    fn from(value: FlaskOfOilSubType) -> usize {
        match value {
            FlaskOfOilSubType::FlaskOfOil => 1,
        }
    }
}

impl TryFrom<usize> for FlaskOfOilSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FlaskOfOilSubType::FlaskOfOil),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::FlaskOfOilSubType;

    #[test]
    fn test_flask_of_oil_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            FlaskOfOilSubType::try_from(1usize).unwrap(),
            FlaskOfOilSubType::FlaskOfOil
        );
    }

    #[test]
    fn test_flask_of_oil_subtype_try_from_usize_rejects_unknown_values() {
        assert!(FlaskOfOilSubType::try_from(0usize).is_err());
        assert!(FlaskOfOilSubType::try_from(2usize).is_err());
    }

    #[test]
    fn test_flask_of_oil_subtype_into_usize_returns_expected_code() {
        let code: usize = FlaskOfOilSubType::FlaskOfOil.into();
        assert_eq!(code, 1);
    }
}
