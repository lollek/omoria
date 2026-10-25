//! Combat hit-chance calculations.
//!
//! This module provides functions for determining whether attacks hit their targets.

use rand::Rng;

use crate::rng::{randint_with_rng, with_active_rng};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ManagedHitResult {
    pub base_to_hit: libc::c_long,
    pub level: libc::c_long,
    pub plus_to_hit: libc::c_long,
    pub max_attack_value: libc::c_long,
    pub attack_value: libc::c_long,
    pub enemy_ac: libc::c_long,
    pub was_rolled: bool,
    pub hit: bool,
}

pub fn managed_to_hit_detailed_with_rng<R: Rng + ?Sized>(
    rng: &mut R,
    base_to_hit: i64,
    level: i64,
    plus_to_hit: i64,
    enemy_ac: i64,
) -> ManagedHitResult {
    let max_attack_value = base_to_hit + level * 3 + plus_to_hit * 3;
    let attack_value = randint_with_rng(rng, max_attack_value);

    ManagedHitResult {
        base_to_hit: base_to_hit as libc::c_long,
        level: level as libc::c_long,
        plus_to_hit: plus_to_hit as libc::c_long,
        max_attack_value: max_attack_value as libc::c_long,
        attack_value: attack_value as libc::c_long,
        enemy_ac: enemy_ac as libc::c_long,
        was_rolled: true,
        hit: attack_value >= enemy_ac,
    }
}

/// Determines whether an attack manages to hit the target.
///
/// This version accepts an injected RNG for deterministic testing.
///
/// # Arguments
/// * `rng` - Random number generator
/// * `base_to_hit` - Base to-hit value of the attacker
/// * `level` - Level of the attacker
/// * `plus_to_hit` - Plus-to-hit bonus from equipment/effects
/// * `enemy_ac` - Armor class of the defender
///
/// # Returns
/// `true` if the attack hits, `false` otherwise.
///
/// # Hit Formula
/// The maximum attack value is: `base_to_hit + level*3 + plus_to_hit*3`
/// A random value in `[1, max]` is rolled.
/// The attack hits if `attack_value >= enemy_ac` OR a natural 1 is rolled on d20.
pub fn managed_to_hit_with_rng<R: Rng + ?Sized>(
    rng: &mut R,
    base_to_hit: i64,
    level: i64,
    plus_to_hit: i64,
    enemy_ac: i64,
) -> bool {
    managed_to_hit_detailed_with_rng(rng, base_to_hit, level, plus_to_hit, enemy_ac).hit
}

#[no_mangle]
pub extern "C" fn managed_to_hit_detailed(
    base_to_hit: libc::c_long,
    level: libc::c_long,
    plus_to_hit: libc::c_long,
    enemy_ac: libc::c_long,
) -> ManagedHitResult {
    with_active_rng(|rng| {
        managed_to_hit_detailed_with_rng(rng, base_to_hit, level, plus_to_hit, enemy_ac)
    })
}

/// Determines whether an attack manages to hit the target (pure Rust version).
///
/// For deterministic tests, use [`managed_to_hit_with_rng`].
#[no_mangle]
pub extern "C" fn managed_to_hit(
    base_to_hit: libc::c_long,
    level: libc::c_long,
    plus_to_hit: libc::c_long,
    enemy_ac: libc::c_long,
) -> bool {
    managed_to_hit_detailed(base_to_hit, level, plus_to_hit, enemy_ac).hit
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, StdRng};

    #[test]
    fn managed_to_hit_uses_the_seeded_active_rng() {
        fn outcomes(seed: u64) -> Vec<bool> {
            let _guard = crate::rng::use_seeded_rng(seed);
            (0..64).map(|_| managed_to_hit(1, 0, 0, 500)).collect()
        }

        assert_eq!(outcomes(42), outcomes(42));
    }

    #[test]
    fn managed_to_hit_hits_when_attack_roll_exceeds_ac() {
        let seed: &[_] = &[42, 42, 42, 42];
        let mut rng = StdRng::from_seed(seed);

        // High stats, low AC - should almost always hit
        let base_to_hit = 50;
        let level = 10;
        let plus_to_hit = 10;
        let enemy_ac = 1; // Very low AC

        // max_attack = 50 + 10*3 + 10*3 = 110
        // Roll in [1, 110], need >= 1, so always hits
        let result = managed_to_hit_with_rng(&mut rng, base_to_hit, level, plus_to_hit, enemy_ac);

        assert!(result, "Expected hit with high attack vs low AC");
    }

    #[test]
    fn detailed_hit_result_reports_attack_roll_and_inputs() {
        let seed: &[_] = &[42, 42, 42, 42];
        let mut rng = StdRng::from_seed(seed);

        let result = managed_to_hit_detailed_with_rng(&mut rng, 50, 10, 10, 1);

        assert_eq!(result.base_to_hit, 50);
        assert_eq!(result.level, 10);
        assert_eq!(result.plus_to_hit, 10);
        assert_eq!(result.max_attack_value, 110);
        assert!((1..=110).contains(&result.attack_value));
        assert_eq!(result.enemy_ac, 1);
        assert!(result.was_rolled);
        assert!(result.hit);
    }

    #[test]
    fn detailed_hit_result_preserves_boolean_outcome_and_rng_sequence() {
        let seed: &[_] = &[5, 6, 7, 8];
        let mut detailed_rng = StdRng::from_seed(seed);
        let mut boolean_rng = StdRng::from_seed(seed);

        let detailed = managed_to_hit_detailed_with_rng(&mut detailed_rng, 1, 0, 0, 1000);
        let boolean = managed_to_hit_with_rng(&mut boolean_rng, 1, 0, 0, 1000);

        assert_eq!(detailed.hit, boolean);
        assert_eq!(
            randint_with_rng(&mut detailed_rng, 100),
            randint_with_rng(&mut boolean_rng, 100)
        );
    }
}
