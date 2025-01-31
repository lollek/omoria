use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{ItemSubType, Potion1SubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PotionTemplate {
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

impl PotionTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(PotionTemplate::AppleJuice),
            Box::new(PotionTemplate::Blindness),
            Box::new(PotionTemplate::Boldliness),
            Box::new(PotionTemplate::Charisma),
            Box::new(PotionTemplate::Confusion),
            Box::new(PotionTemplate::CureCriticalWounds),
            Box::new(PotionTemplate::CureLightWounds),
            Box::new(PotionTemplate::CureSeriousWounds),
            Box::new(PotionTemplate::DetectInvisible),
            Box::new(PotionTemplate::FleaBile),
            Box::new(PotionTemplate::GainConstitution),
            Box::new(PotionTemplate::GainDexterity),
            Box::new(PotionTemplate::GainExperience),
            Box::new(PotionTemplate::GainIntelligence),
            Box::new(PotionTemplate::GainStrength),
            Box::new(PotionTemplate::GainWisdom),
            Box::new(PotionTemplate::HasteSelf),
            Box::new(PotionTemplate::Healing),
            Box::new(PotionTemplate::Heroism),
            Box::new(PotionTemplate::InfraVision),
            Box::new(PotionTemplate::Invulnerability),
            Box::new(PotionTemplate::Learning),
            Box::new(PotionTemplate::LoseIntelligence),
            Box::new(PotionTemplate::LoseMemories),
            Box::new(PotionTemplate::LoseWisdom),
            Box::new(PotionTemplate::NeutralizePoison),
            Box::new(PotionTemplate::Poison),
            Box::new(PotionTemplate::ResistCold),
            Box::new(PotionTemplate::ResistHeat),
            Box::new(PotionTemplate::RestoreCharisma),
            Box::new(PotionTemplate::RestoreConstitution),
            Box::new(PotionTemplate::RestoreDexterity),
            Box::new(PotionTemplate::RestoreIntelligence),
            Box::new(PotionTemplate::RestoreLifeLevels),
            Box::new(PotionTemplate::RestoreMana),
            Box::new(PotionTemplate::RestoreStrength),
            Box::new(PotionTemplate::RestoreWisdom),
            Box::new(PotionTemplate::SaltWater),
            Box::new(PotionTemplate::Sleep),
            Box::new(PotionTemplate::SlimeMoldJuice),
            Box::new(PotionTemplate::SlowPoison),
            Box::new(PotionTemplate::Slowness),
            Box::new(PotionTemplate::SuperHeroism),
            Box::new(PotionTemplate::Ugliness),
            Box::new(PotionTemplate::Water),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        PotionTemplate::vec().into_iter()
    }
}

impl ItemTemplate for PotionTemplate {
    fn name(&self) -> &str {
        match self {
            PotionTemplate::AppleJuice => "& Potion~| of Apple Juice",
            PotionTemplate::Blindness => "& Potion~| of Blindness",
            PotionTemplate::Boldliness => "& Potion~| of Boldliness",
            PotionTemplate::Charisma => "& Potion~| of Charisma",
            PotionTemplate::Confusion => "& Potion~| of Confusion",
            PotionTemplate::CureCriticalWounds => "& Potion~| of Cure Critical Wounds",
            PotionTemplate::CureLightWounds => "& Potion~| of Cure Light Wounds",
            PotionTemplate::CureSeriousWounds => "& Potion~| of Cure Serious Wounds",
            PotionTemplate::DetectInvisible => "& Potion~| of Detect Invisible",
            PotionTemplate::FleaBile => "& Potion~| of Flea Bile",
            PotionTemplate::GainConstitution => "& Potion~| of Gain Constitution",
            PotionTemplate::GainDexterity => "& Potion~| of Gain Dexterity",
            PotionTemplate::GainExperience => "& Potion~| of Gain Experience",
            PotionTemplate::GainIntelligence => "& Potion~| of Restore Strength",
            PotionTemplate::GainStrength => "& Potion~| of Gain Strength",
            PotionTemplate::GainWisdom => "& Potion~| of Gain Wisdom",
            PotionTemplate::HasteSelf => "& Potion~| of Haste Self",
            PotionTemplate::Healing => "& Potion~| of Healing",
            PotionTemplate::Heroism => "& Potion~| of Heroism",
            PotionTemplate::InfraVision => "& Potion~| of Infra-Vision",
            PotionTemplate::Invulnerability => "& Potion~| of Invulnerability",
            PotionTemplate::Learning => "& Potion~| of Learning",
            PotionTemplate::LoseIntelligence => "& Potion~| of Lose Intelligence",
            PotionTemplate::LoseMemories => "& Potion~| of Lose Memories",
            PotionTemplate::LoseWisdom => "& Potion~| of Lose Wisdom",
            PotionTemplate::NeutralizePoison => "& Potion~| of Neutralize Poison",
            PotionTemplate::Poison => "& Potion~| of Poison",
            PotionTemplate::ResistCold => "& Potion~| of Resist Cold",
            PotionTemplate::ResistHeat => "& Potion~| of Resist Heat",
            PotionTemplate::RestoreCharisma => "& Potion~| of Restore Charisma",
            PotionTemplate::RestoreConstitution => "& Potion~| of Restore Consitution",
            PotionTemplate::RestoreDexterity => "& Potion~| of Restore Dexterity",
            PotionTemplate::RestoreIntelligence => "& Potion~| of Restore Intelligence",
            PotionTemplate::RestoreLifeLevels => "& Potion~| of Restore Life Levels",
            PotionTemplate::RestoreMana => "& Potion~| of Restore Mana",
            PotionTemplate::RestoreStrength => "& Potion~| of Restore Strength",
            PotionTemplate::RestoreWisdom => "& Potion~| of Restore Wisdom",
            PotionTemplate::SaltWater => "& Potion~| of Salt Water",
            PotionTemplate::Sleep => "& Potion~| of Sleep",
            PotionTemplate::SlimeMoldJuice => "& Potion~| of Slime Mold Juice",
            PotionTemplate::SlowPoison => "& Potion~| of Slow Poison",
            PotionTemplate::Slowness => "& Potion~| of Slowness",
            PotionTemplate::SuperHeroism => "& Potion~| of Super Heroism",
            PotionTemplate::Ugliness => "& Potion~| of Ugliness",
            PotionTemplate::Water => "& Potion~| of Water",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Potion1
    }

    fn flags1(&self) -> u64 {
        match self {
            PotionTemplate::Blindness => 0x00080000,
            PotionTemplate::Boldliness => 0x00000000,
            PotionTemplate::Charisma => 0x00000200,
            PotionTemplate::Confusion => 0x00100000,
            PotionTemplate::CureCriticalWounds => 0x70004000,
            PotionTemplate::CureLightWounds => 0x10001000,
            PotionTemplate::CureSeriousWounds => 0x30002000,
            PotionTemplate::DetectInvisible => 0x00000000,
            PotionTemplate::FleaBile => 0x00000000,
            PotionTemplate::GainConstitution => 0x00010000,
            PotionTemplate::GainDexterity => 0x02000000,
            PotionTemplate::GainExperience => 0x00020000,
            PotionTemplate::GainIntelligence => 0x00000008,
            PotionTemplate::GainStrength => 0x00000001,
            PotionTemplate::GainWisdom => 0x00000040,
            PotionTemplate::HasteSelf => 0x00400000,
            PotionTemplate::Healing => 0x70008000,
            PotionTemplate::Heroism => 0x00000000,
            PotionTemplate::InfraVision => 0x00000000,
            PotionTemplate::Invulnerability => 0x00000000,
            PotionTemplate::Learning => 0x00000000,
            PotionTemplate::LoseIntelligence => 0x00000010,
            PotionTemplate::LoseMemories => 0x00000000,
            PotionTemplate::LoseWisdom => 0x00000080,
            PotionTemplate::NeutralizePoison => 0x00000000,
            PotionTemplate::Poison => 0x00000002,
            PotionTemplate::ResistCold => 0x00000000,
            PotionTemplate::ResistHeat => 0x00000000,
            PotionTemplate::RestoreCharisma => 0x00000800,
            PotionTemplate::RestoreConstitution => 0x68000000,
            PotionTemplate::RestoreDexterity => 0x04000000,
            PotionTemplate::RestoreIntelligence => 0x00000020,
            PotionTemplate::RestoreLifeLevels => 0x00000000,
            PotionTemplate::RestoreMana => 0x00000000,
            PotionTemplate::RestoreStrength => 0x00000004,
            PotionTemplate::RestoreWisdom => 0x00000100,
            PotionTemplate::SaltWater => 0x00000000,
            PotionTemplate::Sleep => 0x10040000,
            PotionTemplate::SlowPoison => 0x00000000,
            PotionTemplate::Slowness => 0x00800000,
            PotionTemplate::SuperHeroism => 0x00000000,
            PotionTemplate::Ugliness => 0x00000400,
            PotionTemplate::Water => 0x00800000,
            PotionTemplate::SlimeMoldJuice => 0x30000000,
            PotionTemplate::AppleJuice => 0x00800000,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            PotionTemplate::Blindness => 0x00000000,
            PotionTemplate::Boldliness => 0x00000020,
            PotionTemplate::Charisma => 0x00000000,
            PotionTemplate::Confusion => 0x00000000,
            PotionTemplate::CureCriticalWounds => 0x00000000,
            PotionTemplate::CureLightWounds => 0x00000000,
            PotionTemplate::CureSeriousWounds => 0x00000000,
            PotionTemplate::DetectInvisible => 0x00000200,
            PotionTemplate::FleaBile => 0x00004000,
            PotionTemplate::GainConstitution => 0x00000000,
            PotionTemplate::GainDexterity => 0x00000000,
            PotionTemplate::GainExperience => 0x00000000,
            PotionTemplate::GainIntelligence => 0x00000000,
            PotionTemplate::GainStrength => 0x00000000,
            PotionTemplate::GainWisdom => 0x00000000,
            PotionTemplate::HasteSelf => 0x00000000,
            PotionTemplate::Healing => 0x00000000,
            PotionTemplate::Heroism => 0x00000008,
            PotionTemplate::InfraVision => 0x00002000,
            PotionTemplate::Invulnerability => 0x00000004,
            PotionTemplate::Learning => 0x00008000,
            PotionTemplate::LoseIntelligence => 0x00000000,
            PotionTemplate::LoseMemories => 0x00000001,
            PotionTemplate::LoseWisdom => 0x00000000,
            PotionTemplate::NeutralizePoison => 0x00000800,
            PotionTemplate::Poison => 0x00000000,
            PotionTemplate::ResistCold => 0x00000100,
            PotionTemplate::ResistHeat => 0x00000080,
            PotionTemplate::RestoreCharisma => 0x00000000,
            PotionTemplate::RestoreConstitution => 0x00000000,
            PotionTemplate::RestoreDexterity => 0x00010000,
            PotionTemplate::RestoreIntelligence => 0x00000000,
            PotionTemplate::RestoreLifeLevels => 0x00000040,
            PotionTemplate::RestoreMana => 0x00001000,
            PotionTemplate::RestoreStrength => 0x00000000,
            PotionTemplate::RestoreWisdom => 0x00000000,
            PotionTemplate::SaltWater => 0x00000002,
            PotionTemplate::Sleep => 0x00000000,
            PotionTemplate::SlowPoison => 0x00000400,
            PotionTemplate::Slowness => 0x00000000,
            PotionTemplate::SuperHeroism => 0x00000010,
            PotionTemplate::Ugliness => 0x00000000,
            PotionTemplate::Water => 0x00000000,
            PotionTemplate::SlimeMoldJuice => 0x00000000,
            PotionTemplate::AppleJuice => 0x00000000,
        }
    }

    fn p1(&self) -> i64 {
        match self {
            PotionTemplate::Blindness => 0,
            PotionTemplate::Boldliness => 0,
            PotionTemplate::Charisma => 0,
            PotionTemplate::Confusion => 50,
            PotionTemplate::CureCriticalWounds => 100,
            PotionTemplate::CureLightWounds => 50,
            PotionTemplate::CureSeriousWounds => 100,
            PotionTemplate::DetectInvisible => 0,
            PotionTemplate::FleaBile => 0,
            PotionTemplate::GainConstitution => 0,
            PotionTemplate::GainDexterity => 0,
            PotionTemplate::GainExperience => 0,
            PotionTemplate::GainIntelligence => 0,
            PotionTemplate::GainStrength => 0,
            PotionTemplate::GainWisdom => 0,
            PotionTemplate::HasteSelf => 0,
            PotionTemplate::Healing => 200,
            PotionTemplate::Heroism => 0,
            PotionTemplate::InfraVision => 0,
            PotionTemplate::Invulnerability => 0,
            PotionTemplate::Learning => 0,
            PotionTemplate::LoseIntelligence => 0,
            PotionTemplate::LoseMemories => 0,
            PotionTemplate::LoseWisdom => 0,
            PotionTemplate::NeutralizePoison => 0,
            PotionTemplate::Poison => 0,
            PotionTemplate::ResistCold => 0,
            PotionTemplate::ResistHeat => 0,
            PotionTemplate::RestoreCharisma => 0,
            PotionTemplate::RestoreConstitution => 0,
            PotionTemplate::RestoreDexterity => 0,
            PotionTemplate::RestoreIntelligence => 0,
            PotionTemplate::RestoreLifeLevels => 0,
            PotionTemplate::RestoreMana => 0,
            PotionTemplate::RestoreStrength => 0,
            PotionTemplate::RestoreWisdom => 0,
            PotionTemplate::SaltWater => 0,
            PotionTemplate::Sleep => 100,
            PotionTemplate::SlowPoison => 0,
            PotionTemplate::Slowness => 50,
            PotionTemplate::SuperHeroism => 0,
            PotionTemplate::Ugliness => 0,
            PotionTemplate::Water => 200,
            PotionTemplate::SlimeMoldJuice => 400,
            PotionTemplate::AppleJuice => 250,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            PotionTemplate::Blindness => 0,
            PotionTemplate::Boldliness => 10,
            PotionTemplate::Charisma => 6500,
            PotionTemplate::Confusion => 0,
            PotionTemplate::CureCriticalWounds => 100,
            PotionTemplate::CureLightWounds => 15,
            PotionTemplate::CureSeriousWounds => 40,
            PotionTemplate::DetectInvisible => 50,
            PotionTemplate::FleaBile => 50,
            PotionTemplate::GainConstitution => 6500,
            PotionTemplate::GainDexterity => 6500,
            PotionTemplate::GainExperience => 15000,
            PotionTemplate::GainIntelligence => 6500,
            PotionTemplate::GainStrength => 6500,
            PotionTemplate::GainWisdom => 6500,
            PotionTemplate::HasteSelf => 155,
            PotionTemplate::Healing => 2500,
            PotionTemplate::Heroism => 35,
            PotionTemplate::InfraVision => 20,
            PotionTemplate::Invulnerability => 12500,
            PotionTemplate::Learning => 200,
            PotionTemplate::LoseIntelligence => 0,
            PotionTemplate::LoseMemories => 0,
            PotionTemplate::LoseWisdom => 0,
            PotionTemplate::NeutralizePoison => 75,
            PotionTemplate::Poison => 0,
            PotionTemplate::ResistCold => 30,
            PotionTemplate::ResistHeat => 30,
            PotionTemplate::RestoreCharisma => 300,
            PotionTemplate::RestoreConstitution => 300,
            PotionTemplate::RestoreDexterity => 300,
            PotionTemplate::RestoreIntelligence => 300,
            PotionTemplate::RestoreLifeLevels => 400,
            PotionTemplate::RestoreMana => 3000,
            PotionTemplate::RestoreStrength => 300,
            PotionTemplate::RestoreWisdom => 300,
            PotionTemplate::SaltWater => 0,
            PotionTemplate::Sleep => 0,
            PotionTemplate::SlowPoison => 25,
            PotionTemplate::Slowness => 0,
            PotionTemplate::SuperHeroism => 100,
            PotionTemplate::Ugliness => 0,
            PotionTemplate::Water => 0,
            PotionTemplate::SlimeMoldJuice => 2,
            PotionTemplate::AppleJuice => 1,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            PotionTemplate::Blindness => ItemSubType::Potion1(Potion1SubType::Blindness),
            PotionTemplate::Boldliness => ItemSubType::Potion1(Potion1SubType::Boldliness),
            PotionTemplate::Charisma => ItemSubType::Potion1(Potion1SubType::Charisma),
            PotionTemplate::Confusion => ItemSubType::Potion1(Potion1SubType::Confusion),
            PotionTemplate::CureCriticalWounds => {
                ItemSubType::Potion1(Potion1SubType::CureCriticalWounds)
            }
            PotionTemplate::CureLightWounds => {
                ItemSubType::Potion1(Potion1SubType::CureLightWounds)
            }
            PotionTemplate::CureSeriousWounds => {
                ItemSubType::Potion1(Potion1SubType::CureSeriousWounds)
            }
            PotionTemplate::DetectInvisible => {
                ItemSubType::Potion1(Potion1SubType::DetectInvisible)
            }
            PotionTemplate::FleaBile => ItemSubType::Potion1(Potion1SubType::FleaBile),
            PotionTemplate::GainConstitution => {
                ItemSubType::Potion1(Potion1SubType::GainConstitution)
            }
            PotionTemplate::GainDexterity => ItemSubType::Potion1(Potion1SubType::GainDexterity),
            PotionTemplate::GainExperience => ItemSubType::Potion1(Potion1SubType::GainExperience),
            PotionTemplate::GainIntelligence => {
                ItemSubType::Potion1(Potion1SubType::GainIntelligence)
            }
            PotionTemplate::GainStrength => ItemSubType::Potion1(Potion1SubType::GainStrength),
            PotionTemplate::GainWisdom => ItemSubType::Potion1(Potion1SubType::GainWisdom),
            PotionTemplate::HasteSelf => ItemSubType::Potion1(Potion1SubType::HasteSelf),
            PotionTemplate::Healing => ItemSubType::Potion1(Potion1SubType::Healing),
            PotionTemplate::Heroism => ItemSubType::Potion1(Potion1SubType::Heroism),
            PotionTemplate::InfraVision => ItemSubType::Potion1(Potion1SubType::InfraVision),
            PotionTemplate::Invulnerability => {
                ItemSubType::Potion1(Potion1SubType::Invulnerability)
            }
            PotionTemplate::Learning => ItemSubType::Potion1(Potion1SubType::Learning),
            PotionTemplate::LoseIntelligence => {
                ItemSubType::Potion1(Potion1SubType::LoseIntelligence)
            }
            PotionTemplate::LoseMemories => ItemSubType::Potion1(Potion1SubType::LoseMemories),
            PotionTemplate::LoseWisdom => ItemSubType::Potion1(Potion1SubType::LoseWisdom),
            PotionTemplate::NeutralizePoison => {
                ItemSubType::Potion1(Potion1SubType::NeutralizePoison)
            }
            PotionTemplate::Poison => ItemSubType::Potion1(Potion1SubType::Poison),
            PotionTemplate::ResistCold => ItemSubType::Potion1(Potion1SubType::ResistCold),
            PotionTemplate::ResistHeat => ItemSubType::Potion1(Potion1SubType::ResistHeat),
            PotionTemplate::RestoreCharisma => {
                ItemSubType::Potion1(Potion1SubType::RestoreCharisma)
            }
            PotionTemplate::RestoreConstitution => {
                ItemSubType::Potion1(Potion1SubType::RestoreConstitution)
            }
            PotionTemplate::RestoreDexterity => {
                ItemSubType::Potion1(Potion1SubType::RestoreDexterity)
            }
            PotionTemplate::RestoreIntelligence => {
                ItemSubType::Potion1(Potion1SubType::RestoreIntelligence)
            }
            PotionTemplate::RestoreLifeLevels => {
                ItemSubType::Potion1(Potion1SubType::RestoreLifeLevels)
            }
            PotionTemplate::RestoreMana => ItemSubType::Potion1(Potion1SubType::RestoreMana),
            PotionTemplate::RestoreStrength => {
                ItemSubType::Potion1(Potion1SubType::RestoreStrength)
            }
            PotionTemplate::RestoreWisdom => ItemSubType::Potion1(Potion1SubType::RestoreWisdom),
            PotionTemplate::SaltWater => ItemSubType::Potion1(Potion1SubType::SaltWater),
            PotionTemplate::Sleep => ItemSubType::Potion1(Potion1SubType::Sleep),
            PotionTemplate::SlowPoison => ItemSubType::Potion1(Potion1SubType::SlowPoison),
            PotionTemplate::Slowness => ItemSubType::Potion1(Potion1SubType::Slowness),
            PotionTemplate::SuperHeroism => ItemSubType::Potion1(Potion1SubType::SuperHeroism),
            PotionTemplate::Ugliness => ItemSubType::Potion1(Potion1SubType::Ugliness),
            PotionTemplate::Water => ItemSubType::Potion1(Potion1SubType::Water),
            PotionTemplate::SlimeMoldJuice => ItemSubType::Potion1(Potion1SubType::SlimeMoldJuice),
            PotionTemplate::AppleJuice => ItemSubType::Potion1(Potion1SubType::AppleJuice),
        }
    }

    fn weight(&self) -> u16 {
        4
    }
    fn number(&self) -> u16 {
        1
    }
    fn modifier_to_hit(&self) -> i16 {
        0
    }
    fn modifier_to_damage(&self) -> i16 {
        0
    }
    fn base_ac(&self) -> i16 {
        0
    }
    fn modifier_to_ac(&self) -> i16 {
        0
    }
    fn damage(&self) -> &str {
        "1d1"
    }

    fn item_level(&self) -> u8 {
        match self {
            PotionTemplate::SaltWater => 0,
            PotionTemplate::FleaBile => 0,
            PotionTemplate::Blindness => 0,
            PotionTemplate::Confusion => 0,
            PotionTemplate::CureLightWounds => 0,
            PotionTemplate::Water => 0,
            PotionTemplate::Sleep => 0,
            PotionTemplate::SlimeMoldJuice => 0,
            PotionTemplate::AppleJuice => 0,
            PotionTemplate::ResistCold => 1,
            PotionTemplate::HasteSelf => 1,
            PotionTemplate::Boldliness => 1,
            PotionTemplate::ResistHeat => 1,
            PotionTemplate::Heroism => 1,
            PotionTemplate::SlowPoison => 1,
            PotionTemplate::Slowness => 1,
            PotionTemplate::CureSeriousWounds => 3,
            PotionTemplate::DetectInvisible => 3,
            PotionTemplate::InfraVision => 3,
            PotionTemplate::Poison => 3,
            PotionTemplate::SuperHeroism => 3,
            PotionTemplate::NeutralizePoison => 5,
            PotionTemplate::CureCriticalWounds => 5,
            PotionTemplate::LoseMemories => 10,
            PotionTemplate::Healing => 12,
            PotionTemplate::Charisma => 25,
            PotionTemplate::Ugliness => 25,
            PotionTemplate::GainConstitution => 25,
            PotionTemplate::GainDexterity => 25,
            PotionTemplate::GainIntelligence => 25,
            PotionTemplate::GainStrength => 25,
            PotionTemplate::GainWisdom => 25,
            PotionTemplate::LoseIntelligence => 25,
            PotionTemplate::RestoreMana => 25,
            PotionTemplate::LoseWisdom => 25,
            PotionTemplate::RestoreCharisma => 40,
            PotionTemplate::RestoreConstitution => 40,
            PotionTemplate::RestoreDexterity => 40,
            PotionTemplate::RestoreIntelligence => 40,
            PotionTemplate::RestoreStrength => 40,
            PotionTemplate::RestoreWisdom => 40,
            PotionTemplate::RestoreLifeLevels => 40,
            PotionTemplate::Invulnerability => 40,
            PotionTemplate::Learning => 45,
            PotionTemplate::GainExperience => 50,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
