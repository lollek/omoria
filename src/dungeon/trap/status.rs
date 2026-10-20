use super::TRAP_BASE_TO_HIT;
use crate::{combat::fighting::managed_to_hit_with_rng, rng::randint_with_rng};
use rand::Rng;

pub(super) trait StatusEffects {
    fn damage(&mut self, damage: i64, cause: &str);
    fn message(&mut self, message: &'static str);
    fn paralysis(&self) -> i64;
    fn free_action(&self) -> bool;
    fn add_paralysis(&mut self, amount: i64);
    fn add_blindness(&mut self, amount: i64);
    fn add_confusion(&mut self, amount: i64);
    fn add_slow(&mut self, amount: i64);
    fn armor_class(&self) -> i64;
    fn lose_stat(&mut self, stat: StatusStat) -> bool;
    fn mark_stat_for_redraw(&mut self, stat: StatusStat);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StatusStat {
    Strength,
    Constitution,
}

fn sleep_gas_with_rng<R: Rng + ?Sized>(rng: &mut R, effects: &mut impl StatusEffects) {
    if effects.paralysis() != 0 {
        return;
    }

    effects.message("A strange white mist surrounds you!");
    if effects.free_action() {
        effects.message("You are unaffected.");
    } else {
        effects.message("You fall asleep.");
        effects.add_paralysis(randint_with_rng(rng, 10) + 4);
    }
}

fn strength_dart_with_rng<R: Rng + ?Sized>(
    rng: &mut R,
    effects: &mut impl StatusEffects,
    damage: i64,
) {
    if managed_to_hit_with_rng(rng, TRAP_BASE_TO_HIT, 0, 0, effects.armor_class()) {
        if effects.lose_stat(StatusStat::Strength) {
            effects.damage(damage, "a dart trap");
            effects.mark_stat_for_redraw(StatusStat::Strength);
            effects.message("A small dart weakens you!");
        }
    } else {
        effects.message("A small dart barely misses you.");
    }
}

fn blind_gas_with_rng<R: Rng + ?Sized>(rng: &mut R, effects: &mut impl StatusEffects) {
    effects.message("A black gas surrounds you!");
    effects.add_blindness(randint_with_rng(rng, 50) + 50);
}

fn confuse_gas_with_rng<R: Rng + ?Sized>(rng: &mut R, effects: &mut impl StatusEffects) {
    effects.message("A gas of scintillating colors surrounds you!");
    effects.add_confusion(randint_with_rng(rng, 15) + 15);
}

fn slow_dart_with_rng<R: Rng + ?Sized>(rng: &mut R, effects: &mut impl StatusEffects, damage: i64) {
    if managed_to_hit_with_rng(rng, TRAP_BASE_TO_HIT, 0, 0, effects.armor_class()) {
        effects.damage(damage, "a dart trap");
        effects.message("A small dart hits you!");
        effects.add_slow(randint_with_rng(rng, 20) + 10);
    } else {
        effects.message("A small dart barely misses you.");
    }
}

fn constitution_dart_with_rng<R: Rng + ?Sized>(
    rng: &mut R,
    effects: &mut impl StatusEffects,
    damage: i64,
) {
    if managed_to_hit_with_rng(rng, TRAP_BASE_TO_HIT, 0, 0, effects.armor_class()) {
        if effects.lose_stat(StatusStat::Constitution) {
            effects.damage(damage, "a dart trap");
            effects.mark_stat_for_redraw(StatusStat::Constitution);
            effects.message("A small dart weakens you!");
        }
    } else {
        effects.message("A small dart barely misses you.");
    }
}

#[cfg(not(test))]
extern "C" {
    static mut player_flags: crate::model::PlayerFlags;
    static mut print_stat: libc::c_ulong;
    fn player_pac() -> i16;
    fn player_ptoac() -> i16;
    fn lose_stat(
        stat: libc::c_int,
        message: *const libc::c_char,
        sustain_message: *const libc::c_char,
    ) -> bool;
}

#[cfg(not(test))]
struct GlobalStatus;

#[cfg(not(test))]
impl StatusEffects for GlobalStatus {
    fn damage(&mut self, damage: i64, cause: &str) {
        let cause = std::ffi::CString::new(cause).expect("trap damage causes contain no NUL bytes");
        unsafe { crate::player::damage::take_hit(damage, cause.as_ptr()) }
    }

    fn message(&mut self, message: &'static str) {
        crate::term::msg_print(message);
    }
    fn paralysis(&self) -> i64 {
        unsafe { player_flags.paralysis }
    }

    fn free_action(&self) -> bool {
        unsafe { player_flags.free_act != 0 }
    }

    fn add_paralysis(&mut self, amount: i64) {
        unsafe { player_flags.paralysis += amount }
    }

    fn add_blindness(&mut self, amount: i64) {
        unsafe { player_flags.blind += amount }
    }

    fn add_confusion(&mut self, amount: i64) {
        unsafe { player_flags.confused += amount }
    }

    fn add_slow(&mut self, amount: i64) {
        unsafe { player_flags.slow += amount }
    }

    fn armor_class(&self) -> i64 {
        unsafe { i64::from(player_pac()) + i64::from(player_ptoac()) }
    }

    fn lose_stat(&mut self, stat: StatusStat) -> bool {
        let stat = match stat {
            StatusStat::Strength => 0,
            StatusStat::Constitution => 4,
        };
        unsafe {
            lose_stat(
                stat,
                b"\0".as_ptr().cast(),
                b"A small dart hits you.\0".as_ptr().cast(),
            )
        }
    }

    fn mark_stat_for_redraw(&mut self, stat: StatusStat) {
        let bit = match stat {
            StatusStat::Strength => 0x0001,
            StatusStat::Constitution => 0x0004,
        };
        unsafe { print_stat |= bit }
    }
}

#[cfg(not(test))]
pub(super) fn sleep_gas() {
    crate::rng::with_active_rng(|rng| sleep_gas_with_rng(rng, &mut GlobalStatus));
}

#[cfg(not(test))]
pub(super) fn strength_dart(damage: i64) {
    crate::rng::with_active_rng(|rng| strength_dart_with_rng(rng, &mut GlobalStatus, damage));
}

#[cfg(not(test))]
pub(super) fn blind_gas() {
    crate::rng::with_active_rng(|rng| blind_gas_with_rng(rng, &mut GlobalStatus));
}

#[cfg(not(test))]
pub(super) fn confuse_gas() {
    crate::rng::with_active_rng(|rng| confuse_gas_with_rng(rng, &mut GlobalStatus));
}

#[cfg(not(test))]
pub(super) fn slow_dart(damage: i64) {
    crate::rng::with_active_rng(|rng| slow_dart_with_rng(rng, &mut GlobalStatus, damage));
}

#[cfg(not(test))]
pub(super) fn constitution_dart(damage: i64) {
    crate::rng::with_active_rng(|rng| constitution_dart_with_rng(rng, &mut GlobalStatus, damage));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, StdRng};

    #[derive(Default)]
    struct Effects {
        paralysis: i64,
        blind: i64,
        confused: i64,
        slow: i64,
        free_action: bool,
        armor_class: i64,
        sustain_strength: bool,
        sustain_constitution: bool,
        events: Vec<String>,
    }

    impl StatusEffects for Effects {
        fn damage(&mut self, damage: i64, cause: &str) {
            self.events.push(format!("damage {damage}: {cause}"));
        }

        fn message(&mut self, message: &'static str) {
            self.events.push(message.into());
        }
        fn paralysis(&self) -> i64 {
            self.paralysis
        }

        fn free_action(&self) -> bool {
            self.free_action
        }

        fn add_paralysis(&mut self, amount: i64) {
            self.paralysis += amount;
        }

        fn add_blindness(&mut self, amount: i64) {
            self.blind += amount;
        }

        fn add_confusion(&mut self, amount: i64) {
            self.confused += amount;
        }

        fn add_slow(&mut self, amount: i64) {
            self.slow += amount;
        }

        fn armor_class(&self) -> i64 {
            self.armor_class
        }

        fn lose_stat(&mut self, stat: StatusStat) -> bool {
            let sustained = match stat {
                StatusStat::Strength => self.sustain_strength,
                StatusStat::Constitution => self.sustain_constitution,
            };
            self.events.push(format!("lose stat: {stat:?}"));
            if sustained {
                self.events.push("A small dart hits you.".into());
            }
            true
        }

        fn mark_stat_for_redraw(&mut self, stat: StatusStat) {
            self.events.push(format!("redraw stat: {stat:?}"));
        }
    }

    fn seeded_rng() -> StdRng {
        StdRng::from_seed(&[1, 2, 3, 4][..])
    }

    #[test]
    fn sleep_gas_with_free_action_does_not_paralyze() {
        let mut rng = seeded_rng();
        let mut effects = Effects {
            free_action: true,
            ..Effects::default()
        };

        sleep_gas_with_rng(&mut rng, &mut effects);

        assert_eq!(effects.paralysis, 0, "free action prevents paralysis");
        assert_eq!(
            effects.events,
            ["A strange white mist surrounds you!", "You are unaffected."]
        );
    }

    #[test]
    fn sleep_gas_skips_players_already_paralyzed() {
        let mut rng = seeded_rng();
        let mut effects = Effects {
            paralysis: 3,
            ..Effects::default()
        };

        sleep_gas_with_rng(&mut rng, &mut effects);

        assert_eq!(effects.paralysis, 3);
        assert!(effects.events.is_empty());
    }

    #[test]
    fn sleep_gas_adds_its_legacy_duration() {
        let mut rng = seeded_rng();
        let mut effects = Effects::default();

        sleep_gas_with_rng(&mut rng, &mut effects);

        assert!((5..=14).contains(&effects.paralysis));
        assert_eq!(
            effects.events,
            ["A strange white mist surrounds you!", "You fall asleep."]
        );
    }

    #[test]
    fn blind_gas_adds_its_legacy_duration() {
        let mut rng = seeded_rng();
        let mut effects = Effects::default();

        blind_gas_with_rng(&mut rng, &mut effects);

        assert!((51..=100).contains(&effects.blind));
        assert_eq!(effects.events, ["A black gas surrounds you!"]);
    }

    #[test]
    fn confuse_gas_adds_its_legacy_duration() {
        let mut rng = seeded_rng();
        let mut effects = Effects::default();

        confuse_gas_with_rng(&mut rng, &mut effects);

        assert!((16..=30).contains(&effects.confused));
        assert_eq!(
            effects.events,
            ["A gas of scintillating colors surrounds you!"]
        );
    }

    #[test]
    fn slow_dart_hit_damages_then_applies_slowing() {
        let mut rng = seeded_rng();
        let mut effects = Effects::default();

        slow_dart_with_rng(&mut rng, &mut effects, 23);

        assert_eq!(
            effects.events,
            ["damage 23: a dart trap", "A small dart hits you!"]
        );
        assert!((11..=30).contains(&effects.slow));
    }

    #[test]
    fn slow_dart_miss_has_no_damage_or_status_effect() {
        let mut rng = seeded_rng();
        let mut effects = Effects {
            armor_class: i64::MAX,
            ..Effects::default()
        };

        slow_dart_with_rng(&mut rng, &mut effects, 23);

        assert_eq!(effects.slow, 0);
        assert_eq!(effects.events, ["A small dart barely misses you."]);
    }

    #[test]
    fn strength_dart_preserves_sustain_message_and_applies_damage() {
        let mut rng = seeded_rng();
        let mut effects = Effects {
            sustain_strength: true,
            ..Effects::default()
        };

        strength_dart_with_rng(&mut rng, &mut effects, 23);

        assert_eq!(
            effects.events,
            [
                "lose stat: Strength",
                "A small dart hits you.",
                "damage 23: a dart trap",
                "redraw stat: Strength",
                "A small dart weakens you!"
            ]
        );
    }

    #[test]
    fn strength_dart_without_sustain_does_not_emit_the_sustain_message() {
        let mut rng = seeded_rng();
        let mut effects = Effects::default();

        strength_dart_with_rng(&mut rng, &mut effects, 23);

        assert_eq!(
            effects.events,
            [
                "lose stat: Strength",
                "damage 23: a dart trap",
                "redraw stat: Strength",
                "A small dart weakens you!"
            ]
        );
    }

    #[test]
    fn constitution_dart_marks_the_constitution_stat_for_redraw() {
        let mut rng = seeded_rng();
        let mut effects = Effects {
            sustain_constitution: true,
            ..Effects::default()
        };

        constitution_dart_with_rng(&mut rng, &mut effects, 23);

        assert_eq!(
            effects.events,
            [
                "lose stat: Constitution",
                "A small dart hits you.",
                "damage 23: a dart trap",
                "redraw stat: Constitution",
                "A small dart weakens you!"
            ]
        );
    }

    #[test]
    fn constitution_dart_without_sustain_does_not_emit_the_sustain_message() {
        let mut rng = seeded_rng();
        let mut effects = Effects::default();

        constitution_dart_with_rng(&mut rng, &mut effects, 23);

        assert_eq!(
            effects.events,
            [
                "lose stat: Constitution",
                "damage 23: a dart trap",
                "redraw stat: Constitution",
                "A small dart weakens you!"
            ]
        );
    }
}
