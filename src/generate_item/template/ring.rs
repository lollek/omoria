use super::super::item_template::ItemTemplate;
use crate::generate_item::item_template::WornFlag1;
use crate::model::{
    self,
    item_subtype::{ItemSubType, RingSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum RingTemplate {
    RingOfGainStrength,
    RingOfGainDexterity,
    RingOfGainConstitution,
    RingOfGainIntelligence,
    RingOfSpeed1,
    RingOfSpeed2,
    RingOfSearching,
    RingOfTeleportation,
    RingOfSlowDigestion,
    RingOfResistFire,
    RingOfResistCold,
    RingOfFeatherFalling,
    RingOfAdornment1,
    RingOfAdornment2,
    RingOfWeakness,
    RingOfLordlyProtectionFire,
    RingOfLordlyProtectionAcid,
    RingOfLordlyProtectionCold,
    RingOfWoe,
    RingOfStupidity,
    RingOfIncreaseDamage,
    RingOfIncreaseToHit,
    RingOfProtection,
    RingOfAggravateMonsters,
    RingOfSeeInvisible,
    RingOfSustainStrength,
    RingOfSustainIntelligence,
    RingOfSustainWisdom,
    RingOfSustainConstitution,
    RingOfSustainDexterity,
    RingOfSustainCharisma,
    RingOfSlaying,
    RingOfGnomekind,
}

impl RingTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(RingTemplate::RingOfGainStrength),
            Box::new(RingTemplate::RingOfGainDexterity),
            Box::new(RingTemplate::RingOfGainConstitution),
            Box::new(RingTemplate::RingOfGainIntelligence),
            Box::new(RingTemplate::RingOfSpeed1),
            Box::new(RingTemplate::RingOfSpeed2),
            Box::new(RingTemplate::RingOfSearching),
            Box::new(RingTemplate::RingOfTeleportation),
            Box::new(RingTemplate::RingOfSlowDigestion),
            Box::new(RingTemplate::RingOfResistFire),
            Box::new(RingTemplate::RingOfResistCold),
            Box::new(RingTemplate::RingOfFeatherFalling),
            Box::new(RingTemplate::RingOfAdornment1),
            Box::new(RingTemplate::RingOfAdornment2),
            Box::new(RingTemplate::RingOfWeakness),
            Box::new(RingTemplate::RingOfLordlyProtectionFire),
            Box::new(RingTemplate::RingOfLordlyProtectionAcid),
            Box::new(RingTemplate::RingOfLordlyProtectionCold),
            Box::new(RingTemplate::RingOfWoe),
            Box::new(RingTemplate::RingOfStupidity),
            Box::new(RingTemplate::RingOfIncreaseDamage),
            Box::new(RingTemplate::RingOfIncreaseToHit),
            Box::new(RingTemplate::RingOfProtection),
            Box::new(RingTemplate::RingOfAggravateMonsters),
            Box::new(RingTemplate::RingOfSeeInvisible),
            Box::new(RingTemplate::RingOfSustainStrength),
            Box::new(RingTemplate::RingOfSustainIntelligence),
            Box::new(RingTemplate::RingOfSustainWisdom),
            Box::new(RingTemplate::RingOfSustainConstitution),
            Box::new(RingTemplate::RingOfSustainDexterity),
            Box::new(RingTemplate::RingOfSustainCharisma),
            Box::new(RingTemplate::RingOfSlaying),
            Box::new(RingTemplate::RingOfGnomekind),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        RingTemplate::vec().into_iter()
    }
}

impl ItemTemplate for RingTemplate {
    fn name(&self) -> &str {
        match self {
            RingTemplate::RingOfGainStrength => "& Ring| of Gain Strength^ (%p1)",
            RingTemplate::RingOfGainDexterity => "& Ring| of Gain Dexterity^ (%p1)",
            RingTemplate::RingOfGainConstitution => "& Ring| of Gain Constitution^ (%p1)",
            RingTemplate::RingOfGainIntelligence => "& Ring| of Gain Intelligence^ (%p1)",
            RingTemplate::RingOfSpeed1 => "& Ring| of Speed^ (%p1)",
            RingTemplate::RingOfSpeed2 => "& Ring| of Speed^ (%p1)",
            RingTemplate::RingOfSearching => "& Ring| of Searching^ (%p1)",
            RingTemplate::RingOfTeleportation => "& Ring| of Teleportation^",
            RingTemplate::RingOfSlowDigestion => "& Ring| of Slow Digestion^",
            RingTemplate::RingOfResistFire => "& Ring| of Resist Fire^",
            RingTemplate::RingOfResistCold => "& Ring| of Resist Cold^",
            RingTemplate::RingOfFeatherFalling => "& Ring| of Feather Falling^",
            RingTemplate::RingOfAdornment1 => "& Ring| of Adornment^",
            RingTemplate::RingOfAdornment2 => "& Ring| of Adornment^",
            RingTemplate::RingOfWeakness => "& Ring| of Weakness^",
            RingTemplate::RingOfLordlyProtectionFire => "& Ring| of Lordly Protection (Fire)^",
            RingTemplate::RingOfLordlyProtectionAcid => "& Ring| of Lordly Protection (Acid)^",
            RingTemplate::RingOfLordlyProtectionCold => "& Ring| of Lordly Protection (Cold)^",
            RingTemplate::RingOfWoe => "& Ring| of Woe^",
            RingTemplate::RingOfStupidity => "& Ring| of Stupidity^",
            RingTemplate::RingOfIncreaseDamage => "& Ring| of Increase Damage^ (%p3)",
            RingTemplate::RingOfIncreaseToHit => "& Ring| of Increase To-hit^ (%p2)",
            RingTemplate::RingOfProtection => "& Ring| of Protection^ [%p4]",
            RingTemplate::RingOfAggravateMonsters => "& Ring| of Aggravate Monster^",
            RingTemplate::RingOfSeeInvisible => "& Ring| of See Invisible^",
            RingTemplate::RingOfSustainStrength => "& Ring| of Sustain Strength^",
            RingTemplate::RingOfSustainIntelligence => "& Ring| of Sustain Intelligence^",
            RingTemplate::RingOfSustainWisdom => "& Ring| of Sustain Wisdom^",
            RingTemplate::RingOfSustainConstitution => "& Ring| of Sustain Constitution^",
            RingTemplate::RingOfSustainDexterity => "& Ring| of Sustain Dexterity^",
            RingTemplate::RingOfSustainCharisma => "& Ring| of Sustain Charisma^",
            RingTemplate::RingOfSlaying => "& Ring| of Slaying^",
            RingTemplate::RingOfGnomekind => "& Ring| of Gnomekind^",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Ring
    }
    fn flags1(&self) -> u64 {
        match self {
            RingTemplate::RingOfGainStrength => WornFlag1::GivesStrength as u64,
            RingTemplate::RingOfGainDexterity => WornFlag1::GivesDexterity as u64,
            RingTemplate::RingOfGainConstitution => WornFlag1::GivesConstitution as u64,
            RingTemplate::RingOfGainIntelligence => WornFlag1::GivesIntelligence as u64,
            RingTemplate::RingOfSpeed1 |
            RingTemplate::RingOfSpeed2 => WornFlag1::IncreasedSpeed as u64,
            RingTemplate::RingOfSearching => WornFlag1::Searching as u64,
            RingTemplate::RingOfTeleportation => WornFlag1::Searching as u64,
            RingTemplate::RingOfSlowDigestion => WornFlag1::SlowDigestion as u64,
            RingTemplate::RingOfResistFire => WornFlag1::ResistFire as u64,
            RingTemplate::RingOfResistCold => WornFlag1::ResistCold as u64,
            RingTemplate::RingOfFeatherFalling => WornFlag1::FeatherFall as u64,
            RingTemplate::RingOfAdornment1 => 0,
            RingTemplate::RingOfAdornment2 => 0,
            RingTemplate::RingOfWeakness => WornFlag1::Cursed as u64 | WornFlag1::GivesStrength as u64,
            RingTemplate::RingOfLordlyProtectionFire => WornFlag1::ResistFire as u64,
            RingTemplate::RingOfLordlyProtectionAcid => WornFlag1::ResistAcid as u64,
            RingTemplate::RingOfLordlyProtectionCold => WornFlag1::ResistCold as u64,
            RingTemplate::RingOfWoe => WornFlag1::Cursed as u64 |
                WornFlag1::AggravateMonsters as u64 |
                WornFlag1::RandomTeleportation as u64,
            RingTemplate::RingOfStupidity => WornFlag1::Cursed as u64 | WornFlag1::GivesIntelligence as u64,
            RingTemplate::RingOfIncreaseDamage => 0,
            RingTemplate::RingOfIncreaseToHit => 0,
            RingTemplate::RingOfProtection => 0,
            RingTemplate::RingOfAggravateMonsters => WornFlag1::Cursed as u64 |
                WornFlag1::AggravateMonsters as u64,
            RingTemplate::RingOfSeeInvisible => WornFlag1::SeeInvisible as u64,
            RingTemplate::RingOfSustainStrength |
            RingTemplate::RingOfSustainIntelligence |
            RingTemplate::RingOfSustainWisdom |
            RingTemplate::RingOfSustainConstitution |
            RingTemplate::RingOfSustainDexterity |
            RingTemplate::RingOfSustainCharisma => WornFlag1::ResistStatDrain as u64,
            RingTemplate::RingOfSlaying => 0,
            RingTemplate::RingOfGnomekind => WornFlag1::ResistStatDrain as u64 |
                WornFlag1::SlowDigestion as u64 |
                WornFlag1::GivesIntelligence as u64,
        }
    }

    fn flags2(&self) -> u64 {
        0
    }

    fn p1(&self) -> i64 {
        match self {
            RingTemplate::RingOfGainStrength => 0,
            RingTemplate::RingOfGainDexterity => 0,
            RingTemplate::RingOfGainConstitution => 0,
            RingTemplate::RingOfGainIntelligence => 0,
            RingTemplate::RingOfSpeed1 => 0,
            RingTemplate::RingOfSpeed2 => 0,
            RingTemplate::RingOfSearching => 0,
            RingTemplate::RingOfTeleportation => 0,
            RingTemplate::RingOfSlowDigestion => 0,
            RingTemplate::RingOfResistFire => 0,
            RingTemplate::RingOfResistCold => 0,
            RingTemplate::RingOfFeatherFalling => 0,
            RingTemplate::RingOfAdornment1 => 0,
            RingTemplate::RingOfAdornment2 => 0,
            RingTemplate::RingOfWeakness => -5,
            RingTemplate::RingOfLordlyProtectionFire => 0,
            RingTemplate::RingOfLordlyProtectionAcid => 0,
            RingTemplate::RingOfLordlyProtectionCold => 0,
            RingTemplate::RingOfWoe => -5,
            RingTemplate::RingOfStupidity => -5,
            RingTemplate::RingOfIncreaseDamage => 0,
            RingTemplate::RingOfIncreaseToHit => 0,
            RingTemplate::RingOfProtection => 0,
            RingTemplate::RingOfAggravateMonsters => 0,
            RingTemplate::RingOfSeeInvisible => 0,
            RingTemplate::RingOfSustainStrength => 1,
            RingTemplate::RingOfSustainIntelligence => 2,
            RingTemplate::RingOfSustainWisdom => 3,
            RingTemplate::RingOfSustainConstitution => 4,
            RingTemplate::RingOfSustainDexterity => 5,
            RingTemplate::RingOfSustainCharisma => 6,
            RingTemplate::RingOfSlaying => 6,
            RingTemplate::RingOfGnomekind => 2,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            RingTemplate::RingOfGainStrength => 400,
            RingTemplate::RingOfGainDexterity => 400,
            RingTemplate::RingOfGainConstitution => 400,
            RingTemplate::RingOfGainIntelligence => 350,
            RingTemplate::RingOfSpeed1 => 8000,
            RingTemplate::RingOfSpeed2 => 1,
            RingTemplate::RingOfSearching => 250,
            RingTemplate::RingOfTeleportation => 0,
            RingTemplate::RingOfSlowDigestion => 250,
            RingTemplate::RingOfResistFire => 250,
            RingTemplate::RingOfResistCold => 250,
            RingTemplate::RingOfFeatherFalling => 200,
            RingTemplate::RingOfAdornment1 => 20,
            RingTemplate::RingOfAdornment2 => 30,
            RingTemplate::RingOfWeakness => 0,
            RingTemplate::RingOfLordlyProtectionFire => 1200,
            RingTemplate::RingOfLordlyProtectionAcid => 1200,
            RingTemplate::RingOfLordlyProtectionCold => 1200,
            RingTemplate::RingOfWoe => 0,
            RingTemplate::RingOfStupidity => 0,
            RingTemplate::RingOfIncreaseDamage => 100,
            RingTemplate::RingOfIncreaseToHit => 100,
            RingTemplate::RingOfProtection => 100,
            RingTemplate::RingOfAggravateMonsters => 0,
            RingTemplate::RingOfSeeInvisible => 340,
            RingTemplate::RingOfSustainStrength => 750,
            RingTemplate::RingOfSustainIntelligence => 600,
            RingTemplate::RingOfSustainWisdom => 600,
            RingTemplate::RingOfSustainConstitution => 750,
            RingTemplate::RingOfSustainDexterity => 750,
            RingTemplate::RingOfSustainCharisma => 500,
            RingTemplate::RingOfSlaying => 1000,
            RingTemplate::RingOfGnomekind => 2000,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            RingTemplate::RingOfGainStrength => ItemSubType::Ring(RingSubType::RingOfGainStrength),
            RingTemplate::RingOfGainDexterity => {
                ItemSubType::Ring(RingSubType::RingOfGainDexterity)
            }
            RingTemplate::RingOfGainConstitution => {
                ItemSubType::Ring(RingSubType::RingOfGainConstitution)
            }
            RingTemplate::RingOfGainIntelligence => {
                ItemSubType::Ring(RingSubType::RingOfGainIntelligence)
            }
            RingTemplate::RingOfSpeed1 => ItemSubType::Ring(RingSubType::RingOfSpeed1),
            RingTemplate::RingOfSpeed2 => ItemSubType::Ring(RingSubType::RingOfSpeed2),
            RingTemplate::RingOfSearching => ItemSubType::Ring(RingSubType::RingOfSearching),
            RingTemplate::RingOfTeleportation => {
                ItemSubType::Ring(RingSubType::RingOfTeleportation)
            }
            RingTemplate::RingOfSlowDigestion => {
                ItemSubType::Ring(RingSubType::RingOfSlowDigestion)
            }
            RingTemplate::RingOfResistFire => ItemSubType::Ring(RingSubType::RingOfResistFire),
            RingTemplate::RingOfResistCold => ItemSubType::Ring(RingSubType::RingOfResistCold),
            RingTemplate::RingOfFeatherFalling => {
                ItemSubType::Ring(RingSubType::RingOfFeatherFalling)
            }
            RingTemplate::RingOfAdornment1 => ItemSubType::Ring(RingSubType::RingOfAdornment1),
            RingTemplate::RingOfAdornment2 => ItemSubType::Ring(RingSubType::RingOfAdornment2),
            RingTemplate::RingOfWeakness => ItemSubType::Ring(RingSubType::RingOfWeakness),
            RingTemplate::RingOfLordlyProtectionFire => {
                ItemSubType::Ring(RingSubType::RingOfLordlyProtectionFire)
            }
            RingTemplate::RingOfLordlyProtectionAcid => {
                ItemSubType::Ring(RingSubType::RingOfLordlyProtectionAcid)
            }
            RingTemplate::RingOfLordlyProtectionCold => {
                ItemSubType::Ring(RingSubType::RingOfLordlyProtectionCold)
            }
            RingTemplate::RingOfWoe => ItemSubType::Ring(RingSubType::RingOfWoe),
            RingTemplate::RingOfStupidity => ItemSubType::Ring(RingSubType::RingOfStupidity),
            RingTemplate::RingOfIncreaseDamage => {
                ItemSubType::Ring(RingSubType::RingOfIncreaseDamage)
            }
            RingTemplate::RingOfIncreaseToHit => {
                ItemSubType::Ring(RingSubType::RingOfIncreaseToHit)
            }
            RingTemplate::RingOfProtection => ItemSubType::Ring(RingSubType::RingOfProtection),
            RingTemplate::RingOfAggravateMonsters => {
                ItemSubType::Ring(RingSubType::RingOfAggravateMonsters)
            }
            RingTemplate::RingOfSeeInvisible => ItemSubType::Ring(RingSubType::RingOfSeeInvisible),
            RingTemplate::RingOfSustainStrength => {
                ItemSubType::Ring(RingSubType::RingOfSustainStrength)
            }
            RingTemplate::RingOfSustainIntelligence => {
                ItemSubType::Ring(RingSubType::RingOfSustainIntelligence)
            }
            RingTemplate::RingOfSustainWisdom => {
                ItemSubType::Ring(RingSubType::RingOfSustainWisdom)
            }
            RingTemplate::RingOfSustainConstitution => {
                ItemSubType::Ring(RingSubType::RingOfSustainConstitution)
            }
            RingTemplate::RingOfSustainDexterity => {
                ItemSubType::Ring(RingSubType::RingOfSustainDexterity)
            }
            RingTemplate::RingOfSustainCharisma => {
                ItemSubType::Ring(RingSubType::RingOfSustainCharisma)
            }
            RingTemplate::RingOfSlaying => ItemSubType::Ring(RingSubType::RingOfSlaying),
            RingTemplate::RingOfGnomekind => ItemSubType::Ring(RingSubType::RingOfGnomekind),
        }
    }

    fn weight(&self) -> u16 {
        2
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
        match self {
            RingTemplate::RingOfGainStrength => 0,
            RingTemplate::RingOfGainDexterity => 0,
            RingTemplate::RingOfGainConstitution => 0,
            RingTemplate::RingOfGainIntelligence => 0,
            RingTemplate::RingOfSpeed1 => 0,
            RingTemplate::RingOfSpeed2 => 0,
            RingTemplate::RingOfSearching => 0,
            RingTemplate::RingOfTeleportation => 0,
            RingTemplate::RingOfSlowDigestion => 0,
            RingTemplate::RingOfResistFire => 0,
            RingTemplate::RingOfResistCold => 0,
            RingTemplate::RingOfFeatherFalling => 0,
            RingTemplate::RingOfAdornment1 => 0,
            RingTemplate::RingOfAdornment2 => 0,
            RingTemplate::RingOfWeakness => 0,
            RingTemplate::RingOfLordlyProtectionFire => 5,
            RingTemplate::RingOfLordlyProtectionAcid => 5,
            RingTemplate::RingOfLordlyProtectionCold => 5,
            RingTemplate::RingOfWoe => -3,
            RingTemplate::RingOfStupidity => 0,
            RingTemplate::RingOfIncreaseDamage => 0,
            RingTemplate::RingOfIncreaseToHit => 0,
            RingTemplate::RingOfProtection => 0,
            RingTemplate::RingOfAggravateMonsters => 0,
            RingTemplate::RingOfSeeInvisible => 0,
            RingTemplate::RingOfSustainStrength => 0,
            RingTemplate::RingOfSustainIntelligence => 0,
            RingTemplate::RingOfSustainWisdom => 0,
            RingTemplate::RingOfSustainConstitution => 0,
            RingTemplate::RingOfSustainDexterity => 0,
            RingTemplate::RingOfSustainCharisma => 0,
            RingTemplate::RingOfSlaying => 0,
            RingTemplate::RingOfGnomekind => 0,
        }
    }

    fn damage(&self) -> &str {
        "1d1"
    }

    fn item_level(&self) -> u8 {
        match self {
            RingTemplate::RingOfGainStrength => 30,
            RingTemplate::RingOfGainDexterity => 30,
            RingTemplate::RingOfGainConstitution => 30,
            RingTemplate::RingOfGainIntelligence => 30,
            RingTemplate::RingOfSpeed1 => 50,
            RingTemplate::RingOfSpeed2 => 5,
            RingTemplate::RingOfSearching => 7,
            RingTemplate::RingOfTeleportation => 7,
            RingTemplate::RingOfSlowDigestion => 7,
            RingTemplate::RingOfResistFire => 14,
            RingTemplate::RingOfResistCold => 14,
            RingTemplate::RingOfFeatherFalling => 7,
            RingTemplate::RingOfAdornment1 => 7,
            RingTemplate::RingOfAdornment2 => 7,
            RingTemplate::RingOfWeakness => 7,
            RingTemplate::RingOfLordlyProtectionFire => 50,
            RingTemplate::RingOfLordlyProtectionAcid => 50,
            RingTemplate::RingOfLordlyProtectionCold => 50,
            RingTemplate::RingOfWoe => 50,
            RingTemplate::RingOfStupidity => 20,
            RingTemplate::RingOfIncreaseDamage => 20,
            RingTemplate::RingOfIncreaseToHit => 20,
            RingTemplate::RingOfProtection => 7,
            RingTemplate::RingOfAggravateMonsters => 7,
            RingTemplate::RingOfSeeInvisible => 40,
            RingTemplate::RingOfSustainStrength => 44,
            RingTemplate::RingOfSustainIntelligence => 44,
            RingTemplate::RingOfSustainWisdom => 44,
            RingTemplate::RingOfSustainConstitution => 44,
            RingTemplate::RingOfSustainDexterity => 44,
            RingTemplate::RingOfSustainCharisma => 7,
            RingTemplate::RingOfSlaying => 50,
            RingTemplate::RingOfGnomekind => 40,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
