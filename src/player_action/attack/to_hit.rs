use crate::{
    data::class::{self},
    model::{Class, ItemType, PlayerFlags},
    player_action::attack::MeleeAttackType,
};

/// Pure inputs for `calculate_player_tohit_melee_pure`.
#[derive(Debug, Clone)]
pub(crate) struct ToHitMeleeInputs {
    pub attack_type: MeleeAttackType,
    pub maybe_item_type: Option<ItemType>,
    pub class: Class,
    pub level: u8,
    pub dex_modifier: i16,
    pub strength_modifier: i16,
    pub player_flags: PlayerFlags,
    pub bonus_from_equipment: i16,
}

/// Pure inputs for `calculate_player_tohit_ranged_pure`.
#[derive(Debug, Clone)]
pub(crate) struct ToHitRangedInputs {
    pub maybe_item_type: Option<ItemType>,
    pub class: Class,
    pub level: u8,
    pub dex_modifier: i16,
    pub strength_modifier: i16,
    pub player_flags: PlayerFlags,
    pub bonus_from_equipment: i16,
}

// TODO Racial bonus
pub(crate) fn calculate_player_tohit_melee_pure(inputs: &ToHitMeleeInputs) -> i16 {
    // [0-40]
    let from_class_and_level =
        ((class::melee_bonus(&inputs.class) as i16 * inputs.level as i16) / 10) as i16;

    let from_stats = inputs.dex_modifier + inputs.strength_modifier;

    let from_backstab = match inputs.attack_type {
        MeleeAttackType::Standard => 0,
        MeleeAttackType::Backstab => {
            // +1 to hit for every 4 levels of backstab, starting at level 4
            inputs.level as i16 / 4
        }
    };

    let mut non_proficiency_penalty = 0;
    if !class::is_proficient_with_weapon(&inputs.class, inputs.maybe_item_type) {
        non_proficiency_penalty = -5;
    }

    let mut from_flags = 0;
    if inputs.player_flags.shero > 0 {
        from_flags += 24;
    }
    if inputs.player_flags.hero > 0 {
        from_flags += 12;
    }
    if inputs.player_flags.blessed > 0 {
        from_flags += 5;
    }
    from_class_and_level
        + from_stats
        + from_backstab
        + non_proficiency_penalty
        + from_flags
        + inputs.bonus_from_equipment
}

// TODO Racial bonus
// TODO Missile .tohit should be added
pub(crate) fn calculate_player_tohit_ranged_pure(inputs: &ToHitRangedInputs) -> i16 {
    // [0-40]
    let from_class_and_level =
        ((class::ranged_bonus(&inputs.class) as i16 * inputs.level as i16) / 10) as i16;

    let from_stats = inputs.dex_modifier + inputs.strength_modifier;

    let mut non_proficiency_penalty = 0;
    if !class::is_proficient_with_weapon(&inputs.class, inputs.maybe_item_type) {
        non_proficiency_penalty = -5;
    }

    let mut from_flags = 0;
    if inputs.player_flags.shero > 0 {
        from_flags += 24;
    }
    if inputs.player_flags.hero > 0 {
        from_flags += 12;
    }
    if inputs.player_flags.blessed > 0 {
        from_flags += 5;
    }
    from_class_and_level
        + from_stats
        + non_proficiency_penalty
        + from_flags
        + inputs.bonus_from_equipment
}

// TODO Racial bonus
// TODO: should be affected by weight?
// TODO: proficiency seems off
pub(crate) fn calculate_player_tohit_thrown_pure(inputs: &ToHitRangedInputs) -> i16 {
    // [0-40]
    let from_class_and_level =
        ((class::ranged_bonus(&inputs.class) as i16 * inputs.level as i16) / 10) as i16;

    let from_stats = inputs.dex_modifier + inputs.strength_modifier;

    let mut non_proficiency_penalty = 0;
    if !class::is_proficient_with_weapon(&inputs.class, inputs.maybe_item_type) {
        non_proficiency_penalty = -5;
    }

    let mut from_flags = 0;
    if inputs.player_flags.shero > 0 {
        from_flags += 24;
    }
    if inputs.player_flags.hero > 0 {
        from_flags += 12;
    }
    if inputs.player_flags.blessed > 0 {
        from_flags += 5;
    }
    from_class_and_level
        + from_stats
        + non_proficiency_penalty
        + from_flags
        + inputs.bonus_from_equipment
}

#[cfg(test)]
mod melee_tests {
    use super::*;

    /// Helper: build inputs with the given class and level, zeroing dex/str modifiers
    /// and using Standard attack type, so the test isolates class+level contribution.
    fn inputs(class: Class, level: u8, equipment: i16) -> ToHitMeleeInputs {
        ToHitMeleeInputs {
            attack_type: MeleeAttackType::Standard,
            maybe_item_type: Some(ItemType::Pick),
            class,
            level,
            dex_modifier: 0,
            strength_modifier: 0,
            player_flags: PlayerFlags::default(),
            bonus_from_equipment: equipment,
        }
    }

    #[test]
    fn from_proficiency() {
        let mut inputs = inputs(Class::Wizard, 1, 0);
        assert_eq!(calculate_player_tohit_melee_pure(&inputs), 0); // proficient with dagger, so no penalty

        inputs.maybe_item_type = Some(ItemType::Sword); // not proficient with sword
        assert_eq!(calculate_player_tohit_melee_pure(&inputs), -5); // -5 for non-proficiency
    }

    #[test]
    fn from_backstab() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        inputs.attack_type = MeleeAttackType::Backstab;
        let cases = [
            (1, 1),
            (4, 5),   // +1 from backstab at level 4
            (8, 10),  // +2 from backstab at level 8
            (12, 15), // +3 from backstab at level 12
            (16, 20), // +4 from backstab at level 16
            (20, 25), // +5 from backstab at level 20
            (24, 30), // +6 from backstab at level 24
            (28, 35), // +7 from backstab at level 28
            (32, 40), // +8 from backstab at level 32
            (36, 45), // +9 from backstab at level 36
            (40, 50), // +10 from backstab at level 40
        ];
        for (level, expected) in cases.iter() {
            inputs.attack_type = MeleeAttackType::Backstab;
            inputs.level = *level;
            assert_eq!(calculate_player_tohit_melee_pure(&inputs), *expected);
        }
    }

    #[test]
    fn from_equipment() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        assert_eq!(calculate_player_tohit_melee_pure(&inputs), 1); // no equipment bonus

        inputs.bonus_from_equipment = 3; // e.g. from a +3 weapon
        assert_eq!(calculate_player_tohit_melee_pure(&inputs), 4); // equipment bonus contributes to to-hit
    }

    #[test]
    fn from_stats() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        assert_eq!(calculate_player_tohit_melee_pure(&inputs), 1); // no modifiers

        inputs.dex_modifier = 4; // e.g. from 18 dex
        assert_eq!(calculate_player_tohit_melee_pure(&inputs), 5); // dex contributes to to-hit

        inputs.strength_modifier = 3; // e.g. from 16 strength
        assert_eq!(calculate_player_tohit_melee_pure(&inputs), 8); // strength contributes to to-hit
    }

    #[test]
    fn from_player_flags() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        assert_eq!(calculate_player_tohit_melee_pure(&inputs), 1); // no flags

        inputs.player_flags.hero = 1;
        assert_eq!(calculate_player_tohit_melee_pure(&inputs), 13); // +12 from hero

        inputs.player_flags.shero = 1;
        assert_eq!(calculate_player_tohit_melee_pure(&inputs), 37); // +24 from shero (stacks with hero)

        inputs.player_flags.blessed = 1;
        assert_eq!(calculate_player_tohit_melee_pure(&inputs), 42); // +5 from blessed (stacks with hero and shero)
    }

    #[test]
    fn from_class_and_level() {
        // Each entry: (class, level, expected_tohit)
        //
        // Formula: (melee_bonus * level as i16) / 10   (i16 integer division)
        //
        // melee_bonus values:
        //   Fighter=10, Wizard=4, Cleric=6, Rogue=6, Ranger=6,
        //   Paladin=8, Druid=4, Bard=5, Adventurer=6, Monk=8, Barbarian=10
        let cases: &[(Class, u8, i16)] = &[
            // --- level 1 ---
            (Class::Fighter, 1, 1), // 10, Fighter / Barbarian -> 1
            (Class::Paladin, 1, 0), // 8, Paladin / Monk -> 0
            (Class::Cleric, 1, 0),  // 6, Cleric / Adventurer / Rogue / Ranger -> 0
            (Class::Bard, 1, 0),    // 5, Bard -> 0
            (Class::Wizard, 1, 0),  // 4, Wizard / Druid -> 0
            // --- level 5 ---
            (Class::Fighter, 5, 5), // 10, Fighter / Barbarian -> 5
            (Class::Monk, 5, 4),    // 8, Paladin / Monk -> 4
            (Class::Cleric, 5, 3),  // 6, Cleric / Adventurer / Rogue / Ranger -> 3
            (Class::Bard, 5, 2),    // 5, Bard -> 2
            (Class::Wizard, 5, 2),  // 4, Wizard / Druid -> 2
            // --- level 10 ---
            (Class::Fighter, 10, 10),   // 10*10/10 = 10
            (Class::Barbarian, 10, 10), // 10*10/10 = 10
            (Class::Paladin, 10, 8),    //  8*10/10 = 8
            (Class::Monk, 10, 8),       //  8*10/10 = 8
            (Class::Cleric, 10, 6),     //  6*10/10 = 6
            (Class::Rogue, 10, 6),      //  6*10/10 = 6
            (Class::Ranger, 10, 6),     //  6*10/10 = 6
            (Class::Adventurer, 10, 6), //  6*10/10 = 6
            (Class::Bard, 10, 5),       //  5*10/10 = 5
            (Class::Wizard, 10, 4),     //  4*10/10 = 4
            (Class::Druid, 10, 4),      //  4*10/10 = 4
            // --- level 40 ---
            (Class::Fighter, 40, 40), // 10, Fighter / Barbarian -> 40
            (Class::Monk, 40, 32),    // 8, Paladin / Monk -> 32
            (Class::Cleric, 40, 24),  // 6, Cleric / Adventurer / Rogue / Ranger -> 24
            (Class::Bard, 40, 20),    // 5, Bard -> 20
            (Class::Wizard, 40, 16),  // 4, Wizard / Druid -> 16
        ];

        for (class, level, expected) in cases {
            let result = calculate_player_tohit_melee_pure(&inputs(*class, *level, 0));
            assert_eq!(
                result, *expected,
                "{:?} at level {}: expected {}, got {}",
                class, level, expected, result
            );
        }
    }
}

#[cfg(test)]
mod ranged_tests {
    use super::*;

    /// Helper: build inputs with the given class and level, zeroing dex/str modifiers
    /// and using Standard attack type, so the test isolates class+level contribution.
    fn inputs(class: Class, level: u8, equipment: i16) -> ToHitRangedInputs {
        ToHitRangedInputs {
            maybe_item_type: Some(ItemType::RangedWeapon),
            class,
            level,
            dex_modifier: 0,
            strength_modifier: 0,
            player_flags: PlayerFlags::default(),
            bonus_from_equipment: equipment,
        }
    }

    #[test]
    fn from_proficiency() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        assert_eq!(calculate_player_tohit_ranged_pure(&inputs), 1); // proficient with bows, so no penalty

        inputs.class = Class::Wizard; // not proficient with bows
        assert_eq!(calculate_player_tohit_ranged_pure(&inputs), -5); // -5 for non-proficiency
    }

    #[test]
    fn from_equipment() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        assert_eq!(calculate_player_tohit_ranged_pure(&inputs), 1); // no equipment bonus

        inputs.bonus_from_equipment = 3; // e.g. from a +3 weapon
        assert_eq!(calculate_player_tohit_ranged_pure(&inputs), 4); // equipment bonus contributes to to-hit
    }

    #[test]
    fn from_stats() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        assert_eq!(calculate_player_tohit_ranged_pure(&inputs), 1); // no modifiers

        inputs.dex_modifier = 4; // e.g. from 18 dex
        assert_eq!(calculate_player_tohit_ranged_pure(&inputs), 5); // dex contributes to to-hit

        inputs.strength_modifier = 3; // e.g. from 16 strength
        assert_eq!(calculate_player_tohit_ranged_pure(&inputs), 8); // strength contributes to to-hit
    }

    #[test]
    fn from_player_flags() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        assert_eq!(calculate_player_tohit_ranged_pure(&inputs), 1); // no flags

        inputs.player_flags.hero = 1;
        assert_eq!(calculate_player_tohit_ranged_pure(&inputs), 13); // +12 from hero

        inputs.player_flags.shero = 1;
        assert_eq!(calculate_player_tohit_ranged_pure(&inputs), 37); // +24 from shero (stacks with hero)

        inputs.player_flags.blessed = 1;
        assert_eq!(calculate_player_tohit_ranged_pure(&inputs), 42); // +5 from blessed (stacks with hero and shero)
    }

    #[test]
    fn from_class_and_level() {
        // Each entry: (class, level, expected_tohit)
        //
        // Formula: (ranged_bonus * level as i16) / 10   (i16 integer division)
        //
        let cases: &[(Class, u8, i16)] = &[
            // --- level 1 ---
            (Class::Fighter, 1, 1), // 10, Fighter / Barbarian / Ranger / Rogue
            (Class::Druid, 1, 0),   // 7, Druid
            (Class::Paladin, 1, 0), // 6, Paladin / Bard / Adventurer / Monk
            (Class::Cleric, 1, 0 - 5), // 5, Cleric
            (Class::Wizard, 1, 0 - 5), // 4, Wizard
            // --- level 5 ---
            (Class::Fighter, 5, 5),
            (Class::Druid, 5, 3),
            (Class::Paladin, 5, 3),
            (Class::Cleric, 5, 2 - 5),
            (Class::Wizard, 5, 2 - 5),
            // --- level 10 ---
            (Class::Fighter, 10, 10),
            (Class::Barbarian, 10, 10),
            (Class::Rogue, 10, 10),
            (Class::Ranger, 10, 10),
            (Class::Druid, 10, 7),
            (Class::Paladin, 10, 6),
            (Class::Monk, 10, 6 - 5),
            (Class::Adventurer, 10, 6),
            (Class::Bard, 10, 6),
            (Class::Cleric, 10, 5 - 5),
            (Class::Wizard, 10, 4 - 5),
            // --- level 40 ---
            (Class::Fighter, 40, 40),
            (Class::Druid, 40, 28),
            (Class::Paladin, 40, 24),
            (Class::Cleric, 40, 20 - 5),
            (Class::Wizard, 40, 16 - 5),
        ];

        for (class, level, expected) in cases {
            let result = calculate_player_tohit_ranged_pure(&inputs(*class, *level, 0));
            assert_eq!(
                result, *expected,
                "{:?} at level {}: expected {}, got {}",
                class, level, expected, result
            );
        }
    }
}

#[cfg(test)]
mod thrown_tests {
    use super::*;

    /// Helper: build inputs with the given class and level, zeroing dex/str modifiers
    /// and using Standard attack type, so the test isolates class+level contribution.
    fn inputs(class: Class, level: u8, equipment: i16) -> ToHitRangedInputs {
        ToHitRangedInputs {
            maybe_item_type: Some(ItemType::Pick),
            class,
            level,
            dex_modifier: 0,
            strength_modifier: 0,
            player_flags: PlayerFlags::default(),
            bonus_from_equipment: equipment,
        }
    }

    #[test]
    fn from_proficiency() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        assert_eq!(calculate_player_tohit_thrown_pure(&inputs), 1); // proficient with, so no penalty

        inputs.class = Class::Wizard; // not proficient with swords
        inputs.maybe_item_type = Some(ItemType::Sword);
        assert_eq!(calculate_player_tohit_thrown_pure(&inputs), -5); // -5 for non-proficiency
    }

    #[test]
    fn from_equipment() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        assert_eq!(calculate_player_tohit_thrown_pure(&inputs), 1); // no equipment bonus

        inputs.bonus_from_equipment = 3; // e.g. from a +3 weapon
        assert_eq!(calculate_player_tohit_thrown_pure(&inputs), 4); // equipment bonus contributes to to-hit
    }

    #[test]
    fn from_stats() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        assert_eq!(calculate_player_tohit_thrown_pure(&inputs), 1); // no modifiers

        inputs.dex_modifier = 4; // e.g. from 18 dex
        assert_eq!(calculate_player_tohit_thrown_pure(&inputs), 5); // dex contributes to to-hit

        inputs.strength_modifier = 3; // e.g. from 16 strength
        assert_eq!(calculate_player_tohit_thrown_pure(&inputs), 8); // strength contributes to to-hit
    }

    #[test]
    fn from_player_flags() {
        let mut inputs = inputs(Class::Fighter, 1, 0);
        assert_eq!(calculate_player_tohit_thrown_pure(&inputs), 1); // no flags

        inputs.player_flags.hero = 1;
        assert_eq!(calculate_player_tohit_thrown_pure(&inputs), 13); // +12 from hero

        inputs.player_flags.shero = 1;
        assert_eq!(calculate_player_tohit_thrown_pure(&inputs), 37); // +24 from shero (stacks with hero)

        inputs.player_flags.blessed = 1;
        assert_eq!(calculate_player_tohit_thrown_pure(&inputs), 42); // +5 from blessed (stacks with hero and shero)
    }

    #[test]
    fn from_class_and_level() {
        // Each entry: (class, level, expected_tohit)
        //
        // Formula: (ranged_bonus * level as i16) / 10   (i16 integer division)
        //
        let cases: &[(Class, u8, i16)] = &[
            // --- level 1 ---
            (Class::Fighter, 1, 1), // 10, Fighter / Barbarian / Ranger / Rogue
            (Class::Druid, 1, 0),   // 7, Druid
            (Class::Paladin, 1, 0), // 6, Paladin / Bard / Adventurer / Monk
            (Class::Cleric, 1, 0),  // 5, Cleric
            (Class::Wizard, 1, 0),  // 4, Wizard
            // --- level 5 ---
            (Class::Fighter, 5, 5),
            (Class::Druid, 5, 3),
            (Class::Paladin, 5, 3),
            (Class::Cleric, 5, 2),
            (Class::Wizard, 5, 2),
            // --- level 10 ---
            (Class::Fighter, 10, 10),
            (Class::Barbarian, 10, 10),
            (Class::Rogue, 10, 10),
            (Class::Ranger, 10, 10),
            (Class::Druid, 10, 7),
            (Class::Paladin, 10, 6),
            (Class::Monk, 10, 6),
            (Class::Adventurer, 10, 6),
            (Class::Bard, 10, 6),
            (Class::Cleric, 10, 5),
            (Class::Wizard, 10, 4),
            // --- level 40 ---
            (Class::Fighter, 40, 40),
            (Class::Druid, 40, 28),
            (Class::Paladin, 40, 24),
            (Class::Cleric, 40, 20),
            (Class::Wizard, 40, 16),
        ];

        for (class, level, expected) in cases {
            let result = calculate_player_tohit_thrown_pure(&inputs(*class, *level, 0));
            assert_eq!(
                result, *expected,
                "{:?} at level {}: expected {}, got {}",
                class, level, expected, result
            );
        }
    }
}
