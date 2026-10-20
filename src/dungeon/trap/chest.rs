use crate::{
    model::ChestFlags1,
    rng::{rand_rep_with_rng, randint_with_rng},
};
use rand::Rng;

#[derive(Debug, Default, PartialEq, Eq)]
struct ChestTraps {
    strength_needle: bool,
    poison_needle: bool,
    paralysis_gas: bool,
    explosion: bool,
    summoning_runes: bool,
}

impl From<u64> for ChestTraps {
    fn from(flags: u64) -> Self {
        Self {
            strength_needle: flags & ChestFlags1::PoisonNeedle1 as u64 != 0,
            poison_needle: flags & ChestFlags1::PoisonNeedle2 as u64 != 0,
            paralysis_gas: flags & ChestFlags1::GasTrap as u64 != 0,
            explosion: flags & ChestFlags1::ExplosiveTrap as u64 != 0,
            summoning_runes: flags & ChestFlags1::SummoningRunes as u64 != 0,
        }
    }
}

trait ChestTrapEffects {
    fn message(&mut self, message: &'static str);
    fn lose_strength(&mut self) -> bool;
    fn damage(&mut self, amount: i64, cause: &'static str);
    fn mark_strength_for_redraw(&mut self);
    fn free_action(&self) -> bool;
    fn add_poisoned(&mut self, amount: i64);
    fn set_paralysis(&mut self, amount: i64);
    fn delete_chest(&mut self, y: i64, x: i64);
    fn is_water(&self, y: i64, x: i64) -> bool;
    fn summon_monster(&mut self, y: i64, x: i64, water: bool);
}

fn trigger_chest_trap_with_rng<R: Rng + ?Sized>(
    rng: &mut R,
    traps: ChestTraps,
    y: i64,
    x: i64,
    effects: &mut impl ChestTrapEffects,
) {
    if traps.strength_needle {
        effects.message("A small needle has pricked you!");
        if effects.lose_strength() {
            effects.damage(rand_rep_with_rng(rng, 1, 4), "a poison needle");
            effects.mark_strength_for_redraw();
            effects.message("You feel weakened!");
        }
    }

    if traps.poison_needle {
        effects.message("A small needle has pricked you!");
        effects.damage(rand_rep_with_rng(rng, 1, 6), "a poison needle.");
        effects.add_poisoned(10 + randint_with_rng(rng, 20));
    }

    if traps.paralysis_gas {
        effects.message("A puff of yellow gas surrounds you!");
        if effects.free_action() {
            effects.message("You are unaffected.");
        } else {
            effects.message("You choke and pass out.");
            effects.set_paralysis(10 + randint_with_rng(rng, 20));
        }
    }

    if traps.explosion {
        effects.message("There is a sudden explosion!");
        effects.delete_chest(y, x);
        effects.damage(rand_rep_with_rng(rng, 5, 8), "an exploding chest");
    }

    if traps.summoning_runes {
        for _ in 0..3 {
            effects.summon_monster(y, x, effects.is_water(y, x));
        }
    }
}

#[cfg(not(test))]
struct GlobalChestTrap;

#[cfg(not(test))]
impl ChestTrapEffects for GlobalChestTrap {
    fn message(&mut self, message: &'static str) {
        crate::term::msg_print(message);
    }

    fn lose_strength(&mut self) -> bool {
        unsafe {
            lose_stat(
                0,
                b"\0".as_ptr().cast(),
                b"You are unaffected.\0".as_ptr().cast(),
            )
        }
    }

    fn damage(&mut self, amount: i64, cause: &'static str) {
        let cause = std::ffi::CString::new(cause).expect("static trap cause has no NUL bytes");
        unsafe { crate::player::damage::take_hit(amount, cause.as_ptr()) }
    }

    fn mark_strength_for_redraw(&mut self) {
        unsafe { print_stat = 1 }
    }

    fn free_action(&self) -> bool {
        unsafe { player_flags.free_act != 0 }
    }

    fn add_poisoned(&mut self, amount: i64) {
        unsafe { player_flags.poisoned += amount }
    }

    fn set_paralysis(&mut self, amount: i64) {
        unsafe { player_flags.paralysis = amount }
    }

    fn delete_chest(&mut self, y: i64, x: i64) {
        unsafe { delete_object(y as libc::c_long, x as libc::c_long) };
    }

    fn is_water(&self, y: i64, x: i64) -> bool {
        let floor = unsafe { (&*std::ptr::addr_of!(cave))[y as usize][x as usize].fval };
        unsafe {
            crate::pascal::is_in(
                floor as libc::c_long,
                std::ptr::addr_of!(water_set).cast::<u8>(),
            )
        }
    }

    fn summon_monster(&mut self, y: i64, x: i64, water: bool) {
        let mut monster_y = y;
        let mut monster_x = x;
        unsafe {
            if water {
                summon_water_monster(&mut monster_y, &mut monster_x, false);
            } else {
                summon_land_monster(&mut monster_y, &mut monster_x, false);
            }
        }
    }
}

#[cfg(not(test))]
extern "C" {
    static mut cave:
        [[crate::model::Cave; crate::constants::MAX_WIDTH + 1]; crate::constants::MAX_HEIGHT + 1];
    static mut player_flags: crate::model::PlayerFlags;
    static mut print_stat: libc::c_ulong;
    static water_set: [u8; 25];

    fn lose_stat(
        stat: libc::c_int,
        message: *const libc::c_char,
        sustain_message: *const libc::c_char,
    ) -> bool;
    fn delete_object(y: libc::c_long, x: libc::c_long) -> bool;
    fn summon_land_monster(y: *mut i64, x: *mut i64, asleep: bool) -> bool;
    fn summon_water_monster(y: *mut i64, x: *mut i64, asleep: bool) -> bool;
}

#[cfg(not(test))]
fn coordinate_as_i64<T: Into<i64>>(coordinate: T) -> i64 {
    coordinate.into()
}

#[cfg(not(test))]
/// # Safety
/// C globals must be initialized and exclusively accessed on the game thread.
pub(super) unsafe fn trigger_trap_global(y: libc::c_long, x: libc::c_long) {
    let flags = crate::dungeon::cell::with_global_cells(|cells| {
        let (cell, _) = cells.read(coordinate_as_i64(y), coordinate_as_i64(x))?;
        cells.items.get(cell.tptr as usize).map(|item| item.flags)
    });
    let Some(flags) = flags else {
        crate::debug::error(format!("trigger_trap: invalid chest cell ({y}, {x})"));
        return;
    };
    crate::rng::with_active_rng(|rng| {
        trigger_chest_trap_with_rng(
            rng,
            ChestTraps::from(flags),
            coordinate_as_i64(y),
            coordinate_as_i64(x),
            &mut GlobalChestTrap,
        )
    });
}

#[cfg(test)]
mod tests {
    use super::{trigger_chest_trap_with_rng, ChestTrapEffects, ChestTraps};
    use rand::{SeedableRng, StdRng};

    #[derive(Default)]
    struct Effects {
        events: Vec<&'static str>,
        messages: Vec<&'static str>,
        damage: Vec<(i64, &'static str)>,
        strength_loss_succeeds: bool,
        free_action: bool,
        poisoned: i64,
        paralysis: i64,
        print_stat: i64,
        chest_deleted: bool,
        water: bool,
        summons: usize,
    }

    impl ChestTrapEffects for Effects {
        fn message(&mut self, message: &'static str) {
            self.events.push("message");
            self.messages.push(message);
        }

        fn lose_strength(&mut self) -> bool {
            self.events.push("lose_strength");
            self.strength_loss_succeeds
        }

        fn damage(&mut self, amount: i64, cause: &'static str) {
            self.events.push("damage");
            self.damage.push((amount, cause));
        }

        fn mark_strength_for_redraw(&mut self) {
            self.events.push("mark_strength");
            self.print_stat = 1;
        }

        fn free_action(&self) -> bool {
            self.free_action
        }

        fn add_poisoned(&mut self, amount: i64) {
            self.events.push("poison");
            self.poisoned += amount;
        }

        fn set_paralysis(&mut self, amount: i64) {
            self.events.push("paralysis");
            self.paralysis = amount;
        }

        fn delete_chest(&mut self, _y: i64, _x: i64) {
            self.events.push("delete");
            self.chest_deleted = true;
        }

        fn is_water(&self, _y: i64, _x: i64) -> bool {
            self.water
        }

        fn summon_monster(&mut self, _y: i64, _x: i64, water: bool) {
            self.events
                .push(if water { "water_summon" } else { "land_summon" });
            self.summons += 1;
        }
    }

    fn run(flags: u64, effects: &mut Effects) {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng = StdRng::from_seed(seed);
        trigger_chest_trap_with_rng(&mut rng, ChestTraps::from(flags), 7, 9, effects);
    }

    #[test]
    fn chest_traps_decode_each_legacy_flag_and_ignore_unrelated_bits() {
        let cases = [
            (
                0x10,
                ChestTraps {
                    strength_needle: true,
                    ..ChestTraps::default()
                },
            ),
            (
                0x20,
                ChestTraps {
                    poison_needle: true,
                    ..ChestTraps::default()
                },
            ),
            (
                0x40,
                ChestTraps {
                    paralysis_gas: true,
                    ..ChestTraps::default()
                },
            ),
            (
                0x80,
                ChestTraps {
                    explosion: true,
                    ..ChestTraps::default()
                },
            ),
            (
                0x100,
                ChestTraps {
                    summoning_runes: true,
                    ..ChestTraps::default()
                },
            ),
        ];

        for (flag, expected) in cases {
            assert_eq!(ChestTraps::from(flag), expected);
            assert_eq!(ChestTraps::from(flag | !0x1f0), expected);
        }
        assert_eq!(ChestTraps::from(0), ChestTraps::default());
        assert_eq!(ChestTraps::from(!0x1f0), ChestTraps::default());
    }

    #[test]
    fn chest_traps_decode_combined_legacy_flags() {
        assert_eq!(
            ChestTraps::from(0x1f0),
            ChestTraps {
                strength_needle: true,
                poison_needle: true,
                paralysis_gas: true,
                explosion: true,
                summoning_runes: true,
            }
        );
    }

    #[test]
    fn strength_needle_applies_damage_and_redraw_when_stat_loss_succeeds() {
        let mut effects = Effects {
            strength_loss_succeeds: true,
            ..Effects::default()
        };

        run(0x10, &mut effects);

        assert_eq!(
            effects.messages,
            ["A small needle has pricked you!", "You feel weakened!"]
        );
        assert_eq!(effects.damage.len(), 1);
        assert!((1..=4).contains(&effects.damage[0].0));
        assert_eq!(effects.damage[0].1, "a poison needle");
        assert_eq!(effects.print_stat, 1);
    }

    #[test]
    fn strength_needle_does_not_damage_when_stat_loss_fails() {
        let mut effects = Effects::default();

        run(0x10, &mut effects);

        assert_eq!(effects.messages, ["A small needle has pricked you!"]);
        assert!(effects.damage.is_empty());
        assert_eq!(effects.print_stat, 0);
    }

    #[test]
    fn poison_needle_adds_damage_and_poison() {
        let mut effects = Effects::default();

        run(0x20, &mut effects);

        assert_eq!(effects.messages, ["A small needle has pricked you!"]);
        assert_eq!(effects.damage.len(), 1);
        assert!((1..=6).contains(&effects.damage[0].0));
        assert_eq!(effects.damage[0].1, "a poison needle.");
        assert!((11..=30).contains(&effects.poisoned));
    }

    #[test]
    fn paralysis_gas_respects_free_action() {
        let mut effects = Effects {
            free_action: true,
            ..Effects::default()
        };

        run(0x40, &mut effects);

        assert_eq!(
            effects.messages,
            ["A puff of yellow gas surrounds you!", "You are unaffected."]
        );
        assert_eq!(effects.paralysis, 0);
    }

    #[test]
    fn paralysis_gas_sets_duration_without_free_action() {
        let mut effects = Effects {
            paralysis: 3,
            ..Effects::default()
        };

        run(0x40, &mut effects);

        assert_eq!(
            effects.messages,
            [
                "A puff of yellow gas surrounds you!",
                "You choke and pass out."
            ]
        );
        assert!((11..=30).contains(&effects.paralysis));
    }

    #[test]
    fn explosive_trap_deletes_chest_before_damage() {
        let mut effects = Effects::default();

        run(0x80, &mut effects);

        assert_eq!(effects.messages, ["There is a sudden explosion!"]);
        assert!(effects.chest_deleted);
        assert_eq!(effects.damage.len(), 1);
        assert!((5..=40).contains(&effects.damage[0].0));
        assert_eq!(effects.damage[0].1, "an exploding chest");
        assert_eq!(effects.events, ["message", "delete", "damage"]);
    }

    #[test]
    fn summoning_runes_make_three_summon_attempts_for_terrain() {
        let mut effects = Effects {
            water: true,
            ..Effects::default()
        };

        run(0x100, &mut effects);

        assert_eq!(effects.summons, 3);
        assert_eq!(
            effects.events,
            ["water_summon", "water_summon", "water_summon"]
        );
    }

    #[test]
    fn combined_flags_apply_effects_in_c_order() {
        let mut effects = Effects {
            strength_loss_succeeds: true,
            ..Effects::default()
        };

        run(0x1f0, &mut effects);

        assert_eq!(
            effects.messages,
            [
                "A small needle has pricked you!",
                "You feel weakened!",
                "A small needle has pricked you!",
                "A puff of yellow gas surrounds you!",
                "You choke and pass out.",
                "There is a sudden explosion!"
            ]
        );
        assert_eq!(
            effects.events,
            [
                "message",
                "lose_strength",
                "damage",
                "mark_strength",
                "message",
                "message",
                "damage",
                "poison",
                "message",
                "message",
                "paralysis",
                "message",
                "delete",
                "damage",
                "land_summon",
                "land_summon",
                "land_summon"
            ]
        );
        assert_eq!(effects.damage.len(), 3);
        assert!((1..=4).contains(&effects.damage[0].0));
        assert!((1..=6).contains(&effects.damage[1].0));
        assert!((5..=40).contains(&effects.damage[2].0));
        assert!((11..=30).contains(&effects.poisoned));
        assert!((11..=30).contains(&effects.paralysis));
        assert!(effects.chest_deleted);
        assert_eq!(effects.summons, 3);
    }
}
