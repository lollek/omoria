extern crate rand;

use rand::Rng;

/// Returns a random integer in the range `[1, max_value]`.
///
/// This is a convenience wrapper that uses a default RNG. For deterministic
/// tests or RNG injection, prefer [`randint_with_rng`].
///
/// For compatibility with legacy behavior, returns `0` when `max_value <= 0`.
pub fn randint(max_value: i64) -> i64 {
    randint_with_rng(&mut rand::thread_rng(), max_value)
}

/// Rolls `num_rolls` times a die with range `[1, die_sides]` and sums the result.
///
/// This is a convenience wrapper that uses a default RNG. For deterministic
/// tests or RNG injection, prefer [`rand_rep_with_rng`].
pub fn rand_rep(num_rolls: i64, die_sides: i64) -> i64 {
    rand_rep_with_rng(&mut rand::thread_rng(), num_rolls, die_sides)
}

/// Returns a normally distributed integer with mean `mean` and standard deviation `std_dev`.
///
/// This is a convenience wrapper that uses a default RNG. For deterministic
/// tests or RNG injection, prefer [`randnor_with_rng`].
pub fn randnor(mean: i64, std_dev: i64) -> i64 {
    randnor_with_rng(&mut rand::thread_rng(), mean, std_dev)
}

// --- RNG-injected variants ---

/// Returns a random integer in the range `[1, max_value]` using the provided RNG.
///
/// For compatibility with legacy behavior, returns `0` when `max_value <= 0`.
pub fn randint_with_rng(rng: &mut impl Rng, max_value: i64) -> i64 {
    if max_value > 0 {
        rng.gen_range(0, max_value) + 1
    } else {
        0
    }
}

/// Rolls `num_rolls` times a die with range `[1, die_sides]` using the provided RNG and sums the result.
pub fn rand_rep_with_rng(rng: &mut impl Rng, num_rolls: i64, die_sides: i64) -> i64 {
    (0..num_rolls).fold(0, |sum, _| sum + randint_with_rng(rng, die_sides))
}

/// Returns a normally distributed integer with mean `mean` and standard deviation `std_dev`.
///
/// This is a direct port of the legacy implementation, but with RNG injection.
pub fn randnor_with_rng(rng: &mut impl Rng, mean: i64, std_dev: i64) -> i64 {
    // Match the legacy approach: two independent uniform draws in (0, 1).
    // NOTE: randint_with_rng(9_999_999) yields [1, 9_999_999], so division gives (0, 1).
    let u1 = randint_with_rng(rng, 9_999_999) as f64 / 10_000_000.0;
    let u2 = randint_with_rng(rng, 9_999_999) as f64 / 10_000_000.0;

    (((-2.0 * u1.ln()).sqrt() * (6.283 * u2).cos() * std_dev as f64) + mean as f64) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randint_with_rng_is_deterministic_for_a_fixed_seed() {
        // rand 0.4 uses Rand's StdRng + SeedableRng. We assert determinism
        // by comparing two RNGs with identical seeds.
        use rand::{SeedableRng, StdRng};

        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng_a = StdRng::from_seed(seed);
        let mut rng_b = StdRng::from_seed(seed);

        let draws_a: Vec<i64> = (0..10).map(|_| randint_with_rng(&mut rng_a, 100)).collect();
        let draws_b: Vec<i64> = (0..10).map(|_| randint_with_rng(&mut rng_b, 100)).collect();

        assert_eq!(draws_a, draws_b);

        // Additionally, values should be within [1, 100] for positive max_value.
        assert!(draws_a.iter().all(|&v| (1..=100).contains(&v)));
    }

    #[test]
    fn rand_rep_with_rng_sums_num_randint_draws_and_is_deterministic() {
        use rand::{SeedableRng, StdRng};

        let seed: &[_] = &[9, 9, 9, 9];
        let mut rng_a = StdRng::from_seed(seed);
        let mut rng_b = StdRng::from_seed(seed);

        let a = rand_rep_with_rng(&mut rng_a, 5, 6);
        let b = rand_rep_with_rng(&mut rng_b, 5, 6);

        assert_eq!(a, b);
        assert!((5..=30).contains(&a));
    }

    #[test]
    fn randnor_with_rng_is_deterministic_for_a_fixed_seed() {
        use rand::{SeedableRng, StdRng};

        let seed: &[_] = &[7, 6, 5, 4];
        let mut rng_a = StdRng::from_seed(seed);
        let mut rng_b = StdRng::from_seed(seed);

        let a = randnor_with_rng(&mut rng_a, 72, 6);
        let b = randnor_with_rng(&mut rng_b, 72, 6);

        assert_eq!(a, b);
    }

    #[test]
    fn randint_with_rng_matches_existing_edge_behavior_for_non_positive_max_value() {
        use rand::{SeedableRng, StdRng};

        let seed: &[_] = &[0, 0, 0, 0];
        let mut rng = StdRng::from_seed(seed);

        assert_eq!(randint_with_rng(&mut rng, 0), 0);
        assert_eq!(randint_with_rng(&mut rng, -3), 0);
    }
}
