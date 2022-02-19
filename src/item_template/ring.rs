use std::borrow::Cow;

use model;
use item_template;
use logic::item_name;

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
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
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

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        RingTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(RingTemplate::RingOfGainStrength),
            2 => Box::new(RingTemplate::RingOfGainDexterity),
            3 => Box::new(RingTemplate::RingOfGainConstitution),
            4 => Box::new(RingTemplate::RingOfGainIntelligence),
            7 => Box::new(RingTemplate::RingOfSpeed1),
            35 => Box::new(RingTemplate::RingOfSpeed2),
            8 => Box::new(RingTemplate::RingOfSearching),
            9 => Box::new(RingTemplate::RingOfTeleportation),
            10 => Box::new(RingTemplate::RingOfSlowDigestion),
            11 => Box::new(RingTemplate::RingOfResistFire),
            12 => Box::new(RingTemplate::RingOfResistCold),
            13 => Box::new(RingTemplate::RingOfFeatherFalling),
            14 => Box::new(RingTemplate::RingOfAdornment1),
            15 => Box::new(RingTemplate::RingOfAdornment2),
            16 => Box::new(RingTemplate::RingOfWeakness),
            17 => Box::new(RingTemplate::RingOfLordlyProtectionFire),
            18 => Box::new(RingTemplate::RingOfLordlyProtectionAcid),
            19 => Box::new(RingTemplate::RingOfLordlyProtectionCold),
            20 => Box::new(RingTemplate::RingOfWoe),
            21 => Box::new(RingTemplate::RingOfStupidity),
            22 => Box::new(RingTemplate::RingOfIncreaseDamage),
            23 => Box::new(RingTemplate::RingOfIncreaseToHit),
            24 => Box::new(RingTemplate::RingOfProtection),
            25 => Box::new(RingTemplate::RingOfAggravateMonsters),
            26 => Box::new(RingTemplate::RingOfSeeInvisible),
            27 => Box::new(RingTemplate::RingOfSustainStrength),
            28 => Box::new(RingTemplate::RingOfSustainIntelligence),
            29 => Box::new(RingTemplate::RingOfSustainWisdom),
            30 => Box::new(RingTemplate::RingOfSustainConstitution),
            31 => Box::new(RingTemplate::RingOfSustainDexterity),
            32 => Box::new(RingTemplate::RingOfSustainCharisma),
            33 => Box::new(RingTemplate::RingOfSlaying),
            34 => Box::new(RingTemplate::RingOfGnomekind),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for RingTemplate {
    fn name(&self, item: &model::Item) -> String {
        let mut parts = Vec::new();
        parts.push(Cow::from("Ring"));
        if self.is_identified() {
            parts.push(
                Cow::from(match self {
                    RingTemplate::RingOfGainStrength => " of Gain Strength",
                    RingTemplate::RingOfGainDexterity => " of Gain Dexterity",
                    RingTemplate::RingOfGainConstitution => " of Gain Constitution",
                    RingTemplate::RingOfGainIntelligence => " of Gain Intelligence",
                    RingTemplate::RingOfSpeed1 => " of Speed",
                    RingTemplate::RingOfSpeed2 => " of Speed",
                    RingTemplate::RingOfSearching => " of Searching",
                    RingTemplate::RingOfTeleportation => " of Teleportation",
                    RingTemplate::RingOfSlowDigestion => " of Slow Digestion",
                    RingTemplate::RingOfResistFire => " of Resist Fire",
                    RingTemplate::RingOfResistCold => " of Resist Cold",
                    RingTemplate::RingOfFeatherFalling => " of Feather Falling",
                    RingTemplate::RingOfAdornment1 => " of Adornment",
                    RingTemplate::RingOfAdornment2 => " of Adornment",
                    RingTemplate::RingOfWeakness => " of Weakness",
                    RingTemplate::RingOfLordlyProtectionFire => " of Lordly Protection (Fire)",
                    RingTemplate::RingOfLordlyProtectionAcid => " of Lordly Protection (Acid)",
                    RingTemplate::RingOfLordlyProtectionCold => " of Lordly Protection (Cold)",
                    RingTemplate::RingOfWoe => " of Woe",
                    RingTemplate::RingOfStupidity => " of Stupidity",
                    RingTemplate::RingOfIncreaseDamage => " of Increase Damage",
                    RingTemplate::RingOfIncreaseToHit => " of Increase To-hit",
                    RingTemplate::RingOfProtection => " of Protection",
                    RingTemplate::RingOfAggravateMonsters => " of Aggravate Monster",
                    RingTemplate::RingOfSeeInvisible => " of See Invisible",
                    RingTemplate::RingOfSustainStrength => " of Sustain Strength",
                    RingTemplate::RingOfSustainIntelligence => " of Sustain Intelligence",
                    RingTemplate::RingOfSustainWisdom => " of Sustain Wisdom",
                    RingTemplate::RingOfSustainConstitution => " of Sustain Constitution",
                    RingTemplate::RingOfSustainDexterity => " of Sustain Dexterity",
                    RingTemplate::RingOfSustainCharisma => " of Sustain Charisma",
                    RingTemplate::RingOfSlaying => " of Slaying",
                    RingTemplate::RingOfGnomekind => " of Gnomekind",
                }));
        }
        if item.p1 != 0 {
            parts.push(item_name::p1(&item));
        }
        if item.tohit != 0 {
            parts.push(item_name::to_hit(&item));
        }
        if item.todam != 0 {
            parts.push(item_name::to_damage(&item));
        }
        if item.toac != 0 {
            parts.push(item_name::to_ac(&item));
        }
        parts.join("")
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Ring }
    fn flags1(&self) -> u64 { 0 }

    fn flags2(&self) -> u64 {
        match self {
            RingTemplate::RingOfGainStrength => 0x00000001,
            RingTemplate::RingOfGainDexterity => 0x00000002,
            RingTemplate::RingOfGainConstitution => 0x00000004,
            RingTemplate::RingOfGainIntelligence => 0x00000008,
            RingTemplate::RingOfSpeed1 => 0x00001000,
            RingTemplate::RingOfSpeed2 => 0x00001000,
            RingTemplate::RingOfSearching => 0x00000040,
            RingTemplate::RingOfTeleportation => 0x80000400,
            RingTemplate::RingOfSlowDigestion => 0x00000080,
            RingTemplate::RingOfResistFire => 0x00080000,
            RingTemplate::RingOfResistCold => 0x00200000,
            RingTemplate::RingOfFeatherFalling => 0x04000000,
            RingTemplate::RingOfAdornment1 => 0x00000000,
            RingTemplate::RingOfAdornment2 => 0x00000000,
            RingTemplate::RingOfWeakness => 0x80000001,
            RingTemplate::RingOfLordlyProtectionFire => 0x00080000,
            RingTemplate::RingOfLordlyProtectionAcid => 0x00100000,
            RingTemplate::RingOfLordlyProtectionCold => 0x00200000,
            RingTemplate::RingOfWoe => 0x80000600,
            RingTemplate::RingOfStupidity => 0x80000008,
            RingTemplate::RingOfIncreaseDamage => 0x00000000,
            RingTemplate::RingOfIncreaseToHit => 0x00000000,
            RingTemplate::RingOfProtection => 0x00000000,
            RingTemplate::RingOfAggravateMonsters => 0x80000200,
            RingTemplate::RingOfSeeInvisible => 0x01000000,
            RingTemplate::RingOfSustainStrength => 0x00400000,
            RingTemplate::RingOfSustainIntelligence => 0x00400000,
            RingTemplate::RingOfSustainWisdom => 0x00400000,
            RingTemplate::RingOfSustainConstitution => 0x00400000,
            RingTemplate::RingOfSustainDexterity => 0x00400000,
            RingTemplate::RingOfSustainCharisma => 0x00400000,
            RingTemplate::RingOfSlaying => 0x00000000,
            RingTemplate::RingOfGnomekind => 0x00400088,
        }
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

    fn subtype(&self) -> i64 {
        match self {
            RingTemplate::RingOfGainStrength => 1,
            RingTemplate::RingOfGainDexterity => 2,
            RingTemplate::RingOfGainConstitution => 3,
            RingTemplate::RingOfGainIntelligence => 4,
            RingTemplate::RingOfSpeed1 => 7,
            RingTemplate::RingOfSpeed2 => 35,
            RingTemplate::RingOfSearching => 8,
            RingTemplate::RingOfTeleportation => 9,
            RingTemplate::RingOfSlowDigestion => 10,
            RingTemplate::RingOfResistFire => 11,
            RingTemplate::RingOfResistCold => 12,
            RingTemplate::RingOfFeatherFalling => 13,
            RingTemplate::RingOfAdornment1 => 14,
            RingTemplate::RingOfAdornment2 => 15,
            RingTemplate::RingOfWeakness => 16,
            RingTemplate::RingOfLordlyProtectionFire => 17,
            RingTemplate::RingOfLordlyProtectionAcid => 18,
            RingTemplate::RingOfLordlyProtectionCold => 19,
            RingTemplate::RingOfWoe => 20,
            RingTemplate::RingOfStupidity => 21,
            RingTemplate::RingOfIncreaseDamage => 22,
            RingTemplate::RingOfIncreaseToHit => 23,
            RingTemplate::RingOfProtection => 24,
            RingTemplate::RingOfAggravateMonsters => 25,
            RingTemplate::RingOfSeeInvisible => 26,
            RingTemplate::RingOfSustainStrength => 27,
            RingTemplate::RingOfSustainIntelligence => 28,
            RingTemplate::RingOfSustainWisdom => 29,
            RingTemplate::RingOfSustainConstitution => 30,
            RingTemplate::RingOfSustainDexterity => 31,
            RingTemplate::RingOfSustainCharisma => 32,
            RingTemplate::RingOfSlaying => 33,
            RingTemplate::RingOfGnomekind => 34,
        }
    }

    fn weight(&self) -> u16 { 2 }
    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }

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

    fn is_identified(&self) -> bool { false }
}
