use rand::Rng;
use std::ffi::CString;

use super::regeneration::HungerStatus;
use crate::model::PlayerFlags;

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

fn report_eating(outcome: EatOutcome) {
    for message in outcome.messages {
        crate::term::msg_print(message);
    }
    if let Some(cause) = outcome.death_cause {
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
