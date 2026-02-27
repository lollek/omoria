//! Combat hit-chance calculations.
//!
//! This module provides functions for determining whether attacks hit their targets.

use rand::Rng;

use crate::model::PlayerFlags;
use crate::rng::randint_with_rng;

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
pub fn managed_to_hit_with_rng(
    rng: &mut impl Rng,
    base_to_hit: i64,
    level: i64,
    plus_to_hit: i64,
    enemy_ac: i64,
) -> bool {
    let max_possible_attack_value = base_to_hit + level * 3 + plus_to_hit * 3;
    let attack_value = randint_with_rng(rng, max_possible_attack_value);
    (attack_value >= enemy_ac) || randint_with_rng(rng, 20) == 1
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
    managed_to_hit_with_rng(&mut rand::thread_rng(), base_to_hit, level, plus_to_hit, enemy_ac)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, StdRng};

    #[test]
    fn managed_to_hit_sometimes_hits_via_natural_one_on_d20() {
        // A "natural 1" on the d20 should always hit regardless of AC.
        // With impossibly high AC, the only way to hit is via d20=1 (5% chance).
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng = StdRng::from_seed(seed);

        let base_to_hit = 1;
        let level = 1;
        let plus_to_hit = 0;
        let enemy_ac = 1000; // Impossibly high AC

        // Run many trials - at least one should hit via d20=1 (5% chance per trial)
        let hits: i32 = (0..100)
            .map(|_| {
                if managed_to_hit_with_rng(&mut rng, base_to_hit, level, plus_to_hit, enemy_ac) {
                    1
                } else {
                    0
                }
            })
            .sum();

        // With 100 trials at 5% chance, we expect ~5 hits on average.
        assert!(
            hits > 0,
            "Expected at least one hit via natural 1 on d20, got 0 hits"
        );
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
}

