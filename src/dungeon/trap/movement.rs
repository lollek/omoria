//! Movement and dungeon-level trap effects.

use crate::rng::randint_with_rng;
use rand::Rng;

trait MovementEffects {
    fn feather_fall(&self) -> bool;
    fn message(&mut self, message: &'static str);
    fn damage(&mut self, damage: i64, cause: &str);
    fn place_open_pit(&mut self, y: i64, x: i64);
    fn leave_level(&mut self);
    fn descend(&mut self, levels: i64);
    fn request_teleport(&mut self);
}

fn covered_pit_with_effects(effects: &mut impl MovementEffects, damage: i64, y: i64, x: i64) {
    effects.message("You fell into a covered pit.");
    if effects.feather_fall() {
        effects.message("You gently float down.");
    } else {
        effects.damage(damage, "a covered pit");
        effects.place_open_pit(y, x);
    }
}

fn trap_door_with_effects(effects: &mut impl MovementEffects, damage: i64) {
    effects.message("You fell through a trap door!");
    effects.message(" ");
    effects.leave_level();
    effects.descend(1);
    if effects.feather_fall() {
        effects.message("You gently float down.");
    } else {
        effects.damage(damage, "a trap door");
    }
}

fn teleport_with_effects(effects: &mut impl MovementEffects) {
    effects.request_teleport();
    effects.message("You hit a teleport trap!");
}

fn chute_with_rng<R: Rng + ?Sized>(rng: &mut R, effects: &mut impl MovementEffects, damage: i64) {
    effects.message("You fell down a chute!");
    effects.message(" ");
    effects.leave_level();
    effects.descend(randint_with_rng(rng, 6));
    if effects.feather_fall() {
        effects.message("You gently slide down.");
    } else {
        effects.damage(damage, "chute landing");
    }
}

fn whirlpool_with_rng<R: Rng + ?Sized>(
    rng: &mut R,
    effects: &mut impl MovementEffects,
    damage: i64,
) {
    effects.message("You are swept into a whirlpool!");
    effects.message(" ");
    effects.leave_level();
    loop {
        effects.descend(1);
        if !effects.feather_fall() {
            effects.message("You are drowning!");
            effects.damage(damage, "drowning");
        }
        if randint_with_rng(rng, 2) == 1 {
            break;
        }
    }
}

#[cfg(not(test))]
extern "C" {
    static mut player_flags: crate::model::PlayerFlags;
    static mut dun_level: libc::c_long;
    static mut moria_flag: bool;
    static mut teleport_flag: bool;
}

#[cfg(not(test))]
struct GlobalMovement;

#[cfg(not(test))]
impl MovementEffects for GlobalMovement {
    fn feather_fall(&self) -> bool {
        unsafe { player_flags.ffall != 0 }
    }

    fn message(&mut self, message: &'static str) {
        crate::term::msg_print(message);
    }

    fn damage(&mut self, damage: i64, cause: &str) {
        let cause = std::ffi::CString::new(cause).expect("trap damage causes contain no NUL bytes");
        unsafe { crate::player::damage::take_hit(damage, cause.as_ptr()) }
    }

    fn place_open_pit(&mut self, y: i64, x: i64) {
        super::interop::place_trap(y, x, 2, 1);
    }

    fn leave_level(&mut self) {
        unsafe { moria_flag = true }
    }

    fn descend(&mut self, levels: i64) {
        unsafe { dun_level += levels }
    }

    fn request_teleport(&mut self) {
        unsafe { teleport_flag = true }
    }
}

#[cfg(not(test))]
pub(super) fn covered_pit(damage: i64, y: i64, x: i64) {
    covered_pit_with_effects(&mut GlobalMovement, damage, y, x);
}

#[cfg(not(test))]
pub(super) fn trap_door(damage: i64) {
    trap_door_with_effects(&mut GlobalMovement, damage);
}

#[cfg(not(test))]
pub(super) fn teleport() {
    teleport_with_effects(&mut GlobalMovement);
}

#[cfg(not(test))]
pub(super) fn chute(damage: i64) {
    crate::rng::with_active_rng(|rng| chute_with_rng(rng, &mut GlobalMovement, damage));
}

#[cfg(not(test))]
pub(super) fn whirlpool(damage: i64) {
    crate::rng::with_active_rng(|rng| whirlpool_with_rng(rng, &mut GlobalMovement, damage));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, StdRng};
    use std::{cell::RefCell, rc::Rc};

    #[derive(Default)]
    struct Recorder {
        feather_fall: bool,
        level: i64,
        moria_flag: bool,
        teleport_flag: bool,
        events: Rc<RefCell<Vec<String>>>,
    }

    impl MovementEffects for Recorder {
        fn feather_fall(&self) -> bool {
            self.feather_fall
        }

        fn message(&mut self, message: &'static str) {
            self.events.borrow_mut().push(message.into());
        }

        fn damage(&mut self, damage: i64, cause: &str) {
            self.events
                .borrow_mut()
                .push(format!("damage {damage}: {cause}"));
        }

        fn place_open_pit(&mut self, y: i64, x: i64) {
            self.events.borrow_mut().push(format!("pit {y} {x}"));
        }

        fn leave_level(&mut self) {
            self.moria_flag = true;
            self.events.borrow_mut().push("leave level".into());
        }

        fn descend(&mut self, levels: i64) {
            self.level += levels;
            self.events.borrow_mut().push(format!("descend {levels}"));
        }

        fn request_teleport(&mut self) {
            self.teleport_flag = true;
            self.events.borrow_mut().push("teleport".into());
        }
    }

    #[test]
    fn covered_pit_is_replaced_at_trap_coordinates_after_damage() {
        let mut effects = Recorder::default();
        covered_pit_with_effects(&mut effects, 23, 7, 9);
        assert_eq!(
            *effects.events.borrow(),
            [
                "You fell into a covered pit.",
                "damage 23: a covered pit",
                "pit 7 9"
            ]
        );
        assert_eq!(effects.level, 0);
        assert!(!effects.moria_flag && !effects.teleport_flag);
    }

    #[test]
    fn feather_fall_prevents_covered_pit_damage_and_replacement() {
        let mut effects = Recorder {
            feather_fall: true,
            ..Recorder::default()
        };
        covered_pit_with_effects(&mut effects, 23, 7, 9);
        assert_eq!(
            *effects.events.borrow(),
            ["You fell into a covered pit.", "You gently float down."]
        );
        assert_eq!(effects.level, 0);
        assert!(!effects.moria_flag && !effects.teleport_flag);
    }

    #[test]
    fn trap_door_leaves_level_and_descends_before_damage_or_feather_fall() {
        for feather_fall in [false, true] {
            let mut effects = Recorder {
                feather_fall,
                level: 12,
                ..Recorder::default()
            };
            trap_door_with_effects(&mut effects, 23);
            assert_eq!(effects.level, 13);
            assert!(effects.moria_flag && !effects.teleport_flag);
            assert_eq!(
                *effects.events.borrow(),
                [
                    "You fell through a trap door!",
                    " ",
                    "leave level",
                    "descend 1",
                    if feather_fall {
                        "You gently float down."
                    } else {
                        "damage 23: a trap door"
                    }
                ]
            );
        }
    }

    #[test]
    fn teleport_sets_only_teleport_flag_before_message() {
        for feather_fall in [false, true] {
            let mut effects = Recorder {
                feather_fall,
                level: 12,
                ..Recorder::default()
            };
            teleport_with_effects(&mut effects);
            assert!(effects.teleport_flag && !effects.moria_flag);
            assert_eq!(effects.level, 12);
            assert_eq!(
                *effects.events.borrow(),
                ["teleport", "You hit a teleport trap!"]
            );
        }
    }

    #[test]
    fn chute_descends_one_d6_before_damage_or_feather_fall() {
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut expected_rng = rng;
        let mut depths = Vec::new();
        for feather_fall in [false, true] {
            for _ in 0..64 {
                let levels = randint_with_rng(&mut expected_rng, 6);
                depths.push(levels);
                let mut effects = Recorder {
                    feather_fall,
                    level: 12,
                    ..Recorder::default()
                };
                chute_with_rng(&mut rng, &mut effects, 23);
                assert_eq!(effects.level, 12 + levels);
                assert!(effects.moria_flag && !effects.teleport_flag);
                assert_eq!(
                    *effects.events.borrow(),
                    [
                        "You fell down a chute!".to_string(),
                        " ".into(),
                        "leave level".into(),
                        format!("descend {levels}"),
                        if feather_fall {
                            "You gently slide down.".into()
                        } else {
                            "damage 23: chute landing".into()
                        }
                    ]
                );
            }
        }
        assert!(
            depths.contains(&1) && depths.contains(&6),
            "exercise both d6 boundaries"
        );
        assert_eq!(
            rng.next_u64(),
            expected_rng.next_u64(),
            "consume exactly one roll per chute"
        );
    }

    #[test]
    fn whirlpool_descends_and_damages_before_each_continuation_roll() {
        struct ObservedRng<'a> {
            rng: &'a mut StdRng,
            events: Rc<RefCell<Vec<String>>>,
        }

        impl Rng for ObservedRng<'_> {
            fn next_u32(&mut self) -> u32 {
                self.events.borrow_mut().push("roll".into());
                self.rng.next_u32()
            }

            fn next_u64(&mut self) -> u64 {
                self.events.borrow_mut().push("roll".into());
                self.rng.next_u64()
            }
        }

        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut expected_rng = rng;
        let mut saw_single = false;
        let mut saw_multiple = false;
        for feather_fall in [false, true] {
            for _ in 0..32 {
                let mut levels = 1;
                while randint_with_rng(&mut expected_rng, 2) != 1 {
                    levels += 1;
                }
                saw_single |= levels == 1;
                saw_multiple |= levels > 1;
                let mut effects = Recorder {
                    feather_fall,
                    level: 12,
                    ..Recorder::default()
                };
                let mut observed_rng = ObservedRng {
                    rng: &mut rng,
                    events: Rc::clone(&effects.events),
                };
                whirlpool_with_rng(&mut observed_rng, &mut effects, 23);
                assert_eq!(effects.level, 12 + levels);
                assert!(effects.moria_flag && !effects.teleport_flag);
                let mut expected = vec!["You are swept into a whirlpool!", " ", "leave level"];
                for _ in 0..levels {
                    expected.extend_from_slice(if feather_fall {
                        &["descend 1", "roll"]
                    } else {
                        &[
                            "descend 1",
                            "You are drowning!",
                            "damage 23: drowning",
                            "roll",
                        ]
                    });
                }
                assert_eq!(*effects.events.borrow(), expected);
            }
        }
        assert!(
            saw_single && saw_multiple,
            "exercise first-roll exit and repeated descent"
        );
        assert_eq!(
            rng.next_u64(),
            expected_rng.next_u64(),
            "roll once after every descent, even with feather fall"
        );
    }
}
