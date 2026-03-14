use crate::{data::class, model::Class};

/// Describes the weapon the player is holding for attack-count purposes.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum WeaponState {
    /// No weapon equipped (tval == 0).
    BareHands,
    /// Holding ammo (sling ammo, bolt, arrow) — not a melee weapon.
    Ammo,
    /// A real weapon with the given weight.
    Weapon { weight: i16 },
}

/// Pure inputs for `calculate_number_of_attacks_pure`.
#[derive(Debug, Clone)]
pub(crate) struct NumberOfAttacksInputs {
    pub weapon: WeaponState,
    pub class: Class,
    pub max_wield: i16,
    pub level: u8,
    pub dex_modifier: i16,
    pub strength_modifier: i16,
}

/// Pure, testable version of `calculate_number_of_attacks`.
pub(crate) fn calculate_number_of_attacks_pure(inputs: &NumberOfAttacksInputs) -> i16 {
    if inputs.weapon == WeaponState::Ammo {
        return 1;
    } else if inputs.weapon == WeaponState::BareHands && inputs.class != Class::Monk {
        return 2;
    }
    // [0-1] attacks
    let attacks_from_class = match inputs.weapon {
        WeaponState::BareHands => 1, // monks get a bonus attack when unarmed
        _ => 0
    };

    // [0-2] attacks, see attacks_from_level test for details
    let attacks_from_level =
        (class::melee_bonus(&inputs.class) as i16 * (inputs.level + 10) as i16) / 250;

    // [1-350]
    let weapon_weight = match inputs.weapon {
        WeaponState::Weapon { weight } => weight,
        WeaponState::BareHands => 5, // arbitrary small weight for unarmed combat
        WeaponState::Ammo => unreachable!(),
    };

    // This if-case is probably redundant since we compare bulk with weapon weight.
    // Feel free to remove it to tidy up the logic
    if inputs.max_wield < weapon_weight {
        return 0;
    }

    // [160-600] avg 350
    let strength_capacity = (10 + inputs.strength_modifier * 2) * 30;
    // For each extra attack from dex you need to be an additional 1x over strength capacity to use it
    let attacks_from_dexterity_limit = strength_capacity / weapon_weight;
    // [1-4] attacks, 1 attack per 3 points of dex modifier
    let initial_attacks_from_dexterity = (5 + inputs.dex_modifier).clamp(3, 12) / 3;
    let attacks_from_dexterity = initial_attacks_from_dexterity.min(attacks_from_dexterity_limit);

    // Total [1-8] attacks
    attacks_from_class + attacks_from_level + attacks_from_dexterity
}


#[cfg(test)]
mod tests {
    use crate::generate_item::{ItemTemplate, template::{DaggerTemplate, MaceTemplate, PolearmTemplate, SwordTemplate}};

    use super::*;

    fn make_inputs(weapon: WeaponState, class: Class, level: u8, dex_mod: i16, str_mod: i16) -> NumberOfAttacksInputs {
        // max_wield large enough to not be a bottleneck by default
        NumberOfAttacksInputs {
            weapon,
            class,
            max_wield: 500,
            level,
            dex_modifier: dex_mod,
            strength_modifier: str_mod,
        }
    }

    #[test]
    fn bare_hands_non_monk_always_returns_2() {
        let inputs = make_inputs(WeaponState::BareHands, Class::Fighter, 1, 0, 0);
        assert_eq!(calculate_number_of_attacks_pure(&inputs), 2);
    }

    #[test]
    fn ammo_always_returns_1() {
        let inputs = make_inputs(WeaponState::Ammo, Class::Fighter, 10, 5, 5);
        assert_eq!(calculate_number_of_attacks_pure(&inputs), 1);
    }

    #[test]
    fn weapon_too_heavy_returns_0() {
        let inputs = NumberOfAttacksInputs {
            weapon: WeaponState::Weapon { weight: 100 },
            class: Class::Fighter,
            max_wield: 50, // can't wield it
            level: 10,
            dex_modifier: 5,
            strength_modifier: 5,
        };
        assert_eq!(calculate_number_of_attacks_pure(&inputs), 0);
    }

    #[test]
    fn monk_bare_hands_gets_class_bonus() {
        // Monk bare hands: attacks_from_class=1, plus level/dex contributions
        let inputs = make_inputs(WeaponState::BareHands, Class::Monk, 20, 5, 5);
        let attacks = calculate_number_of_attacks_pure(&inputs);
        // Monk class bonus=1, level bonus = (8 * 30)/250 = 0, dex=(5+5)/3=3, min(3, 200/5=40) => 3+1+0=4
        assert!(attacks >= 2, "monk bare hands should get multiple attacks, got {}", attacks);
    }

    #[test]
    fn fighter_with_light_weapon_high_level() {
        // Fighter melee_bonus=10, level=50 => attacks_from_level = (10*60)/250 = 2
        // dex_modifier=5 => initial_dex = (5+5)/3 = 3
        // str_modifier=5 => capacity = (10+10)*10 = 200, limit = 200/30 = 6
        // attacks_from_dex = min(3,6) = 3
        // total = 0 + 2 + 3 = 5
        let inputs = make_inputs(WeaponState::Weapon { weight: 30 }, Class::Fighter, 50, 5, 5);
        assert_eq!(calculate_number_of_attacks_pure(&inputs), 5);
    }

    /// Table-driven test for attacks_from_dexterity, parameterized by
    /// (weapon_weight, strength_modifier, dex_modifier).
    ///
    /// Uses Wizard level=1 so attacks_from_class=0, attacks_from_level=0.
    /// Total = attacks_from_dexterity only.
    ///
    /// initial_attacks_from_dex = clamp(5 + dex_mod, 3, 12) / 3   → [1..4]
    /// strength_capacity        = (10 + str_mod * 2) * 30
    /// dex_limit                = strength_capacity / weapon_weight
    /// attacks_from_dex         = min(initial, dex_limit)
    #[test]
    fn attacks_from_dexterity() {
        let longsword_weight = SwordTemplate::Longsword.weight() as i16;
        let stiletto_weight = DaggerTemplate::Stiletto.weight() as i16;
        let halberd_weight = PolearmTemplate::Halberd.weight() as i16;
        let ogre_maul_weight = MaceTemplate::OgreMaul.weight() as i16;
        //  (weight, str, dex, expected)
        let cases: &[(i16, i16, i16, i16)] = &[
            // --- only look at dex if weapon is light ---
            (20,  0, -5,  1), // at least one attack even with bad dex
            (20,  0,  0,  1), // 10 dex (0 mod) -> 1 attack
            (20,  0,  1,  2), // 12 dex (1 mod) -> 2 attack
            (20,  0,  3,  2), // 16 dex (3 mod) -> 2 attack
            (20,  0,  4,  3), // 18 dex (4 mod) -> 3 attack
            (20,  0,  6,  3), // 22 dex (6 mod) -> 3 attack
            (20,  0,  7,  4), // 24 dex (7 mod) -> 4 attack
            (20,  0, 20,  4), // We top out at 4 attacks

            // --- weapon caps for some example weapons ---
            // stiletto (12 weight)
            (stiletto_weight, -2, 20, 4), // 6 str -> 180 capacity -> full attacks
            // longsword (130 weight)
            (longsword_weight,-1, 20,  1), // 8 str -> 240 capacity -> 1 attacks
            (longsword_weight, 0, 20,  2), // 10 str -> 300 capacity -> 2 attacks
            (longsword_weight, 2, 20,  3), // 14 str -> 420 capacity -> 3 attacks
            (longsword_weight, 4, 20,  4), // 18 str -> 540 capacity -> 4 attacks
            // halberd (280 weight)
            (halberd_weight,-1, 20,  0), //  8 str -> 240 capacity -> 0 attacks
            (halberd_weight, 0, 20,  1), // 10 str -> 300 capacity -> 1 attacks
            (halberd_weight, 5, 20,  2), // 20 str -> 600 capacity -> 2 attacks
            (halberd_weight, 9, 20,  3), // 28 str -> 840 capacity -> 3 attacks
            // ogre maul (350 weight)
            (ogre_maul_weight, 0, 20,  0), // 10 str -> 300 capacity -> 0 attacks
            (ogre_maul_weight, 1, 20,  1), // 12 str -> 360 capacity -> 1 attacks
            (ogre_maul_weight, 7, 20,  2), // 24 str -> 720 capacity -> 2 attacks
        ];

        for &(weight, str_mod, dex_mod, expected) in cases {
            let inputs = make_inputs(
                WeaponState::Weapon { weight },
                Class::Wizard,
                1,
                dex_mod,
                str_mod,
            );
            let total = calculate_number_of_attacks_pure(&inputs);
            assert_eq!(
                total, expected,
                "weight={}, str={}, dex={}: expected {}, got {}",
                weight, str_mod, dex_mod, expected, total
            );
        }
    }

    /// Table-driven test for the attacks_from_level component.
    ///
    /// With a non-Monk wielding a Weapon(20), dex=0, str=0 the other
    /// components are fixed: attacks_from_class=0, attacks_from_dex=1.
    /// So total = attacks_from_level + 1.
    ///
    /// attacks_from_level = (melee_bonus(class) * (level + 10)) / 250
    #[test]
    fn attacks_from_level() {
        let cases: &[(Class, u8, i16)] = &[
            // Fighter/Barbarian  melee_bonus=10
            (Class::Fighter,  1,  0), // (10 * 11) / 250 = 0
            (Class::Fighter, 14,  0), // (10 * 24) / 250 = 0
            (Class::Fighter, 15,  1), // (10 * 25) / 250 = 1
            (Class::Fighter, 39,  1), // (10 * 49) / 250 = 1
            (Class::Fighter, 40,  2), // (10 * 50) / 250 = 2
            // Paladin  melee_bonus=8
            (Class::Paladin,  1,  0), // (8 * 11) / 250 = 0
            (Class::Paladin, 21,  0), // (8 * 31) / 250 = 0
            (Class::Paladin, 22,  1), // (8 * 32) / 250 = 1
            (Class::Paladin, 40,  1), // (8 * 50) / 250 = 1
            // Cleric/Rogue/Ranger  melee_bonus=6
            (Class::Rogue,   31,  0), // (6 * 41) / 250 = 0
            (Class::Rogue,   32,  1), // (6 * 42) / 250 = 1
            (Class::Rogue,   40,  1), // (6 * 50) / 250 = 1
            // Wizard  melee_bonus=4
            (Class::Wizard,   1,  0), // (4 * 11) / 250 = 0
            (Class::Wizard,  40,  0), // (4 * 50) / 250 = 0
        ];

        let dex_baseline = 1; // attacks_from_dex with dex=0, str=0, weight=20

        for &(ref class, level, expected_from_level) in cases {
            let inputs = make_inputs(WeaponState::Weapon { weight: 20 }, class.clone(), level, 0, 0);
            let total = calculate_number_of_attacks_pure(&inputs);
            assert_eq!(
                total,
                expected_from_level + dex_baseline,
                "class={:?}, level={}: expected attacks_from_level={}, total={}",
                class, level, expected_from_level, total
            );
        }
    }
}