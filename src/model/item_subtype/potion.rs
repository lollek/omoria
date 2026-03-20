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

impl From<Potion1SubType> for usize {
    fn from(value: Potion1SubType) -> usize {
        match value {
            Potion1SubType::Blindness => 276,
            Potion1SubType::Boldliness => 293,
            Potion1SubType::Charisma => 266,
            Potion1SubType::Confusion => 277,
            Potion1SubType::CureCriticalWounds => 271,
            Potion1SubType::CureLightWounds => 269,
            Potion1SubType::CureSeriousWounds => 270,
            Potion1SubType::DetectInvisible => 297,
            Potion1SubType::FleaBile => 302,
            Potion1SubType::GainConstitution => 273,
            Potion1SubType::GainDexterity => 284,
            Potion1SubType::GainExperience => 274,
            Potion1SubType::GainIntelligence => 260,
            Potion1SubType::GainStrength => 257,
            Potion1SubType::GainWisdom => 263,
            Potion1SubType::HasteSelf => 279,
            Potion1SubType::Healing => 272,
            Potion1SubType::Heroism => 291,
            Potion1SubType::InfraVision => 301,
            Potion1SubType::Invulnerability => 290,
            Potion1SubType::Learning => 287,
            Potion1SubType::LoseIntelligence => 261,
            Potion1SubType::LoseMemories => 288,
            Potion1SubType::LoseWisdom => 264,
            Potion1SubType::NeutralizePoison => 299,
            Potion1SubType::Poison => 258,
            Potion1SubType::ResistCold => 296,
            Potion1SubType::ResistHeat => 295,
            Potion1SubType::RestoreCharisma => 268,
            Potion1SubType::RestoreConstitution => 286,
            Potion1SubType::RestoreDexterity => 285,
            Potion1SubType::RestoreIntelligence => 262,
            Potion1SubType::RestoreLifeLevels => 294,
            Potion1SubType::RestoreMana => 300,
            Potion1SubType::RestoreStrength => 259,
            Potion1SubType::RestoreWisdom => 265,
            Potion1SubType::SaltWater => 289,
            Potion1SubType::Sleep => 275,
            Potion1SubType::SlowPoison => 298,
            Potion1SubType::Slowness => 280,
            Potion1SubType::SuperHeroism => 292,
            Potion1SubType::Ugliness => 267,
            Potion1SubType::Water => 283,
            Potion1SubType::SlimeMoldJuice => 281,
            Potion1SubType::AppleJuice => 282,
        }
    }
}

impl TryFrom<usize> for Potion1SubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            276 => Ok(Potion1SubType::Blindness),
            293 => Ok(Potion1SubType::Boldliness),
            266 => Ok(Potion1SubType::Charisma),
            277 => Ok(Potion1SubType::Confusion),
            271 => Ok(Potion1SubType::CureCriticalWounds),
            269 => Ok(Potion1SubType::CureLightWounds),
            270 => Ok(Potion1SubType::CureSeriousWounds),
            297 => Ok(Potion1SubType::DetectInvisible),
            302 => Ok(Potion1SubType::FleaBile),
            273 => Ok(Potion1SubType::GainConstitution),
            284 => Ok(Potion1SubType::GainDexterity),
            274 => Ok(Potion1SubType::GainExperience),
            260 => Ok(Potion1SubType::GainIntelligence),
            257 => Ok(Potion1SubType::GainStrength),
            263 => Ok(Potion1SubType::GainWisdom),
            279 => Ok(Potion1SubType::HasteSelf),
            272 => Ok(Potion1SubType::Healing),
            291 => Ok(Potion1SubType::Heroism),
            301 => Ok(Potion1SubType::InfraVision),
            290 => Ok(Potion1SubType::Invulnerability),
            287 => Ok(Potion1SubType::Learning),
            261 => Ok(Potion1SubType::LoseIntelligence),
            288 => Ok(Potion1SubType::LoseMemories),
            264 => Ok(Potion1SubType::LoseWisdom),
            299 => Ok(Potion1SubType::NeutralizePoison),
            258 => Ok(Potion1SubType::Poison),
            296 => Ok(Potion1SubType::ResistCold),
            295 => Ok(Potion1SubType::ResistHeat),
            268 => Ok(Potion1SubType::RestoreCharisma),
            286 => Ok(Potion1SubType::RestoreConstitution),
            285 => Ok(Potion1SubType::RestoreDexterity),
            262 => Ok(Potion1SubType::RestoreIntelligence),
            294 => Ok(Potion1SubType::RestoreLifeLevels),
            300 => Ok(Potion1SubType::RestoreMana),
            259 => Ok(Potion1SubType::RestoreStrength),
            265 => Ok(Potion1SubType::RestoreWisdom),
            289 => Ok(Potion1SubType::SaltWater),
            275 => Ok(Potion1SubType::Sleep),
            298 => Ok(Potion1SubType::SlowPoison),
            280 => Ok(Potion1SubType::Slowness),
            292 => Ok(Potion1SubType::SuperHeroism),
            267 => Ok(Potion1SubType::Ugliness),
            283 => Ok(Potion1SubType::Water),
            281 => Ok(Potion1SubType::SlimeMoldJuice),
            282 => Ok(Potion1SubType::AppleJuice),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod potion1_tests {
    use std::convert::TryFrom;

    use super::Potion1SubType;

    #[test]
    fn test_potion1_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            Potion1SubType::try_from(257usize).unwrap(),
            Potion1SubType::GainStrength
        );
    }

    #[test]
    fn test_potion1_subtype_try_from_usize_rejects_unknown_value() {
        assert!(Potion1SubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_potion1_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(Potion1SubType::AppleJuice), 282);
    }
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
