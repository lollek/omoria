use rand::Rng;
use std::ffi::CString;

use super::regeneration::HungerStatus;
use crate::model::PlayerFlags;

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

const PLAYER_FOOD_MAX: i64 = 15000;
const PLAYER_FOOD_FULL: i64 = 10000;
const PLAYER_FOOD_ALERT: i64 = 2000;
const PLAYER_FOOD_WEAK: i64 = 1000;
const PLAYER_FOOD_FAINT: i64 = 300;
const IS_HUNGERY: u64 = 0x0000_0001;
const IS_WEAK: u64 = 0x0000_0002;

extern "C" {
    static mut player_flags: PlayerFlags;
    #[link_name = "player_wt"]
    static mut C_PLAYER_WT: u16;
    static mut death: bool;
    static mut moria_flag: bool;
    static mut total_winner: bool;
    static mut died_from: [libc::c_char; 82];
    fn prt_hunger();
    fn max_allowable_weight() -> u16;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HungerState {
    foodc: i64,
    status_flags: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct EatOutcome {
    messages: Vec<&'static str>,
    redraw_hunger: bool,
    death_cause: Option<&'static str>,
}

trait RecalculationContext {
    fn has_hunger_item(&self) -> bool;
    fn food_level(&self) -> i64;
    fn is_hungry(&self) -> bool;
    fn is_weak(&self) -> bool;
    fn set_food_level(&mut self, food_level: i64);
    fn mark_hungry(&mut self);
    fn mark_weak(&mut self);
    fn weight(&self) -> u16;
    fn set_weight(&mut self, weight: u16);
    fn min_weight(&self) -> u16;
    fn add_paralysis(&mut self, turns: i64);
    fn message(&mut self, message: &'static str);
    fn interrupt(&mut self);
    fn redraw(&mut self);
    fn starve(&mut self);
}

#[cfg(test)]
fn recalculate_with_rng(rng: &mut impl Rng, context: &mut impl RecalculationContext) {
    recalculate(context, &mut |max| crate::rng::randint_with_rng(rng, max));
}

fn recalculate(context: &mut impl RecalculationContext, roll: &mut impl FnMut(i64) -> i64) {
    if context.has_hunger_item() && context.food_level() > PLAYER_FOOD_ALERT + 15 {
        context.set_food_level(PLAYER_FOOD_ALERT + 15);
    }
    if context.food_level() >= PLAYER_FOOD_ALERT {
        return;
    }
    if context.food_level() >= PLAYER_FOOD_WEAK {
        if !context.is_hungry() {
            context.mark_hungry();
            context.message("You are getting hungry.");
            context.interrupt();
            context.redraw();
        }
        return;
    }
    if !context.is_weak() {
        context.mark_hungry();
        context.mark_weak();
        context.message("You are getting weak from hunger.");
        context.interrupt();
        context.redraw();
        let weight = context.weight();
        context.set_weight(weight - (f64::from(weight) * 0.015).trunc() as u16);
        context.message("Your clothes seem to be getting loose.");
        if context.weight() < context.min_weight() {
            context.message("Oh no...  Now you've done it.");
            context.starve();
        }
    }
    // Preserve the legacy nesting: even the second fainting roll requires negative food.
    if context.food_level() < 0 {
        if roll(5) == 1 {
            context.add_paralysis(roll(3));
            context.message("You faint from the lack of food.");
            context.interrupt();
        } else if context.food_level() < PLAYER_FOOD_FAINT && roll(8) == 1 {
            context.add_paralysis(roll(5));
            context.message("You faint from the lack of food.");
            context.interrupt();
        }
    }
}

fn status(foodc: i64) -> HungerStatus {
    if foodc > PLAYER_FOOD_FULL {
        HungerStatus::Bloated
    } else if foodc > PLAYER_FOOD_ALERT {
        HungerStatus::Full
    } else if foodc > PLAYER_FOOD_WEAK {
        HungerStatus::Hungry
    } else if foodc > PLAYER_FOOD_FAINT {
        HungerStatus::Weak
    } else {
        HungerStatus::Dying
    }
}

fn set_status(state: &mut HungerState, hunger_status: HungerStatus) {
    match hunger_status {
        HungerStatus::Dying => {
            state.foodc = PLAYER_FOOD_FAINT;
            state.status_flags |= IS_HUNGERY | IS_WEAK;
        }
        HungerStatus::Weak => {
            state.foodc = PLAYER_FOOD_WEAK;
            state.status_flags |= IS_HUNGERY | IS_WEAK;
        }
        HungerStatus::Hungry => {
            state.foodc = PLAYER_FOOD_ALERT;
            state.status_flags |= IS_HUNGERY;
            state.status_flags &= !IS_WEAK;
        }
        HungerStatus::Full => {
            state.foodc = PLAYER_FOOD_FULL;
            state.status_flags &= !(IS_WEAK | IS_HUNGERY);
        }
        HungerStatus::Bloated => {
            state.foodc = PLAYER_FOOD_MAX;
            state.status_flags &= !(IS_WEAK | IS_HUNGERY);
        }
    }
}

fn eat_with_rng<R: Rng + ?Sized>(
    rng: &mut R,
    state: &mut HungerState,
    player_wt: &mut u16,
    max_weight: u16,
    amount: i64,
) -> EatOutcome {
    state.status_flags &= !(IS_HUNGERY | IS_WEAK);
    if state.foodc < 0 {
        state.foodc = 0;
    }
    state.foodc += amount;

    let hunger_status = status(state.foodc);
    let mut outcome = EatOutcome {
        messages: Vec::new(),
        redraw_hunger: true,
        death_cause: None,
    };
    if hunger_status == HungerStatus::Bloated {
        outcome
            .messages
            .push("You're getting fat from eating so much.");
        state.foodc = PLAYER_FOOD_MAX;
        *player_wt = player_wt.wrapping_add((*player_wt as f64 * 0.1).trunc() as u16);
        if *player_wt > max_weight {
            outcome.messages.push("Oh no...  Now you've done it.");
            outcome.redraw_hunger = false;
            outcome.death_cause = Some("gluttony");
            return outcome;
        }

        outcome
            .messages
            .push(match crate::rng::randint_with_rng(rng, 3) {
                1 => "Buuurrrppppp !",
                2 => "Remember, obesity kills.",
                3 => "Your armor doesn't seem to fit too well anymore.",
                _ => unreachable!("randint(3) returns a value in 1..=3"),
            });
        return outcome;
    }

    if matches!(hunger_status, HungerStatus::Full | HungerStatus::Bloated) {
        outcome.messages.push("You are full.");
    }
    outcome
}

fn global_state() -> HungerState {
    unsafe {
        HungerState {
            foodc: player_flags.foodc,
            status_flags: player_flags.status,
        }
    }
}

fn store_global_state(state: HungerState) {
    unsafe {
        player_flags.foodc = state.foodc;
        player_flags.status = state.status_flags;
    }
}

fn report_death(cause: &str) {
    unsafe {
        death = true;
        moria_flag = true;
        total_winner = false;
        let cause = CString::new(cause).unwrap();
        libc::strcpy(
            std::ptr::addr_of_mut!(died_from).cast::<libc::c_char>(),
            cause.as_ptr(),
        );
    }
}

fn report_eating(outcome: EatOutcome) {
    for message in outcome.messages {
        crate::term::msg_print(message);
    }
    if let Some(cause) = outcome.death_cause {
        report_death(cause);
    } else if outcome.redraw_hunger {
        unsafe { prt_hunger() };
    }
}

#[no_mangle]
pub extern "C" fn player_hunger_status() -> libc::c_int {
    status(unsafe { player_flags.foodc }) as libc::c_int
}

#[no_mangle]
pub extern "C" fn player_hunger_set_status(hunger_status: libc::c_int) {
    let hunger_status = match hunger_status {
        0 => HungerStatus::Dying,
        1 => HungerStatus::Weak,
        2 => HungerStatus::Hungry,
        3 => HungerStatus::Full,
        4 => HungerStatus::Bloated,
        _ => return,
    };
    let mut state = global_state();
    set_status(&mut state, hunger_status);
    store_global_state(state);
}

#[no_mangle]
pub extern "C" fn player_hunger_eat(amount: libc::c_long) {
    let mut state = global_state();
    let mut weight = unsafe { C_PLAYER_WT };
    let max_weight = unsafe { max_allowable_weight() };
    let outcome = crate::rng::with_active_rng(|rng| {
        eat_with_rng(rng, &mut state, &mut weight, max_weight, amount)
    });
    store_global_state(state);
    unsafe { C_PLAYER_WT = weight };
    report_eating(outcome);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, StdRng};

    struct RecalculationFixture {
        flags: PlayerFlags,
        weight: u16,
        min_weight: u16,
        events: Vec<String>,
    }

    impl RecalculationFixture {
        fn new(foodc: i64, status: u64) -> Self {
            let mut flags = PlayerFlags::new();
            flags.foodc = foodc;
            flags.status = status;
            Self {
                flags,
                weight: 100,
                min_weight: 50,
                events: Vec::new(),
            }
        }

        fn recalculate(&mut self) {
            recalculate_with_rng(&mut StdRng::from_seed(&[1, 2, 3, 4][..]), self);
        }
    }

    impl RecalculationContext for RecalculationFixture {
        fn has_hunger_item(&self) -> bool {
            self.flags.hunger_item != 0
        }
        fn food_level(&self) -> i64 {
            self.flags.foodc
        }
        fn is_hungry(&self) -> bool {
            self.flags.status & IS_HUNGERY != 0
        }
        fn is_weak(&self) -> bool {
            self.flags.status & IS_WEAK != 0
        }
        fn set_food_level(&mut self, food_level: i64) {
            self.flags.foodc = food_level;
        }
        fn mark_hungry(&mut self) {
            self.flags.status |= IS_HUNGERY;
        }
        fn mark_weak(&mut self) {
            self.flags.status |= IS_WEAK;
        }
        fn weight(&self) -> u16 {
            self.weight
        }
        fn set_weight(&mut self, weight: u16) {
            self.weight = weight;
            self.events.push(format!("weight:{weight}"));
        }
        fn min_weight(&self) -> u16 {
            self.min_weight
        }
        fn add_paralysis(&mut self, turns: i64) {
            self.flags.paralysis += turns;
            self.events.push(format!("paralysis:{turns}"));
        }
        fn message(&mut self, message: &'static str) {
            self.events.push(message.to_string());
        }
        fn interrupt(&mut self) {
            self.events.push("interrupt".to_string());
        }
        fn redraw(&mut self) {
            self.events.push("redraw".to_string());
        }
        fn starve(&mut self) {
            self.events.push("starve".to_string());
        }
    }

    #[test]
    fn recalculation_context_reports_hunger_state_semantically() {
        let mut context = RecalculationFixture::new(1234, 0x80);
        context.flags.hunger_item = 42;

        assert!(context.has_hunger_item());
        assert_eq!(context.food_level(), 1234);
        assert!(!context.is_hungry());
        assert!(!context.is_weak());

        context.flags.hunger_item = 0;
        context.flags.status |= IS_HUNGERY | IS_WEAK;
        assert!(!context.has_hunger_item());
        assert!(context.is_hungry());
        assert!(context.is_weak());
    }

    #[test]
    fn recalculation_hunger_item_caps_only_food_above_alert_plus_fifteen() {
        for (foodc, expected) in [(2016, 2015), (2015, 2015), (2000, 2000)] {
            let mut context = RecalculationFixture::new(foodc, 0x80);
            context.flags.hunger_item = 1;
            context.recalculate();
            assert_eq!(context.flags.foodc, expected);
            assert_eq!(context.flags.status, 0x80);
            assert!(context.events.is_empty());
        }
        let mut context = RecalculationFixture::new(3000, 0);
        context.recalculate();
        assert_eq!(context.flags.foodc, 3000);
    }

    #[test]
    fn recalculation_hungry_thresholds_interrupt_and_redraw_only_on_transition() {
        for foodc in [1000, 1001, 1999, 2000, 2001] {
            let mut context = RecalculationFixture::new(foodc, 0x80);
            context.recalculate();
            if foodc < 2000 {
                assert_eq!(context.flags.status, 0x80 | IS_HUNGERY);
                assert_eq!(
                    context.events,
                    ["You are getting hungry.", "interrupt", "redraw"]
                );
            } else {
                assert_eq!(context.flags.status, 0x80);
                assert!(context.events.is_empty());
            }
            context.events.clear();
            context.recalculate();
            assert!(context.events.is_empty());
            assert_eq!(context.flags.foodc, foodc);
            assert_eq!(context.weight, 100);
        }
    }

    #[test]
    fn recalculation_weak_transition_truncates_weight_loss_once() {
        for (weight, expected) in [(0, 0), (66, 66), (67, 66), (100, 99), (200, 197)] {
            let mut context = RecalculationFixture::new(999, 0x80);
            context.weight = weight;
            context.min_weight = 0;
            context.recalculate();
            assert_eq!(context.flags.status, 0x80 | IS_HUNGERY | IS_WEAK);
            assert_eq!(context.flags.foodc, 999);
            assert_eq!(context.weight, expected);
            assert_eq!(
                context.events,
                [
                    "You are getting weak from hunger.".to_string(),
                    "interrupt".to_string(),
                    "redraw".to_string(),
                    format!("weight:{expected}"),
                    "Your clothes seem to be getting loose.".to_string(),
                ]
            );
            context.events.clear();
            context.recalculate();
            assert_eq!(context.weight, expected);
            assert!(context.events.is_empty());
        }
    }

    #[test]
    fn recalculation_starves_only_below_minimum_weight_on_weak_transition() {
        for (weight, dies) in [(99, true), (100, false), (101, false)] {
            let mut context = RecalculationFixture::new(0, 0);
            context.weight = weight;
            context.min_weight = 99;
            context.recalculate();
            assert_eq!(context.events.iter().any(|event| event == "starve"), dies);
            if dies {
                assert_eq!(
                    &context.events[5..],
                    ["Oh no...  Now you've done it.", "starve"]
                );
            }
        }
    }

    #[test]
    fn recalculation_does_not_faint_at_nonnegative_food_even_below_faint_threshold() {
        for foodc in [0, 1, 299, 300, 301, 999] {
            let mut context = RecalculationFixture::new(foodc, IS_HUNGERY | IS_WEAK);
            context.recalculate();
            assert_eq!(context.flags.paralysis, 0);
            assert!(context.events.is_empty(), "foodc={}", foodc);
        }
    }

    #[test]
    fn recalculation_seeded_fainting_matches_both_legacy_roll_paths_and_no_faint() {
        let mut covered = [false; 3];
        for seed in 1..=256 {
            let seed = [seed, 2, 3, 4];
            let mut expected_rng = StdRng::from_seed(&seed[..]);
            let (path, turns) = if crate::rng::randint_with_rng(&mut expected_rng, 5) == 1 {
                (0, crate::rng::randint_with_rng(&mut expected_rng, 3))
            } else if crate::rng::randint_with_rng(&mut expected_rng, 8) == 1 {
                (1, crate::rng::randint_with_rng(&mut expected_rng, 5))
            } else {
                (2, 0)
            };
            let mut context = RecalculationFixture::new(-1, IS_HUNGERY | IS_WEAK);
            context.flags.paralysis = 7;
            let mut rng = StdRng::from_seed(&seed[..]);
            recalculate_with_rng(&mut rng, &mut context);
            assert_eq!(context.flags.paralysis, 7 + turns);
            assert_eq!(
                crate::rng::randint_with_rng(&mut rng, 1000),
                crate::rng::randint_with_rng(&mut expected_rng, 1000),
                "RNG consumption differs for seed {seed:?}"
            );
            if turns > 0 {
                assert_eq!(
                    context.events,
                    [
                        format!("paralysis:{turns}"),
                        "You faint from the lack of food.".to_string(),
                        "interrupt".to_string(),
                    ]
                );
            } else {
                assert!(context.events.is_empty());
            }
            covered[path] = true;
        }
        assert!(covered.iter().all(|covered| *covered), "all fainting paths");
    }

    #[test]
    fn status_matches_every_threshold_boundary() {
        let cases = [
            (i64::MIN, HungerStatus::Dying),
            (300, HungerStatus::Dying),
            (301, HungerStatus::Weak),
            (1000, HungerStatus::Weak),
            (1001, HungerStatus::Hungry),
            (2000, HungerStatus::Hungry),
            (2001, HungerStatus::Full),
            (10000, HungerStatus::Full),
            (10001, HungerStatus::Bloated),
        ];

        for (foodc, expected) in cases {
            assert_eq!(status(foodc), expected, "foodc={foodc}");
        }
    }

    #[test]
    fn setting_status_preserves_unrelated_flags_and_matches_legacy_food_values() {
        let cases = [
            (HungerStatus::Dying, 300, IS_HUNGERY | IS_WEAK),
            (HungerStatus::Weak, 1000, IS_HUNGERY | IS_WEAK),
            (HungerStatus::Hungry, 2000, IS_HUNGERY),
            (HungerStatus::Full, 10000, 0),
            (HungerStatus::Bloated, 15000, 0),
        ];

        for (hunger_status, foodc, expected_flags) in cases {
            let mut state = HungerState {
                foodc: -1,
                status_flags: 0x80 | IS_HUNGERY | IS_WEAK,
            };
            set_status(&mut state, hunger_status);
            assert_eq!(state.foodc, foodc);
            assert_eq!(state.status_flags, 0x80 | expected_flags);
        }
    }

    #[test]
    fn eating_clears_hunger_and_saturates_negative_food_before_adding() {
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut state = HungerState {
            foodc: -50,
            status_flags: 0x80 | IS_HUNGERY | IS_WEAK,
        };
        let mut player_wt = 100;

        let outcome = eat_with_rng(&mut rng, &mut state, &mut player_wt, 1000, 1500);

        assert_eq!(state.foodc, 1500);
        assert_eq!(state.status_flags, 0x80);
        assert_eq!(outcome.messages, Vec::<&str>::new());
        assert!(outcome.redraw_hunger);
        assert_eq!(outcome.death_cause, None);
    }

    #[test]
    fn eating_across_thresholds_updates_food_status_and_boundary_messages() {
        let cases = [
            (299, HungerStatus::Dying, None),
            (300, HungerStatus::Weak, None),
            (1000, HungerStatus::Hungry, None),
            (2000, HungerStatus::Full, Some("You are full.")),
            (
                10000,
                HungerStatus::Bloated,
                Some("You're getting fat from eating so much."),
            ),
        ];

        for (foodc, expected_status, expected_message) in cases {
            let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
            let mut state = HungerState {
                foodc,
                status_flags: IS_HUNGERY | IS_WEAK,
            };
            let mut player_wt = 100;
            let outcome = eat_with_rng(&mut rng, &mut state, &mut player_wt, 1000, 1);

            assert_eq!(status(state.foodc), expected_status, "foodc={foodc}");
            assert_eq!(state.status_flags, 0, "foodc={foodc}");
            assert_eq!(
                outcome.messages.first().copied(),
                expected_message,
                "foodc={foodc}"
            );
            assert!(outcome.redraw_hunger, "foodc={}", foodc);
            assert_eq!(outcome.death_cause, None, "foodc={foodc}");
        }
    }

    #[test]
    fn eating_until_bloated_caps_food_and_reports_seeded_burp_message() {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng = StdRng::from_seed(seed);
        let expected_message = match crate::rng::randint_with_rng(&mut StdRng::from_seed(seed), 3) {
            1 => "Buuurrrppppp !",
            2 => "Remember, obesity kills.",
            3 => "Your armor doesn't seem to fit too well anymore.",
            _ => unreachable!(),
        };
        let mut state = HungerState {
            foodc: 9999,
            status_flags: IS_HUNGERY | IS_WEAK,
        };
        let mut player_wt = 100;

        let outcome = eat_with_rng(&mut rng, &mut state, &mut player_wt, 1000, 2);

        assert_eq!(state.foodc, 15000);
        assert_eq!(state.status_flags, 0);
        assert_eq!(player_wt, 110);
        assert_eq!(
            outcome.messages,
            vec!["You're getting fat from eating so much.", expected_message]
        );
        assert!(outcome.redraw_hunger);
        assert_eq!(outcome.death_cause, None);
    }

    #[test]
    fn eating_past_full_reports_full_message_without_changing_food_to_full_cap() {
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut state = HungerState {
            foodc: 2000,
            status_flags: IS_HUNGERY | IS_WEAK,
        };
        let mut player_wt = 100;

        let outcome = eat_with_rng(&mut rng, &mut state, &mut player_wt, 1000, 1);

        assert_eq!(state.foodc, 2001);
        assert_eq!(state.status_flags, 0);
        assert_eq!(outcome.messages, vec!["You are full."]);
        assert!(outcome.redraw_hunger);
        assert_eq!(outcome.death_cause, None);
    }

    #[test]
    fn overeating_past_weight_limit_sets_gluttony_death_without_burp() {
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut state = HungerState {
            foodc: 9999,
            status_flags: IS_HUNGERY | IS_WEAK,
        };
        let mut player_wt = 100;

        let outcome = eat_with_rng(&mut rng, &mut state, &mut player_wt, 109, 2);

        assert_eq!(player_wt, 110);
        assert_eq!(
            outcome.messages,
            vec![
                "You're getting fat from eating so much.",
                "Oh no...  Now you've done it."
            ]
        );
        assert!(!outcome.redraw_hunger);
        assert_eq!(outcome.death_cause, Some("gluttony"));
    }
}
