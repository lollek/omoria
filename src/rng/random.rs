extern crate rand;

use rand::{Rng, SeedableRng, StdRng};
use std::cell::RefCell;
use std::rc::Rc;

/// Returns a random integer in the range `[1, max_value]`.
///
/// This is a convenience wrapper that uses the active RNG (see [`use_seeded_rng`]).
/// For RNG injection, prefer [`randint_with_rng`].
///
/// For compatibility with legacy behavior, returns `0` when `max_value <= 0`.
pub fn randint(max_value: i64) -> i64 {
    with_active_rng(|rng| randint_with_rng(rng, max_value))
}

/// Rolls `num_rolls` times a die with range `[1, die_sides]` and sums the result.
///
/// This is a convenience wrapper that uses the active RNG (see [`use_seeded_rng`]).
/// For RNG injection, prefer [`rand_rep_with_rng`].
pub fn rand_rep(num_rolls: i64, die_sides: i64) -> i64 {
    with_active_rng(|rng| rand_rep_with_rng(rng, num_rolls, die_sides))
}

/// Returns a normally distributed integer with mean `mean` and standard deviation `std_dev`.
///
/// This is a convenience wrapper that uses the active RNG (see [`use_seeded_rng`]).
/// For RNG injection, prefer [`randnor_with_rng`].
pub fn randnor(mean: i64, std_dev: i64) -> i64 {
    with_active_rng(|rng| randnor_with_rng(rng, mean, std_dev))
}

// --- RNG-injected variants ---

/// Returns a random integer in the range `[1, max_value]` using the provided RNG.
///
/// For compatibility with legacy behavior, returns `0` when `max_value <= 0`.
pub fn randint_with_rng<R: Rng + ?Sized>(mut rng: &mut R, max_value: i64) -> i64 {
    if max_value > 0 {
        (&mut rng).gen_range(0, max_value) + 1
    } else {
        0
    }
}

/// Rolls `num_rolls` times a die with range `[1, die_sides]` using the provided RNG and sums the result.
pub fn rand_rep_with_rng<R: Rng + ?Sized>(rng: &mut R, num_rolls: i64, die_sides: i64) -> i64 {
    (0..num_rolls).fold(0, |sum, _| sum + randint_with_rng(rng, die_sides))
}

/// Returns a normally distributed integer with mean `mean` and standard deviation `std_dev`.
///
/// This is a direct port of the legacy implementation, but with RNG injection.
#[allow(clippy::approx_constant)] // Exact legacy rounding depends on 6.283 rather than TAU.
pub fn randnor_with_rng<R: Rng + ?Sized>(rng: &mut R, mean: i64, std_dev: i64) -> i64 {
    // Match the legacy approach: two independent uniform draws in (0, 1).
    // NOTE: randint_with_rng(9_999_999) yields [1, 9_999_999], so division gives (0, 1).
    let u1 = randint_with_rng(rng, 9_999_999) as f64 / 10_000_000.0;
    let u2 = randint_with_rng(rng, 9_999_999) as f64 / 10_000_000.0;

    (((-2.0 * u1.ln()).sqrt() * (6.283 * u2).cos() * std_dev as f64) + mean as f64) as i64
}

thread_local! {
    // Thread-local so parallel test threads never share an override.
    static SEEDED_RNGS: RefCell<Vec<Rc<RefCell<StdRng>>>> = const { RefCell::new(Vec::new()) };
}

/// Routes `randint`, `rand_rep`, and `randnor` through a seeded RNG while alive.
#[must_use]
pub struct SeededRngGuard {
    rng: Rc<RefCell<StdRng>>,
}

thread_local! {
    // Guards opened through the C API; `end_c_seeded_rng` pops the newest one.
    static C_SEEDED_RNGS: RefCell<Vec<SeededRngGuard>> = const { RefCell::new(Vec::new()) };
}

/// Routes the default-RNG functions through an RNG seeded with `seed` on this thread until the guard drops.
///
/// The newest live guard wins. Dropping a guard restores the previous source, even out of order.
pub fn use_seeded_rng(seed: u64) -> SeededRngGuard {
    let rng = Rc::new(RefCell::new(StdRng::from_seed(&seed_words(seed)[..])));
    SEEDED_RNGS.with(|stack| stack.borrow_mut().push(Rc::clone(&rng)));
    SeededRngGuard { rng }
}

impl Drop for SeededRngGuard {
    fn drop(&mut self) {
        // try_with: this can run during thread-local teardown, where `with` would panic.
        let _ = SEEDED_RNGS
            .try_with(|stack| stack.borrow_mut().retain(|rng| !Rc::ptr_eq(rng, &self.rng)));
    }
}

/// Opens a seeded override for C callers. Pair each call with [`end_c_seeded_rng`] on the same thread.
pub fn begin_c_seeded_rng(seed: u64) {
    let guard = use_seeded_rng(seed);
    C_SEEDED_RNGS.with(|guards| guards.borrow_mut().push(guard));
}

/// Closes the newest override opened with [`begin_c_seeded_rng`].
pub fn end_c_seeded_rng() {
    let guard = C_SEEDED_RNGS.with(|guards| guards.borrow_mut().pop());
    drop(guard);
}

/// Splits the seed into two native-width words, the form `StdRng::from_seed` takes.
fn seed_words(seed: u64) -> [usize; 2] {
    [seed as u32 as usize, (seed >> 32) as usize]
}

/// Calls `draw` with the newest seeded RNG on this thread, or the thread RNG when none is active.
fn with_active_rng<T>(draw: impl FnOnce(&mut dyn Rng) -> T) -> T {
    let seeded = SEEDED_RNGS.with(|stack| stack.borrow().last().cloned());
    match seeded {
        Some(rng) => draw(&mut *rng.borrow_mut()),
        None => draw(&mut rand::thread_rng()),
    }
}

#[cfg(test)]
pub fn active_seeded_rng_count() -> usize {
    SEEDED_RNGS.with(|stack| stack.borrow().len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded_values<T>(seed: u64, count: usize, draw: impl Fn() -> T) -> Vec<T> {
        let _guard = use_seeded_rng(seed);
        (0..count).map(|_| draw()).collect()
    }

    #[test]
    fn seeded_randint_repeats_for_same_seed() {
        let first = seeded_values(42, 10, || randint(1_000));
        let second = seeded_values(42, 10, || randint(1_000));
        assert_eq!(first, second);
    }

    #[test]
    fn seeded_randint_differs_for_different_seeds() {
        let first = seeded_values(42, 20, || randint(1_000));
        let second = seeded_values(43, 20, || randint(1_000));
        assert_ne!(first, second);
    }

    #[test]
    fn seed_high_word_is_not_dropped() {
        let low = seeded_values(0, 10, || randint(1_000));
        let high = seeded_values(1 << 32, 10, || randint(1_000));
        assert_ne!(low, high);
    }

    #[test]
    fn seeded_rand_rep_repeats_for_same_seed() {
        let first = seeded_values(42, 10, || rand_rep(5, 6));
        let second = seeded_values(42, 10, || rand_rep(5, 6));
        assert_eq!(first, second);
    }

    #[test]
    fn seeded_randnor_repeats_for_same_seed() {
        let first = seeded_values(42, 10, || randnor(72, 6));
        let second = seeded_values(42, 10, || randnor(72, 6));
        assert_eq!(first, second);
    }

    #[test]
    fn dropping_seeded_guard_restores_default_source() {
        {
            let _guard = use_seeded_rng(5);
            assert_eq!(active_seeded_rng_count(), 1);
        }
        assert_eq!(active_seeded_rng_count(), 0);
    }

    #[test]
    fn nested_seeded_guard_uses_newest_seed_and_restores_outer_sequence() {
        let expected_outer = seeded_values(1, 4, || randint(1_000));
        let expected_inner = seeded_values(2, 2, || randint(1_000));
        let outer = use_seeded_rng(1);
        let mut outer_draws: Vec<i64> = (0..2).map(|_| randint(1_000)).collect();
        let inner_draws: Vec<i64> = {
            let _inner = use_seeded_rng(2);
            (0..2).map(|_| randint(1_000)).collect()
        };
        outer_draws.extend((0..2).map(|_| randint(1_000)));
        drop(outer);
        assert_eq!(inner_draws, expected_inner);
        assert_eq!(outer_draws, expected_outer);
    }

    #[test]
    fn dropping_outer_guard_keeps_newest_seed_active() {
        let expected = seeded_values(2, 3, || randint(1_000));
        let outer = use_seeded_rng(1);
        let inner = use_seeded_rng(2);
        drop(outer);
        let draws: Vec<i64> = (0..3).map(|_| randint(1_000)).collect();
        drop(inner);
        assert_eq!(draws, expected);
        assert_eq!(active_seeded_rng_count(), 0);
    }

    #[test]
    fn panic_inside_seeded_guard_restores_default_source() {
        let result = std::panic::catch_unwind(|| {
            let _guard = use_seeded_rng(7);
            panic!("Seeded scope failure");
        });
        assert!(result.is_err());
        assert_eq!(active_seeded_rng_count(), 0);
    }

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

    #[test]
    fn injected_helpers_accept_a_rng_trait_object() {
        use rand::{SeedableRng, StdRng};

        let mut seeded = StdRng::from_seed(&[4, 3, 2, 1][..]);
        let rng: &mut dyn Rng = &mut seeded;

        assert!((1..=10).contains(&randint_with_rng(rng, 10)));
        assert!((3..=15).contains(&rand_rep_with_rng(rng, 3, 5)));
        assert!((-100..=100).contains(&randnor_with_rng(rng, 0, 10)));
    }
}
