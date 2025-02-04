use crate::model::item_subtype::Potion1SubType;

pub fn from_usize(subtype: usize) -> Option<Potion1SubType> {
    match subtype {
        276 => Some(Potion1SubType::Blindness),
        293 => Some(Potion1SubType::Boldliness),
        266 => Some(Potion1SubType::Charisma),
        277 => Some(Potion1SubType::Confusion),
        271 => Some(Potion1SubType::CureCriticalWounds),
        269 => Some(Potion1SubType::CureLightWounds),
        270 => Some(Potion1SubType::CureSeriousWounds),
        297 => Some(Potion1SubType::DetectInvisible),
        302 => Some(Potion1SubType::FleaBile),
        273 => Some(Potion1SubType::GainConstitution),
        284 => Some(Potion1SubType::GainDexterity),
        274 => Some(Potion1SubType::GainExperience),
        260 => Some(Potion1SubType::GainIntelligence),
        257 => Some(Potion1SubType::GainStrength),
        263 => Some(Potion1SubType::GainWisdom),
        279 => Some(Potion1SubType::HasteSelf),
        272 => Some(Potion1SubType::Healing),
        291 => Some(Potion1SubType::Heroism),
        301 => Some(Potion1SubType::InfraVision),
        290 => Some(Potion1SubType::Invulnerability),
        287 => Some(Potion1SubType::Learning),
        261 => Some(Potion1SubType::LoseIntelligence),
        288 => Some(Potion1SubType::LoseMemories),
        264 => Some(Potion1SubType::LoseWisdom),
        299 => Some(Potion1SubType::NeutralizePoison),
        258 => Some(Potion1SubType::Poison),
        296 => Some(Potion1SubType::ResistCold),
        295 => Some(Potion1SubType::ResistHeat),
        268 => Some(Potion1SubType::RestoreCharisma),
        286 => Some(Potion1SubType::RestoreConstitution),
        285 => Some(Potion1SubType::RestoreDexterity),
        262 => Some(Potion1SubType::RestoreIntelligence),
        294 => Some(Potion1SubType::RestoreLifeLevels),
        300 => Some(Potion1SubType::RestoreMana),
        259 => Some(Potion1SubType::RestoreStrength),
        265 => Some(Potion1SubType::RestoreWisdom),
        289 => Some(Potion1SubType::SaltWater),
        275 => Some(Potion1SubType::Sleep),
        298 => Some(Potion1SubType::SlowPoison),
        280 => Some(Potion1SubType::Slowness),
        292 => Some(Potion1SubType::SuperHeroism),
        267 => Some(Potion1SubType::Ugliness),
        283 => Some(Potion1SubType::Water),
        281 => Some(Potion1SubType::SlimeMoldJuice),
        282 => Some(Potion1SubType::AppleJuice),
        _ => None,
    }
}

pub fn to_usize(subtype: Potion1SubType) -> usize {
    match subtype {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_conversion() {
        (0..1000).for_each(|i| {
            if let Some(subtype) = from_usize(i) {
                assert_eq!(i, to_usize(subtype));
            }
        })
    }
}
